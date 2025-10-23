//! Connection tracking implementation

use super::types::{Connection, ConnectionSummary, ProcessBandwidth};
use chrono::Utc;
use std::collections::HashMap;
use sysinfo::System;

/// Tracks active network connections and bandwidth usage
pub struct ConnectionTracker {
    system: System,
    /// Previous bandwidth measurements for calculating rates
    previous_measurements: HashMap<u32, (u64, u64)>, // pid -> (sent, received)
}

impl Default for ConnectionTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionTracker {
    /// Create a new connection tracker
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            previous_measurements: HashMap::new(),
        }
    }

    /// Get all active connections
    pub fn get_connections(&mut self) -> crate::error::Result<Vec<Connection>> {
        self.system.refresh_all();

        let mut connections = Vec::new();

        #[cfg(target_os = "linux")]
        {
            connections.extend(self.parse_proc_net_tcp()?);
            connections.extend(self.parse_proc_net_udp()?);
        }

        #[cfg(target_os = "macos")]
        {
            connections.extend(self.parse_netstat_macos()?);
        }

        #[cfg(target_os = "windows")]
        {
            connections.extend(self.parse_netstat_windows()?);
        }

        Ok(connections)
    }

    /// Get connection summary statistics
    pub fn get_summary(&mut self) -> crate::error::Result<ConnectionSummary> {
        let connections = self.get_connections()?;

        let total_connections = connections.len();
        let tcp_connections = connections.iter().filter(|c| c.protocol == "TCP").count();
        let udp_connections = connections.iter().filter(|c| c.protocol == "UDP").count();
        let listening_sockets = connections.iter().filter(|c| c.state == "LISTEN").count();
        let established_connections = connections
            .iter()
            .filter(|c| c.state == "ESTABLISHED")
            .count();

        Ok(ConnectionSummary {
            total_connections,
            tcp_connections,
            udp_connections,
            listening_sockets,
            established_connections,
            timestamp: Utc::now(),
        })
    }

    /// Get top bandwidth consumers
    pub fn get_top_bandwidth_consumers(
        &mut self,
        limit: usize,
    ) -> crate::error::Result<Vec<ProcessBandwidth>> {
        self.system.refresh_all();

        let mut bandwidth_stats: HashMap<u32, ProcessBandwidth> = HashMap::new();
        let connections = self.get_connections()?;

        // Group connections by PID
        let mut connections_by_pid: HashMap<u32, Vec<&Connection>> = HashMap::new();
        for conn in &connections {
            if let Some(pid) = conn.pid {
                connections_by_pid.entry(pid).or_default().push(conn);
            }
        }

        // Calculate bandwidth for each process
        for (pid, conns) in connections_by_pid {
            if let Some(process) = self.system.process(sysinfo::Pid::from_u32(pid)) {
                let process_name = process.name().to_string_lossy().to_string();

                // Get disk I/O as a proxy for network I/O (sysinfo limitation)
                let disk_usage = process.disk_usage();
                let total_bytes_sent = disk_usage.written_bytes;
                let total_bytes_received = disk_usage.read_bytes;

                // Calculate rate by comparing with previous measurement
                let (bytes_sent_per_sec, bytes_received_per_sec) =
                    if let Some((prev_sent, prev_recv)) = self.previous_measurements.get(&pid) {
                        (
                            total_bytes_sent.saturating_sub(*prev_sent),
                            total_bytes_received.saturating_sub(*prev_recv),
                        )
                    } else {
                        (0, 0)
                    };

                // Update previous measurement
                self.previous_measurements
                    .insert(pid, (total_bytes_sent, total_bytes_received));

                bandwidth_stats.insert(
                    pid,
                    ProcessBandwidth {
                        pid,
                        process_name,
                        bytes_sent_per_sec,
                        bytes_received_per_sec,
                        total_bytes_sent,
                        total_bytes_received,
                        connection_count: conns.len() as u32,
                        timestamp: Utc::now(),
                    },
                );
            }
        }

        // Sort by total bandwidth (sent + received per second) and take top N
        let mut result: Vec<ProcessBandwidth> = bandwidth_stats.into_values().collect();
        result.sort_by(|a, b| {
            let a_total = a.bytes_sent_per_sec + a.bytes_received_per_sec;
            let b_total = b.bytes_sent_per_sec + b.bytes_received_per_sec;
            b_total.cmp(&a_total)
        });
        result.truncate(limit);

        Ok(result)
    }

    /// Parse /proc/net/tcp on Linux
    #[cfg(target_os = "linux")]
    fn parse_proc_net_tcp(&self) -> crate::error::Result<Vec<Connection>> {
        use std::fs;

        let mut connections = Vec::new();

        // Parse both IPv4 and IPv6
        for (file_path, is_ipv6) in [("/proc/net/tcp", false), ("/proc/net/tcp6", true)] {
            if let Ok(content) = fs::read_to_string(file_path) {
                for line in content.lines().skip(1) {
                    // Skip header
                    if let Some(conn) = self.parse_proc_net_line(line, "TCP", is_ipv6) {
                        connections.push(conn);
                    }
                }
            }
        }

        Ok(connections)
    }

    /// Parse /proc/net/udp on Linux
    #[cfg(target_os = "linux")]
    fn parse_proc_net_udp(&self) -> crate::error::Result<Vec<Connection>> {
        use std::fs;

        let mut connections = Vec::new();

        for (file_path, is_ipv6) in [("/proc/net/udp", false), ("/proc/net/udp6", true)] {
            if let Ok(content) = fs::read_to_string(file_path) {
                for line in content.lines().skip(1) {
                    if let Some(conn) = self.parse_proc_net_line(line, "UDP", is_ipv6) {
                        connections.push(conn);
                    }
                }
            }
        }

        Ok(connections)
    }

    /// Parse a single line from /proc/net/tcp or /proc/net/udp
    #[cfg(target_os = "linux")]
    fn parse_proc_net_line(
        &self,
        line: &str,
        protocol: &str,
        _is_ipv6: bool,
    ) -> Option<Connection> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 {
            return None;
        }

        // Parse local address and port
        let local = parts[1].split(':').collect::<Vec<_>>();
        if local.len() != 2 {
            return None;
        }
        let local_address = Self::parse_hex_address(local[0]);
        let local_port = u16::from_str_radix(local[1], 16).ok()?;

        // Parse remote address and port
        let remote = parts[2].split(':').collect::<Vec<_>>();
        if remote.len() != 2 {
            return None;
        }
        let remote_address = Self::parse_hex_address(remote[0]);
        let remote_port = u16::from_str_radix(remote[1], 16).ok()?;

        // Parse state
        let state_code = u8::from_str_radix(parts[3], 16).ok()?;
        let state = Self::tcp_state_to_string(state_code);

        // Parse inode to find PID
        let inode = parts[9].parse::<u64>().ok()?;
        let (pid, process_name) = self.find_process_by_inode(inode);

        Some(Connection {
            protocol: protocol.to_string(),
            local_address,
            local_port,
            remote_address,
            remote_port,
            state,
            pid,
            process_name,
            timestamp: Utc::now(),
        })
    }

    /// Parse hex address from /proc/net format
    #[cfg(target_os = "linux")]
    fn parse_hex_address(hex: &str) -> String {
        if hex.len() == 8 {
            // IPv4 (little-endian hex)
            if let Ok(addr) = u32::from_str_radix(hex, 16) {
                let bytes = addr.to_le_bytes();
                return format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]);
            }
        }
        // IPv6 or unparseable
        hex.to_string()
    }

    /// Convert TCP state code to string
    #[cfg(target_os = "linux")]
    fn tcp_state_to_string(state: u8) -> String {
        match state {
            0x01 => "ESTABLISHED",
            0x02 => "SYN_SENT",
            0x03 => "SYN_RECV",
            0x04 => "FIN_WAIT1",
            0x05 => "FIN_WAIT2",
            0x06 => "TIME_WAIT",
            0x07 => "CLOSE",
            0x08 => "CLOSE_WAIT",
            0x09 => "LAST_ACK",
            0x0A => "LISTEN",
            0x0B => "CLOSING",
            _ => "UNKNOWN",
        }
        .to_string()
    }

    /// Find process by socket inode
    #[cfg(target_os = "linux")]
    fn find_process_by_inode(&self, inode: u64) -> (Option<u32>, Option<String>) {
        use std::fs;

        // This is a simplified implementation
        // Full implementation would scan /proc/[pid]/fd/* for socket:[inode]
        for process in self.system.processes().values() {
            let pid = process.pid().as_u32();
            let fd_path = format!("/proc/{}/fd", pid);

            if let Ok(entries) = fs::read_dir(&fd_path) {
                for entry in entries.flatten() {
                    if let Ok(link) = fs::read_link(entry.path()) {
                        if let Some(link_str) = link.to_str() {
                            if link_str == format!("socket:[{}]", inode) {
                                return (
                                    Some(pid),
                                    Some(process.name().to_string_lossy().to_string()),
                                );
                            }
                        }
                    }
                }
            }
        }

        (None, None)
    }

    /// Parse netstat output on macOS
    #[cfg(target_os = "macos")]
    fn parse_netstat_macos(&self) -> crate::error::Result<Vec<Connection>> {
        use std::process::Command;

        let output = Command::new("netstat")
            .args(["-anv", "-p", "tcp"])
            .output()?;

        let mut connections = Vec::new();

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().skip(2) {
                // Skip headers
                if let Some(conn) = self.parse_netstat_line_macos(line, "TCP") {
                    connections.push(conn);
                }
            }
        }

        // Also get UDP connections
        let output_udp = Command::new("netstat")
            .args(["-anv", "-p", "udp"])
            .output()?;

        if output_udp.status.success() {
            let stdout = String::from_utf8_lossy(&output_udp.stdout);
            for line in stdout.lines().skip(2) {
                if let Some(conn) = self.parse_netstat_line_macos(line, "UDP") {
                    connections.push(conn);
                }
            }
        }

        Ok(connections)
    }

    /// Parse a single netstat line on macOS
    #[cfg(target_os = "macos")]
    fn parse_netstat_line_macos(&self, line: &str, protocol: &str) -> Option<Connection> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            return None;
        }

        // Parse local address
        let local_parts: Vec<&str> = parts[3].rsplitn(2, '.').collect();
        if local_parts.len() != 2 {
            return None;
        }
        let local_port = local_parts[0].parse::<u16>().ok()?;
        let local_address = local_parts[1].to_string();

        // Parse remote address
        let remote_parts: Vec<&str> = parts[4].rsplitn(2, '.').collect();
        let (remote_port, remote_address) = if remote_parts.len() == 2 {
            (
                remote_parts[0].parse::<u16>().unwrap_or(0),
                remote_parts[1].to_string(),
            )
        } else {
            (0, "*".to_string())
        };

        // State is typically in parts[5] for TCP
        let state = if protocol == "TCP" && parts.len() > 5 {
            parts[5].to_string()
        } else {
            "NONE".to_string()
        };

        // Try to find PID (netstat -anv doesn't always show it)
        let (pid, process_name) = self.find_process_by_port(local_port);

        Some(Connection {
            protocol: protocol.to_string(),
            local_address,
            local_port,
            remote_address,
            remote_port,
            state,
            pid,
            process_name,
            timestamp: Utc::now(),
        })
    }

    /// Parse netstat output on Windows
    #[cfg(target_os = "windows")]
    fn parse_netstat_windows(&self) -> crate::error::Result<Vec<Connection>> {
        use std::process::Command;

        let output = Command::new("netstat").args(["-ano"]).output()?;

        let mut connections = Vec::new();

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().skip(4) {
                // Skip headers
                if let Some(conn) = self.parse_netstat_line_windows(line) {
                    connections.push(conn);
                }
            }
        }

        Ok(connections)
    }

    /// Parse a single netstat line on Windows
    #[cfg(target_os = "windows")]
    fn parse_netstat_line_windows(&self, line: &str) -> Option<Connection> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            return None;
        }

        let protocol = parts[0].to_uppercase();

        // Parse local address
        let local_parts: Vec<&str> = parts[1].rsplitn(2, ':').collect();
        if local_parts.len() != 2 {
            return None;
        }
        let local_port = local_parts[0].parse::<u16>().ok()?;
        let local_address = local_parts[1].to_string();

        // Parse remote address
        let remote_parts: Vec<&str> = parts[2].rsplitn(2, ':').collect();
        let (remote_port, remote_address) = if remote_parts.len() == 2 {
            (
                remote_parts[0].parse::<u16>().unwrap_or(0),
                remote_parts[1].to_string(),
            )
        } else {
            (0, "*".to_string())
        };

        // State
        let state = if protocol == "TCP" && parts.len() > 3 {
            parts[3].to_string()
        } else {
            "NONE".to_string()
        };

        // PID is last field
        let pid = if parts.len() > 4 {
            parts[parts.len() - 1].parse::<u32>().ok()
        } else {
            None
        };

        let process_name = pid.and_then(|p| {
            self.system
                .process(sysinfo::Pid::from_u32(p))
                .map(|proc| proc.name().to_string_lossy().to_string())
        });

        Some(Connection {
            protocol,
            local_address,
            local_port,
            remote_address,
            remote_port,
            state,
            pid,
            process_name,
            timestamp: Utc::now(),
        })
    }

    /// Find process by port (helper for macOS)
    #[cfg(target_os = "macos")]
    fn find_process_by_port(&self, port: u16) -> (Option<u32>, Option<String>) {
        use std::process::Command;

        // Use lsof to find process by port
        if let Ok(output) = Command::new("lsof")
            .args(["-nP", "-iTCP", &format!(":{}", port)])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 1 {
                        if let Ok(pid) = parts[1].parse::<u32>() {
                            let process_name = parts[0].to_string();
                            return (Some(pid), Some(process_name));
                        }
                    }
                }
            }
        }

        (None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_creation() {
        let tracker = ConnectionTracker::new();
        assert_eq!(tracker.previous_measurements.len(), 0);
    }

    #[test]
    fn test_get_connections() {
        let mut tracker = ConnectionTracker::new();
        let result = tracker.get_connections();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_summary() {
        let mut tracker = ConnectionTracker::new();
        let result = tracker.get_summary();
        assert!(result.is_ok());
        // Just verify we got a summary - no need to check >= 0 for usize
        let _ = result.unwrap();
    }

    #[test]
    fn test_get_top_bandwidth_consumers() {
        let mut tracker = ConnectionTracker::new();
        let result = tracker.get_top_bandwidth_consumers(10);
        assert!(result.is_ok());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_hex_address() {
        let addr = ConnectionTracker::parse_hex_address("0100007F");
        assert_eq!(addr, "127.0.0.1");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_tcp_state_to_string() {
        assert_eq!(ConnectionTracker::tcp_state_to_string(0x01), "ESTABLISHED");
        assert_eq!(ConnectionTracker::tcp_state_to_string(0x0A), "LISTEN");
    }
}
