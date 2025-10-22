//! Core business logic for Sentinel.
//!
//! This module contains the main components:
//! - Configuration system
//! - Process manager
//! - System monitor

pub mod config;
pub mod log_buffer;
pub mod metrics_buffer;
pub mod process_manager;
pub mod system_monitor;

pub use config::ConfigManager;
pub use log_buffer::{LogBuffer, LogLine, LogStream};
pub use metrics_buffer::{MetricsBuffer, TimedMetric};
pub use process_manager::ProcessManager;
pub use system_monitor::SystemMonitor;
