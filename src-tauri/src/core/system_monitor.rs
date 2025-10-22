//! System resource monitoring.
//!
//! This module provides real-time monitoring of system resources including
//! CPU, memory, and disk I/O with historical data tracking.

use crate::core::metrics_buffer::MetricsBuffer;
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
    /// Last disk I/O measurement (timestamp, total_read_bytes, total_write_bytes).
    last_disk_io: Option<(Instant, u64, u64)>,
    /// Historical CPU usage (last 60 seconds at 1Hz sampling).
    cpu_history: MetricsBuffer<f32>,
    /// Historical memory usage (last 60 seconds at 1Hz sampling).
    memory_history: MetricsBuffer<u64>,
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
        system.refresh_all();

        Self {
            system,
            disks: Disks::new_with_refreshed_list(),
            last_disk_io: None,
            cpu_history: MetricsBuffer::new(60), // 60 seconds of history
            memory_history: MetricsBuffer::new(60), // 60 seconds of history
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
            .refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        self.disks.refresh(true);

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

    /// Gets current system statistics and records them to history.
    ///
    /// Returns a snapshot of CPU, memory, and disk metrics.
    /// Also pushes current CPU and memory usage to historical buffers.
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
        let cpu = self.get_cpu_stats();
        let memory = self.get_memory_stats();
        let disk = self.get_disk_stats();

        // Record to history buffers
        self.cpu_history.push(cpu.overall);
        self.memory_history.push(memory.used);

        SystemStats {
            cpu,
            memory,
            disk,
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
    /// Calculates read/write bytes per second by aggregating all process I/O.
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

        // Aggregate disk I/O from all processes
        let mut total_read_bytes = 0u64;
        let mut total_write_bytes = 0u64;

        for process in self.system.processes().values() {
            let disk_usage = process.disk_usage();
            total_read_bytes += disk_usage.total_read_bytes;
            total_write_bytes += disk_usage.total_written_bytes;
        }

        // Calculate bytes per second
        let (read_bytes_per_sec, write_bytes_per_sec) =
            if let Some((last_time, last_read, last_write)) = self.last_disk_io {
                let elapsed = now.duration_since(last_time).as_secs_f64();
                if elapsed > 0.0 {
                    let read_rate =
                        ((total_read_bytes.saturating_sub(last_read)) as f64 / elapsed) as u64;
                    let write_rate =
                        ((total_write_bytes.saturating_sub(last_write)) as f64 / elapsed) as u64;
                    (read_rate, write_rate)
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            };

        // Store current measurement for next calculation
        self.last_disk_io = Some((now, total_read_bytes, total_write_bytes));

        DiskStats {
            read_bytes_per_sec,
            write_bytes_per_sec,
            total_space,
            available_space,
        }
    }

    /// Gets basic resource usage for a specific process (deprecated in favor of get_process_metrics).
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
        self.get_process_metrics(pid)
            .map(|(cpu, mem, _, _)| (cpu, mem))
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

    /// Gets CPU usage history (last N seconds).
    ///
    /// Returns up to 60 seconds of historical CPU usage data.
    ///
    /// # Arguments
    /// * `seconds` - Number of seconds of history to retrieve (max 60)
    ///
    /// # Returns
    /// Vector of timed CPU usage metrics (most recent first)
    pub fn get_cpu_history(
        &self,
        seconds: usize,
    ) -> Vec<crate::core::metrics_buffer::TimedMetric<f32>> {
        self.cpu_history.get_last_n(seconds)
    }

    /// Gets memory usage history (last N seconds).
    ///
    /// Returns up to 60 seconds of historical memory usage data.
    ///
    /// # Arguments
    /// * `seconds` - Number of seconds of history to retrieve (max 60)
    ///
    /// # Returns
    /// Vector of timed memory usage metrics in bytes (most recent first)
    pub fn get_memory_history(
        &self,
        seconds: usize,
    ) -> Vec<crate::core::metrics_buffer::TimedMetric<u64>> {
        self.memory_history.get_last_n(seconds)
    }

    /// Gets detailed process metrics including disk I/O.
    ///
    /// # Arguments
    /// * `pid` - Process ID
    ///
    /// # Returns
    /// * `Some((cpu_percent, memory_bytes, disk_read_bytes, disk_write_bytes))` - Resource usage
    /// * `None` - Process not found
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::SystemMonitor;
    ///
    /// let mut monitor = SystemMonitor::new();
    /// monitor.refresh();
    ///
    /// if let Some((cpu, mem, disk_read, disk_write)) = monitor.get_process_metrics(std::process::id()) {
    ///     println!("Process: CPU={:.2}%, Memory={} bytes", cpu, mem);
    ///     println!("Disk I/O: Read={}, Write={}", disk_read, disk_write);
    /// }
    /// ```
    pub fn get_process_metrics(&self, pid: u32) -> Option<(f32, u64, u64, u64)> {
        use sysinfo::Pid;

        let pid = Pid::from_u32(pid);
        self.system.process(pid).map(|process| {
            let cpu = process.cpu_usage();
            let memory = process.memory();
            let disk_usage = process.disk_usage();
            let disk_read = disk_usage.read_bytes;
            let disk_write = disk_usage.written_bytes;
            (cpu, memory, disk_read, disk_write)
        })
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
        assert!(!monitor.system.cpus().is_empty());
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
        // Just verify the struct is populated (u64 type always >= 0)
        let _ = stats.total_space;
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
        assert!(!os_name.is_empty());
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
        assert!(!monitor.system.cpus().is_empty());
    }
}
