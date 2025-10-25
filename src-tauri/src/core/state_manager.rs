//! Runtime state management for process tracking.

use crate::error::{Result, SentinelError};
use crate::models::RuntimeState;
use std::fs;
use std::path::PathBuf;

/// Manages runtime state persistence.
pub struct StateManager;

impl StateManager {
    /// Gets the default state file path.
    ///
    /// Returns: `~/.config/sentinel/.sentinel-state.json`
    pub fn get_state_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("sentinel").join(".sentinel-state.json")
        } else {
            PathBuf::from(".sentinel-state.json")
        }
    }

    /// Loads runtime state from file.
    ///
    /// If file doesn't exist, returns empty state.
    pub fn load() -> Result<RuntimeState> {
        let path = Self::get_state_path();

        if !path.exists() {
            return Ok(RuntimeState::new());
        }

        let contents = fs::read_to_string(&path).map_err(|source| SentinelError::FileIoError {
            path: path.clone(),
            source,
        })?;

        serde_json::from_str(&contents)
            .map_err(|e| SentinelError::Other(format!("Failed to parse state file: {}", e)))
    }

    /// Saves runtime state to file.
    pub fn save(state: &RuntimeState) -> Result<()> {
        let path = Self::get_state_path();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                SentinelError::Other(format!("Failed to create state directory: {}", e))
            })?;
        }

        let contents = serde_json::to_string_pretty(state)
            .map_err(|e| SentinelError::Other(format!("Failed to serialize state: {}", e)))?;

        fs::write(&path, contents).map_err(|source| SentinelError::FileIoError {
            path: path.clone(),
            source,
        })?;

        Ok(())
    }

    /// Clears the state file (removes it).
    pub fn clear() -> Result<()> {
        let path = Self::get_state_path();

        if path.exists() {
            fs::remove_file(&path).map_err(|source| SentinelError::FileIoError {
                path: path.clone(),
                source,
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_path() {
        let path = StateManager::get_state_path();
        assert!(path.to_string_lossy().contains("sentinel"));
        assert!(path.to_string_lossy().contains(".sentinel-state.json"));
    }

    #[test]
    fn test_load_nonexistent_state() {
        // Should return empty state, not error
        let state = StateManager::load().unwrap();
        assert_eq!(state.processes.len(), 0);
    }

    #[test]
    fn test_save_and_load() {
        use crate::models::ProcessRuntimeInfo;

        let mut state = RuntimeState::new();
        state.upsert_process(
            "test".to_string(),
            ProcessRuntimeInfo::new(12345, "hash123".to_string()),
        );

        // Save
        StateManager::save(&state).unwrap();

        // Load
        let loaded = StateManager::load().unwrap();
        assert_eq!(loaded.processes.len(), 1);
        assert!(loaded.processes.contains_key("test"));

        // Cleanup
        let _ = StateManager::clear();
    }
}
