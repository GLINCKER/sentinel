//! Application state management.
//!
//! This module manages the global application state that is shared across
//! Tauri commands.

use crate::core::{
    ExternalProcessMonitor, ProcessConfigStore, ProcessController, ProcessManager,
    PtyProcessManager, SystemMonitor,
};
use crate::models::Config;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// Global application state.
///
/// This struct is managed by Tauri and accessible from all commands.
pub struct AppState {
    /// Process manager instance.
    pub process_manager: Arc<Mutex<ProcessManager>>,
    /// System monitor instance.
    pub system_monitor: Arc<Mutex<SystemMonitor>>,
    /// External process monitor instance.
    pub external_process_monitor: Arc<Mutex<ExternalProcessMonitor>>,
    /// PTY process manager instance.
    pub pty_manager: Arc<Mutex<PtyProcessManager>>,
    /// Process configuration store.
    pub process_config_store: Arc<Mutex<ProcessConfigStore>>,
    /// Process controller for managed processes.
    pub process_controller: Arc<Mutex<ProcessController>>,
    /// Current configuration.
    pub config: Arc<RwLock<Option<Config>>>,
}

impl AppState {
    /// Creates a new AppState with default instances.
    pub fn new() -> Self {
        let pty_manager = Arc::new(Mutex::new(PtyProcessManager::new()));
        let process_controller = Arc::new(Mutex::new(ProcessController::new(pty_manager.clone())));

        Self {
            process_manager: Arc::new(Mutex::new(ProcessManager::new())),
            system_monitor: Arc::new(Mutex::new(SystemMonitor::new())),
            external_process_monitor: Arc::new(Mutex::new(ExternalProcessMonitor::new())),
            pty_manager,
            process_config_store: Arc::new(Mutex::new(ProcessConfigStore::new())),
            process_controller,
            config: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
