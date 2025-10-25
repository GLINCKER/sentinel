use chrono::{DateTime, Utc};
use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::error::{Result as SentinelResult, SentinelError};

/// Event emitted when process produces output
#[derive(Clone, Serialize, Deserialize)]
pub struct ProcessOutputEvent {
    pub process_id: String,
    pub output: String,
    pub stream: String, // "stdout" or "stderr"
    pub timestamp: DateTime<Utc>,
}

/// Event emitted when process exits
#[derive(Clone, Serialize, Deserialize)]
pub struct ProcessExitEvent {
    pub process_id: String,
    pub exit_code: Option<i32>,
    pub timestamp: DateTime<Utc>,
}

/// Process configuration for persistence
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProcessConfig {
    pub process_id: String,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: Option<HashMap<String, String>>,
}

/// Handle to a running PTY process
struct ProcessHandle {
    process_id: String,
    pid: u32,
    #[allow(dead_code)]
    config: ProcessConfig,
    _pty_pair: PtyPair,
    reader_handle: JoinHandle<()>,
}

/// Manages PTY-based process spawning and lifecycle
pub struct PtyProcessManager {
    processes: Arc<Mutex<HashMap<String, ProcessHandle>>>,
    configs: Arc<Mutex<HashMap<String, ProcessConfig>>>, // Store configs for restart
}

