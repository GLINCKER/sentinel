//! Core business logic for Sentinel.
//!
//! This module contains the main components:
//! - Configuration system
//! - Process manager
//! - System monitor

pub mod config;
pub mod process_manager;
pub mod system_monitor;

pub use config::ConfigManager;
pub use process_manager::ProcessManager;
pub use system_monitor::SystemMonitor;
