//! Process lifecycle management.
//!
//! This module handles spawning, monitoring, and managing child processes.

use crate::error::{Result, SentinelError};
use crate::models::{ProcessConfig, ProcessInfo, ProcessState};
use chrono::Utc;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::{Child, Command};
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
///     command: "echo hello".to_string(),
///     cwd: None,
///     env: HashMap::new(),
///     auto_restart: false,
///     restart_limit: 0,
///     restart_delay: 1000,
///     depends_on: vec![],
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
    ///     command: "npm start".to_string(),
    ///     cwd: Some("./backend".into()),
    ///     env: HashMap::new(),
    ///     auto_restart: true,
    ///     restart_limit: 5,
    ///     restart_delay: 1000,
    ///     depends_on: vec![],
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

        // Parse command and arguments
        let parts: Vec<&str> = config.command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(SentinelError::InvalidConfig {
                reason: format!("Empty command for process '{}'", name),
            });
        }

        let (program, args) = (parts[0], &parts[1..]);

        // Build command
        let mut cmd = Command::new(program);
        cmd.args(args);

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
        let child = cmd.spawn().map_err(|source| SentinelError::SpawnFailed {
            name: name.clone(),
            source,
        })?;

        let pid = child.id().unwrap_or(0);

        debug!("Process '{}' spawned with PID {}", name, pid);

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
        };

        self.processes.insert(name, handle);

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
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(name: &str, command: &str) -> ProcessConfig {
        ProcessConfig {
            name: name.to_string(),
            command: command.to_string(),
            cwd: None,
            env: HashMap::new(),
            auto_restart: false,
            restart_limit: 0,
            restart_delay: 100,
            depends_on: vec![],
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
}
