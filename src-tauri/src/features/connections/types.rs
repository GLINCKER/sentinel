//! Connection monitoring data types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents an active network connection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    /// Connection protocol (TCP/UDP)
    pub protocol: String,
    /// Local address
    pub local_address: String,
    /// Local port
    pub local_port: u16,
    /// Remote address
    pub remote_address: String,
    /// Remote port
    pub remote_port: u16,
    /// Connection state (ESTABLISHED, LISTEN, etc.)
    pub state: String,
    /// Process ID
    pub pid: Option<u32>,
    /// Process name
    pub process_name: Option<String>,
    /// Timestamp when this connection was observed
    pub timestamp: DateTime<Utc>,
}

/// Bandwidth usage information for a process
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessBandwidth {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub process_name: String,
    /// Bytes sent per second
    pub bytes_sent_per_sec: u64,
    /// Bytes received per second
    pub bytes_received_per_sec: u64,
    /// Total bytes sent
    pub total_bytes_sent: u64,
    /// Total bytes received
    pub total_bytes_received: u64,
    /// Number of active connections
    pub connection_count: u32,
    /// Timestamp of this measurement
    pub timestamp: DateTime<Utc>,
}

/// Summary of active connections
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionSummary {
    /// Total number of active connections
    pub total_connections: usize,
    /// Number of TCP connections
    pub tcp_connections: usize,
    /// Number of UDP connections
    pub udp_connections: usize,
    /// Number of listening sockets
    pub listening_sockets: usize,
    /// Number of established connections
    pub established_connections: usize,
    /// Timestamp of this summary
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_creation() {
        let conn = Connection {
            protocol: "TCP".to_string(),
            local_address: "127.0.0.1".to_string(),
            local_port: 8080,
            remote_address: "192.168.1.1".to_string(),
            remote_port: 443,
            state: "ESTABLISHED".to_string(),
            pid: Some(1234),
            process_name: Some("test".to_string()),
            timestamp: Utc::now(),
        };

        assert_eq!(conn.protocol, "TCP");
        assert_eq!(conn.local_port, 8080);
        assert_eq!(conn.remote_port, 443);
    }

    #[test]
    fn test_process_bandwidth_creation() {
        let bandwidth = ProcessBandwidth {
            pid: 1234,
            process_name: "test".to_string(),
            bytes_sent_per_sec: 1000,
            bytes_received_per_sec: 2000,
            total_bytes_sent: 10000,
            total_bytes_received: 20000,
            connection_count: 5,
            timestamp: Utc::now(),
        };

        assert_eq!(bandwidth.pid, 1234);
        assert_eq!(bandwidth.connection_count, 5);
    }
}
