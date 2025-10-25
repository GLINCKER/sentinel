//! Process control for managed configurations.
//!
//! This module manages starting/stopping processes from configurations.

use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::core::process_config::{ProcessConfig, ProcessStatus, ProcessStatusInfo};
use crate::core::PtyProcessManager;
use crate::error::Result as SentinelResult;

/// Tracks running processes from configurations
#[derive(Clone)]
struct RunningProcess {
    #[allow(dead_code)]
    config_id: String,
    process_id: String,
    pid: u32,
    started_at: chrono::DateTime<Utc>,
}

/// Process controller that manages the lifecycle of configured processes
pub struct ProcessController {
    pty_manager: Arc<Mutex<PtyProcessManager>>,
    running: Arc<Mutex<HashMap<String, RunningProcess>>>, // config_id -> RunningProcess
}

impl ProcessController {
    pub fn new(pty_manager: Arc<Mutex<PtyProcessManager>>) -> Self {
        Self {
            pty_manager,
            running: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Start a process from a configuration
    pub async fn start_from_config(
        &self,
        config: ProcessConfig,
        app: AppHandle,
    ) -> SentinelResult<ProcessStatusInfo> {
        // Check if already running
        {
            let running = self.running.lock().await;
            if running.contains_key(&config.id) {
                return self.get_status(&config.id).await;
            }
        }

        // Use config.name as process_id for PTY
        let process_id = config.name.clone();

        // Spawn the process
        let pid = self
            .pty_manager
            .lock()
            .await
            .spawn_process(
                process_id.clone(),
                config.command.clone(),
                config.args.clone(),
                Some(config.working_dir.clone()),
                if config.env_vars.is_empty() {
                    None
                } else {
                    Some(config.env_vars.clone())
                },
                app,
            )
            .await?;

        // Track running process
        {
            let mut running = self.running.lock().await;
            running.insert(
                config.id.clone(),
                RunningProcess {
                    config_id: config.id.clone(),
                    process_id: process_id.clone(),
                    pid,
                    started_at: Utc::now(),
                },
            );
        }

        // Return status
        self.get_status(&config.id).await
    }

    /// Stop a process by config ID
    pub async fn stop_by_config_id(&self, config_id: &str) -> SentinelResult<()> {
        let process_id = {
            let running = self.running.lock().await;
            running
                .get(config_id)
                .map(|p| p.process_id.clone())
                .ok_or_else(|| crate::error::SentinelError::ProcessNotFound {
                    name: config_id.to_string(),
                })?
        };

        // Kill the PTY process
        self.pty_manager
            .lock()
            .await
            .kill_process(&process_id)
            .await?;

        // Remove from running
        {
            let mut running = self.running.lock().await;
            running.remove(config_id);
        }

        Ok(())
    }

    /// Restart a process
    pub async fn restart(
        &self,
        config: ProcessConfig,
        app: AppHandle,
    ) -> SentinelResult<ProcessStatusInfo> {
        // Stop if running
        if self.is_running(&config.id).await {
            let _ = self.stop_by_config_id(&config.id).await;
            // Give it a moment to stop
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }

        // Start again
        self.start_from_config(config, app).await
    }

    /// Get process status by config ID
    pub async fn get_status(&self, config_id: &str) -> SentinelResult<ProcessStatusInfo> {
        let running = self.running.lock().await;

        if let Some(proc) = running.get(config_id) {
            // Process is running
            let uptime = (Utc::now() - proc.started_at).num_seconds() as u64;

            Ok(ProcessStatusInfo {
                config_id: config_id.to_string(),
                running: true,
                process_id: Some(proc.process_id.clone()),
                pid: Some(proc.pid),
                status: Some(ProcessStatus::Running),
                uptime_seconds: Some(uptime),
                last_health_check: None,
            })
        } else {
            // Process is not running
            Ok(ProcessStatusInfo {
                config_id: config_id.to_string(),
                running: false,
                process_id: None,
                pid: None,
                status: Some(ProcessStatus::Stopped),
                uptime_seconds: None,
                last_health_check: None,
            })
        }
    }

    /// Check if a process is running
    pub async fn is_running(&self, config_id: &str) -> bool {
        let running = self.running.lock().await;
        running.contains_key(config_id)
    }

    /// Clean up stopped processes
    pub async fn cleanup_stopped(&self) {
        let mut running = self.running.lock().await;
        let _pty_manager = self.pty_manager.lock().await;

        // Remove entries for processes that are no longer running
        running.retain(|_, _proc| {
            // This is a simple check; ideally we'd query PTY manager
            // For now, assume all tracked processes are still running
            // This will be improved when we add process exit event handling
            true
        });
    }

    /// Get process ID (PTY name) from config ID
    pub async fn get_process_id(&self, config_id: &str) -> Option<String> {
        let running = self.running.lock().await;
        running.get(config_id).map(|p| p.process_id.clone())
    }
}
