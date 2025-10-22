//! Historical metrics buffering.
//!
//! This module provides circular buffers for storing time-series metrics data.
//! Used for tracking system metrics history (CPU, memory, etc.) over time.
//!
//! Part of Sentinel - Your Development Guardian
//! Built by Glincker (A GLINR Product)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A time-series metric with timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedMetric<T> {
    /// Timestamp when the metric was recorded.
    pub timestamp: DateTime<Utc>,
    /// The metric value.
    pub value: T,
}

/// Circular buffer for storing time-series metrics.
///
/// Automatically drops oldest data when capacity is reached.
/// Optimized for real-time monitoring with O(1) push operations.
///
/// # Examples
/// ```
/// use sentinel::core::metrics_buffer::MetricsBuffer;
///
/// let mut buffer = MetricsBuffer::<f32>::new(60);
///
/// // Push metrics (oldest dropped automatically when full)
/// buffer.push(45.2);
/// buffer.push(48.7);
///
/// // Get recent data
/// let last_10 = buffer.get_last_n(10);
/// assert_eq!(last_10.len(), 2);
/// ```
pub struct MetricsBuffer<T> {
    /// Circular buffer of timed metrics.
    data: VecDeque<TimedMetric<T>>,
    /// Maximum number of data points to retain.
    max_size: usize,
}

impl<T: Clone> MetricsBuffer<T> {
    /// Creates a new metrics buffer with specified capacity.
    ///
    /// # Arguments
    /// * `max_size` - Maximum number of data points to store (typically 60 for 1-minute history at 1Hz)
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::metrics_buffer::MetricsBuffer;
    ///
    /// let buffer = MetricsBuffer::<f64>::new(60);
    /// assert_eq!(buffer.len(), 0);
    /// assert_eq!(buffer.capacity(), 60);
    /// ```
    pub fn new(max_size: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    /// Pushes a new metric value with current timestamp.
    ///
    /// If buffer is at capacity, oldest value is automatically dropped.
    ///
    /// # Arguments
    /// * `value` - The metric value to store
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::metrics_buffer::MetricsBuffer;
    ///
    /// let mut buffer = MetricsBuffer::new(3);
    /// buffer.push(1.0);
    /// buffer.push(2.0);
    /// buffer.push(3.0);
    /// buffer.push(4.0); // Oldest (1.0) is dropped
    ///
    /// assert_eq!(buffer.len(), 3);
    /// let all = buffer.get_all();
    /// assert_eq!(all[0].value, 2.0);
    /// ```
    pub fn push(&mut self, value: T) {
        if self.data.len() >= self.max_size {
            self.data.pop_front();
        }
        self.data.push_back(TimedMetric {
            timestamp: Utc::now(),
            value,
        });
    }

    /// Gets the last N metrics (most recent first).
    ///
    /// # Arguments
    /// * `n` - Number of recent metrics to retrieve
    ///
    /// # Returns
    /// Vector of up to N most recent metrics, in reverse chronological order
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::metrics_buffer::MetricsBuffer;
    ///
    /// let mut buffer = MetricsBuffer::new(10);
    /// buffer.push(1);
    /// buffer.push(2);
    /// buffer.push(3);
    ///
    /// let last_2 = buffer.get_last_n(2);
    /// assert_eq!(last_2.len(), 2);
    /// assert_eq!(last_2[0].value, 3); // Most recent
    /// assert_eq!(last_2[1].value, 2);
    /// ```
    pub fn get_last_n(&self, n: usize) -> Vec<TimedMetric<T>> {
        self.data.iter().rev().take(n).cloned().collect()
    }

    /// Gets all metrics in chronological order (oldest first).
    ///
    /// # Returns
    /// Vector of all metrics in the buffer
    ///
    /// # Examples
    /// ```
    /// use sentinel::core::metrics_buffer::MetricsBuffer;
    ///
    /// let mut buffer = MetricsBuffer::new(5);
    /// buffer.push(10);
    /// buffer.push(20);
    ///
    /// let all = buffer.get_all();
    /// assert_eq!(all.len(), 2);
    /// assert_eq!(all[0].value, 10); // Oldest
    /// assert_eq!(all[1].value, 20); // Newest
    /// ```
    pub fn get_all(&self) -> Vec<TimedMetric<T>> {
        self.data.iter().cloned().collect()
    }

    /// Returns the number of metrics currently stored.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if buffer contains no metrics.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the maximum capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.max_size
    }

