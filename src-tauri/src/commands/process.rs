//! Process management commands.

use crate::models::{ProcessConfig, ProcessInfo};
use crate::state::AppState;
use tauri::State;

/// Starts a process from configuration.
///
/// # Arguments
/// * `config` - Process configuration
/// * `state` - Application state
///
/// # Returns
/// * `Ok(ProcessInfo)` - Successfully started process
/// * `Err(String)` - Error message
#[tauri::command]
pub async fn start_process(
    config: ProcessConfig,
    state: State<'_, AppState>,
) -> Result<ProcessInfo, String> {
    let mut manager = state.process_manager.lock().await;
    manager.start(config).await.map_err(|e| e.to_string())
}

/// Stops a running process.
///
/// # Arguments
/// * `name` - Process name
/// * `state` - Application state
///
/// # Returns
/// * `Ok(())` - Process stopped
/// * `Err(String)` - Error message
#[tauri::command]
pub async fn stop_process(name: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut manager = state.process_manager.lock().await;
    manager.stop(&name).await.map_err(|e| e.to_string())
}

/// Restarts a process.
///
/// # Arguments
/// * `name` - Process name
/// * `state` - Application state
///
/// # Returns
/// * `Ok(ProcessInfo)` - Restarted process info
/// * `Err(String)` - Error message
#[tauri::command]
pub async fn restart_process(
    name: String,
    state: State<'_, AppState>,
) -> Result<ProcessInfo, String> {
    let mut manager = state.process_manager.lock().await;
    manager.restart(&name).await.map_err(|e| e.to_string())
}

/// Gets information about a specific process.
///
/// # Arguments
/// * `name` - Process name
/// * `state` - Application state
///
/// # Returns
/// * `Ok(ProcessInfo)` - Process information
/// * `Err(String)` - Process not found
#[tauri::command]
pub async fn get_process(name: String, state: State<'_, AppState>) -> Result<ProcessInfo, String> {
    let manager = state.process_manager.lock().await;
    manager
        .get(&name)
        .cloned()
        .ok_or_else(|| format!("Process '{}' not found", name))
}

/// Lists all processes.
///
/// # Arguments
/// * `state` - Application state
///
/// # Returns
/// Vector of all process information
#[tauri::command]
pub async fn list_processes(state: State<'_, AppState>) -> Result<Vec<ProcessInfo>, String> {
    let manager = state.process_manager.lock().await;
    Ok(manager.list())
}

/// Stops all running processes.
///
/// # Arguments
/// * `state` - Application state
///
/// # Returns
/// * `Ok(())` - All processes stopped
/// * `Err(String)` - Error message
#[tauri::command]
pub async fn stop_all_processes(state: State<'_, AppState>) -> Result<(), String> {
    let mut manager = state.process_manager.lock().await;
    manager.stop_all().await.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn test_state() -> AppState {
        AppState::new()
    }

    fn test_config(name: &str) -> ProcessConfig {
        ProcessConfig {
            name: name.to_string(),
            command: "echo test".to_string(),
            cwd: None,
            env: HashMap::new(),
            auto_restart: false,
            restart_limit: 0,
            restart_delay: 100,
            depends_on: vec![],
        }
    }

    #[tokio::test]
    async fn test_start_process_command() {
        let state = test_state();
        let config = test_config("test");

        let result = start_process(config, state).await;
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(info.name, "test");
    }

    #[tokio::test]
    async fn test_list_processes_command() {
        let state = test_state();

        let result = list_processes(state).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_stop_nonexistent_process() {
        let state = test_state();

        let result = stop_process("nonexistent".to_string(), state).await;
        assert!(result.is_err());
    }
}
