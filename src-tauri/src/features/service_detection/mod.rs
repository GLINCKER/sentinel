//! Service detection and identification module
//!
//! Automatically detects services running on discovered ports using pattern matching,
//! health checks, and metadata enrichment.

mod detector;
mod patterns;

#[cfg(test)]
mod tests;

pub use detector::{HealthStatus, ServiceCategory, ServiceDetector, ServiceInfo};

use crate::error::Result;
use std::sync::{Arc, Mutex};
use tauri::State;

/// Application state for service detector
pub struct ServiceDetectorState(pub Arc<Mutex<ServiceDetector>>);

/// Detect service from port information
#[tauri::command]
pub async fn detect_service(
    port: u16,
    pid: u32,
    process_name: String,
    command: Option<String>,
    state: State<'_, ServiceDetectorState>,
) -> Result<Option<ServiceInfo>> {
    tracing::info!(
        "detect_service called for port {}, pid {}, process {}",
        port,
        pid,
        process_name
    );

    let mut detector = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock detector: {}", e);
        e.into_inner()
    });

    let result = detector.detect(port, pid, &process_name, command.as_deref());

    if let Some(ref service) = result {
        tracing::info!(
            "Service detected: {} (confidence: {:.2})",
            service.name,
            service.confidence
        );
    } else {
        tracing::debug!("No service detected for port {}", port);
    }

    Ok(result)
}

/// Clear service detection cache
#[tauri::command]
pub async fn clear_service_cache(state: State<'_, ServiceDetectorState>) -> Result<()> {
    tracing::info!("clear_service_cache called");

    let mut detector = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock detector: {}", e);
        e.into_inner()
    });

    detector.clear_cache();
    tracing::info!("Service detection cache cleared");

    Ok(())
}

/// Get cache size
#[tauri::command]
pub async fn get_service_cache_size(state: State<'_, ServiceDetectorState>) -> Result<usize> {
    let detector = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock detector: {}", e);
        e.into_inner()
    });

    Ok(detector.cache_size())
}
