//! Log buffer implementation with circular buffer pattern.
//!
//! Efficiently stores last N log lines per process using VecDeque.
//!
//! Part of Sentinel - Your Development Guardian
//! Built by Glincker (A GLINR Product)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Maximum log lines to retain per process (10,000 lines).
const DEFAULT_MAX_LINES: usize = 10_000;

/// Log line with timestamp and stream information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogLine {
    /// UTC timestamp when log was received
    pub timestamp: DateTime<Utc>,
    /// Stream type (stdout or stderr)
    pub stream: LogStream,
    /// The actual log line content
    pub line: String,
}

/// Log stream type (stdout or stderr).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogStream {
    /// Standard output
    Stdout,
    /// Standard error
    Stderr,
}

/// Circular buffer for storing log lines.
///
/// Automatically drops oldest lines when capacity is reached.
/// Uses VecDeque for O(1) push/pop at both ends.
///
/// # Examples
/// ```
/// use sentinel::core::log_buffer::{LogBuffer, LogLine, LogStream};
/// use chrono::Utc;
///
/// let mut buffer = LogBuffer::new();
///
/// buffer.push(LogLine {
///     timestamp: Utc::now(),
///     stream: LogStream::Stdout,
///     line: "Hello, world!".to_string(),
/// });
///
/// assert_eq!(buffer.len(), 1);
/// ```
pub struct LogBuffer {
    /// Circular buffer of log lines
    lines: VecDeque<LogLine>,
    /// Maximum number of lines to retain
    max_lines: usize,
}

impl LogBuffer {
    /// Creates a new log buffer with default capacity (10,000 lines).
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_MAX_LINES)
    }

    /// Creates a new log buffer with specified capacity.
    pub fn with_capacity(max_lines: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(max_lines),
            max_lines,
        }
    }

    /// Pushes a new log line to the buffer.
    ///
    /// If buffer is at capacity, drops the oldest line (FIFO).
    pub fn push(&mut self, line: LogLine) {
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front();
        }
        self.lines.push_back(line);
    }

    /// Returns all log lines as a vector (cloned).
    pub fn get_all(&self) -> Vec<LogLine> {
        self.lines.iter().cloned().collect()
    }

    /// Returns last N log lines.
    pub fn get_last_n(&self, n: usize) -> Vec<LogLine> {
        self.lines.iter().rev().take(n).cloned().rev().collect()
    }

    /// Searches for lines containing the query string (case-insensitive).
    pub fn search(&self, query: &str) -> Vec<LogLine> {
        let query_lower = query.to_lowercase();
        self.lines
            .iter()
            .filter(|line| line.line.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }

    /// Filters logs by stream type.
    pub fn filter_by_stream(&self, stream: LogStream) -> Vec<LogLine> {
        self.lines
            .iter()
            .filter(|line| line.stream == stream)
            .cloned()
            .collect()
    }

    /// Returns the number of lines currently stored.
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Returns true if buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Clears all log lines from the buffer.
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Returns the maximum capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.max_lines
    }
}

impl Default for LogBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_log_line(content: &str, stream: LogStream) -> LogLine {
        LogLine {
            timestamp: Utc::now(),
            stream,
            line: content.to_string(),
        }
    }

    #[test]
    fn test_buffer_creation() {
        let buffer = LogBuffer::new();
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), DEFAULT_MAX_LINES);
    }

    #[test]
    fn test_push_and_get() {
        let mut buffer = LogBuffer::with_capacity(5);

        buffer.push(create_log_line("line 1", LogStream::Stdout));
        buffer.push(create_log_line("line 2", LogStream::Stderr));

        assert_eq!(buffer.len(), 2);

        let lines = buffer.get_all();
        assert_eq!(lines[0].line, "line 1");
        assert_eq!(lines[1].line, "line 2");
    }

    #[test]
    fn test_circular_buffer_overflow() {
        let mut buffer = LogBuffer::with_capacity(3);

        for i in 0..5 {
            buffer.push(create_log_line(&format!("line {}", i), LogStream::Stdout));
        }

        // Should only keep last 3 lines (2, 3, 4)
        assert_eq!(buffer.len(), 3);

        let lines = buffer.get_all();
        assert_eq!(lines[0].line, "line 2");
        assert_eq!(lines[1].line, "line 3");
        assert_eq!(lines[2].line, "line 4");
    }

    #[test]
    fn test_get_last_n() {
        let mut buffer = LogBuffer::with_capacity(10);

        for i in 0..5 {
            buffer.push(create_log_line(&format!("line {}", i), LogStream::Stdout));
        }

        let last_2 = buffer.get_last_n(2);
        assert_eq!(last_2.len(), 2);
        assert_eq!(last_2[0].line, "line 3");
        assert_eq!(last_2[1].line, "line 4");
    }

    #[test]
    fn test_search() {
        let mut buffer = LogBuffer::new();

        buffer.push(create_log_line(
            "Error: something went wrong",
            LogStream::Stderr,
        ));
        buffer.push(create_log_line(
            "Info: operation successful",
            LogStream::Stdout,
        ));
        buffer.push(create_log_line("Error: another issue", LogStream::Stderr));

        let results = buffer.search("error");
        assert_eq!(results.len(), 2);
        assert!(results[0].line.contains("Error"));
    }

    #[test]
    fn test_filter_by_stream() {
        let mut buffer = LogBuffer::new();

        buffer.push(create_log_line("stdout line 1", LogStream::Stdout));
        buffer.push(create_log_line("stderr line 1", LogStream::Stderr));
        buffer.push(create_log_line("stdout line 2", LogStream::Stdout));

        let stdout_logs = buffer.filter_by_stream(LogStream::Stdout);
        assert_eq!(stdout_logs.len(), 2);

        let stderr_logs = buffer.filter_by_stream(LogStream::Stderr);
        assert_eq!(stderr_logs.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut buffer = LogBuffer::new();

        buffer.push(create_log_line("line 1", LogStream::Stdout));
        buffer.push(create_log_line("line 2", LogStream::Stdout));

        assert_eq!(buffer.len(), 2);

        buffer.clear();
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }
}
