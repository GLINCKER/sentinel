//! Platform-specific output parsers for lsof and netstat

use anyhow::{Context, Result};
use regex::Regex;

use super::types::{NetworkTraffic, PortInfo, PortState, Protocol};

/// Parse lsof output (macOS/Linux)
///
/// Example line:
/// ```text
/// node    12345 user   20u  IPv4 0x123  TCP 127.0.0.1:3000 (LISTEN)
/// node    12346 user   21u  IPv4 0x124  TCP 127.0.0.1:3000->192.168.1.5:54321 (ESTABLISHED)
/// ```
pub fn parse_lsof_output(output: &str) -> Result<Vec<PortInfo>> {
    let mut ports = Vec::new();

    // Regex for lsof output
    // Captures: process_name, pid, protocol, local_addr, port, remote_addr (optional), state
    // Format: COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME
    // Example: node    12345 user   20u  IPv4 0x123  0t0    TCP 127.0.0.1:3000 (LISTEN)
    let re = Regex::new(
        r"(?m)^(\S+)\s+(\d+)\s+\S+\s+\S+\s+IPv[46]\s+\S+\s+\S+\s+(TCP|UDP)\s+([^:]+):(\d+)(?:->([^:]+):(\d+))?\s+\(([^)]+)\)"
    ).context("Failed to compile lsof regex")?;

    for line in output.lines() {
        if let Some(port_info) = parse_lsof_line(line, &re)? {
            ports.push(port_info);
        }
    }

    Ok(ports)
}

fn parse_lsof_line(line: &str, re: &Regex) -> Result<Option<PortInfo>> {
    if line.starts_with("COMMAND") || line.trim().is_empty() {
        return Ok(None);
    }

    let Some(caps) = re.captures(line) else {
        // Skip lines that don't match (e.g., non-network entries)
        return Ok(None);
    };

    let process_name = caps.get(1).unwrap().as_str().to_string();
    let pid = caps
        .get(2)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .context("Failed to parse PID")?;
    let protocol = match caps.get(3).unwrap().as_str() {
        "TCP" => Protocol::TCP,
        "UDP" => Protocol::UDP,
        _ => return Ok(None),
    };
    let local_address = caps.get(4).unwrap().as_str().to_string();
    let port = caps
        .get(5)
        .unwrap()
        .as_str()
        .parse::<u16>()
        .context("Failed to parse port number")?;
    let remote_address = caps.get(6).map(|m| {
        let addr = m.as_str();
        let port = caps.get(7).unwrap().as_str();
        format!("{}:{}", addr, port)
    });
    let state_str = caps.get(8).unwrap().as_str();
    let state = parse_port_state(state_str);

    Ok(Some(PortInfo {
        port,
        protocol,
        process_name,
        pid,
        state,
        local_address,
        remote_address,
        command: None, // Will be enriched later with sysinfo
        traffic: NetworkTraffic::default(),
    }))
}

/// Parse netstat output (Windows)
///
/// Example line:
/// ```text
/// TCP    127.0.0.1:3000         0.0.0.0:0              LISTENING       12345
/// TCP    127.0.0.1:3000         192.168.1.5:54321      ESTABLISHED     12346
/// ```
pub fn parse_netstat_output(output: &str) -> Result<Vec<PortInfo>> {
    let mut ports = Vec::new();

    // Regex for netstat -ano output
    // Captures: protocol, local_addr, local_port, remote_addr, remote_port, state, pid
    let re = Regex::new(r"(?m)^\s*(TCP|UDP)\s+([^:]+):(\d+)\s+([^:]+):(\d+)\s+(\S+)\s+(\d+)")
        .context("Failed to compile netstat regex")?;

    for line in output.lines() {
        if let Some(port_info) = parse_netstat_line(line, &re)? {
            ports.push(port_info);
        }
    }

    Ok(ports)
}

fn parse_netstat_line(line: &str, re: &Regex) -> Result<Option<PortInfo>> {
    if line.starts_with("Active") || line.starts_with("Proto") || line.trim().is_empty() {
        return Ok(None);
    }

    let Some(caps) = re.captures(line) else {
        return Ok(None);
    };

    let protocol = match caps.get(1).unwrap().as_str() {
        "TCP" => Protocol::TCP,
        "UDP" => Protocol::UDP,
        _ => return Ok(None),
    };
    let local_address = caps.get(2).unwrap().as_str().to_string();
    let port = caps
        .get(3)
        .unwrap()
        .as_str()
        .parse::<u16>()
        .context("Failed to parse port number")?;
    let remote_addr = caps.get(4).unwrap().as_str();
    let remote_port = caps.get(5).unwrap().as_str();
    let remote_address = if remote_addr != "0.0.0.0" && remote_addr != "[::]" {
        Some(format!("{}:{}", remote_addr, remote_port))
    } else {
        None
    };
    let state_str = caps.get(6).unwrap().as_str();
    let state = parse_port_state(state_str);
    let pid = caps
        .get(7)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .context("Failed to parse PID")?;

    // Get process name from PID (Windows-specific, simplified)
    let process_name = format!("pid-{}", pid); // TODO: Use tasklist to get actual name

    Ok(Some(PortInfo {
        port,
        protocol,
        process_name,
        pid,
        state,
        local_address,
        remote_address,
        command: None, // Will be enriched later with sysinfo
        traffic: NetworkTraffic::default(),
    }))
}

