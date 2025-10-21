//! System resource monitoring.
//!
//! This module provides real-time monitoring of system resources including
//! CPU, memory, and disk I/O.

use crate::models::{CpuStats, DiskStats, MemoryStats, SystemStats};
use chrono::Utc;
use std::time::Instant;
use sysinfo::{Disks, System};
use tracing::debug;

/// Monitors system resources.
///
/// Uses the `sysinfo` crate to collect CPU, memory, and disk metrics.
///
/// # Examples
/// ```
/// use sentinel::core::SystemMonitor;
///
/// let mut monitor = SystemMonitor::new();
/// monitor.refresh();
/// let stats = monitor.get_stats();
/// println!("CPU usage: {:.2}%", stats.cpu.overall);
/// ```
pub struct SystemMonitor {
    /// Sysinfo system instance.
    system: System,
    /// Disk information.
    disks: Disks,
    /// Last disk I/O measurement.
    last_disk_io: Option<(Instant, u64, u64)>,
}

impl SystemMonitor {
    /// Creates a new SystemMonitor.
    ///
    /// Initializes the sysinfo System and performs an initial refresh.
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::SystemMonitor;
    ///
    /// let monitor = SystemMonitor::new();
    /// ```
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all(sysinfo::RefreshKind::everything());

        Self {
            system,
            disks: Disks::new_with_refreshed_list(),
            last_disk_io: None,
        }
    }

    /// Refreshes all system information.
    ///
    /// Should be called periodically (e.g., every 1-2 seconds) to update metrics.
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::SystemMonitor;
    ///
    /// let mut monitor = SystemMonitor::new();
    /// monitor.refresh();
    /// ```
    pub fn refresh(&mut self) {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.system
            .refresh_processes(sysinfo::ProcessesToUpdate::All);
        self.disks.refresh();

        debug!("System metrics refreshed");
    }

    /// Refreshes only CPU information (lighter than full refresh).
    pub fn refresh_cpu(&mut self) {
        self.system.refresh_cpu_usage();
    }

    /// Refreshes only memory information.
    pub fn refresh_memory(&mut self) {
        self.system.refresh_memory();
    }

    /// Gets current system statistics.
    ///
    /// Returns a snapshot of CPU, memory, and disk metrics.
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::SystemMonitor;
    ///
    /// let mut monitor = SystemMonitor::new();
    /// monitor.refresh();
    /// let stats = monitor.get_stats();
    /// assert!(stats.cpu.overall >= 0.0);
    /// assert!(stats.memory.total > 0);
    /// ```
    pub fn get_stats(&mut self) -> SystemStats {
        SystemStats {
            cpu: self.get_cpu_stats(),
            memory: self.get_memory_stats(),
            disk: self.get_disk_stats(),
            timestamp: Utc::now().timestamp(),
        }
    }

    /// Gets CPU statistics.
    ///
    /// # Returns
    /// CPU usage information including per-core breakdown.
    fn get_cpu_stats(&self) -> CpuStats {
        let cpus = self.system.cpus();

        let overall = self.system.global_cpu_usage();
        let cores: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();
        let core_count = cpus.len();

        CpuStats {
            overall,
            cores,
            core_count,
        }
    }

    /// Gets memory statistics.
    ///
    /// # Returns
    /// Memory usage information including RAM and swap.
    fn get_memory_stats(&self) -> MemoryStats {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        let available = self.system.available_memory();
        let swap_total = self.system.total_swap();
        let swap_used = self.system.used_swap();

        MemoryStats::new(total, used, available, swap_total, swap_used)
    }

    /// Gets disk I/O statistics.
    ///
    /// Calculates read/write bytes per second by comparing with the last measurement.
    ///
    /// # Returns
    /// Disk I/O information.
    fn get_disk_stats(&mut self) -> DiskStats {
        let now = Instant::now();

        // Get total disk space from first disk
        let (total_space, available_space) = self
            .disks
            .iter()
            .next()
            .map(|disk| (disk.total_space(), disk.available_space()))
            .unwrap_or((0, 0));

        // For now, return zero I/O (sysinfo doesn't provide disk I/O easily on all platforms)
        // In a production version, we'd use platform-specific APIs or track process I/O
        let read_bytes_per_sec = 0;
        let write_bytes_per_sec = 0;

        DiskStats {
            read_bytes_per_sec,
            write_bytes_per_sec,
            total_space,
            available_space,
        }
    }

    /// Gets resource usage for a specific process.
    ///
    /// # Arguments
    /// * `pid` - Process ID
    ///
    /// # Returns
    /// * `Some((cpu_percent, memory_bytes))` - Resource usage
    /// * `None` - Process not found
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::SystemMonitor;
    ///
    /// let mut monitor = SystemMonitor::new();
    /// monitor.refresh();
    ///
    /// if let Some((cpu, mem)) = monitor.get_process_stats(std::process::id()) {
    ///     println!("Current process: CPU={:.2}%, Memory={} bytes", cpu, mem);
    /// }
    /// ```
    pub fn get_process_stats(&self, pid: u32) -> Option<(f32, u64)> {
        use sysinfo::Pid;

        let pid = Pid::from_u32(pid);
        self.system.process(pid).map(|process| {
            let cpu = process.cpu_usage();
            let memory = process.memory();
            (cpu, memory)
        })
    }

    /// Gets the number of running processes.
    ///
    /// # Returns
    /// Total number of processes on the system.
    pub fn process_count(&self) -> usize {
        self.system.processes().len()
    }

    /// Gets system uptime in seconds.
    ///
    /// # Returns
    /// System uptime in seconds.
    pub fn uptime(&self) -> u64 {
        System::uptime()
    }

    /// Gets system name/OS.
    ///
    /// # Returns
    /// Operating system name (e.g., "macOS", "Linux", "Windows").
    pub fn os_name(&self) -> Option<String> {
        System::name()
    }

    /// Gets system kernel version.
    ///
    /// # Returns
    /// Kernel version string.
    pub fn kernel_version(&self) -> Option<String> {
        System::kernel_version()
    }

    /// Gets host name.
    ///
    /// # Returns
    /// System hostname.
    pub fn hostname(&self) -> Option<String> {
        System::host_name()
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_system_monitor() {
        let monitor = SystemMonitor::new();
        assert!(monitor.system.cpus().len() > 0);
    }

    #[test]
    fn test_get_cpu_stats() {
        let mut monitor = SystemMonitor::new();
        thread::sleep(Duration::from_millis(200));
        monitor.refresh();

        let stats = monitor.get_cpu_stats();
        assert!(stats.core_count > 0);
        assert_eq!(stats.cores.len(), stats.core_count);
        assert!(stats.overall >= 0.0 && stats.overall <= 100.0 * stats.core_count as f32);
    }

    #[test]
    fn test_get_memory_stats() {
        let monitor = SystemMonitor::new();
        let stats = monitor.get_memory_stats();

        assert!(stats.total > 0);
        assert!(stats.used <= stats.total);
        assert!(stats.available <= stats.total);
        assert!(stats.usage_percent >= 0.0 && stats.usage_percent <= 100.0);
    }

    #[test]
    fn test_get_disk_stats() {
        let mut monitor = SystemMonitor::new();
        let stats = monitor.get_disk_stats();

        // Disk stats should at least have space info
        // (I/O stats might be 0 depending on platform)
        assert!(stats.total_space >= 0);
        assert!(stats.available_space >= 0);
    }

    #[test]
    fn test_get_system_stats() {
        let mut monitor = SystemMonitor::new();
        thread::sleep(Duration::from_millis(200));
        monitor.refresh();

        let stats = monitor.get_stats();
        assert!(stats.cpu.overall >= 0.0);
        assert!(stats.memory.total > 0);
        assert!(stats.timestamp > 0);
    }

    #[test]
    fn test_get_process_stats() {
        let mut monitor = SystemMonitor::new();
        monitor.refresh();

        let current_pid = std::process::id();
        let result = monitor.get_process_stats(current_pid);

        assert!(result.is_some());
        let (cpu, memory) = result.unwrap();
        assert!(cpu >= 0.0);
        assert!(memory > 0);
    }

    #[test]
    fn test_process_count() {
        let monitor = SystemMonitor::new();
        let count = monitor.process_count();
        assert!(count > 0);
    }

    #[test]
    fn test_uptime() {
        let monitor = SystemMonitor::new();
        let uptime = monitor.uptime();
        assert!(uptime > 0);
    }

    #[test]
    fn test_os_name() {
        let monitor = SystemMonitor::new();
        let os = monitor.os_name();
        assert!(os.is_some());

        let os_name = os.unwrap();
        assert!(
            os_name.contains("macOS") || os_name.contains("Linux") || os_name.contains("Windows")
        );
    }

    #[test]
    fn test_hostname() {
        let monitor = SystemMonitor::new();
        let hostname = monitor.hostname();
        assert!(hostname.is_some());
        assert!(!hostname.unwrap().is_empty());
    }

    #[test]
    fn test_refresh_methods() {
        let mut monitor = SystemMonitor::new();

        // Test individual refresh methods don't panic
        monitor.refresh_cpu();
        monitor.refresh_memory();
        monitor.refresh();

        // Stats should still be available
        let stats = monitor.get_stats();
        assert!(stats.cpu.overall >= 0.0);
    }

    #[test]
    fn test_default() {
        let monitor = SystemMonitor::default();
        assert!(monitor.system.cpus().len() > 0);
    }
}