impl PtyProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
            configs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Spawn a process with PTY for terminal emulation
    pub async fn spawn_process(
        &self,
        process_id: String,
        command: String,
        args: Vec<String>,
        cwd: Option<String>,
        env: Option<HashMap<String, String>>,
        app: AppHandle,
    ) -> SentinelResult<u32> {
        tracing::info!(
            "Spawning PTY process: {} with command: {} {:?}",
            process_id,
            command,
            args
        );

        // Clone for config storage (before they get moved)
        let cwd_clone = cwd.clone();
        let env_clone = env.clone();

        // 1. Create PTY pair with reasonable terminal size
        let pty_system = native_pty_system();
        let pty_pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| SentinelError::Other(format!("Failed to create PTY: {}", e)))?;

        // 2. Build command
        let mut cmd = CommandBuilder::new(&command);
        cmd.args(&args);

        if let Some(ref cwd_path) = cwd {
            cmd.cwd(cwd_path);
        }

        if let Some(ref env_vars) = env {
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
        }

        // 3. Spawn process in PTY
        let mut child = pty_pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| SentinelError::Other(format!("Failed to spawn command: {}", e)))?;

        let pid = child
            .process_id()
            .ok_or_else(|| SentinelError::Other("Failed to get process ID".to_string()))?;

        tracing::info!("Process {} spawned with PID: {}", process_id, pid);

        // 4. Read output in background task
        let mut reader = pty_pair
            .master
            .try_clone_reader()
            .map_err(|e| SentinelError::Other(format!("Failed to clone PTY reader: {}", e)))?;

        let process_id_clone = process_id.clone();
        let app_clone = app.clone();

        let reader_handle = tokio::task::spawn_blocking(move || {
            let mut buffer = [0u8; 8192];

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => {
                        // EOF - process exited
                        tracing::info!("Process {} exited (EOF)", process_id_clone);

                        let _ = app_clone.emit(
                            "process-exit",
                            ProcessExitEvent {
                                process_id: process_id_clone.clone(),
                                exit_code: None,
                                timestamp: Utc::now(),
                            },
                        );

                        break;
                    }
                    Ok(n) => {
                        let output = String::from_utf8_lossy(&buffer[..n]).to_string();

                        let _ = app_clone.emit(
                            "process-output",
                            ProcessOutputEvent {
                                process_id: process_id_clone.clone(),
                                output,
                                stream: "stdout".to_string(),
                                timestamp: Utc::now(),
                            },
                        );
                    }
                    Err(e) => {
                        tracing::error!("Error reading PTY for {}: {}", process_id_clone, e);
                        break;
                    }
                }
            }

            // Wait for child process to fully exit
            let exit_status = child.wait();
            tracing::info!(
                "Process {} wait completed: {:?}",
                process_id_clone,
                exit_status
            );
        });

        // 5. Store config for restart capability
        let config = ProcessConfig {
            process_id: process_id.clone(),
            command,
            args,
            cwd: cwd_clone,
            env: env_clone,
        };
        self.configs
            .lock()
            .await
            .insert(process_id.clone(), config.clone());

        // 6. Store handle
        let handle = ProcessHandle {
            process_id: process_id.clone(),
            pid,
            config,
            _pty_pair: pty_pair,
            reader_handle,
        };

        self.processes.lock().await.insert(process_id, handle);

        Ok(pid)
    }

    /// Kill a process
    pub async fn kill_process(&self, process_id: &str) -> SentinelResult<()> {
        let mut processes = self.processes.lock().await;

        if let Some(handle) = processes.remove(process_id) {
            tracing::info!("Killing process: {}", process_id);

            // Kill using system signal
            #[cfg(unix)]
            {
                use libc::{kill, SIGTERM};
                unsafe {
                    kill(handle.pid as i32, SIGTERM);
                }
            }

            #[cfg(windows)]
            {
                // Windows kill implementation
                tracing::warn!("Windows process kill not yet implemented");
            }

            // Cancel the reader task
            handle.reader_handle.abort();

            Ok(())
        } else {
            Err(SentinelError::ProcessNotFound {
                name: process_id.to_string(),
            })
        }
    }

    /// Get list of running processes managed by this manager
    pub async fn list_processes(&self) -> Vec<ProcessInfo> {
        let processes = self.processes.lock().await;

        processes
            .values()
            .map(|handle| ProcessInfo {
                process_id: handle.process_id.clone(),
                pid: handle.pid,
            })
            .collect()
    }

    /// Check if a process is running
    pub async fn is_running(&self, process_id: &str) -> bool {
        self.processes.lock().await.contains_key(process_id)
    }

    /// Restart a process using its stored configuration
    pub async fn restart_process(&self, process_id: &str, app: AppHandle) -> SentinelResult<u32> {
        // Get the stored config
        let config = self
            .configs
            .lock()
            .await
            .get(process_id)
            .cloned()
            .ok_or_else(|| SentinelError::ProcessNotFound {
                name: process_id.to_string(),
            })?;

        // Kill existing process if running
        if self.is_running(process_id).await {
            self.kill_process(process_id).await?;
            // Give it a moment to clean up
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        // Respawn with same config
        self.spawn_process(
            config.process_id,
            config.command,
            config.args,
            config.cwd,
            config.env,
            app,
        )
        .await
    }

    /// Get all stored process configurations
    pub async fn get_all_configs(&self) -> Vec<ProcessConfig> {
        self.configs.lock().await.values().cloned().collect()
    }

    /// Save a process configuration
    pub async fn save_config(&self, config: ProcessConfig) {
        self.configs
            .lock()
            .await
            .insert(config.process_id.clone(), config);
    }

    /// Remove a process configuration
    pub async fn remove_config(&self, process_id: &str) {
        self.configs.lock().await.remove(process_id);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub process_id: String,
    pub pid: u32,
}

impl Default for PtyProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a test app handle - will need to be implemented
    fn create_test_app_handle() -> AppHandle {
        unimplemented!("Test app handle creation")
    }

    #[tokio::test]
    #[ignore] // Ignore until we have test app handle
    async fn test_spawn_echo_command() {
        let manager = PtyProcessManager::new();
        let app = create_test_app_handle();

        let result = manager
            .spawn_process(
                "test-echo".to_string(),
                "echo".to_string(),
                vec!["Hello World".to_string()],
                None,
                None,
                app,
            )
            .await;

        assert!(result.is_ok());
        let pid = result.unwrap();
        assert!(pid > 0);
    }

    #[tokio::test]
    async fn test_list_processes() {
        let manager = PtyProcessManager::new();
        let processes = manager.list_processes().await;
        assert_eq!(processes.len(), 0);
    }
}
