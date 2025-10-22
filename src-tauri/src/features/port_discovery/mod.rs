//! # Port Discovery Module
//!
//! Provides automatic network port scanning and process-to-port mapping.
//!
//! ## Features
//! - Cross-platform port scanning (macOS/Linux/Windows)
//! - Process-to-port mapping
//! - Network traffic statistics
//! - No root/sudo required
//!
//! ## Example
//!
//! ```rust,no_run
//! use sentinel::features::port_discovery::PortScanner;
//!
//! #[tokio::main]
//! async fn main() {
//!     let scanner = PortScanner::new();
//!     let ports = scanner.scan().await.unwrap();
//!
//!     for port in ports {
//!         println!("{}: {} (PID {})", port.port, port.process_name, port.pid);
//!     }
//! }
//! ```

mod parser;
mod scanner;
mod types;

pub use scanner::PortScanner;
pub use types::*;

use crate::error::Result;

/// Scans all active ports and returns port-to-process mapping
#[tauri::command]
pub async fn scan_ports() -> Result<Vec<PortInfo>> {
    tracing::info!("scan_ports command called");
    let scanner = PortScanner::new();
    let result = scanner.scan().await?;
    tracing::info!("scan_ports found {} ports", result.len());
    Ok(result)
}

/// Kill process by port number
#[tauri::command]
pub async fn kill_process_by_port(port: u16) -> Result<()> {
    let scanner = PortScanner::new();
    Ok(scanner.kill_by_port(port).await?)
}

/// Get detailed information about a specific port
#[tauri::command]
pub async fn get_port_info(port: u16) -> Result<Option<PortInfo>> {
    let scanner = PortScanner::new();
    Ok(scanner.get_port_info(port).await?)
}
