//! Network traffic collection using sysinfo

use super::buffer::CircularBuffer;
use super::types::{NetworkInterfaceStats, NetworkSnapshot, ProcessNetworkStats, ProtocolStats};
use chrono::Utc;
use sysinfo::{Networks, System};

/// Collects and stores network traffic statistics
pub struct TrafficCollector {
    system: System,
    networks: Networks,
    buffer: CircularBuffer,
    last_snapshot: Option<NetworkSnapshot>,
}

impl Default for TrafficCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl TrafficCollector {
    /// Create a new traffic collector with 5-minute buffer (300 samples @ 1s interval)
    pub fn new() -> Self {
        Self::with_capacity(300)
    }

    /// Create a new traffic collector with custom buffer capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            system: System::new_all(),
            networks: Networks::new_with_refreshed_list(),
            buffer: CircularBuffer::new(capacity),
            last_snapshot: None,
        }
    }

    /// Collect current network statistics
    pub fn collect(&mut self) -> NetworkSnapshot {
        // Refresh system and network data
        self.system.refresh_all();
        self.networks.refresh(false); // Don't remove not-listed interfaces

        // Aggregate network interface statistics
        let (total_sent, total_received, total_packets_sent, total_packets_received) =
            self.aggregate_network_stats();

        // Collect per-process stats (simplified version)
        let processes = self.collect_process_stats();

        // Collect protocol stats
        let protocol_stats = self.collect_protocol_stats();

        let snapshot = NetworkSnapshot {
            timestamp: Utc::now(),
            total_bytes_sent: total_sent,
            total_bytes_received: total_received,
            total_packets_sent,
            total_packets_received,
            processes,
            protocol_stats,
        };

        // Store in buffer
        self.buffer.push(snapshot.clone());
        self.last_snapshot = Some(snapshot.clone());

        snapshot
    }

    /// Get historical snapshots for the last N seconds
    pub fn get_history(&self, seconds: u64) -> Vec<NetworkSnapshot> {
        self.buffer.get_last_seconds(seconds)
    }

    /// Clear historical data
    pub fn clear_history(&mut self) {
        self.buffer.clear();
    }

    /// Get the most recent snapshot
    pub fn get_latest(&self) -> Option<&NetworkSnapshot> {
        self.last_snapshot.as_ref()
    }

    /// Aggregate stats from all network interfaces
    fn aggregate_network_stats(&self) -> (u64, u64, u64, u64) {
        let mut total_sent = 0u64;
        let mut total_received = 0u64;
        let mut total_packets_sent = 0u64;
        let mut total_packets_received = 0u64;

        for (_interface_name, data) in self.networks.iter() {
            total_sent += data.total_transmitted();
            total_received += data.total_received();
            total_packets_sent += data.total_packets_transmitted();
            total_packets_received += data.total_packets_received();
        }

        (
            total_sent,
            total_received,
            total_packets_sent,
            total_packets_received,
        )
    }

    /// Get per-interface network statistics
    pub fn get_interfaces(&mut self) -> Vec<NetworkInterfaceStats> {
        self.networks.refresh(false);

        self.networks
            .iter()
            .map(|(name, data)| {
                let interface_type = Self::classify_interface(name);
                let is_up = data.total_transmitted() > 0 || data.total_received() > 0;

                NetworkInterfaceStats {
                    name: name.clone(),
                    bytes_sent: data.total_transmitted(),
                    bytes_received: data.total_received(),
                    packets_sent: data.total_packets_transmitted(),
                    packets_received: data.total_packets_received(),
                    errors_sent: data.total_errors_on_transmitted(),
                    errors_received: data.total_errors_on_received(),
                    mac_address: {
                        let mac = data.mac_address();
                        Some(format!(
                            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                            mac.0[0], mac.0[1], mac.0[2], mac.0[3], mac.0[4], mac.0[5]
                        ))
                    },
                    interface_type,
                    is_up,
                }
            })
            .collect()
    }

    /// Classify interface by name
    fn classify_interface(name: &str) -> String {
        if name.starts_with("lo") {
            "Loopback".to_string()
        } else if name.starts_with("en") {
            "Ethernet".to_string()
        } else if name.starts_with("wlan") || name.starts_with("wi") {
            "Wi-Fi".to_string()
        } else if name.starts_with("eth") {
            "Ethernet".to_string()
        } else if name.starts_with("utun") || name.starts_with("tun") {
            "VPN Tunnel".to_string()
        } else if name.starts_with("awdl") {
            "Apple Wireless Direct Link".to_string()
        } else if name.starts_with("anpi") {
            "Apple Network Provider Interface".to_string()
        } else if name.starts_with("gif") {
            "Generic Tunnel Interface".to_string()
        } else if name.starts_with("bridge") {
            "Network Bridge".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// Collect per-process network statistics
    /// Note: This is a simplified implementation
    /// Full implementation would parse lsof/netstat output
    fn collect_process_stats(&self) -> Vec<ProcessNetworkStats> {
        // For Phase 3D initial implementation, we return empty
        // This would be populated by parsing port discovery data
        // and correlating with bandwidth usage
        Vec::new()
    }

    /// Collect protocol-level statistics
    /// Note: This would parse connection table in full implementation
    fn collect_protocol_stats(&self) -> ProtocolStats {
        // For Phase 3D initial implementation, return default
        // Full implementation would parse netstat/lsof output
        ProtocolStats::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_creation() {
        let collector = TrafficCollector::new();
        assert_eq!(collector.buffer.capacity(), 300);
        assert!(collector.get_latest().is_none());
    }

    #[test]
    fn test_collector_with_capacity() {
        let collector = TrafficCollector::with_capacity(100);
        assert_eq!(collector.buffer.capacity(), 100);
    }

    #[test]
    fn test_collect_snapshot() {
        let mut collector = TrafficCollector::new();

        let snapshot = collector.collect();

        // Should have valid timestamp
        assert!(snapshot.timestamp <= Utc::now());

        // Should have network stats (u64 is always >= 0, just verify they exist)
        let _sent = snapshot.total_bytes_sent;
        let _received = snapshot.total_bytes_received;
    }

    #[test]
    fn test_multiple_collections() {
        let mut collector = TrafficCollector::new();

        let snap1 = collector.collect();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let snap2 = collector.collect();

        // Timestamps should be different
        assert!(snap2.timestamp > snap1.timestamp);
    }

    #[test]
    fn test_history_storage() {
        let mut collector = TrafficCollector::new();

        collector.collect();
        collector.collect();
        collector.collect();

        let history = collector.get_history(60);
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_clear_history() {
        let mut collector = TrafficCollector::new();

        collector.collect();
        collector.collect();
        assert!(collector.get_latest().is_some());

        collector.clear_history();

        let history = collector.get_history(60);
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_get_latest() {
        let mut collector = TrafficCollector::new();

        assert!(collector.get_latest().is_none());

        let snapshot = collector.collect();
        let latest = collector.get_latest().unwrap();

        assert_eq!(latest.timestamp, snapshot.timestamp);
    }

    #[test]
    fn test_buffer_overflow() {
        let mut collector = TrafficCollector::with_capacity(2);

        collector.collect();
        collector.collect();
        collector.collect(); // This should push out the oldest

        let history = collector.get_history(300);
        assert_eq!(history.len(), 2); // Buffer capacity is 2
    }
}
