use serde::{Deserialize, Serialize};

/// Port information with process details
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PortInfo {
    /// Port number (1-65535)
    pub port: u16,
    /// TCP or UDP
    pub protocol: Protocol,
    /// Process name (e.g., "node", "postgres")
    pub process_name: String,
    /// Process ID
    pub pid: u32,
    /// Port state (LISTEN, ESTABLISHED, etc.)
    pub state: PortState,
    /// Local address (e.g., "127.0.0.1", "0.0.0.0")
    pub local_address: String,
    /// Remote address (for ESTABLISHED connections)
    pub remote_address: Option<String>,
    /// Process command line (for service detection)
    pub command: Option<String>,
    /// Network traffic statistics
    pub traffic: NetworkTraffic,
}

/// Network protocol
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Protocol {
    TCP,
    UDP,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::TCP => write!(f, "TCP"),
            Protocol::UDP => write!(f, "UDP"),
        }
    }
}

/// Port/connection state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PortState {
    /// Listening for connections
    Listen,
    /// Active connection
    Established,
    /// Connection closing
    TimeWait,
    /// Waiting to close
    CloseWait,
    /// Unknown state
    Unknown,
}

impl std::fmt::Display for PortState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortState::Listen => write!(f, "LISTEN"),
            PortState::Established => write!(f, "ESTABLISHED"),
            PortState::TimeWait => write!(f, "TIME_WAIT"),
            PortState::CloseWait => write!(f, "CLOSE_WAIT"),
            PortState::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

/// Network traffic statistics
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NetworkTraffic {
    /// Total bytes sent
    pub bytes_sent: u64,
    /// Total bytes received
    pub bytes_received: u64,
    /// Total packets sent
    pub packets_sent: u64,
    /// Total packets received
    pub packets_received: u64,
    /// Number of active connections
    pub connections: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_display() {
        assert_eq!(Protocol::TCP.to_string(), "TCP");
        assert_eq!(Protocol::UDP.to_string(), "UDP");
    }

    #[test]
    fn test_port_state_display() {
        assert_eq!(PortState::Listen.to_string(), "LISTEN");
        assert_eq!(PortState::Established.to_string(), "ESTABLISHED");
    }

    #[test]
    fn test_network_traffic_default() {
        let traffic = NetworkTraffic::default();
        assert_eq!(traffic.bytes_sent, 0);
        assert_eq!(traffic.bytes_received, 0);
        assert_eq!(traffic.connections, 0);
    }
}
