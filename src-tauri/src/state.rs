//! Application state management.
//!
//! This module manages the global application state that is shared across
//! Tauri commands.

use crate::core::{ProcessManager, SystemMonitor};
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
    /// Current configuration.
    pub config: Arc<RwLock<Option<Config>>>,
}

impl AppState {
    /// Creates a new AppState with default instances.
    pub fn new() -> Self {
        Self {
            process_manager: Arc::new(Mutex::new(ProcessManager::new())),
            system_monitor: Arc::new(Mutex::new(SystemMonitor::new())),
            config: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
