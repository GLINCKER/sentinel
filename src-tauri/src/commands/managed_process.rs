//! Tauri commands for managed process configuration.

use tauri::{AppHandle, State};

use crate::core::{
    detect_framework, get_framework_templates, DetectedProject, FrameworkDetection,
    ManagedProcessConfig, ProcessStatusInfo, ProcessTemplate,
};
use crate::state::AppState;

/// Create a new process configuration
#[tauri::command]
pub async fn create_process_config(
    config: ManagedProcessConfig,
    state: State<'_, AppState>,
) -> Result<ManagedProcessConfig, String> {
    state
        .process_config_store
        .lock()
        .await
        .create(config)
        .await
        .map_err(|e| e.to_string())
}

/// Update an existing configuration
#[tauri::command]
pub async fn update_process_config(
    config: ManagedProcessConfig,
    state: State<'_, AppState>,
) -> Result<ManagedProcessConfig, String> {
    state
        .process_config_store
        .lock()
        .await
        .update(config)
        .await
        .map_err(|e| e.to_string())
}

/// Delete a configuration
#[tauri::command]
pub async fn delete_process_config(id: String, state: State<'_, AppState>) -> Result<(), String> {
    // First stop the process if running
    let controller = state.process_controller.lock().await;
    if controller.is_running(&id).await {
        let _ = controller.stop_by_config_id(&id).await;
    }

    // Then delete the config
    state
        .process_config_store
        .lock()
        .await
        .delete(&id)
        .await
        .map_err(|e| e.to_string())
}

/// Get all configurations
#[tauri::command]
pub async fn list_process_configs(
    state: State<'_, AppState>,
) -> Result<Vec<ManagedProcessConfig>, String> {
    Ok(state.process_config_store.lock().await.list().await)
}

/// Get a single configuration by ID
#[tauri::command]
pub async fn get_process_config(
    id: String,
    state: State<'_, AppState>,
) -> Result<ManagedProcessConfig, String> {
    state
        .process_config_store
        .lock()
        .await
        .get(&id)
        .await
        .map_err(|e| e.to_string())
}

/// Detect framework from a working directory
#[tauri::command]
pub async fn detect_framework_type(working_dir: String) -> Result<FrameworkDetection, String> {
    detect_framework(&working_dir)
        .await
        .map_err(|e| e.to_string())
}

/// Get built-in framework templates
#[tauri::command]
pub async fn get_framework_templates_list() -> Result<Vec<ProcessTemplate>, String> {
    Ok(get_framework_templates())
}

/// Start a process from a configuration
#[tauri::command]
pub async fn start_process_from_config(
    config_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ProcessStatusInfo, String> {
    // Load the config
    let config = state
        .process_config_store
        .lock()
        .await
        .get(&config_id)
        .await
        .map_err(|e| e.to_string())?;

    // Start the process
    state
        .process_controller
        .lock()
        .await
        .start_from_config(config, app)
        .await
        .map_err(|e| e.to_string())
}

/// Stop a running process by config ID
#[tauri::command]
pub async fn stop_process_by_config_id(
    config_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .process_controller
        .lock()
        .await
        .stop_by_config_id(&config_id)
        .await
        .map_err(|e| e.to_string())
}

/// Restart a process
#[tauri::command]
pub async fn restart_managed_process(
    config_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ProcessStatusInfo, String> {
    // Load the config
    let config = state
        .process_config_store
        .lock()
        .await
        .get(&config_id)
        .await
        .map_err(|e| e.to_string())?;

    // Restart the process
    state
        .process_controller
        .lock()
        .await
        .restart(config, app)
        .await
        .map_err(|e| e.to_string())
}

/// Get process status by config ID
#[tauri::command]
pub async fn get_process_status_by_config(
    config_id: String,
    state: State<'_, AppState>,
) -> Result<ProcessStatusInfo, String> {
    state
        .process_controller
        .lock()
        .await
        .get_status(&config_id)
        .await
        .map_err(|e| e.to_string())
}

/// Export all configurations as JSON
#[tauri::command]
pub async fn export_process_configs(state: State<'_, AppState>) -> Result<String, String> {
    state
        .process_config_store
        .lock()
        .await
        .export()
        .await
        .map_err(|e| e.to_string())
}

/// Import configurations from JSON
#[tauri::command]
pub async fn import_process_configs(
    json: String,
    state: State<'_, AppState>,
) -> Result<Vec<ManagedProcessConfig>, String> {
    state
        .process_config_store
        .lock()
        .await
        .import(&json)
        .await
        .map_err(|e| e.to_string())
}

/// Scan a directory for projects (supports monorepos)
#[tauri::command]
pub async fn scan_directory_for_projects(dir_path: String) -> Result<Vec<DetectedProject>, String> {
    crate::core::scan_directory_for_projects(&dir_path)
        .await
        .map_err(|e| e.to_string())
}

/// Get logs for a managed process by config ID
#[tauri::command]
pub async fn get_managed_process_logs(
    config_id: String,
    _count: usize,
    state: State<'_, AppState>,
) -> Result<Vec<crate::core::log_buffer::LogLine>, String> {
    // Check if process is running
    let _process_id = state
        .process_controller
        .lock()
        .await
        .get_process_id(&config_id)
        .await
        .ok_or_else(|| format!("Process with config ID '{}' is not running", config_id))?;

    // TODO: Implement log storage for PTY processes
    // For now, return empty logs - logs are emitted via events
    Ok(vec![])
}
