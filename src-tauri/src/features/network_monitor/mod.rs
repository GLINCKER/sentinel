//! Network traffic monitoring and metrics collection
//!
//! Provides real-time network statistics, per-process bandwidth tracking,
//! and historical data buffering for visualization.

mod buffer;
mod collector;
mod types;

pub use buffer::CircularBuffer;
pub use collector::TrafficCollector;
pub use types::*;

use crate::error::Result;
use std::sync::{Arc, Mutex};
use tauri::State;

/// Application state for network monitor
pub struct NetworkMonitorState(pub Arc<Mutex<TrafficCollector>>);

/// Get current network statistics
#[tauri::command]
pub async fn get_network_stats(state: State<'_, NetworkMonitorState>) -> Result<NetworkSnapshot> {
    let mut collector = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock network collector: {}", e);
        e.into_inner()
    });

    Ok(collector.collect())
}

/// Get historical network data
#[tauri::command]
pub async fn get_network_history(
    state: State<'_, NetworkMonitorState>,
    duration_seconds: u64,
) -> Result<Vec<NetworkSnapshot>> {
    let collector = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock network collector: {}", e);
        e.into_inner()
    });

    Ok(collector.get_history(duration_seconds))
}

/// Clear network statistics history
#[tauri::command]
pub async fn clear_network_history(state: State<'_, NetworkMonitorState>) -> Result<()> {
    let mut collector = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock network collector: {}", e);
        e.into_inner()
    });

    collector.clear_history();
    Ok(())
}

/// Get per-interface network statistics
#[tauri::command]
pub async fn get_network_interfaces(
    state: State<'_, NetworkMonitorState>,
) -> Result<Vec<NetworkInterfaceStats>> {
    let mut collector = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock network collector: {}", e);
        e.into_inner()
    });

    Ok(collector.get_interfaces())
}
