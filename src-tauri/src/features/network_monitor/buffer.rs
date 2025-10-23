//! Circular buffer for storing historical network data

use super::types::NetworkSnapshot;
use std::collections::VecDeque;

/// Fixed-size circular buffer for network snapshots
pub struct CircularBuffer {
    data: VecDeque<NetworkSnapshot>,
    capacity: usize,
}

impl CircularBuffer {
    /// Create a new circular buffer with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Add a snapshot to the buffer
    /// If buffer is full, removes oldest entry
    pub fn push(&mut self, snapshot: NetworkSnapshot) {
        if self.data.len() >= self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(snapshot);
    }

    /// Get all snapshots in chronological order
    pub fn get_all(&self) -> Vec<NetworkSnapshot> {
        self.data.iter().cloned().collect()
    }

    /// Get snapshots from the last N seconds
    pub fn get_last_seconds(&self, seconds: u64) -> Vec<NetworkSnapshot> {
        if self.data.is_empty() {
            return Vec::new();
        }

        let now = chrono::Utc::now();
        let cutoff = now - chrono::Duration::seconds(seconds as i64);

        self.data
            .iter()
            .filter(|snapshot| snapshot.timestamp > cutoff)
            .cloned()
            .collect()
    }

    /// Get the most recent snapshot
    pub fn get_latest(&self) -> Option<&NetworkSnapshot> {
        self.data.back()
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get buffer size
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::network_monitor::types::ProtocolStats;
    use chrono::Utc;

    fn create_test_snapshot(offset_seconds: i64) -> NetworkSnapshot {
        NetworkSnapshot {
            timestamp: Utc::now() + chrono::Duration::seconds(offset_seconds),
            total_bytes_sent: 1000,
            total_bytes_received: 2000,
            total_packets_sent: 10,
            total_packets_received: 20,
            processes: vec![],
            protocol_stats: ProtocolStats::default(),
        }
    }

    #[test]
    fn test_buffer_creation() {
        let buffer = CircularBuffer::new(100);
        assert_eq!(buffer.capacity(), 100);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_buffer_push() {
        let mut buffer = CircularBuffer::new(5);

        buffer.push(create_test_snapshot(0));
        assert_eq!(buffer.len(), 1);

        buffer.push(create_test_snapshot(0));
        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn test_buffer_overflow() {
        let mut buffer = CircularBuffer::new(3);

        buffer.push(create_test_snapshot(-3));
        buffer.push(create_test_snapshot(-2));
        buffer.push(create_test_snapshot(-1));
        assert_eq!(buffer.len(), 3);

        // Adding 4th element should remove oldest
        buffer.push(create_test_snapshot(0));
        assert_eq!(buffer.len(), 3);

        // Latest snapshot should be the most recent
        let latest = buffer.get_latest().unwrap();
        assert!(latest.timestamp > buffer.get_all()[0].timestamp);
    }

    #[test]
    fn test_get_last_seconds() {
        let mut buffer = CircularBuffer::new(10);

        // Add snapshots from 5 seconds ago to now
        buffer.push(create_test_snapshot(-5));
        buffer.push(create_test_snapshot(-3));
        buffer.push(create_test_snapshot(-1));
        buffer.push(create_test_snapshot(0));

        // Get last 2 seconds
        let recent = buffer.get_last_seconds(2);
        assert!(recent.len() >= 2); // Should have at least 2 recent entries
    }

    #[test]
    fn test_buffer_clear() {
        let mut buffer = CircularBuffer::new(5);

        buffer.push(create_test_snapshot(0));
        buffer.push(create_test_snapshot(0));
        assert_eq!(buffer.len(), 2);

        buffer.clear();
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_get_latest_empty() {
        let buffer = CircularBuffer::new(5);
        assert!(buffer.get_latest().is_none());
    }

    #[test]
    fn test_get_all() {
        let mut buffer = CircularBuffer::new(5);

        buffer.push(create_test_snapshot(-2));
        buffer.push(create_test_snapshot(-1));
        buffer.push(create_test_snapshot(0));

        let all = buffer.get_all();
        assert_eq!(all.len(), 3);

        // Should be in chronological order (oldest first)
        assert!(all[0].timestamp < all[1].timestamp);
        assert!(all[1].timestamp < all[2].timestamp);
    }
}
