//! Port scanner implementation using OS-native commands

use anyhow::{Context, Result};
use std::time::Duration;
use tokio::process::Command;

use super::parser::{parse_lsof_output, parse_netstat_output};
use super::types::PortInfo;

/// Port scanner that uses OS-native commands (lsof/netstat)
pub struct PortScanner {
    platform: Platform,
}

#[derive(Debug, Clone, Copy)]
enum Platform {
    Unix, // macOS, Linux
    Windows,
}

impl PortScanner {
    /// Create a new port scanner
    pub fn new() -> Self {
        let platform = if cfg!(target_os = "windows") {
            Platform::Windows
        } else {
            Platform::Unix
        };

        Self { platform }
    }

    /// Scan all active ports
    ///
    /// Returns a list of all ports with process information.
    /// Uses lsof on Unix systems and netstat on Windows.
    pub async fn scan(&self) -> Result<Vec<PortInfo>> {
        match self.platform {
            Platform::Unix => self.scan_unix().await,
            Platform::Windows => self.scan_windows().await,
        }
    }

    /// Get information about a specific port
    pub async fn get_port_info(&self, port: u16) -> Result<Option<PortInfo>> {
        let all_ports = self.scan().await?;
        Ok(all_ports.into_iter().find(|p| p.port == port))
    }

    /// Kill process by port number
    pub async fn kill_by_port(&self, port: u16) -> Result<()> {
        let port_info = self
            .get_port_info(port)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Port {} not found", port))?;

        self.kill_process(port_info.pid).await
    }

    /// Scan using lsof (macOS/Linux)
    async fn scan_unix(&self) -> Result<Vec<PortInfo>> {
        // Execute lsof command with timeout
        let output_future = Command::new("lsof")
            .args(["-i", "-n", "-P"]) // -i: internet, -n: no DNS, -P: no port names
            .output();

        let output = tokio::time::timeout(Duration::from_secs(10), output_future)
            .await
            .context("lsof command timed out after 10 seconds")?
            .context("Failed to execute lsof. Is it installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("lsof failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_lsof_output(&stdout).context("Failed to parse lsof output")
    }

    /// Scan using netstat (Windows)
    async fn scan_windows(&self) -> Result<Vec<PortInfo>> {
        // Execute netstat command with timeout
        let output_future = Command::new("netstat")
            .args(["-ano"]) // -a: all, -n: numeric, -o: PID
            .output();

        let output = tokio::time::timeout(Duration::from_secs(10), output_future)
            .await
            .context("netstat command timed out after 10 seconds")?
            .context("Failed to execute netstat")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("netstat failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_netstat_output(&stdout).context("Failed to parse netstat output")
    }

    /// Kill a process by PID
    async fn kill_process(&self, pid: u32) -> Result<()> {
        match self.platform {
            Platform::Unix => {
                let output = Command::new("kill")
                    .arg(pid.to_string())
                    .output()
                    .await
                    .context("Failed to execute kill command")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    anyhow::bail!("Failed to kill process {}: {}", pid, stderr);
                }
            }
            Platform::Windows => {
                let output = Command::new("taskkill")
                    .args(["/PID", &pid.to_string(), "/F"])
                    .output()
                    .await
                    .context("Failed to execute taskkill command")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    anyhow::bail!("Failed to kill process {}: {}", pid, stderr);
                }
            }
        }

        Ok(())
    }
}

impl Default for PortScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_creation() {
        let scanner = PortScanner::new();

        #[cfg(target_os = "windows")]
        assert!(matches!(scanner.platform, Platform::Windows));

        #[cfg(not(target_os = "windows"))]
        assert!(matches!(scanner.platform, Platform::Unix));
    }

    #[tokio::test]
    async fn test_scan_basic() {
        let scanner = PortScanner::new();
        let result = scanner.scan().await;

        // Should not error (though may return empty on CI)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_port_info_not_found() {
        let scanner = PortScanner::new();

        // Port 64999 is unlikely to be in use
        let result = scanner.get_port_info(64999).await.unwrap();
        // Result could be None (port not found) or Some (if port happens to be in use)
        // Just verify it doesn't error
        let _ = result;
    }
}
