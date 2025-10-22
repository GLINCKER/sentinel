//! Process management commands.

use crate::core::LogLine;
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

/// Gets all logs for a process.
///
/// # Arguments
/// * `name` - Process name
/// * `state` - Application state
///
/// # Returns
/// * `Ok(Vec<LogLine>)` - All log lines
/// * `Err(String)` - Process not found
#[tauri::command]
pub async fn get_process_logs(
    name: String,
    state: State<'_, AppState>,
) -> Result<Vec<LogLine>, String> {
    let manager = state.process_manager.lock().await;
    manager
        .get_logs(&name)
        .await
        .ok_or_else(|| format!("Process '{}' not found", name))
}

/// Gets the most recent N logs for a process.
///
/// # Arguments
/// * `name` - Process name
/// * `count` - Number of recent logs to retrieve
/// * `state` - Application state
///
/// # Returns
/// * `Ok(Vec<LogLine>)` - Recent log lines
/// * `Err(String)` - Process not found
#[tauri::command]
pub async fn get_recent_process_logs(
    name: String,
    count: usize,
    state: State<'_, AppState>,
) -> Result<Vec<LogLine>, String> {
    let manager = state.process_manager.lock().await;
    manager
        .get_recent_logs(&name, count)
        .await
        .ok_or_else(|| format!("Process '{}' not found", name))
}

/// Searches logs for a process.
///
/// # Arguments
/// * `name` - Process name
/// * `query` - Search query (case-insensitive substring match)
/// * `state` - Application state
///
/// # Returns
/// * `Ok(Vec<LogLine>)` - Matching log lines
/// * `Err(String)` - Process not found
#[tauri::command]
pub async fn search_process_logs(
    name: String,
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<LogLine>, String> {
    let manager = state.process_manager.lock().await;
    manager
        .search_logs(&name, &query)
        .await
        .ok_or_else(|| format!("Process '{}' not found", name))
}

/// Checks health of all processes and auto-restarts crashed ones.
///
/// This performs health checks on all managed processes, detects crashes,
/// and automatically restarts processes with auto_restart enabled
/// (respecting restart_limit and using exponential backoff).
///
/// # Arguments
/// * `state` - Application state
///
/// # Returns
/// * `Ok(Vec<String>)` - List of process names that were restarted
/// * `Err(String)` - Error message
#[tauri::command]
pub async fn check_process_health(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut manager = state.process_manager.lock().await;
    Ok(manager.check_health().await)
}

/// Gracefully stops a process with timeout and force kill fallback.
///
/// On Unix: Sends SIGTERM, waits 5 seconds, then sends SIGKILL if needed.
/// On Windows: Terminates the process after 5 second timeout.
///
/// # Arguments
/// * `name` - Process name
/// * `state` - Application state
///
/// # Returns
/// * `Ok(())` - Process stopped gracefully
/// * `Err(String)` - Error message
#[tauri::command]
pub async fn stop_process_gracefully(
    name: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut manager = state.process_manager.lock().await;
    manager
        .stop_gracefully(&name)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[allow(dead_code)]
    fn test_state() -> AppState {
        AppState::new()
    }

    #[allow(dead_code)]
    fn test_config(name: &str) -> ProcessConfig {
        ProcessConfig {
            name: name.to_string(),
            command: "echo test".to_string(),
            args: vec![],
            cwd: None,
            env: HashMap::new(),
            auto_restart: false,
            restart_limit: 0,
            restart_delay: 100,
            depends_on: vec![],
            health_check: None,
        }
    }
}
