//! Process lifecycle management.
//!
//! This module handles spawning, monitoring, and managing child processes.
use crate::core::log_buffer::{LogBuffer, LogLine, LogStream};
use crate::error::{Result, SentinelError};
use crate::models::{ProcessConfig, ProcessInfo, ProcessState};
use chrono::Utc;
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

/// Manages the lifecycle of multiple processes.
///
/// # Examples
/// ```no_run
/// use sentinel::core::ProcessManager;
/// use sentinel::models::ProcessConfig;
/// use std::collections::HashMap;
///
/// # tokio_test::block_on(async {
/// let mut manager = ProcessManager::new();
/// let config = ProcessConfig {
///     name: "test".to_string(),
///     command: "echo".to_string(),
///     args: vec!["hello".to_string()],
///     cwd: None,
///     env: HashMap::new(),
///     auto_restart: false,
///     restart_limit: 0,
///     restart_delay: 1000,
///     depends_on: vec![],
///     health_check: None,
/// };
///
/// let info = manager.start(config).await?;
/// println!("Started process: {}", info.name);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # });
/// ```
pub struct ProcessManager {
    /// Map of process name to process handle and info.
    processes: HashMap<String, ProcessHandle>,
}

/// Handle for a running process.
struct ProcessHandle {
    /// Process information.
    info: ProcessInfo,
    /// Child process handle (if running).
    child: Option<Child>,
    /// Configuration used to spawn the process.
    config: ProcessConfig,
    /// Log buffer (last 10,000 lines). Thread-safe with Arc<Mutex>.
    log_buffer: Arc<Mutex<LogBuffer>>,
    /// Number of restarts performed.
    restart_count: u32,
    /// Last restart timestamp (for exponential backoff).
    last_restart: Option<std::time::Instant>,
}

impl ProcessHandle {
    #[allow(dead_code)]
    fn new(info: ProcessInfo, child: Child, config: ProcessConfig) -> Self {
        Self {
            info,
            child: Some(child),
            config,
            log_buffer: Arc::new(Mutex::new(LogBuffer::new())),
            restart_count: 0,
            last_restart: None,
        }
    }
}

impl ProcessManager {
    /// Creates a new ProcessManager.
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    /// Starts a process from configuration.
    ///
    /// # Arguments
    /// * `config` - Process configuration
    ///
    /// # Returns
    /// * `Ok(ProcessInfo)` - Successfully started process information
    /// * `Err(SentinelError)` - Failed to start process
    ///
    /// # Errors
    /// Returns error if:
    /// - Process with same name is already running
    /// - Failed to spawn the process
    /// - Working directory doesn't exist
    ///
    /// # Examples
    /// ```no_run
    /// # use sentinel::core::ProcessManager;
    /// # use sentinel::models::ProcessConfig;
    /// # use std::collections::HashMap;
    /// # tokio_test::block_on(async {
    /// let mut manager = ProcessManager::new();
    /// let config = ProcessConfig {
    ///     name: "api".to_string(),
    ///     command: "npm".to_string(),
    ///     args: vec!["start".to_string()],
    ///     cwd: Some("./backend".into()),
    ///     env: HashMap::new(),
    ///     auto_restart: true,
    ///     restart_limit: 5,
    ///     restart_delay: 1000,
    ///     depends_on: vec![],
    ///     health_check: None,
    /// };
    ///
    /// let info = manager.start(config).await?;
    /// assert_eq!(info.state, sentinel::models::ProcessState::Running);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn start(&mut self, config: ProcessConfig) -> Result<ProcessInfo> {
        let name = config.name.clone();

        // Check if process already exists
        if let Some(handle) = self.processes.get(&name) {
            if handle.info.is_running() {
                return Err(SentinelError::ProcessAlreadyRunning {
                    name: name.clone(),
                    pid: handle.info.pid.unwrap_or(0),
                });
            }
        }

        info!("Starting process: {}", name);

