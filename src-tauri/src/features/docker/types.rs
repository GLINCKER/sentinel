//! Docker monitoring data types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Docker container information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerInfo {
    /// Container ID (short form)
    pub id: String,
    /// Full container ID
    pub full_id: String,
    /// Container name
    pub name: String,
    /// Image name
    pub image: String,
    /// Container status (running, exited, etc.)
    pub status: String,
    /// Container state (running, paused, stopped, etc.)
    pub state: String,
    /// Port mappings
    pub ports: Vec<PortMapping>,
    /// CPU usage percentage
    pub cpu_percent: Option<f64>,
    /// Memory usage in bytes
    pub memory_usage: Option<u64>,
    /// Memory limit in bytes
    pub memory_limit: Option<u64>,
    /// Network RX bytes
    pub network_rx_bytes: Option<u64>,
    /// Network TX bytes
    pub network_tx_bytes: Option<u64>,
    /// Container created timestamp
    pub created: DateTime<Utc>,
    /// Labels
    pub labels: Vec<(String, String)>,
}

/// Port mapping information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortMapping {
    /// Container port
    pub container_port: u16,
    /// Host port (if mapped)
    pub host_port: Option<u16>,
    /// Protocol (tcp/udp)
    pub protocol: String,
    /// Host IP (if bound to specific IP)
    pub host_ip: Option<String>,
}

/// Docker container statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStats {
    /// Container ID
    pub container_id: String,
    /// CPU usage percentage
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Memory limit in bytes
    pub memory_limit: u64,
    /// Memory usage percentage
    pub memory_percent: f64,
    /// Network RX bytes
    pub network_rx_bytes: u64,
    /// Network TX bytes
    pub network_tx_bytes: u64,
    /// Block I/O read bytes
    pub block_io_read: u64,
    /// Block I/O write bytes
    pub block_io_write: u64,
    /// Number of PIDs
    pub pids: u64,
    /// Timestamp of these stats
    pub timestamp: DateTime<Utc>,
}

/// Docker system information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerInfo {
    /// Docker daemon available
    pub available: bool,
    /// Docker version
    pub version: Option<String>,
    /// Number of containers
    pub containers_count: Option<usize>,
    /// Number of running containers
    pub containers_running: Option<usize>,
    /// Number of paused containers
    pub containers_paused: Option<usize>,
    /// Number of stopped containers
    pub containers_stopped: Option<usize>,
    /// Number of images
    pub images_count: Option<usize>,
    /// Server version
    pub server_version: Option<String>,
    /// Operating system
    pub operating_system: Option<String>,
    /// Architecture
    pub architecture: Option<String>,
}

/// Docker image information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    /// Image ID (short form)
    pub id: String,
    /// Full image ID
    pub full_id: String,
    /// Repository tags (e.g., ["nginx:latest", "nginx:1.21"])
    pub repo_tags: Vec<String>,
    /// Repository digests
    pub repo_digests: Vec<String>,
    /// Image size in bytes
    pub size: u64,
    /// Image created timestamp
    pub created: DateTime<Utc>,
    /// Labels
    pub labels: Vec<(String, String)>,
}

/// Result of a container operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerOperationResult {
    /// Whether the operation succeeded
    pub success: bool,
    /// Container ID that was operated on
    pub container_id: String,
    /// Operation that was performed
    pub operation: String,
    /// Error message if failed
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_info_creation() {
        let info = ContainerInfo {
            id: "abc123".to_string(),
            full_id: "abc123def456".to_string(),
            name: "test-container".to_string(),
            image: "nginx:latest".to_string(),
            status: "Up 2 hours".to_string(),
            state: "running".to_string(),
            ports: vec![],
            cpu_percent: Some(5.5),
            memory_usage: Some(1024 * 1024),
            memory_limit: Some(512 * 1024 * 1024),
            network_rx_bytes: Some(1000),
            network_tx_bytes: Some(2000),
            created: Utc::now(),
            labels: vec![],
        };

        assert_eq!(info.name, "test-container");
        assert_eq!(info.state, "running");
    }

    #[test]
    fn test_port_mapping_creation() {
        let port = PortMapping {
            container_port: 80,
            host_port: Some(8080),
            protocol: "tcp".to_string(),
            host_ip: Some("0.0.0.0".to_string()),
        };

        assert_eq!(port.container_port, 80);
        assert_eq!(port.host_port, Some(8080));
    }

    #[test]
    fn test_docker_info_unavailable() {
        let info = DockerInfo {
            available: false,
            version: None,
            containers_count: None,
            containers_running: None,
            containers_paused: None,
            containers_stopped: None,
            images_count: None,
            server_version: None,
            operating_system: None,
            architecture: None,
        };

        assert!(!info.available);
    }
}
