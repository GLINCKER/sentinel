//! # Docker Integration Module
//!
//! Provides Docker container monitoring and management capabilities.
//!
//! ## Features
//! - List Docker containers (running and stopped)
//! - Monitor container statistics (CPU, memory, network, I/O)
//! - Control containers (start, stop, restart, pause, unpause)
//! - Get Docker system information
//!
//! ## Example
//!
//! ```rust,no_run
//! use sentinel::features::docker::DockerMonitor;
//!
//! #[tokio::main]
//! async fn main() {
//!     let monitor = DockerMonitor::new();
//!
//!     if monitor.is_available() {
//!         let containers = monitor.list_containers(true).await.unwrap();
//!         for container in containers {
//!             println!("{}: {} [{}]", container.name, container.image, container.state);
//!         }
//!     } else {
//!         println!("Docker is not available");
//!     }
//! }
//! ```

mod monitor;
mod types;

pub use monitor::DockerMonitor;
pub use types::*;

use crate::error::Result;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// Application state for Docker monitor
pub struct DockerMonitorState(pub Arc<Mutex<DockerMonitor>>);

/// Get Docker system information
#[tauri::command]
pub async fn get_docker_info(state: State<'_, DockerMonitorState>) -> Result<DockerInfo> {
    let monitor = state.0.lock().await;
    monitor.get_info().await
}

/// List Docker containers
#[tauri::command]
pub async fn list_docker_containers(
    state: State<'_, DockerMonitorState>,
    all: Option<bool>,
) -> Result<Vec<ContainerInfo>> {
    let monitor = state.0.lock().await;
    monitor.list_containers(all.unwrap_or(false)).await
}

/// List Docker images
#[tauri::command]
pub async fn list_docker_images(state: State<'_, DockerMonitorState>) -> Result<Vec<ImageInfo>> {
    let monitor = state.0.lock().await;
    monitor.list_images().await
}

/// Get container statistics
#[tauri::command]
pub async fn get_docker_container_stats(
    state: State<'_, DockerMonitorState>,
    container_id: String,
) -> Result<Option<ContainerStats>> {
    let monitor = state.0.lock().await;
    monitor.get_container_stats(&container_id).await
}

/// Start a Docker container
#[tauri::command]
pub async fn start_docker_container(
    state: State<'_, DockerMonitorState>,
    container_id: String,
) -> Result<ContainerOperationResult> {
    let monitor = state.0.lock().await;
    monitor.start_container(&container_id).await
}

/// Stop a Docker container
#[tauri::command]
pub async fn stop_docker_container(
    state: State<'_, DockerMonitorState>,
    container_id: String,
    timeout: Option<i64>,
) -> Result<ContainerOperationResult> {
    let monitor = state.0.lock().await;
    monitor.stop_container(&container_id, timeout).await
}

/// Restart a Docker container
#[tauri::command]
pub async fn restart_docker_container(
    state: State<'_, DockerMonitorState>,
    container_id: String,
    timeout: Option<i64>,
) -> Result<ContainerOperationResult> {
    let monitor = state.0.lock().await;
    monitor.restart_container(&container_id, timeout).await
}

/// Pause a Docker container
#[tauri::command]
pub async fn pause_docker_container(
    state: State<'_, DockerMonitorState>,
    container_id: String,
) -> Result<ContainerOperationResult> {
    let monitor = state.0.lock().await;
    monitor.pause_container(&container_id).await
}

/// Unpause a Docker container
#[tauri::command]
pub async fn unpause_docker_container(
    state: State<'_, DockerMonitorState>,
    container_id: String,
) -> Result<ContainerOperationResult> {
    let monitor = state.0.lock().await;
    monitor.unpause_container(&container_id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitor_state_creation() {
        let state = DockerMonitorState(Arc::new(Mutex::new(DockerMonitor::new())));
        let monitor = state.0.lock().await;
        // Just ensure it doesn't panic
        drop(monitor);
    }
}
