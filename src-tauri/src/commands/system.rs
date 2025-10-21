//! System monitoring commands.

use crate::models::SystemStats;
use crate::state::AppState;
use tauri::State;

/// Gets current system statistics.
///
/// # Arguments
/// * `state` - Application state
///
/// # Returns
/// Current system statistics (CPU, memory, disk)
#[tauri::command]
pub async fn get_system_stats(state: State<'_, AppState>) -> Result<SystemStats, String> {
    let mut monitor = state.system_monitor.lock().await;
    monitor.refresh();
    Ok(monitor.get_stats())
}

/// Gets resource usage for a specific process.
///
/// # Arguments
/// * `pid` - Process ID
/// * `state` - Application state
///
/// # Returns
/// * `Ok((cpu_percent, memory_bytes))` - Resource usage
/// * `Err(String)` - Process not found
#[tauri::command]
pub async fn get_process_stats(pid: u32, state: State<'_, AppState>) -> Result<(f32, u64), String> {
    let monitor = state.system_monitor.lock().await;
    monitor
        .get_process_stats(pid)
        .ok_or_else(|| format!("Process with PID {} not found", pid))
}

/// Gets system information.
///
/// # Arguments
/// * `state` - Application state
///
/// # Returns
/// System information (OS name, hostname, uptime, etc.)
#[tauri::command]
pub async fn get_system_info(state: State<'_, AppState>) -> Result<SystemInfo, String> {
    let monitor = state.system_monitor.lock().await;

    Ok(SystemInfo {
        os_name: monitor.os_name(),
        kernel_version: monitor.kernel_version(),
        hostname: monitor.hostname(),
        uptime: monitor.uptime(),
        process_count: monitor.process_count(),
    })
}

/// System information structure.
#[derive(serde::Serialize)]
pub struct SystemInfo {
    pub os_name: Option<String>,
    pub kernel_version: Option<String>,
    pub hostname: Option<String>,
    pub uptime: u64,
    pub process_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state() -> AppState {
        AppState::new()
    }

    #[tokio::test]
    async fn test_get_system_stats_command() {
        let state = test_state();

        let result = get_system_stats(state).await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert!(stats.cpu.overall >= 0.0);
        assert!(stats.memory.total > 0);
    }

    #[tokio::test]
    async fn test_get_process_stats_command() {
        let state = test_state();
        let current_pid = std::process::id();

        let result = get_process_stats(current_pid, state).await;
        assert!(result.is_ok());

        let (cpu, memory) = result.unwrap();
        assert!(cpu >= 0.0);
        assert!(memory > 0);
    }

    #[tokio::test]
    async fn test_get_system_info_command() {
        let state = test_state();

        let result = get_system_info(state).await;
        assert!(result.is_ok());

        let info = result.unwrap();
        assert!(info.os_name.is_some());
        assert!(info.uptime > 0);
        assert!(info.process_count > 0);
    }
}
