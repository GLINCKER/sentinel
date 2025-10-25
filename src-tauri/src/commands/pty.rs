//! PTY process management commands
use crate::core::{ProcessInfo, PtyProcessConfig};
use crate::state::AppState;
use std::collections::HashMap;
use tauri::{AppHandle, State};

/// Spawn a new process with PTY
#[tauri::command]
pub async fn spawn_pty_process(
    process_id: String,
    command: String,
    args: Vec<String>,
    cwd: Option<String>,
    env: Option<HashMap<String, String>>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<u32, String> {
    tracing::info!(
        "spawn_pty_process called: id={}, command={}, args={:?}",
        process_id,
        command,
        args
    );

    state
        .pty_manager
        .lock()
        .await
        .spawn_process(process_id, command, args, cwd, env, app)
        .await
        .map_err(|e| e.to_string())
}

/// Kill a PTY process
#[tauri::command]
pub async fn kill_pty_process(
    process_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("kill_pty_process called: id={}", process_id);

    state
        .pty_manager
        .lock()
        .await
        .kill_process(&process_id)
        .await
        .map_err(|e| e.to_string())
}

/// List all PTY processes
#[tauri::command]
pub async fn list_pty_processes(state: State<'_, AppState>) -> Result<Vec<ProcessInfo>, String> {
    Ok(state.pty_manager.lock().await.list_processes().await)
}

/// Check if a PTY process is running
#[tauri::command]
pub async fn is_pty_process_running(
    process_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    Ok(state.pty_manager.lock().await.is_running(&process_id).await)
}

/// Restart a PTY process
#[tauri::command]
pub async fn restart_pty_process(
    process_id: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<u32, String> {
    tracing::info!("restart_pty_process called: id={}", process_id);

    state
        .pty_manager
        .lock()
        .await
        .restart_process(&process_id, app)
        .await
        .map_err(|e| e.to_string())
}

/// Get all stored PTY process configurations
#[tauri::command]
pub async fn get_pty_configs(state: State<'_, AppState>) -> Result<Vec<PtyProcessConfig>, String> {
    Ok(state.pty_manager.lock().await.get_all_configs().await)
}
