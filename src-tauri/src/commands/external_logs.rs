//! Commands for external process log monitoring.

use crate::core::ProcessAttachment;
use crate::state::AppState;
use tauri::{AppHandle, State};

/// Attach to an external process for log monitoring
#[tauri::command]
pub async fn attach_to_external_process(
    pid: u32,
    port: Option<u16>,
    state: State<'_, AppState>,
) -> Result<ProcessAttachment, String> {
    let monitor = state.inner().external_process_monitor.lock().await;
    monitor
        .attach_to_process(pid, port)
        .await
        .map_err(|e| e.to_string())
}

/// Tail a log file and stream lines to the frontend
#[tauri::command]
pub async fn tail_log_file(
    path: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let monitor = state.inner().external_process_monitor.lock().await;
    monitor
        .tail_log_file(path, app)
        .await
        .map_err(|e| e.to_string())
}

/// Capture logs using dtrace (macOS only)
#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn capture_with_dtrace(
    pid: u32,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let monitor = state.inner().external_process_monitor.lock().await;
    monitor
        .capture_with_dtrace(pid, app)
        .await
        .map_err(|e| e.to_string())
}

/// Detach from a log file or process
#[tauri::command]
pub async fn detach_external_logs(
    attachment_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let monitor = state.inner().external_process_monitor.lock().await;
    monitor
        .detach(&attachment_id)
        .await
        .map_err(|e| e.to_string())
}
