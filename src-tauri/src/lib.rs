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
//!     command: "npm".to_string(),
//!     args: vec!["start".to_string()],
//!     cwd: Some("./backend".into()),
//!     env: HashMap::new(),
//!     auto_restart: true,
//!     restart_limit: 5,
//!     restart_delay: 1000,
//!     depends_on: vec![],
//!     health_check: None,
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
pub mod features;
pub mod models;
pub mod state;

// Re-export commonly used types
pub use error::{Result, SentinelError};
pub use state::AppState;

/// Runs the Tauri application.
///
/// This is the main entry point called from `main.rs`.
pub fn run() {
    use tauri::{
        menu::{Menu, MenuItem},
        tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
        Manager,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_pty::init())
        .manage(AppState::new())
        .manage(features::service_detection::ServiceDetectorState(
            std::sync::Arc::new(std::sync::Mutex::new(
                features::service_detection::ServiceDetector::new(),
            )),
        ))
        .manage(features::network_monitor::NetworkMonitorState(
            std::sync::Arc::new(std::sync::Mutex::new(
                features::network_monitor::TrafficCollector::new(),
            )),
        ))
        .invoke_handler(tauri::generate_handler![
            // Process commands
            commands::start_process,
            commands::stop_process,
            commands::restart_process,
            commands::get_process,
            commands::list_processes,
            commands::stop_all_processes,
            // Process log commands
            commands::get_process_logs,
            commands::get_recent_process_logs,
            commands::search_process_logs,
            // Process health commands
            commands::check_process_health,
            commands::stop_process_gracefully,
            // System commands
            commands::get_system_stats,
            commands::get_process_stats,
            commands::get_system_info,
            // Port discovery commands
            features::port_discovery::scan_ports,
            features::port_discovery::kill_process_by_port,
            features::port_discovery::get_port_info,
            // Service detection commands
            features::service_detection::detect_service,
            features::service_detection::clear_service_cache,
            features::service_detection::get_service_cache_size,
            // Network monitoring commands
            features::network_monitor::get_network_stats,
            features::network_monitor::get_network_history,
            features::network_monitor::clear_network_history,
            features::network_monitor::get_network_interfaces,
        ])
        .setup(|app| {
            // Initialize tracing
            tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::from_default_env()
                        .add_directive(tracing::Level::INFO.into()),
                )
                .init();

            tracing::info!("Sentinel starting up...");

            let show_i = MenuItem::with_id(app, "show", "Show Sentinel", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

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
