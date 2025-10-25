//! Core business logic for Sentinel.
//!
//! This module contains the main components:
//! - Configuration system
//! - Process manager
//! - System monitor
//! - External process monitoring

pub mod config;
pub mod external_process_monitor;
pub mod framework_detector;
pub mod log_buffer;
pub mod metrics_buffer;
pub mod process_config;
pub mod process_control;
pub mod process_manager;
pub mod pty_process_manager;
pub mod state_manager;
pub mod system_monitor;

pub use config::ConfigManager;
pub use external_process_monitor::{
    ExternalProcessMonitor, LogLineEvent, LogSource, ProcessAttachment,
};
pub use framework_detector::{
    detect_framework, get_framework_templates, scan_directory_for_projects,
};
pub use log_buffer::{LogBuffer, LogLine, LogStream};
pub use metrics_buffer::{MetricsBuffer, TimedMetric};
pub use process_config::{
    DetectedProject, FrameworkDetection, FrameworkType, HealthCheckResult,
    ProcessConfig as ManagedProcessConfig, ProcessConfigStore, ProcessStatus, ProcessStatusInfo,
    ProcessTemplate,
};
pub use process_control::ProcessController;
pub use process_manager::ProcessManager;
pub use pty_process_manager::{
    ProcessConfig as PtyProcessConfig, ProcessExitEvent, ProcessInfo, ProcessOutputEvent,
    PtyProcessManager,
};
pub use state_manager::StateManager;
pub use system_monitor::SystemMonitor;
