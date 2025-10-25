//! Process management commands.

use crate::core::{ConfigManager, LogLine};
use crate::models::{Config, ProcessConfig, ProcessInfo};
use crate::state::AppState;
use std::path::PathBuf;
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

/// Starts a stopped process by name.
///
/// This command is used to re-start a process that was previously configured and stopped.
/// It uses the process's stored configuration to start it again.
///
/// # Arguments
/// * `name` - Process name
/// * `state` - Application state
///
/// # Returns
/// * `Ok(ProcessInfo)` - Started process info
/// * `Err(String)` - Error message if process not found or already running
#[tauri::command]
pub async fn start_process_by_name(
    name: String,
    state: State<'_, AppState>,
) -> Result<ProcessInfo, String> {
    let mut manager = state.process_manager.lock().await;
    manager
        .start_by_name(&name)
        .await
        .map_err(|e| e.to_string())
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
    let mut manager = state.process_manager.lock().await;
    // Update CPU and memory usage before returning list
    manager.update_resource_usage();
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

/// Clears all buffered logs for a process.
///
/// # Arguments
/// * `name` - Process name
/// * `state` - Application state
///
/// # Returns
/// * `Ok(())` - Logs cleared
/// * `Err(String)` - Process not found
#[tauri::command]
pub async fn clear_process_logs(name: String, state: State<'_, AppState>) -> Result<(), String> {
    let manager = state.process_manager.lock().await;
    manager.clear_logs(&name).await.map_err(|e| e.to_string())
}

/// Gets the default config file path.
///
/// Searches in order:
/// 1. ~/.config/sentinel/sentinel.yaml
/// 2. ./sentinel.yaml
///
/// # Returns
/// Path to config file (may not exist yet)
fn get_config_path() -> PathBuf {
    // Try user config directory first
    if let Some(config_dir) = dirs::config_dir() {
        let sentinel_dir = config_dir.join("sentinel");
        let config_path = sentinel_dir.join("sentinel.yaml");
        if config_path.exists() {
            return config_path;
        }
        // Return this path even if it doesn't exist (will be created)
        return config_path;
    }

    // Fallback to current directory
    PathBuf::from("sentinel.yaml")
}

/// Loads configuration from file.
///
/// # Arguments
/// * `path` - Optional custom path. If None, uses default location.
///
/// # Returns
/// * `Ok(Config)` - Loaded configuration
/// * `Err(String)` - Error loading config
#[tauri::command]
pub async fn load_config(path: Option<String>) -> Result<Config, String> {
    let config_path = path.map(PathBuf::from).unwrap_or_else(get_config_path);

    // If file doesn't exist, return default config
    if !config_path.exists() {
        return Ok(ConfigManager::default_config());
    }

    ConfigManager::load_from_file(&config_path).map_err(|e| e.to_string())
}

/// Saves a process to the config file.
///
/// # Arguments
/// * `config` - Process configuration to save
/// * `path` - Optional custom config path
///
/// # Returns
/// * `Ok(())` - Process saved successfully
/// * `Err(String)` - Error saving config
#[tauri::command]
pub async fn save_process_to_config(
    process_config: ProcessConfig,
    path: Option<String>,
) -> Result<(), String> {
    let config_path = path.map(PathBuf::from).unwrap_or_else(get_config_path);

    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // Load existing config or create new
    let mut config = if config_path.exists() {
        ConfigManager::load_from_file(&config_path).map_err(|e| e.to_string())?
    } else {
        Config {
            processes: vec![],
            settings: Default::default(),
            global_env: Default::default(),
        }
    };

    // Check if process already exists
    if let Some(existing) = config
        .processes
        .iter_mut()
        .find(|p| p.name == process_config.name)
    {
        // Update existing process
        *existing = process_config;
    } else {
        // Add new process
        config.processes.push(process_config);
    }

    // Save config
    ConfigManager::save_to_file(&config, &config_path).map_err(|e| e.to_string())
}

/// Removes a process from the config file.
///
/// # Arguments
/// * `name` - Process name to remove
/// * `path` - Optional custom config path
///
/// # Returns
/// * `Ok(())` - Process removed successfully
/// * `Err(String)` - Error updating config
#[tauri::command]
pub async fn remove_process_from_config(name: String, path: Option<String>) -> Result<(), String> {
    let config_path = path.map(PathBuf::from).unwrap_or_else(get_config_path);

    // Load existing config
    if !config_path.exists() {
        return Err(format!(
            "Process '{}' is not saved to config. No config file exists at {}",
            name,
            config_path.display()
        ));
    }

    let mut config = ConfigManager::load_from_file(&config_path).map_err(|e| e.to_string())?;

    // Remove process
    let original_len = config.processes.len();
    config.processes.retain(|p| p.name != name);

    if config.processes.len() == original_len {
        return Err(format!(
            "Process '{}' not found in config file. It may have been started without saving to config.",
            name
        ));
    }

    // Save updated config
    ConfigManager::save_to_file(&config, &config_path).map_err(|e| e.to_string())
}

/// Gets the current config file path.
///
/// # Returns
/// Path to the config file that would be used
#[tauri::command]
pub async fn get_config_file_path() -> Result<String, String> {
    Ok(get_config_path().to_string_lossy().to_string())
}

/// Starts processes from config file on app launch.
///
/// This performs smart reconciliation:
/// 1. Loads config file
/// 2. Loads runtime state
/// 3. Checks if processes from state are still running
/// 4. Starts processes that should be running but aren't
///
/// # Arguments
/// * `state` - Application state
/// * `auto_start_only` - If true, only starts processes marked with auto_restart
///
/// # Returns
/// * `Ok(Vec<String>)` - Names of processes that were started
/// * `Err(String)` - Error loading config or starting processes
#[tauri::command]
pub async fn start_processes_from_config(
    state: State<'_, AppState>,
    auto_start_only: Option<bool>,
) -> Result<Vec<String>, String> {
    use crate::core::{ConfigManager, StateManager};
    use crate::models::ProcessRuntimeInfo;
    use sysinfo::{Pid, ProcessRefreshKind, System};

    let config_path = get_config_path();

    // Load config
    if !config_path.exists() {
        return Ok(vec![]); // No config file, nothing to start
    }

    let config = ConfigManager::load_from_file(&config_path).map_err(|e| e.to_string())?;

    // Load runtime state
    let mut runtime_state = StateManager::load().map_err(|e| e.to_string())?;

    // Get system info to check running processes
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything(),
    );

    let mut started = Vec::new();
    let mut manager = state.process_manager.lock().await;

    for process_config in config.processes {
        // Skip if auto_start_only is true and process doesn't have auto_restart
        let should_auto_start = auto_start_only.unwrap_or(false);
        if should_auto_start && !process_config.auto_restart {
            continue;
        }

        // Check runtime state
        let is_running = if let Some(runtime_info) = runtime_state.get_process(&process_config.name)
        {
            // Check if PID from state is still running
            if let Some(pid) = runtime_info.pid {
                sys.process(Pid::from_u32(pid)).is_some()
            } else {
                false
            }
        } else {
            false
        };

        // Start if not running
        if !is_running {
            match manager.start(process_config.clone()).await {
                Ok(info) => {
                    // Update runtime state
                    if let Some(pid) = info.pid {
                        let config_hash = format!("{:?}", process_config); // Simple hash
                        runtime_state.upsert_process(
                            process_config.name.clone(),
                            ProcessRuntimeInfo::new(pid, config_hash),
                        );
                    }
                    started.push(process_config.name);
                }
                Err(e) => {
                    tracing::warn!("Failed to start process '{}': {}", process_config.name, e);
                }
            }
        }
    }

    // Save updated state
    if !started.is_empty() {
        StateManager::save(&runtime_state).map_err(|e| e.to_string())?;
    }

    Ok(started)
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