        let mut cmd = if config.args.is_empty() {
            let parts: Vec<&str> = config.command.split_whitespace().collect();
            if parts.is_empty() {
                return Err(SentinelError::InvalidConfig {
                    reason: format!("Empty command for process '{}'", name),
                });
            }
            let (program, args) = (parts[0], &parts[1..]);
            let mut cmd = Command::new(program);
            cmd.args(args);
            cmd
        } else {
            let mut cmd = Command::new(&config.command);
            cmd.args(&config.args);
            cmd
        };

        // Set working directory
        if let Some(cwd) = &config.cwd {
            cmd.current_dir(cwd);
        }

        // Set environment variables
        for (key, value) in &config.env {
            cmd.env(key, value);
        }

        // Configure stdio
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::null());

        // Spawn process
        let mut child = cmd.spawn().map_err(|source| SentinelError::SpawnFailed {
            name: name.clone(),
            source,
        })?;

        let pid = child.id().unwrap_or(0);

        debug!("Process '{}' spawned with PID {}", name, pid);

        // Create log buffer (shared between log readers)
        let log_buffer = Arc::new(Mutex::new(LogBuffer::new()));

        // Spawn log reader tasks for stdout and stderr
        if let Some(stdout) = child.stdout.take() {
            let buffer = log_buffer.clone();
            let process_name = name.clone();
            tokio::spawn(async move {
                read_stream(stdout, buffer, LogStream::Stdout, &process_name).await;
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let buffer = log_buffer.clone();
            let process_name = name.clone();
            tokio::spawn(async move {
                read_stream(stderr, buffer, LogStream::Stderr, &process_name).await;
            });
        }

        // Create process info
        let info = ProcessInfo {
            name: name.clone(),
            state: ProcessState::Running,
            pid: Some(pid),
            command: config.command.clone(),
            cwd: config.cwd.as_ref().map(|p| p.display().to_string()),
            cpu_usage: 0.0,
            memory_usage: 0,
            restart_count: 0,
            started_at: Some(Utc::now()),
            stopped_at: None,
        };

        // Store process handle
        let handle = ProcessHandle {
            info: info.clone(),
            child: Some(child),
            config,
            log_buffer,
            restart_count: 0,
            last_restart: None,
        };

        self.processes.insert(name, handle);

        info!("Process '{}' started successfully", info.name);

        Ok(info)
    }

    /// Stops a running process.
    ///
    /// Sends SIGTERM (Unix) or terminates (Windows) and waits for graceful shutdown.
    ///
    /// # Arguments
    /// * `name` - Name of the process to stop
    ///
    /// # Returns
    /// * `Ok(())` - Process stopped successfully
    /// * `Err(SentinelError)` - Process not found or failed to stop
    ///
    /// # Examples
    /// ```no_run
    /// # use sentinel::core::ProcessManager;
    /// # tokio_test::block_on(async {
    /// # let mut manager = ProcessManager::new();
    /// manager.stop("api").await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn stop(&mut self, name: &str) -> Result<()> {
        let handle =
            self.processes
                .get_mut(name)
                .ok_or_else(|| SentinelError::ProcessNotFound {
                    name: name.to_string(),
                })?;

        if !handle.info.is_running() {
            return Ok(());
        }

        info!("Stopping process: {}", name);
        handle.info.state = ProcessState::Stopping;

        if let Some(mut child) = handle.child.take() {
            // Try to kill the process
            #[cfg(unix)]
            {
                // Send SIGTERM for graceful shutdown
                if let Some(pid) = child.id() {
                    unsafe {
                        libc::kill(pid as i32, libc::SIGTERM);
                    }
                }
            }

            #[cfg(not(unix))]
            {
                let _ = child.kill().await;
            }

            // Wait for process to exit (with timeout)
            let timeout = Duration::from_secs(10);
            match tokio::time::timeout(timeout, child.wait()).await {
                Ok(Ok(status)) => {
                    debug!("Process '{}' exited with status: {:?}", name, status);
                }
                Ok(Err(e)) => {
                    warn!("Error waiting for process '{}': {}", name, e);
                }
                Err(_) => {
                    warn!(
                        "Process '{}' did not stop within timeout, force killing",
                        name
                    );
                    let _ = child.kill().await;
                }
            }
        }

        handle.info.state = ProcessState::Stopped;
        handle.info.pid = None;
        handle.info.stopped_at = Some(Utc::now());

        Ok(())
    }

    /// Restarts a process.
    ///
    /// Stops the process if running, then starts it again.
    ///
    /// # Arguments
    /// * `name` - Name of the process to restart
    ///
    /// # Examples
    /// ```no_run
    /// # use sentinel::core::ProcessManager;
    /// # tokio_test::block_on(async {
    /// # let mut manager = ProcessManager::new();
    /// manager.restart("api").await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn restart(&mut self, name: &str) -> Result<ProcessInfo> {
        info!("Restarting process: {}", name);

        // Get config before stopping
        let config = self
            .processes
            .get(name)
            .ok_or_else(|| SentinelError::ProcessNotFound {
                name: name.to_string(),
            })?
            .config
            .clone();

        // Stop if running
        let _ = self.stop(name).await;

        // Wait a bit before restarting
        sleep(Duration::from_millis(config.restart_delay)).await;

        // Start again
        self.start(config).await
    }

    /// Gets information about a process.
    ///
    /// # Arguments
    /// * `name` - Name of the process
    ///
    /// # Returns
    /// * `Some(ProcessInfo)` - Process information
    /// * `None` - Process not found
    pub fn get(&self, name: &str) -> Option<&ProcessInfo> {
        self.processes.get(name).map(|h| &h.info)
    }

    /// Lists all processes.
    ///
    /// # Returns
    /// Vector of all process information.
    pub fn list(&self) -> Vec<ProcessInfo> {
        self.processes.values().map(|h| h.info.clone()).collect()
    }

    /// Checks if a process is running.
    ///
    /// # Arguments
    /// * `name` - Name of the process
    ///
    /// # Returns
    /// * `true` - Process is running
    /// * `false` - Process is not running or doesn't exist
    pub fn is_running(&self, name: &str) -> bool {
        self.processes
            .get(name)
            .map(|h| h.info.is_running())
            .unwrap_or(false)
    }

    /// Stops all running processes.
    ///
    /// # Examples
    /// ```no_run
    /// # use sentinel::core::ProcessManager;
    /// # tokio_test::block_on(async {
    /// # let mut manager = ProcessManager::new();
    /// manager.stop_all().await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn stop_all(&mut self) -> Result<()> {
        info!("Stopping all processes");

        let names: Vec<String> = self.processes.keys().cloned().collect();

        for name in names {
            if let Err(e) = self.stop(&name).await {
                error!("Failed to stop process '{}': {}", name, e);
            }
        }

        Ok(())
    }

    /// Removes a stopped process from management.
    ///
    /// # Arguments
    /// * `name` - Name of the process to remove
    ///
    /// # Returns
    /// * `Ok(())` - Process removed
    /// * `Err(SentinelError)` - Process is still running or doesn't exist
    pub fn remove(&mut self, name: &str) -> Result<()> {
        if self.is_running(name) {
            return Err(SentinelError::Other(
                "Cannot remove running process. Stop it first.".to_string(),
            ));
        }

        self.processes.remove(name);
        Ok(())
    }

    /// Gets logs for a specific process.
    ///
    /// # Arguments
    /// * `name` - Name of the process
    ///
    /// # Returns
    /// * `Some(Vec<LogLine>)` - Log lines for the process
    /// * `None` - Process not found
    pub async fn get_logs(&self, name: &str) -> Option<Vec<LogLine>> {
        let handle = self.processes.get(name)?;
        let buffer = handle.log_buffer.lock().await;
        Some(buffer.get_all())
    }

    /// Gets last N logs for a specific process.
    ///
    /// # Arguments
    /// * `name` - Name of the process
    /// * `n` - Number of recent logs to retrieve
    ///
    /// # Returns
    /// * `Some(Vec<LogLine>)` - Last N log lines
    /// * `None` - Process not found
    pub async fn get_recent_logs(&self, name: &str, n: usize) -> Option<Vec<LogLine>> {
        let handle = self.processes.get(name)?;
        let buffer = handle.log_buffer.lock().await;
        Some(buffer.get_last_n(n))
    }

    /// Searches logs for a specific process.
    ///
    /// # Arguments
    /// * `name` - Name of the process
    /// * `query` - Search query (case-insensitive)
    ///
    /// # Returns
    /// * `Some(Vec<LogLine>)` - Matching log lines
    /// * `None` - Process not found
    pub async fn search_logs(&self, name: &str, query: &str) -> Option<Vec<LogLine>> {
        let handle = self.processes.get(name)?;
        let buffer = handle.log_buffer.lock().await;
        Some(buffer.search(query))
    }

    /// Checks health of all processes and restarts crashed ones with auto_restart enabled.
    ///
    /// Uses exponential backoff for restart delays:
    /// - First restart: restart_delay ms
    /// - Second restart: restart_delay * 2 ms
    /// - Third restart: restart_delay * 4 ms
    /// - Max: restart_delay * 2^(restart_count)
    ///
    /// Returns list of process names that were restarted.
    pub async fn check_health(&mut self) -> Vec<String> {
        let mut restarted = Vec::new();
        let process_names: Vec<String> = self.processes.keys().cloned().collect();

        for name in process_names {
            let should_restart = {
                let handle = match self.processes.get_mut(&name) {
                    Some(h) => h,
                    None => continue,
                };

                // Check if process has exited
                if let Some(child) = &mut handle.child {
                    match child.try_wait() {
                        Ok(Some(exit_status)) => {
                            // Process has exited
                            let exit_code = exit_status.code().unwrap_or(-1);
                            warn!("Process '{}' exited with status: {:?}", name, exit_status);
                            handle.info.state = ProcessState::Crashed { exit_code };
                            handle.info.pid = None;
                            handle.info.stopped_at = Some(Utc::now());
                            handle.child = None;

                            // Check if auto-restart is enabled and limit not exceeded
                            if handle.config.auto_restart {
                                if handle.config.restart_limit == 0
                                    || handle.restart_count < handle.config.restart_limit
                                {
                                    true
                                } else {
                                    error!(
                                        "Process '{}' exceeded restart limit ({})",
                                        name, handle.config.restart_limit
                                    );
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        Ok(None) => {
                            // Process still running
                            false
                        }
                        Err(e) => {
                            error!("Error checking process '{}' status: {}", name, e);
                            false
                        }
                    }
                } else {
                    false
                }
            };

            if should_restart {
                // Calculate exponential backoff delay
                let handle = self.processes.get(&name).unwrap();
                let base_delay = handle.config.restart_delay;
                let backoff_multiplier = 2_u64.pow(handle.restart_count);
                let delay_ms = base_delay.saturating_mul(backoff_multiplier);

                info!(
                    "Auto-restarting process '{}' (attempt {}) after {}ms",
                    name,
                    handle.restart_count + 1,
                    delay_ms
                );

                // Wait with exponential backoff
                sleep(Duration::from_millis(delay_ms)).await;

                // Get config and increment restart counter
                let config = handle.config.clone();
                let restart_count = handle.restart_count;
                let last_restart = Some(std::time::Instant::now());

                // Try to restart
                match self.start(config).await {
                    Ok(_) => {
                        // Update restart tracking
                        if let Some(handle) = self.processes.get_mut(&name) {
                            handle.restart_count = restart_count + 1;
                            handle.last_restart = last_restart;
                            handle.info.restart_count = restart_count + 1;
                        }
                        restarted.push(name.clone());
                    }
                    Err(e) => {
                        error!("Failed to auto-restart process '{}': {}", name, e);
                    }
                }
            }
        }

        restarted
    }

    /// Gracefully stops a process with timeout and force kill fallback.
    ///
    /// On Unix: Sends SIGTERM, waits 5 seconds, then sends SIGKILL if needed.
    /// On Windows: Terminates the process after 5 second timeout.
    ///
    /// # Arguments
    /// * `name` - Name of the process to stop
    ///
    /// # Returns
    /// * `Ok(())` - Process stopped
    /// * `Err(SentinelError)` - Process not found or error occurred
    pub async fn stop_gracefully(&mut self, name: &str) -> Result<()> {
        let handle =
            self.processes
                .get_mut(name)
                .ok_or_else(|| SentinelError::ProcessNotFound {
                    name: name.to_string(),
                })?;

        if !handle.info.is_running() {
            return Ok(());
        }

        info!("Gracefully stopping process: {}", name);
        handle.info.state = ProcessState::Stopping;

        if let Some(mut child) = handle.child.take() {
            #[cfg(unix)]
            {
                // Send SIGTERM for graceful shutdown
                if let Some(pid) = child.id() {
                    debug!("Sending SIGTERM to process '{}' (PID: {})", name, pid);
                    unsafe {
                        libc::kill(pid as i32, libc::SIGTERM);
                    }
                }

                // Wait up to 5 seconds for graceful shutdown
                let graceful_timeout = Duration::from_secs(5);
                match tokio::time::timeout(graceful_timeout, child.wait()).await {
                    Ok(Ok(status)) => {
                        debug!(
                            "Process '{}' gracefully exited with status: {:?}",
                            name, status
                        );
                    }
                    Ok(Err(e)) => {
                        warn!("Error waiting for process '{}': {}", name, e);
                    }
                    Err(_) => {
                        warn!(
                            "Process '{}' did not stop gracefully, sending SIGKILL",
                            name
                        );
                        if let Some(pid) = child.id() {
                            unsafe {
                                libc::kill(pid as i32, libc::SIGKILL);
                            }
                        }
                        let _ = child.wait().await;
                    }
                }
            }

            #[cfg(not(unix))]
            {
                // Windows: just kill with timeout
                let timeout = Duration::from_secs(5);
                match tokio::time::timeout(timeout, child.wait()).await {
                    Ok(Ok(status)) => {
                        debug!("Process '{}' exited with status: {:?}", name, status);
                    }
                    Ok(Err(e)) => {
                        warn!("Error waiting for process '{}': {}", name, e);
                    }
                    Err(_) => {
                        warn!(
                            "Process '{}' did not stop within timeout, force killing",
                            name
                        );
                        let _ = child.kill().await;
                    }
                }
            }
        }

        handle.info.state = ProcessState::Stopped;
        handle.info.pid = None;
        handle.info.stopped_at = Some(Utc::now());

        Ok(())
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Asynchronously reads lines from a process stream (stdout/stderr).
///
/// Pushes log lines to the shared buffer. Runs until stream closes.
///
/// # Arguments
/// * `stream` - The stdout or stderr stream from the child process
/// * `buffer` - Shared log buffer (Arc<Mutex<LogBuffer>>)
/// * `stream_type` - Whether this is stdout or stderr
/// * `process_name` - Name of the process for logging
async fn read_stream<R>(
    stream: R,
    buffer: Arc<Mutex<LogBuffer>>,
    stream_type: LogStream,
    process_name: &str,
) where
    R: tokio::io::AsyncRead + Unpin,
{
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let log_line = LogLine {
            timestamp: Utc::now(),
            stream: stream_type,
            line,
        };

        let mut buf = buffer.lock().await;
        buf.push(log_line);
    }

    debug!(
        "Log stream ({:?}) closed for process: {}",
        stream_type, process_name
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(name: &str, command: &str) -> ProcessConfig {
        ProcessConfig {
            name: name.to_string(),
            command: command.to_string(),
            args: vec![],
            cwd: None,
            env: HashMap::new(),
            auto_restart: false,
            restart_limit: 0,
            restart_delay: 100,
            depends_on: vec![],
            health_check: None,
        }
    }

    #[tokio::test]
    async fn test_start_process() {
        let mut manager = ProcessManager::new();
        let config = test_config("test", "echo hello");

        let info = manager.start(config).await.unwrap();
        assert_eq!(info.name, "test");
        assert_eq!(info.state, ProcessState::Running);
        assert!(info.pid.is_some());
    }

    #[tokio::test]
    async fn test_process_already_running() {
        let mut manager = ProcessManager::new();
        let config = test_config("test", "sleep 10");

        manager.start(config.clone()).await.unwrap();
        let result = manager.start(config).await;

        assert!(matches!(
            result,
            Err(SentinelError::ProcessAlreadyRunning { .. })
        ));
    }

    #[tokio::test]
    async fn test_stop_process() {
        let mut manager = ProcessManager::new();
        let config = test_config("test", "sleep 5");

        manager.start(config).await.unwrap();
        assert!(manager.is_running("test"));

        manager.stop("test").await.unwrap();
        assert!(!manager.is_running("test"));
    }

    #[tokio::test]
    async fn test_stop_nonexistent_process() {
        let mut manager = ProcessManager::new();
        let result = manager.stop("nonexistent").await;

        assert!(matches!(result, Err(SentinelError::ProcessNotFound { .. })));
    }

    #[tokio::test]
    async fn test_restart_process() {
        let mut manager = ProcessManager::new();
        let config = test_config("test", "echo test");

        manager.start(config).await.unwrap();
        let old_pid = manager.get("test").unwrap().pid;

        sleep(Duration::from_millis(100)).await;

        let info = manager.restart("test").await.unwrap();
        let new_pid = info.pid;

        // PIDs should be different (new process)
        assert_ne!(old_pid, new_pid);
    }

    #[tokio::test]
    async fn test_list_processes() {
        let mut manager = ProcessManager::new();

        manager.start(test_config("proc1", "echo 1")).await.unwrap();
        manager.start(test_config("proc2", "echo 2")).await.unwrap();

        let list = manager.list();
        assert_eq!(list.len(), 2);

        let names: Vec<&str> = list.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"proc1"));
        assert!(names.contains(&"proc2"));
    }

    #[tokio::test]
    async fn test_get_process() {
        let mut manager = ProcessManager::new();
        manager
            .start(test_config("test", "echo test"))
            .await
            .unwrap();

        let info = manager.get("test");
        assert!(info.is_some());
        assert_eq!(info.unwrap().name, "test");

        let nonexistent = manager.get("nonexistent");
        assert!(nonexistent.is_none());
    }

    #[tokio::test]
    async fn test_stop_all() {
        let mut manager = ProcessManager::new();

        manager
            .start(test_config("proc1", "sleep 10"))
            .await
            .unwrap();
        manager
            .start(test_config("proc2", "sleep 10"))
            .await
            .unwrap();

        assert!(manager.is_running("proc1"));
        assert!(manager.is_running("proc2"));

        manager.stop_all().await.unwrap();

        assert!(!manager.is_running("proc1"));
        assert!(!manager.is_running("proc2"));
    }

    #[tokio::test]
    async fn test_remove_stopped_process() {
        let mut manager = ProcessManager::new();
        manager
            .start(test_config("test", "echo test"))
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;
        manager.stop("test").await.unwrap();

        manager.remove("test").unwrap();
        assert!(manager.get("test").is_none());
    }

    #[tokio::test]
    async fn test_cannot_remove_running_process() {
        let mut manager = ProcessManager::new();
        manager
            .start(test_config("test", "sleep 10"))
            .await
            .unwrap();

        let result = manager.remove("test");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_log_capture() {
        let mut manager = ProcessManager::new();

        // Start a process that outputs to stdout
        let config = test_config("logger", "echo 'Hello from stdout'");
        manager.start(config).await.unwrap();

        // Give time for log capture
        sleep(Duration::from_millis(200)).await;

        // Retrieve logs
        let logs = manager.get_logs("logger").await.unwrap();

        assert!(!logs.is_empty(), "Logs should be captured");
        assert!(
            logs.iter()
                .any(|log| log.line.contains("Hello from stdout")),
            "Log should contain output"
        );
    }

    #[tokio::test]
    async fn test_log_search() {
        let mut manager = ProcessManager::new();

        // Process that outputs multiple lines
        let config = test_config(
            "multi-logger",
            "sh -c 'echo Error: test failed; echo Info: test passed'",
        );
        manager.start(config).await.unwrap();

        sleep(Duration::from_millis(200)).await;

        // Search for "Error"
        let results = manager.search_logs("multi-logger", "Error").await.unwrap();
        assert!(!results.is_empty(), "Should find error logs");
        assert!(
            results.iter().any(|log| log.line.contains("Error")),
            "Should match error line"
        );
    }

    #[tokio::test]
    async fn test_get_recent_logs() {
        let mut manager = ProcessManager::new();

        let config = test_config(
            "counter",
            "sh -c 'for i in 1 2 3 4 5; do echo Line $i; done'",
        );
        manager.start(config).await.unwrap();

        sleep(Duration::from_millis(300)).await;

        // Get last 3 logs
        let recent = manager.get_recent_logs("counter", 3).await.unwrap();
        assert!(recent.len() <= 5, "Should have at most 5 logs");
    }

    #[tokio::test]
    async fn test_health_check_auto_restart() {
        let mut manager = ProcessManager::new();

        // Create a process that exits immediately but has auto_restart enabled
        let mut config = test_config("auto-restart", "echo 'Starting'; exit 1");
        config.auto_restart = true;
        config.restart_limit = 2;
        config.restart_delay = 50;

        manager.start(config).await.unwrap();

        // Wait for process to exit
        sleep(Duration::from_millis(100)).await;

        // Run health check - should detect crash and restart
        let restarted = manager.check_health().await;

        assert!(
            !restarted.is_empty(),
            "Health check should restart crashed process"
        );
        assert_eq!(restarted[0], "auto-restart");

        // Check restart count incremented
        let handle = manager.processes.get("auto-restart").unwrap();
        assert_eq!(handle.restart_count, 1, "Restart count should be 1");
    }

    #[tokio::test]
    async fn test_health_check_respects_restart_limit() {
        let mut manager = ProcessManager::new();

        // Create a process with restart_limit = 1
        let mut config = test_config("limited-restart", "sh -c 'exit 1'");
        config.auto_restart = true;
        config.restart_limit = 1;
        config.restart_delay = 50;

        manager.start(config).await.unwrap();
        sleep(Duration::from_millis(100)).await;

        // First restart
        manager.check_health().await;
        sleep(Duration::from_millis(100)).await;

        // Process will exit again, but restart limit reached
        manager.check_health().await;

        let handle = manager.processes.get("limited-restart").unwrap();
        assert!(handle.restart_count <= 1, "Should not exceed restart limit");
    }

    #[tokio::test]
    async fn test_graceful_shutdown() {
        let mut manager = ProcessManager::new();

        // Start a long-running process
        let config = test_config("graceful-test", "sleep 30");
        manager.start(config).await.unwrap();
        assert!(manager.is_running("graceful-test"));

        // Stop gracefully
        manager.stop_gracefully("graceful-test").await.unwrap();
        assert!(!manager.is_running("graceful-test"));

        let info = manager.get("graceful-test").unwrap();
        assert_eq!(info.state, ProcessState::Stopped);
    }
}
