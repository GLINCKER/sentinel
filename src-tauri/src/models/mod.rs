//! Data models for Sentinel.
//!
//! This module contains all data structures used throughout the application,
//! including process information, configuration, and system metrics.

pub mod config;
pub mod process;
pub mod state;
pub mod system;

pub use config::{Config, GlobalSettings, HealthCheck, ProcessConfig};
pub use process::{ProcessInfo, ProcessState};
pub use state::{ProcessRuntimeInfo, RuntimeState};
pub use system::{CpuStats, DiskStats, MemoryStats, SystemStats};