fn parse_port_state(state_str: &str) -> PortState {
    match state_str.to_uppercase().as_str() {
        "LISTEN" | "LISTENING" => PortState::Listen,
        "ESTABLISHED" => PortState::Established,
        "TIME_WAIT" => PortState::TimeWait,
        "CLOSE_WAIT" => PortState::CloseWait,
        _ => PortState::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lsof_listen() {
        let output = "node    12345 user   20u  IPv4 0x123  0t0  TCP 127.0.0.1:3000 (LISTEN)";
        let result = parse_lsof_output(output).unwrap();

        assert_eq!(result.len(), 1);
        let port = &result[0];
        assert_eq!(port.port, 3000);
        assert_eq!(port.protocol, Protocol::TCP);
        assert_eq!(port.process_name, "node");
        assert_eq!(port.pid, 12345);
        assert_eq!(port.state, PortState::Listen);
        assert_eq!(port.local_address, "127.0.0.1");
        assert!(port.remote_address.is_none());
    }

    #[test]
    fn test_parse_lsof_established() {
        let output = "postgres    5432 user   21u  IPv4 0x124  0t0  TCP 127.0.0.1:5432->192.168.1.5:54321 (ESTABLISHED)";
        let result = parse_lsof_output(output).unwrap();

        assert_eq!(result.len(), 1);
        let port = &result[0];
        assert_eq!(port.port, 5432);
        assert_eq!(port.protocol, Protocol::TCP);
        assert_eq!(port.process_name, "postgres");
        assert_eq!(port.pid, 5432);
        assert_eq!(port.state, PortState::Established);
        assert_eq!(port.remote_address, Some("192.168.1.5:54321".to_string()));
    }

    #[test]
    fn test_parse_netstat_listen() {
        let output = "  TCP    127.0.0.1:3000         0.0.0.0:0              LISTENING       12345";
        let result = parse_netstat_output(output).unwrap();

        assert_eq!(result.len(), 1);
        let port = &result[0];
        assert_eq!(port.port, 3000);
        assert_eq!(port.protocol, Protocol::TCP);
        assert_eq!(port.pid, 12345);
        assert_eq!(port.state, PortState::Listen);
        assert!(port.remote_address.is_none());
    }

    #[test]
    fn test_parse_netstat_established() {
        let output = "  TCP    127.0.0.1:5432         192.168.1.5:54321      ESTABLISHED     5432";
        let result = parse_netstat_output(output).unwrap();

        assert_eq!(result.len(), 1);
        let port = &result[0];
        assert_eq!(port.port, 5432);
        assert_eq!(port.state, PortState::Established);
        assert_eq!(port.remote_address, Some("192.168.1.5:54321".to_string()));
    }

    #[test]
    fn test_parse_port_state() {
        assert_eq!(parse_port_state("LISTEN"), PortState::Listen);
        assert_eq!(parse_port_state("LISTENING"), PortState::Listen);
        assert_eq!(parse_port_state("ESTABLISHED"), PortState::Established);
        assert_eq!(parse_port_state("TIME_WAIT"), PortState::TimeWait);
        assert_eq!(parse_port_state("UNKNOWN_STATE"), PortState::Unknown);
    }

    #[test]
    fn test_parse_empty_output() {
        assert_eq!(parse_lsof_output("").unwrap().len(), 0);
        assert_eq!(parse_netstat_output("").unwrap().len(), 0);
    }

    #[test]
    fn test_parse_header_lines() {
        let lsof_output = "COMMAND     PID   USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME\nnode    12345 user   20u  IPv4 0x123  0t0  TCP 127.0.0.1:3000 (LISTEN)";
        let result = parse_lsof_output(lsof_output).unwrap();
        assert_eq!(result.len(), 1); // Header should be skipped

        let netstat_output = "Active Connections\n\n  Proto  Local Address          Foreign Address        State           PID\n  TCP    127.0.0.1:3000         0.0.0.0:0              LISTENING       12345";
        let result = parse_netstat_output(netstat_output).unwrap();
        assert_eq!(result.len(), 1); // Headers should be skipped
    }

    #[test]
    fn test_parse_real_macos_lsof() {
        // Real macOS lsof format with 0t0 column
        let output = "COMMAND     PID  USER   FD   TYPE             DEVICE SIZE/OFF   NODE NAME
node      21379 gdsks   13u  IPv6 0x4862c84cb6baeae6      0t0    TCP *:3001 (LISTEN)
node      49053 gdsks   13u  IPv6 0x2aba2dd1c8c4085d      0t0    TCP *:3002 (LISTEN)";

        let result = parse_lsof_output(output).unwrap();
        println!("Parsed {} ports", result.len());
        for port in &result {
            println!(
                "Port: {} PID: {} Process: {}",
                port.port, port.pid, port.process_name
            );
        }

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].port, 3001);
        assert_eq!(result[0].pid, 21379);
        assert_eq!(result[1].port, 3002);
        assert_eq!(result[1].pid, 49053);
    }
}