    /// Clears all metrics from the buffer.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Gets metrics within a time range.
    ///
    /// # Arguments
    /// * `start` - Start of time range (inclusive)
    /// * `end` - End of time range (inclusive)
    ///
    /// # Returns
    /// Vector of metrics within the specified time range
    pub fn get_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<TimedMetric<T>> {
        self.data
            .iter()
            .filter(|m| m.timestamp >= start && m.timestamp <= end)
            .cloned()
            .collect()
    }
}

impl<T: Clone> Default for MetricsBuffer<T> {
    /// Creates a default metrics buffer with 60-second capacity.
    fn default() -> Self {
        Self::new(60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_buffer_creation() {
        let buffer = MetricsBuffer::<f64>::new(10);
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 10);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_push_and_get() {
        let mut buffer = MetricsBuffer::new(5);
        buffer.push(1.0);
        buffer.push(2.0);
        buffer.push(3.0);

        assert_eq!(buffer.len(), 3);
        assert!(!buffer.is_empty());

        let all = buffer.get_all();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].value, 1.0);
        assert_eq!(all[2].value, 3.0);
    }

    #[test]
    fn test_circular_overflow() {
        let mut buffer = MetricsBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4); // Should drop 1
        buffer.push(5); // Should drop 2

        assert_eq!(buffer.len(), 3);
        let all = buffer.get_all();
        assert_eq!(all[0].value, 3);
        assert_eq!(all[1].value, 4);
        assert_eq!(all[2].value, 5);
    }

    #[test]
    fn test_get_last_n() {
        let mut buffer = MetricsBuffer::new(10);
        for i in 0..5 {
            buffer.push(i);
        }

        let last_3 = buffer.get_last_n(3);
        assert_eq!(last_3.len(), 3);
        assert_eq!(last_3[0].value, 4); // Most recent
        assert_eq!(last_3[1].value, 3);
        assert_eq!(last_3[2].value, 2);
    }

    #[test]
    fn test_get_last_n_more_than_available() {
        let mut buffer = MetricsBuffer::new(10);
        buffer.push(1);
        buffer.push(2);

        let last_10 = buffer.get_last_n(10);
        assert_eq!(last_10.len(), 2); // Only 2 available
    }

    #[test]
    fn test_clear() {
        let mut buffer = MetricsBuffer::new(5);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(buffer.len(), 2);

        buffer.clear();
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_get_range() {
        let mut buffer = MetricsBuffer::new(10);

        let _start_time = Utc::now();
        buffer.push(1.0);

        sleep(Duration::from_millis(50));
        let mid_time = Utc::now();
        buffer.push(2.0);
        buffer.push(3.0);

        sleep(Duration::from_millis(50));
        let end_time = Utc::now();

        // Get metrics after start
        let range = buffer.get_range(mid_time, end_time);
        assert!(range.len() >= 2); // Should get the last 2-3 metrics
    }

    #[test]
    fn test_default() {
        let buffer = MetricsBuffer::<i32>::default();
        assert_eq!(buffer.capacity(), 60);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_timestamps_are_set() {
        let mut buffer = MetricsBuffer::new(3);
        let before = Utc::now();
        buffer.push(42);
        let after = Utc::now();

        let all = buffer.get_all();
        assert!(all[0].timestamp >= before);
        assert!(all[0].timestamp <= after);
    }
}
