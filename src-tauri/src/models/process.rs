//! Process-related data models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the state of a managed process.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessState {
    /// Process is not running.
    Stopped,
    /// Process is starting up.
    Starting,
    /// Process is running normally.
    Running,
    /// Process is being stopped.
    Stopping,
    /// Process crashed with an exit code.
    Crashed { exit_code: i32 },
    /// Process failed to start.
    Failed { reason: String },
}

/// Information about a managed process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// Unique name of the process.
    pub name: String,
    /// Current state of the process.
    pub state: ProcessState,
    /// Process ID (if running).
    pub pid: Option<u32>,
    /// Command that was executed.
    pub command: String,
    /// Working directory.
    pub cwd: Option<String>,
    /// CPU usage percentage (0-100 per core).
    pub cpu_usage: f32,
    /// Memory usage in bytes.
    pub memory_usage: u64,
    /// Number of restart attempts.
    pub restart_count: u32,
    /// Time when the process was started.
    pub started_at: Option<DateTime<Utc>>,
    /// Time when the process was stopped.
    pub stopped_at: Option<DateTime<Utc>>,
}

impl ProcessInfo {
    /// Creates a new ProcessInfo in the Stopped state.
    pub fn new(name: String, command: String) -> Self {
        Self {
            name,
            state: ProcessState::Stopped,
            pid: None,
            command,
            cwd: None,
            cpu_usage: 0.0,
            memory_usage: 0,
            restart_count: 0,
            started_at: None,
            stopped_at: None,
        }
    }

    /// Checks if the process is currently running.
    pub fn is_running(&self) -> bool {
        matches!(self.state, ProcessState::Running)
    }

    /// Checks if the process is stopped.
    pub fn is_stopped(&self) -> bool {
        matches!(self.state, ProcessState::Stopped)
    }

    /// Checks if the process has crashed.
    pub fn is_crashed(&self) -> bool {
        matches!(self.state, ProcessState::Crashed { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_info_new() {
        let info = ProcessInfo::new("test".to_string(), "npm start".to_string());
        assert_eq!(info.name, "test");
        assert_eq!(info.command, "npm start");
        assert_eq!(info.state, ProcessState::Stopped);
        assert!(info.is_stopped());
        assert!(!info.is_running());
    }

    #[test]
    fn test_process_state_serialization() {
        let running = ProcessState::Running;
        let json = serde_json::to_string(&running).unwrap();
        assert_eq!(json, "\"running\"");

        let crashed = ProcessState::Crashed { exit_code: 1 };
        let json = serde_json::to_string(&crashed).unwrap();
        assert!(json.contains("crashed"));
        assert!(json.contains("exit_code"));
    }

    #[test]
    fn test_is_running() {
        let mut info = ProcessInfo::new("test".to_string(), "cmd".to_string());
        assert!(!info.is_running());

        info.state = ProcessState::Running;
        assert!(info.is_running());
    }

    #[test]
    fn test_is_crashed() {
        let mut info = ProcessInfo::new("test".to_string(), "cmd".to_string());
        assert!(!info.is_crashed());

        info.state = ProcessState::Crashed { exit_code: 1 };
        assert!(info.is_crashed());
    }
}
