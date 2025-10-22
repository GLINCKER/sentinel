//! Configuration data models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main configuration structure for Sentinel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// List of processes to manage.
    pub processes: Vec<ProcessConfig>,
    /// Global settings.
    #[serde(default)]
    pub settings: GlobalSettings,
    /// Global environment variables applied to all processes.
    #[serde(default, rename = "globalEnv")]
    pub global_env: HashMap<String, String>,
}

/// Configuration for a single process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessConfig {
    /// Unique name for the process.
    pub name: String,
    /// Command to execute.
    pub command: String,
    /// Command arguments (optional).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Working directory (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<PathBuf>,
    /// Environment variables.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Whether to automatically restart on crash.
    #[serde(default = "default_auto_restart", rename = "autoRestart")]
    pub auto_restart: bool,
    /// Maximum number of restart attempts (0 = unlimited).
    #[serde(
        default = "default_restart_limit",
        rename = "restartLimit",
        alias = "max_restarts"
    )]
    pub restart_limit: u32,
    /// Delay between restarts in milliseconds.
    #[serde(
        default = "default_restart_delay",
        rename = "restartDelay",
        alias = "restart_delay_ms"
    )]
    pub restart_delay: u64,
    /// List of process names this process depends on.
    #[serde(default, rename = "dependsOn")]
    pub depends_on: Vec<String>,
    /// Health check configuration (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<HealthCheck>,
}

/// Health check configuration for a process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Command to execute for health check.
    pub command: String,
    /// Arguments for the health check command.
    #[serde(default)]
    pub args: Vec<String>,
    /// Interval between health checks in milliseconds.
    #[serde(rename = "intervalMs")]
    pub interval_ms: u64,
    /// Timeout for health check command in milliseconds.
    #[serde(rename = "timeoutMs")]
    pub timeout_ms: u64,
    /// Number of retries before marking as unhealthy.
    pub retries: u32,
}

/// Global application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {
    /// Log level (trace, debug, info, warn, error).
    #[serde(default = "default_log_level", rename = "logLevel")]
    pub log_level: String,
    /// Directory for storing logs.
    #[serde(skip_serializing_if = "Option::is_none", rename = "logDirectory")]
    pub log_directory: Option<PathBuf>,
    /// Maximum log file size in bytes.
    #[serde(default = "default_max_log_size", rename = "maxLogSize")]
    pub max_log_size: u64,
    /// Maximum number of log files to keep.
    #[serde(default = "default_max_log_files", rename = "maxLogFiles")]
    pub max_log_files: u32,
    /// Graceful shutdown timeout in milliseconds.
    #[serde(
        default = "default_graceful_shutdown_timeout",
        rename = "gracefulShutdownTimeout"
    )]
    pub graceful_shutdown_timeout: u64,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            log_directory: None,
            max_log_size: default_max_log_size(),
            max_log_files: default_max_log_files(),
            graceful_shutdown_timeout: default_graceful_shutdown_timeout(),
        }
    }
}

// Default value functions
fn default_auto_restart() -> bool {
    true
}

fn default_restart_limit() -> u32 {
    5
}

fn default_restart_delay() -> u64 {
    1000 // 1 second
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_max_log_size() -> u64 {
    10 * 1024 * 1024 // 10MB
}

fn default_max_log_files() -> u32 {
    5
}

fn default_graceful_shutdown_timeout() -> u64 {
    30_000 // 30 seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_deserialization_yaml() {
        let yaml = r#"
processes:
  - name: api
    command: npm start
    cwd: ./backend
    env:
      PORT: "3000"
    autoRestart: true
    restartLimit: 5
settings:
  logLevel: debug
"#;

        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.processes.len(), 1);
        assert_eq!(config.processes[0].name, "api");
        assert_eq!(config.processes[0].command, "npm start");
        assert_eq!(
            config.processes[0].env.get("PORT"),
            Some(&"3000".to_string())
        );
        assert_eq!(config.settings.log_level, "debug");
    }

    #[test]
    fn test_process_config_defaults() {
        let yaml = r#"
name: test
command: echo hello
"#;

        let config: ProcessConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.name, "test");
        assert_eq!(config.command, "echo hello");
        assert!(config.auto_restart); // Default
        assert_eq!(config.restart_limit, 5); // Default
        assert_eq!(config.restart_delay, 1000); // Default
        assert!(config.depends_on.is_empty());
    }

    #[test]
    fn test_global_settings_defaults() {
        let settings = GlobalSettings::default();
        assert_eq!(settings.log_level, "info");
        assert_eq!(settings.max_log_size, 10 * 1024 * 1024);
        assert_eq!(settings.max_log_files, 5);
        assert_eq!(settings.graceful_shutdown_timeout, 30_000);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            processes: vec![ProcessConfig {
                name: "test".to_string(),
                command: "echo test".to_string(),
                args: vec![],
                cwd: None,
                env: HashMap::new(),
                auto_restart: true,
                restart_limit: 3,
                restart_delay: 2000,
                depends_on: vec![],
                health_check: None,
            }],
            settings: GlobalSettings::default(),
            global_env: HashMap::new(),
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        assert!(yaml.contains("name: test"));
        assert!(yaml.contains("command: echo test"));
    }
}
