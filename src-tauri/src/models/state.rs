//! Runtime state models for process tracking.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Runtime state for all managed processes.
///
/// This is persisted separately from configuration to track:
/// - Current PIDs
/// - Process start times
/// - Config hashes (to detect config drift)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeState {
    /// Map of process name to runtime info
    pub processes: HashMap<String, ProcessRuntimeInfo>,

    /// Last time state was synchronized with running processes
    pub last_sync: Option<DateTime<Utc>>,
}

/// Runtime information for a single process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRuntimeInfo {
    /// Current process ID (if running)
    pub pid: Option<u32>,

    /// When this process was started by Sentinel
    pub started_at: Option<DateTime<Utc>>,

    /// Hash of the process configuration (to detect changes)
    pub config_hash: String,

    /// Whether this process is actively managed by Sentinel
    pub managed_by_sentinel: bool,

    /// Number of times this process has been restarted
    pub restart_count: u32,

    /// Last known exit code (if process exited)
    pub last_exit_code: Option<i32>,
}

impl RuntimeState {
    /// Creates a new empty runtime state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or updates process runtime info.
    pub fn upsert_process(&mut self, name: String, info: ProcessRuntimeInfo) {
        self.processes.insert(name, info);
        self.last_sync = Some(Utc::now());
    }

    /// Removes process runtime info.
    pub fn remove_process(&mut self, name: &str) -> Option<ProcessRuntimeInfo> {
        let result = self.processes.remove(name);
        self.last_sync = Some(Utc::now());
        result
    }

    /// Gets process runtime info.
    pub fn get_process(&self, name: &str) -> Option<&ProcessRuntimeInfo> {
        self.processes.get(name)
    }

    /// Updates the last sync timestamp.
    pub fn mark_synced(&mut self) {
        self.last_sync = Some(Utc::now());
    }
}

impl ProcessRuntimeInfo {
    /// Creates new runtime info for a process.
    pub fn new(pid: u32, config_hash: String) -> Self {
        Self {
            pid: Some(pid),
            started_at: Some(Utc::now()),
            config_hash,
            managed_by_sentinel: true,
            restart_count: 0,
            last_exit_code: None,
        }
    }

    /// Marks process as stopped.
    pub fn mark_stopped(&mut self, exit_code: Option<i32>) {
        self.pid = None;
        self.last_exit_code = exit_code;
    }

    /// Increments restart counter.
    pub fn increment_restart(&mut self) {
        self.restart_count += 1;
    }
}
