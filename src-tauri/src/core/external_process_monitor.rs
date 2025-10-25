//! External process monitoring and log attachment.
//!
//! This module allows attaching to processes started outside of Sentinel
//! to monitor their logs without managing their lifecycle.

use crate::error::{Result, SentinelError};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use tauri::{AppHandle, Emitter};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;

/// Information about an attached external process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessAttachment {
    pub pid: u32,
    pub port: Option<u16>,
    pub name: String,
    pub command: String,
    pub log_source: LogSource,
}

/// Where to get logs from for this process
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LogSource {
    /// Read from a log file
    File { path: String },

    /// Read from Docker container logs
    DockerLogs { container_id: String },

    /// Capture stdout/stderr using dtrace/dtruss (macOS)
    DTrace { pid: u32 },

    /// Cannot auto-detect - show instructions to user
    Manual { instructions: String },
}

/// Event emitted for each log line from external process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogLineEvent {
    pub attachment_id: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub line: String,
    pub stream: String,
}

/// Manager for external process attachments
pub struct ExternalProcessMonitor {
    /// Map of attachment_id -> running task handle
    attachments: Arc<Mutex<HashMap<String, tokio::task::JoinHandle<()>>>>,
}

impl ExternalProcessMonitor {
    pub fn new() -> Self {
        Self {
            attachments: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Attach to an external process for log monitoring
    pub async fn attach_to_process(
        &self,
        pid: u32,
        port: Option<u16>,
    ) -> Result<ProcessAttachment> {
        // Verify process exists
        let mut sys = System::new_all();
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::everything(),
        );

        let sysinfo_pid = Pid::from_u32(pid);
        let process = sys
            .process(sysinfo_pid)
            .ok_or_else(|| SentinelError::ProcessNotFound {
                name: pid.to_string(),
            })?;

        let name = process.name().to_string_lossy().to_string();
        let command = process
            .cmd()
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        // Detect log source
        let log_source = self.detect_log_source(pid, port, &name, &command).await?;

        Ok(ProcessAttachment {
            pid,
            port,
            name,
            command,
            log_source,
        })
    }

    /// Detect where logs are coming from
    async fn detect_log_source(
        &self,
        pid: u32,
        port: Option<u16>,
        process_name: &str,
        command: &str,
    ) -> Result<LogSource> {
        // 1. Check if it's a Docker container
        if let Some(port) = port {
            if let Ok(Some(container_id)) = self.get_docker_container_by_port(port).await {
                return Ok(LogSource::DockerLogs { container_id });
            }
        }

        // 2. Check for explicit log file in command line
        if let Some(log_path) = self.extract_log_file_from_cmd(command) {
            if log_path.exists() {
                return Ok(LogSource::File {
                    path: log_path.to_string_lossy().to_string(),
                });
            }
        }

        // 3. Check common log locations based on process type
        if let Some(log_path) = self.check_common_log_paths(process_name, pid, port) {
            if log_path.exists() {
                tracing::info!("Auto-detected log file: {}", log_path.display());
                return Ok(LogSource::File {
                    path: log_path.to_string_lossy().to_string(),
                });
            }
        }

        // 4. Try dtrace (macOS only) - capture stdout/stderr directly
        #[cfg(target_os = "macos")]
        {
            // Always try dtrace - it will show instructions if permissions are needed
            tracing::info!(
                "Will attempt to use dtrace to capture stdout/stderr for PID {}",
                pid
            );
            Ok(LogSource::DTrace { pid })
        }

        // 5. Fallback: Provide instructions (non-macOS or if all detection methods failed)
        #[cfg(not(target_os = "macos"))]
        {
            Ok(LogSource::Manual {
                instructions: format!(
                    "Cannot auto-detect logs for {} (PID: {}).\n\n\
                     To monitor logs:\n\
                     1. Check if the process writes to a log file\n\
                     2. Restart the process with output redirection:\n\
                        command > output.log 2>&1\n\
                     3. Use 'Attach Log File' to manually select the log file\n\n\
                     Command: {}",
                    process_name, pid, command
                ),
            })
        }
    }

    /// Tail a log file and stream to frontend
    pub async fn tail_log_file(&self, path: String, app: AppHandle) -> Result<String> {
        let path_buf = PathBuf::from(&path);

        if !path_buf.exists() {
            return Err(SentinelError::Other(format!(
                "Log file does not exist: {}",
                path
            )));
        }

        let file = File::open(&path_buf)
            .await
            .map_err(|e| SentinelError::Other(format!("Failed to open log file: {}", e)))?;

        // Generate unique attachment ID
        let attachment_id = uuid::Uuid::new_v4().to_string();
        let attachment_id_clone = attachment_id.clone();

        // Spawn task to stream lines
        let handle = tokio::spawn(async move {
            let mut reader = BufReader::new(file);
            let mut line = String::new();

            // First, read and emit all existing content
            tracing::info!("Reading existing log content from file");
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        // Reached EOF, now start tailing for new content
                        tracing::info!("Reached EOF, starting tail mode");
                        break;
                    }
                    Ok(_) => {
                        let timestamp = Utc::now();
                        let _ = app.emit(
                            "log-line",
                            &LogLineEvent {
                                attachment_id: attachment_id_clone.clone(),
                                timestamp,
                                line: line.trim_end().to_string(),
                                stream: "file".to_string(),
                            },
                        );
                    }
                    Err(e) => {
                        tracing::error!("Error reading initial log content: {}", e);
                        return;
                    }
                }
            }

            // Now tail for new content
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        // EOF - wait a bit and try again (tailing behavior)
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        continue;
                    }
                    Ok(_) => {
                        let timestamp = Utc::now();
                        let _ = app.emit(
                            "log-line",
                            &LogLineEvent {
                                attachment_id: attachment_id_clone.clone(),
                                timestamp,
                                line: line.trim_end().to_string(),
                                stream: "file".to_string(),
                            },
                        );
                    }
                    Err(e) => {
                        tracing::error!("Error reading log file: {}", e);
                        break;
                    }
                }
            }
        });

        // Store the handle so we can cancel it later
        self.attachments
            .lock()
            .await
            .insert(attachment_id.clone(), handle);

        Ok(attachment_id)
    }

    /// Stop tailing a log file
    pub async fn detach(&self, attachment_id: &str) -> Result<()> {
        let mut attachments = self.attachments.lock().await;

        if let Some(handle) = attachments.remove(attachment_id) {
            handle.abort();
            Ok(())
        } else {
            Err(SentinelError::Other(format!(
                "Attachment not found: {}",
                attachment_id
            )))
        }
    }

    /// Capture stdout/stderr using dtrace (macOS only)
    #[cfg(target_os = "macos")]
    pub async fn capture_with_dtrace(&self, pid: u32, app: AppHandle) -> Result<String> {
        // Generate unique attachment ID
        let attachment_id = uuid::Uuid::new_v4().to_string();
        let attachment_id_clone = attachment_id.clone();

        tracing::info!("Starting dtrace capture for PID {}", pid);

        // Show helpful message about SIP limitations and alternatives
        let handle = tokio::spawn(async move {
            let _ = app.emit("log-line", &LogLineEvent {
                attachment_id: attachment_id_clone.clone(),
                timestamp: Utc::now(),
                line: "⚠️  macOS System Integrity Protection (SIP) Blocks Direct Log Capture\n\n\
                      Unfortunately, Sentinel cannot directly capture stdout/stderr from already-running processes \n\
                      because macOS System Integrity Protection blocks the dtrace syscall provider.\n\n\
                      ✅ RECOMMENDED SOLUTION: Use Log Files\n\n\
                      When starting your development server, redirect output to a log file:\n\n\
                      Example for Next.js:\n\
                      pnpm dev > ~/logs/myapp.log 2>&1\n\n\
                      Then you can monitor that log file using Sentinel's log viewer.\n\n\
                      ⚙️  ADVANCED: Enable dtrace (Optional)\n\n\
                      If you really need direct log capture, you can partially disable SIP:\n\
                      1. Restart in Recovery Mode (hold Cmd+R during boot)\n\
                      2. Open Terminal\n\
                      3. Run: csrutil enable --without dtrace\n\
                      4. Restart normally\n\n\
                      Note: This reduces security protections on your Mac.\n\n\
                      📖 Learn more: https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection"
                    .to_string(),
                stream: "info".to_string(),
            });
        });

        // Store the handle
        self.attachments
            .lock()
            .await
            .insert(attachment_id.clone(), handle);

        Ok(attachment_id)
    }

    /// Check if dtrace is available (macOS only)
    #[cfg(target_os = "macos")]
    #[allow(dead_code)]
    async fn check_dtrace_available(&self) -> bool {
        // Try to run a simple dtrace command to check if it's available
        match Command::new("sudo")
            .arg("-n") // Non-interactive (don't prompt for password)
            .arg("dtrace")
            .arg("-V")
            .output()
            .await
        {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    /// Extract log file path from command line arguments
    fn extract_log_file_from_cmd(&self, command: &str) -> Option<PathBuf> {
        // Look for common log file patterns in command
        // --log-file=/path/to/file.log
        // > output.log
        // 2>&1 | tee app.log

        if let Some(idx) = command.find("--log-file=") {
            let rest = &command[idx + 11..];
            if let Some(end) = rest.find(char::is_whitespace) {
                let path = &rest[..end];
                return Some(PathBuf::from(path));
            } else {
                return Some(PathBuf::from(rest));
            }
        }

        if let Some(idx) = command.find("> ") {
            let rest = &command[idx + 2..];
            if let Some(end) = rest.find(char::is_whitespace) {
                let path = &rest[..end];
                return Some(PathBuf::from(path));
            }
        }

        None
    }

    /// Check common log file locations based on process name
    fn check_common_log_paths(
        &self,
        process_name: &str,
        pid: u32,
        _port: Option<u16>,
    ) -> Option<PathBuf> {
        let process_lower = process_name.to_lowercase();

        // Node.js apps
        if process_lower.contains("node") {
            let mut candidates = vec![];

            // Try to get process working directory and search for log files there
            if let Some(cwd) = self.get_process_cwd(pid) {
                // Look for logs directory in the process's working directory
                let logs_dir = cwd.join("logs");
                if logs_dir.exists() {
                    // Search for any .log files in the logs directory
                    if let Ok(entries) = std::fs::read_dir(&logs_dir) {
                        for entry in entries.flatten() {
                            if let Some(ext) = entry.path().extension() {
                                if ext == "log" {
                                    candidates.push(entry.path());
                                }
                            }
                        }
                    }
                }

                // Also check common locations relative to CWD
                candidates.extend(vec![
                    cwd.join("logs").join("app.log"),
                    cwd.join("output.log"),
                    cwd.join("server.log"),
                    cwd.join("app.log"),
                ]);

                // Walk up to find monorepo root (check for workspace indicators)
                let mut search_path = cwd.clone();
                for _ in 0..5 {
                    // Check up to 5 levels
                    if let Some(parent) = search_path.parent() {
                        // Look for monorepo indicators
                        if parent.join("pnpm-workspace.yaml").exists()
                            || parent.join("turbo.json").exists()
                            || parent.join("lerna.json").exists()
                        {
                            // Found monorepo root, check for logs directory there
                            let root_logs = parent.join("logs");
                            if root_logs.exists() {
                                if let Ok(entries) = std::fs::read_dir(&root_logs) {
                                    for entry in entries.flatten() {
                                        if let Some(ext) = entry.path().extension() {
                                            if ext == "log" {
                                                candidates.push(entry.path());
                                            }
                                        }
                                    }
                                }
                            }
                            break;
                        }
                        search_path = parent.to_path_buf();
                    } else {
                        break;
                    }
                }
            } else {
                // Fallback: check current directory
                candidates.extend(vec![
                    PathBuf::from("./logs/app.log"),
                    PathBuf::from("./output.log"),
                    PathBuf::from("./server.log"),
                ]);
            }

            for path in candidates {
                if path.exists() {
                    return Some(path);
                }
            }

            // Check PM2 logs
            if let Ok(home) = std::env::var("HOME") {
                let pm2_out = PathBuf::from(format!("{}/.pm2/logs/app-out.log", home));
                let pm2_err = PathBuf::from(format!("{}/.pm2/logs/app-error.log", home));
                if pm2_out.exists() {
                    return Some(pm2_out);
                }
                if pm2_err.exists() {
                    return Some(pm2_err);
                }
            }
        }

        // Python apps
        if process_lower.contains("python") {
            let candidates = vec![
                PathBuf::from("./logs/app.log"),
                PathBuf::from("./app.log"),
                PathBuf::from("/var/log/app.log"),
            ];

            for path in candidates {
                if path.exists() {
                    return Some(path);
                }
            }
        }

        // Java apps
        if process_lower.contains("java") {
            let candidates = vec![
                PathBuf::from("./logs/spring.log"),
                PathBuf::from("./catalina.out"),
                PathBuf::from("./application.log"),
            ];

            for path in candidates {
                if path.exists() {
                    return Some(path);
                }
            }
        }

        None
    }

    /// Get the current working directory of a process
    fn get_process_cwd(&self, pid: u32) -> Option<PathBuf> {
        let sys = System::new_all();
        if let Some(process) = sys.process(sysinfo::Pid::from_u32(pid)) {
            return process.cwd().map(|p| p.to_path_buf());
        }
        None
    }

    /// Get Docker container ID by port
    async fn get_docker_container_by_port(&self, port: u16) -> Result<Option<String>> {
        // This would integrate with the Docker monitoring feature
        // For now, return None (will implement when integrating with Docker module)
        let _ = port; // Suppress unused warning
        Ok(None)
    }
}

impl Default for ExternalProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_log_file_from_cmd() {
        let monitor = ExternalProcessMonitor::new();

        // Test --log-file= pattern
        let cmd1 = "python app.py --log-file=/var/log/app.log --port 8000";
        assert_eq!(
            monitor.extract_log_file_from_cmd(cmd1),
            Some(PathBuf::from("/var/log/app.log"))
        );

        // Test > redirect pattern
        let cmd2 = "npm run dev > output.log 2>&1";
        assert_eq!(
            monitor.extract_log_file_from_cmd(cmd2),
            Some(PathBuf::from("output.log"))
        );

        // Test no log file
        let cmd3 = "npm run dev";
        assert_eq!(monitor.extract_log_file_from_cmd(cmd3), None);
    }
}
