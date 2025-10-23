//! Error types for Sentinel.
//!
//! This module defines custom error types used throughout the application.
//! All errors implement `std::error::Error` and can be converted to user-friendly
//! messages for display in the UI.

use serde::Serialize;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Main error type for Sentinel operations.
///
/// This enum covers all possible errors that can occur during process management,
/// system monitoring, and configuration handling.
#[derive(Debug, Error, Serialize)]
pub enum SentinelError {
    /// Process with the specified name was not found.
    #[error("Process '{name}' not found")]
    ProcessNotFound { name: String },

    /// Failed to spawn a new process.
    #[error("Failed to spawn process '{name}': {source}")]
    SpawnFailed {
        name: String,
        #[source]
        #[serde(skip)]
        source: io::Error,
    },

    /// Process is already running.
    #[error("Process '{name}' is already running with PID {pid}")]
    ProcessAlreadyRunning { name: String, pid: u32 },

    /// Process failed to stop within the timeout period.
    #[error("Process '{name}' failed to stop within {timeout_secs} seconds")]
    StopTimeout { name: String, timeout_secs: u64 },

    /// Invalid configuration provided.
    #[error("Invalid configuration: {reason}")]
    InvalidConfig { reason: String },

    /// Configuration file not found.
    #[error("Configuration file not found: {}", path.display())]
    ConfigNotFound { path: PathBuf },

    /// Failed to parse configuration file.
    #[error("Failed to parse config file {}: {source}", path.display())]
    ConfigParseFailed {
        path: PathBuf,
        #[source]
        #[serde(skip)]
        source: serde_yaml::Error,
    },

    /// Failed to read or write a file.
    #[error("File I/O error for {}: {source}", path.display())]
    FileIoError {
        path: PathBuf,
        #[source]
        #[serde(skip)]
        source: io::Error,
    },

    /// System monitoring error.
    #[error("System monitoring error: {message}")]
    MonitoringError { message: String },

    /// Dependency cycle detected in process configuration.
    #[error("Dependency cycle detected: {}", deps.join(" -> "))]
    DependencyCycle { deps: Vec<String> },

    /// Unknown dependency referenced.
    #[error("Process '{process}' depends on unknown process '{dependency}'")]
    UnknownDependency { process: String, dependency: String },

    /// Maximum restart limit exceeded.
    #[error("Process '{name}' exceeded restart limit of {limit} attempts")]
    RestartLimitExceeded { name: String, limit: u32 },

    /// Generic I/O error.
    #[error("I/O error: {0}")]
    Io(
        #[from]
        #[serde(skip)]
        io::Error,
    ),

    /// Port discovery error.
    #[error("Port scanning failed: {0}")]
    PortDiscoveryError(String),

    /// Port not found.
    #[error("Port {0} not found")]
    PortNotFound(u16),

    /// Docker error.
    #[error("Docker error: {0}")]
    DockerError(String),

    /// Generic error with custom message.
    #[error("{0}")]
    Other(String),
}

/// Convert anyhow::Error to SentinelError
impl From<anyhow::Error> for SentinelError {
    fn from(err: anyhow::Error) -> Self {
        SentinelError::Other(err.to_string())
    }
}

/// Convert bollard::errors::Error to SentinelError
impl From<bollard::errors::Error> for SentinelError {
    fn from(err: bollard::errors::Error) -> Self {
        SentinelError::DockerError(err.to_string())
    }
}

/// Specialized Result type for Sentinel operations.
pub type Result<T> = std::result::Result<T, SentinelError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_not_found_error() {
        let err = SentinelError::ProcessNotFound {
            name: "test-process".to_string(),
        };
        assert_eq!(err.to_string(), "Process 'test-process' not found");
    }

    #[test]
    fn test_invalid_config_error() {
        let err = SentinelError::InvalidConfig {
            reason: "duplicate process names".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Invalid configuration: duplicate process names"
        );
    }

    #[test]
    fn test_dependency_cycle_error() {
        let err = SentinelError::DependencyCycle {
            deps: vec!["A".to_string(), "B".to_string(), "A".to_string()],
        };
        assert_eq!(err.to_string(), "Dependency cycle detected: A -> B -> A");
    }

    #[test]
    fn test_restart_limit_exceeded() {
        let err = SentinelError::RestartLimitExceeded {
            name: "api".to_string(),
            limit: 5,
        };
        assert_eq!(
            err.to_string(),
            "Process 'api' exceeded restart limit of 5 attempts"
        );
    }
}
