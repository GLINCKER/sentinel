//! Service detector implementation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub category: ServiceCategory,
    pub port: u16,
    pub pid: u32,
    pub version: Option<String>,
    pub health: HealthStatus,
    pub description: String,
    pub docs_url: Option<String>,
    pub health_check_path: Option<String>,
    pub icon: String,
    pub detected_at: DateTime<Utc>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceCategory {
    WebFramework,
    Database,
    MessageQueue,
    Cache,
    Proxy,
    Development,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ServicePattern {
    pub name: String,
    pub category: ServiceCategory,
    pub process_patterns: Vec<String>,
    pub port_hints: Vec<u16>,
    pub command_patterns: Vec<String>,
    pub description: String,
    pub docs_url: Option<String>,
    pub health_check_path: Option<String>,
    pub icon: String,
}

pub struct ServiceDetector {
    pub(super) patterns: Vec<ServicePattern>,
    cache: HashMap<String, ServiceInfo>,
}

impl ServiceDetector {
    pub fn new() -> Self {
        Self {
            patterns: super::patterns::get_builtin_patterns(),
            cache: HashMap::new(),
        }
    }

    /// Detect service from port info
    pub fn detect(
        &mut self,
        port: u16,
        pid: u32,
        process_name: &str,
        command: Option<&str>,
    ) -> Option<ServiceInfo> {
        // Check cache first
        let cache_key = format!("{}:{}:{}", port, pid, process_name);
        if let Some(cached) = self.cache.get(&cache_key) {
            return Some(cached.clone());
        }

        // Try to match against patterns
        let mut best_match: Option<(ServicePattern, f32)> = None;
        let process_lower = process_name.to_lowercase();
        let command_lower = command.map(|c| c.to_lowercase());

        for pattern in &self.patterns {
            let mut confidence = 0.0;

            // Check port match (high confidence)
            if pattern.port_hints.contains(&port) {
                confidence += 0.4;
            }

            // Check process name match
            for proc_pattern in &pattern.process_patterns {
                if process_lower.contains(&proc_pattern.to_lowercase()) {
                    confidence += 0.3;
                    break;
                }
            }

            // Check command match if available
            if let Some(cmd) = &command_lower {
                for cmd_pattern in &pattern.command_patterns {
                    if cmd.contains(&cmd_pattern.to_lowercase()) {
                        confidence += 0.3;
                        break;
                    }
                }
            }

            // Update best match if this one is better
            if confidence > 0.3 {
                // Threshold for detection
                if let Some((_, best_confidence)) = &best_match {
                    if confidence > *best_confidence {
                        best_match = Some((pattern.clone(), confidence));
                    }
                } else {
                    best_match = Some((pattern.clone(), confidence));
                }
            }
        }

        // Create ServiceInfo from best match
        if let Some((pattern, confidence)) = best_match {
            let service_info = ServiceInfo {
                id: cache_key.clone(),
                name: pattern.name,
                category: pattern.category,
                port,
                pid,
                version: None,
                health: HealthStatus::Unknown,
                description: pattern.description,
                docs_url: pattern.docs_url,
                health_check_path: pattern.health_check_path,
                icon: pattern.icon,
                detected_at: Utc::now(),
                confidence,
            };

            // Cache the result
            self.cache.insert(cache_key, service_info.clone());
            Some(service_info)
        } else {
            None
        }
    }

    /// Clear detection cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get number of cached detections
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_nextjs() {
        let mut detector = ServiceDetector::new();
        let result = detector.detect(3000, 12345, "node", Some("next dev"));

        assert!(result.is_some());
        let service = result.unwrap();
        assert_eq!(service.name, "Next.js");
        assert_eq!(service.category, ServiceCategory::WebFramework);
        assert!(service.confidence > 0.6);
    }

    #[test]
    fn test_detect_postgres() {
        let mut detector = ServiceDetector::new();
        let result = detector.detect(5432, 54321, "postgres", None);

        assert!(result.is_some());
        let service = result.unwrap();
        assert_eq!(service.name, "PostgreSQL");
        assert_eq!(service.category, ServiceCategory::Database);
    }

    #[test]
    fn test_detect_redis() {
        let mut detector = ServiceDetector::new();
        let result = detector.detect(6379, 67890, "redis-server", None);

        assert!(result.is_some());
        let service = result.unwrap();
        assert_eq!(service.name, "Redis");
        assert_eq!(service.category, ServiceCategory::Cache);
    }

    #[test]
    fn test_detect_no_match() {
        let mut detector = ServiceDetector::new();
        let result = detector.detect(9999, 11111, "unknown-process", None);

        assert!(result.is_none());
    }

    #[test]
    fn test_cache() {
        let mut detector = ServiceDetector::new();

        // First detection
        let result1 = detector.detect(3000, 12345, "node", Some("next dev"));
        assert!(result1.is_some());
        assert_eq!(detector.cache_size(), 1);

        // Second detection should use cache
        let result2 = detector.detect(3000, 12345, "node", Some("next dev"));
        assert!(result2.is_some());
        assert_eq!(detector.cache_size(), 1);

        // Different service
        detector.detect(5432, 54321, "postgres", None);
        assert_eq!(detector.cache_size(), 2);

        // Clear cache
        detector.clear_cache();
        assert_eq!(detector.cache_size(), 0);
    }

    #[test]
    fn test_confidence_scoring() {
        let mut detector = ServiceDetector::new();

        // Perfect match: port + process + command
        let result = detector.detect(3000, 12345, "node", Some("next dev"));
        assert!(result.is_some());
        assert!(result.unwrap().confidence > 0.9);

        detector.clear_cache();

        // Good match: port + process
        let result = detector.detect(5432, 54321, "postgres", None);
        assert!(result.is_some());
        assert!(result.unwrap().confidence >= 0.7);
    }
}
