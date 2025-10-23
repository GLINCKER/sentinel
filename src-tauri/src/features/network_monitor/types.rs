//! Network monitoring data types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Network snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSnapshot {
    /// Timestamp of this snapshot
    pub timestamp: DateTime<Utc>,
    /// Total bytes sent across all interfaces
    pub total_bytes_sent: u64,
    /// Total bytes received across all interfaces
    pub total_bytes_received: u64,
    /// Total packets sent
    pub total_packets_sent: u64,
    /// Total packets received
    pub total_packets_received: u64,
    /// Per-process network statistics
    pub processes: Vec<ProcessNetworkStats>,
    /// Protocol breakdown
    pub protocol_stats: ProtocolStats,
}

/// Network statistics for a single process
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessNetworkStats {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub process_name: String,
    /// Bytes sent by this process
    pub bytes_sent: u64,
    /// Bytes received by this process
    pub bytes_received: u64,
    /// Number of active connections
    pub connections: u32,
    /// Ports used by this process
    pub ports: Vec<u16>,
}

/// Protocol-level statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolStats {
    /// TCP connections count
    pub tcp_connections: u32,
    /// UDP connections count
    pub udp_connections: u32,
    /// HTTP connections (port 80, 8080, etc.)
    pub http_connections: u32,
    /// HTTPS connections (port 443)
    pub https_connections: u32,
}

/// Network interface statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterfaceStats {
    /// Interface name (e.g., en0, lo0, eth0)
    pub name: String,
    /// Total bytes transmitted
    pub bytes_sent: u64,
    /// Total bytes received
    pub bytes_received: u64,
    /// Total packets transmitted
    pub packets_sent: u64,
    /// Total packets received
    pub packets_received: u64,
    /// Errors on transmission
    pub errors_sent: u64,
    /// Errors on reception
    pub errors_received: u64,
    /// MAC address if available
    pub mac_address: Option<String>,
    /// Interface type/description
    pub interface_type: String,
    /// Is this interface active/up
    pub is_up: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_stats_default() {
        let stats = ProtocolStats::default();
        assert_eq!(stats.tcp_connections, 0);
        assert_eq!(stats.udp_connections, 0);
        assert_eq!(stats.http_connections, 0);
        assert_eq!(stats.https_connections, 0);
    }

    #[test]
    fn test_process_network_stats_creation() {
        let stats = ProcessNetworkStats {
            pid: 12345,
            process_name: "test".to_string(),
            bytes_sent: 1000,
            bytes_received: 2000,
            connections: 5,
            ports: vec![8080, 443],
        };

        assert_eq!(stats.pid, 12345);
        assert_eq!(stats.process_name, "test");
        assert_eq!(stats.bytes_sent, 1000);
        assert_eq!(stats.bytes_received, 2000);
        assert_eq!(stats.connections, 5);
        assert_eq!(stats.ports.len(), 2);
    }
}
