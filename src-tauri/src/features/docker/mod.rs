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

/// Reconnect to Docker daemon (forces fresh connection check)
#[tauri::command]
pub async fn reconnect_docker(state: State<'_, DockerMonitorState>) -> Result<String> {
    let mut monitor = state.0.lock().await;
    monitor.reconnect();
    if monitor.is_available() {
        Ok("Docker reconnected successfully".to_string())
    } else {
        Ok("Docker connection failed - daemon may not be running".to_string())
    }
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

/// Detect which Docker runtime is available (Docker Desktop, Colima, Podman, etc.)
async fn detect_docker_runtime() -> Option<String> {
    use std::process::Command;

    // Check if Colima is running
    if let Ok(output) = Command::new("colima").arg("status").output() {
        if output.status.success() {
            return Some("colima".to_string());
        }
    }

    // Check if Docker Desktop is installed
    #[cfg(target_os = "macos")]
    {
        if std::path::Path::new("/Applications/Docker.app").exists() {
            return Some("docker-desktop".to_string());
        }
    }

    // Check if Podman is available
    if let Ok(output) = Command::new("podman").arg("--version").output() {
        if output.status.success() {
            return Some("podman".to_string());
        }
    }

    None
}

/// Start Docker daemon (supports Docker Desktop, Colima, Podman)
#[tauri::command]
pub async fn start_docker_desktop() -> Result<String> {
    use std::process::Command;

    tracing::info!("Attempting to start Docker daemon...");

    // Detect which runtime to use
    let runtime = detect_docker_runtime().await;

    #[cfg(target_os = "macos")]
    {
        match runtime.as_deref() {
            Some("colima") => {
                tracing::info!("Starting Colima...");
                let output = Command::new("colima").arg("start").output().map_err(|e| {
                    tracing::error!("Failed to start Colima: {}", e);
                    crate::error::SentinelError::Other(format!("Failed to start Colima: {}", e))
                })?;

                if output.status.success() {
                    tracing::info!("Colima started successfully");
                    Ok("Colima is starting...".to_string())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    tracing::error!("Colima start failed: {}", stderr);
                    Err(crate::error::SentinelError::Other(format!(
                        "Failed to start Colima: {}",
                        stderr
                    )))
                }
            }
            Some("docker-desktop") => {
                tracing::info!("Starting Docker Desktop...");
                let output = Command::new("open")
                    .arg("/Applications/Docker.app")
                    .output()
                    .map_err(|e| {
                        tracing::error!("Failed to start Docker Desktop: {}", e);
                        crate::error::SentinelError::Other(format!(
                            "Failed to start Docker Desktop: {}",
                            e
                        ))
                    })?;

                if output.status.success() {
                    tracing::info!("Docker Desktop start command sent");
                    Ok("Docker Desktop is starting...".to_string())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    tracing::error!("Docker Desktop start failed: {}", stderr);
                    Err(crate::error::SentinelError::Other(format!(
                        "Failed to start Docker Desktop: {}",
                        stderr
                    )))
                }
            }
            Some("podman") => {
                tracing::info!("Starting Podman machine...");
                let output = Command::new("podman")
                    .args(["machine", "start"])
                    .output()
                    .map_err(|e| {
                        tracing::error!("Failed to start Podman: {}", e);
                        crate::error::SentinelError::Other(format!("Failed to start Podman: {}", e))
                    })?;

                if output.status.success() {
                    tracing::info!("Podman machine started");
                    Ok("Podman machine is starting...".to_string())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    tracing::error!("Podman start failed: {}", stderr);
                    Err(crate::error::SentinelError::Other(format!(
                        "Failed to start Podman: {}",
                        stderr
                    )))
                }
            }
            _ => {
                tracing::warn!("No Docker runtime detected");
                Err(crate::error::SentinelError::Other(
                    "No Docker runtime found. Please install Docker Desktop, Colima, or Podman."
                        .to_string(),
                ))
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = Command::new("cmd")
            .args(["/C", "start", "", "Docker Desktop"])
            .output()
            .map_err(|e| {
                crate::error::SentinelError::Other(format!("Failed to start Docker Desktop: {}", e))
            })?;

        if output.status.success() {
            Ok("Docker Desktop is starting...".to_string())
        } else {
            Err(crate::error::SentinelError::Other(format!(
                "Failed to start Docker Desktop: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err(crate::error::SentinelError::Other(
            "Docker Desktop control is only supported on macOS and Windows".to_string(),
        ))
    }
}

/// Stop Docker daemon (supports Docker Desktop, Colima, Podman)
#[tauri::command]
pub async fn stop_docker_desktop() -> Result<String> {
    use std::process::Command;

    tracing::info!("Attempting to stop Docker daemon...");

    // Detect which runtime is running
    let runtime = detect_docker_runtime().await;

    #[cfg(target_os = "macos")]
    {
        match runtime.as_deref() {
            Some("colima") => {
                tracing::info!("Stopping Colima...");
                let output = Command::new("colima").arg("stop").output().map_err(|e| {
                    tracing::error!("Failed to stop Colima: {}", e);
                    crate::error::SentinelError::Other(format!("Failed to stop Colima: {}", e))
                })?;

                if output.status.success() {
                    tracing::info!("Colima stopped successfully");
                    Ok("Colima is stopping...".to_string())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    tracing::error!("Colima stop failed: {}", stderr);
                    Err(crate::error::SentinelError::Other(format!(
                        "Failed to stop Colima: {}",
                        stderr
                    )))
                }
            }
            Some("docker-desktop") => {
                tracing::info!("Stopping Docker Desktop...");
                let output = Command::new("osascript")
                    .args(["-e", "quit app \"Docker\""])
                    .output()
                    .map_err(|e| {
                        tracing::error!("Failed to stop Docker Desktop: {}", e);
                        crate::error::SentinelError::Other(format!(
                            "Failed to stop Docker Desktop: {}",
                            e
                        ))
                    })?;

                if output.status.success() {
                    tracing::info!("Docker Desktop stop command sent");
                    Ok("Docker Desktop is stopping...".to_string())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    tracing::error!("Docker Desktop stop failed: {}", stderr);
                    Err(crate::error::SentinelError::Other(format!(
                        "Failed to stop Docker Desktop: {}",
                        stderr
                    )))
                }
            }
            Some("podman") => {
                tracing::info!("Stopping Podman machine...");
                let output = Command::new("podman")
                    .args(["machine", "stop"])
                    .output()
                    .map_err(|e| {
                        tracing::error!("Failed to stop Podman: {}", e);
                        crate::error::SentinelError::Other(format!("Failed to stop Podman: {}", e))
                    })?;

                if output.status.success() {
                    tracing::info!("Podman machine stopped");
                    Ok("Podman machine is stopping...".to_string())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    tracing::error!("Podman stop failed: {}", stderr);
                    Err(crate::error::SentinelError::Other(format!(
                        "Failed to stop Podman: {}",
                        stderr
                    )))
                }
            }
            _ => {
                tracing::warn!("No Docker runtime detected");
                Err(crate::error::SentinelError::Other(
                    "No Docker runtime found to stop.".to_string(),
                ))
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(crate::error::SentinelError::Other(
            "Docker daemon control is only supported on macOS currently".to_string(),
        ))
    }
}

/// Restart Docker Desktop
#[tauri::command]
pub async fn restart_docker_desktop() -> Result<String> {
    stop_docker_desktop().await?;
    // Wait a bit for Docker to stop
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    start_docker_desktop().await
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
