//! # Active Connections Module
//!
//! Provides tracking of active network connections and bandwidth monitoring.
//!
//! ## Features
//! - Cross-platform connection tracking (TCP/UDP)
//! - Process-to-connection mapping
//! - Bandwidth usage monitoring per process
//! - Connection state tracking
//!
//! ## Example
//!
//! ```rust,no_run
//! use sentinel::features::connections::ConnectionTracker;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut tracker = ConnectionTracker::new();
//!     let connections = tracker.get_connections().unwrap();
//!
//!     for conn in connections {
//!         println!("{} {}:{} -> {}:{} [{}]",
//!             conn.protocol,
//!             conn.local_address,
//!             conn.local_port,
//!             conn.remote_address,
//!             conn.remote_port,
//!             conn.state
//!         );
//!     }
//! }
//! ```

mod tracker;
mod types;

pub use tracker::ConnectionTracker;
pub use types::*;

use crate::error::Result;
use std::sync::{Arc, Mutex};
use tauri::State;

/// Application state for connection tracker
pub struct ConnectionTrackerState(pub Arc<Mutex<ConnectionTracker>>);

/// Get all active network connections
#[tauri::command]
pub async fn get_active_connections(
    state: State<'_, ConnectionTrackerState>,
) -> Result<Vec<Connection>> {
    let mut tracker = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock connection tracker: {}", e);
        e.into_inner()
    });

    tracker.get_connections()
}

/// Get connection summary statistics
#[tauri::command]
pub async fn get_connection_summary(
    state: State<'_, ConnectionTrackerState>,
) -> Result<ConnectionSummary> {
    let mut tracker = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock connection tracker: {}", e);
        e.into_inner()
    });

    tracker.get_summary()
}

/// Get top bandwidth consuming processes
#[tauri::command]
pub async fn get_bandwidth_consumers(
    state: State<'_, ConnectionTrackerState>,
    limit: Option<usize>,
) -> Result<Vec<ProcessBandwidth>> {
    let mut tracker = state.0.lock().unwrap_or_else(|e| {
        tracing::error!("Failed to lock connection tracker: {}", e);
        e.into_inner()
    });

    tracker.get_top_bandwidth_consumers(limit.unwrap_or(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_state_creation() {
        let state = ConnectionTrackerState(Arc::new(Mutex::new(ConnectionTracker::new())));
        let tracker = state.0.lock().unwrap();
        // Just ensure it doesn't panic
        drop(tracker);
    }
}
