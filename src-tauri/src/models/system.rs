//! System monitoring data models.

use serde::{Deserialize, Serialize};

/// System-wide statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    /// CPU usage statistics.
    pub cpu: CpuStats,
    /// Memory usage statistics.
    pub memory: MemoryStats,
    /// Disk I/O statistics.
    pub disk: DiskStats,
    /// Timestamp when stats were collected.
    pub timestamp: i64,
}

/// CPU usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    /// Overall CPU usage percentage (0-100).
    pub overall: f32,
    /// Per-core CPU usage percentages.
    pub cores: Vec<f32>,
    /// Number of CPU cores.
    pub core_count: usize,
}

/// Memory usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Total memory in bytes.
    pub total: u64,
    /// Used memory in bytes.
    pub used: u64,
    /// Available memory in bytes.
    pub available: u64,
    /// Total swap in bytes.
    pub swap_total: u64,
    /// Used swap in bytes.
    pub swap_used: u64,
    /// Memory usage percentage (0-100).
    pub usage_percent: f32,
}

/// Disk I/O statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStats {
    /// Bytes read per second.
    pub read_bytes_per_sec: u64,
    /// Bytes written per second.
    pub write_bytes_per_sec: u64,
    /// Total disk space in bytes.
    pub total_space: u64,
    /// Available disk space in bytes.
    pub available_space: u64,
}

impl CpuStats {
    /// Creates a new CpuStats with zero values.
    pub fn zero(core_count: usize) -> Self {
        Self {
            overall: 0.0,
            cores: vec![0.0; core_count],
            core_count,
        }
    }
}

impl MemoryStats {
    /// Calculates usage percentage.
    pub fn calculate_usage_percent(used: u64, total: u64) -> f32 {
        if total == 0 {
            return 0.0;
        }
        (used as f64 / total as f64 * 100.0) as f32
    }

    /// Creates a new MemoryStats with calculated percentage.
    pub fn new(total: u64, used: u64, available: u64, swap_total: u64, swap_used: u64) -> Self {
        let usage_percent = Self::calculate_usage_percent(used, total);
        Self {
            total,
            used,
            available,
            swap_total,
            swap_used,
            usage_percent,
        }
    }
}

impl DiskStats {
    /// Creates a new DiskStats with zero I/O.
    pub fn zero() -> Self {
        Self {
            read_bytes_per_sec: 0,
            write_bytes_per_sec: 0,
            total_space: 0,
            available_space: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_stats_zero() {
        let stats = CpuStats::zero(4);
        assert_eq!(stats.overall, 0.0);
        assert_eq!(stats.cores.len(), 4);
        assert_eq!(stats.core_count, 4);
    }

    #[test]
    fn test_memory_stats_usage_percent() {
        let percent =
            MemoryStats::calculate_usage_percent(5 * 1024 * 1024 * 1024, 10 * 1024 * 1024 * 1024);
        assert!((percent - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_memory_stats_new() {
        let stats = MemoryStats::new(
            10 * 1024 * 1024 * 1024, // 10GB total
            8 * 1024 * 1024 * 1024,  // 8GB used
            2 * 1024 * 1024 * 1024,  // 2GB available
            4 * 1024 * 1024 * 1024,  // 4GB swap total
            1 * 1024 * 1024 * 1024,  // 1GB swap used
        );
        assert_eq!(stats.total, 10 * 1024 * 1024 * 1024);
        assert_eq!(stats.used, 8 * 1024 * 1024 * 1024);
        assert!((stats.usage_percent - 80.0).abs() < 0.1);
    }

    #[test]
    fn test_disk_stats_zero() {
        let stats = DiskStats::zero();
        assert_eq!(stats.read_bytes_per_sec, 0);
        assert_eq!(stats.write_bytes_per_sec, 0);
    }

    #[test]
    fn test_system_stats_serialization() {
        let stats = SystemStats {
            cpu: CpuStats::zero(2),
            memory: MemoryStats::new(100, 50, 50, 20, 10),
            disk: DiskStats::zero(),
            timestamp: 1234567890,
        };

        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("cpu"));
        assert!(json.contains("memory"));
        assert!(json.contains("disk"));
    }
}
