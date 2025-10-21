//! Sentinel - Your Development Guardian
//!
//! A modern, secure process manager and system monitor built with Rust and Tauri.
//!
//! # Architecture
//!
//! Sentinel is organized into several layers:
//!
//! - **Models** (`models`) - Data structures for processes, configuration, and system metrics
//! - **Core** (`core`) - Business logic for process management, configuration, and monitoring
//! - **Commands** (`commands`) - Tauri command handlers (frontend API)
//! - **State** (`state`) - Global application state management
//!
//! # Examples
//!
//! ## Using the Process Manager
//!
//! ```no_run
//! use sentinel::core::ProcessManager;
//! use sentinel::models::ProcessConfig;
//! use std::collections::HashMap;
//!
//! # tokio_test::block_on(async {
//! let mut manager = ProcessManager::new();
//!
//! let config = ProcessConfig {
//!     name: "api-server".to_string(),
//!     command: "npm start".to_string(),
//!     cwd: Some("./backend".into()),
//!     env: HashMap::new(),
//!     auto_restart: true,
//!     restart_limit: 5,
//!     restart_delay: 1000,
//!     depends_on: vec![],
//! };
//!
//! let info = manager.start(config).await?;
//! println!("Started process: {} (PID: {:?})", info.name, info.pid);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//!
//! ## Using the System Monitor
//!
//! ```
//! use sentinel::core::SystemMonitor;
//!
//! let mut monitor = SystemMonitor::new();
//! monitor.refresh();
//!
//! let stats = monitor.get_stats();
//! println!("CPU: {:.2}%", stats.cpu.overall);
//! println!("Memory: {} / {} bytes", stats.memory.used, stats.memory.total);
//! ```
//!
//! ## Loading Configuration
//!
//! ```no_run
//! use sentinel::core::ConfigManager;
//! use std::path::Path;
//!
//! let config = ConfigManager::load_from_file(Path::new("sentinel.yaml"))?;
//! println!("Loaded {} processes", config.processes.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod core;
pub mod error;
pub mod models;
pub mod state;

// Re-export commonly used types
pub use error::{Result, SentinelError};
pub use state::AppState;

/// Runs the Tauri application.
///
/// This is the main entry point called from `main.rs`.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            // Process commands
            commands::start_process,
            commands::stop_process,
            commands::restart_process,
            commands::get_process,
            commands::list_processes,
            commands::stop_all_processes,
            // System commands
            commands::get_system_stats,
            commands::get_process_stats,
            commands::get_system_info,
        ])
        .setup(|_app| {
            // Initialize tracing
            tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::from_default_env()
                        .add_directive(tracing::Level::INFO.into()),
                )
                .init();

            tracing::info!("Sentinel starting up...");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let _state = AppState::new();
        // Just ensure it doesn't panic during creation
    }
}
