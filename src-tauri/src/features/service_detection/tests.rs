//! Unit tests for service detection module

use super::detector::{ServiceCategory, ServiceDetector};
use super::patterns::get_builtin_patterns;

#[test]
fn test_detector_creation() {
    let detector = ServiceDetector::new();
    assert!(
        detector.patterns.len() >= 20,
        "Should have at least 20 built-in patterns"
    );
}

#[test]
fn test_builtin_patterns_count() {
    let patterns = get_builtin_patterns();
    assert!(
        patterns.len() >= 20,
        "Expected at least 20 built-in patterns, got {}",
        patterns.len()
    );
}

#[test]
fn test_detect_nextjs_by_port_and_process() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(3000, 12345, "node", Some("next dev"));

    assert!(result.is_some(), "Should detect Next.js");
    let service = result.unwrap();
    assert_eq!(service.name, "Next.js");
    assert_eq!(service.port, 3000);
    assert_eq!(service.pid, 12345);
    assert!(
        service.confidence > 0.5,
        "Confidence should be > 0.5, got {}",
        service.confidence
    );
    assert_eq!(service.icon, "nextdotjs");
}

#[test]
fn test_detect_vite_by_port() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(5173, 54321, "node", Some("vite"));

    assert!(result.is_some(), "Should detect Vite");
    let service = result.unwrap();
    assert_eq!(service.name, "Vite");
    assert_eq!(service.category, ServiceCategory::Development);
    assert_eq!(service.icon, "vite");
}

#[test]
fn test_detect_postgresql_by_port_and_process() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(5432, 99999, "postgres", None);

    assert!(result.is_some(), "Should detect PostgreSQL");
    let service = result.unwrap();
    assert_eq!(service.name, "PostgreSQL");
    assert_eq!(service.category, ServiceCategory::Database);
    assert_eq!(service.port, 5432);
    assert_eq!(service.icon, "postgresql");
}

#[test]
fn test_detect_redis_by_port_and_process() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(6379, 88888, "redis-server", None);

    assert!(result.is_some(), "Should detect Redis");
    let service = result.unwrap();
    assert_eq!(service.name, "Redis");
    assert_eq!(service.category, ServiceCategory::Cache);
    assert_eq!(service.icon, "redis");
}

#[test]
fn test_detect_mongodb() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(27017, 77777, "mongod", None);

    assert!(result.is_some(), "Should detect MongoDB");
    let service = result.unwrap();
    assert_eq!(service.name, "MongoDB");
    assert_eq!(service.category, ServiceCategory::Database);
    assert_eq!(service.icon, "mongodb");
}

#[test]
fn test_detect_nginx() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(80, 11111, "nginx", None);

    assert!(result.is_some(), "Should detect nginx");
    let service = result.unwrap();
    assert_eq!(service.name, "nginx");
    assert_eq!(service.category, ServiceCategory::Proxy);
    assert_eq!(service.icon, "nginx");
}

#[test]
fn test_detect_docker() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(2375, 22222, "dockerd", None);

    assert!(result.is_some(), "Should detect Docker");
    let service = result.unwrap();
    assert_eq!(service.name, "Docker");
    assert_eq!(service.category, ServiceCategory::Development);
    assert_eq!(service.icon, "docker");
}

#[test]
fn test_detect_nodejs_generic() {
    let mut detector = ServiceDetector::new();

    // Should match generic Node.js pattern when no specific framework is detected
    let result = detector.detect(4000, 33333, "node", None);

    assert!(result.is_some(), "Should detect generic Node.js");
    let service = result.unwrap();
    assert_eq!(service.name, "Node.js");
    assert_eq!(service.icon, "nodedotjs");
}

#[test]
fn test_no_detection_for_unknown_service() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(9999, 44444, "unknown_process", None);

    assert!(result.is_none(), "Should not detect unknown service");
}

#[test]
fn test_confidence_scoring() {
    let mut detector = ServiceDetector::new();

    // Perfect match: port + process + command
    let high_confidence = detector.detect(3000, 12345, "node", Some("next dev"));
    assert!(high_confidence.is_some());
    assert!(
        high_confidence.unwrap().confidence > 0.8,
        "Should have high confidence for perfect match"
    );

    // Partial match: port + process only
    let medium_confidence = detector.detect(5432, 54321, "postgres", None);
    assert!(medium_confidence.is_some());
    assert!(
        medium_confidence.unwrap().confidence >= 0.5,
        "Should have medium confidence for partial match"
    );
}

#[test]
fn test_pattern_priority() {
    let mut detector = ServiceDetector::new();

    // Next.js should match before generic Node.js
    let result = detector.detect(3000, 12345, "node", Some("next dev"));
    assert!(result.is_some());
    assert_eq!(
        result.unwrap().name,
        "Next.js",
        "Should match specific pattern over generic"
    );
}

#[test]
fn test_all_patterns_have_required_fields() {
    let patterns = get_builtin_patterns();

    for pattern in patterns {
        assert!(!pattern.name.is_empty(), "Pattern name should not be empty");
        assert!(
            !pattern.description.is_empty(),
            "Pattern description should not be empty"
        );
        assert!(!pattern.icon.is_empty(), "Pattern icon should not be empty");
        assert!(
            !pattern.process_patterns.is_empty() || !pattern.port_hints.is_empty(),
            "Pattern {} should have either process patterns or port hints",
            pattern.name
        );
    }
}

#[test]
fn test_service_info_fields() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(5432, 99999, "postgres", None);
    assert!(result.is_some());

    let service = result.unwrap();
    assert!(!service.id.is_empty(), "Service ID should not be empty");
    assert!(!service.name.is_empty(), "Service name should not be empty");
    assert!(
        !service.description.is_empty(),
        "Service description should not be empty"
    );
    assert!(
        service.docs_url.is_some(),
        "PostgreSQL should have docs URL"
    );
    assert!(
        service.confidence > 0.0 && service.confidence <= 1.0,
        "Confidence should be between 0 and 1"
    );
}

#[test]
fn test_cache_behavior() {
    let mut detector = ServiceDetector::new();

    // First detection
    let result1 = detector.detect(5432, 99999, "postgres", None);
    assert!(result1.is_some());

    // Second detection of same service should use cache
    let result2 = detector.detect(5432, 99999, "postgres", None);
    assert!(result2.is_some());

    // Both results should be identical
    assert_eq!(result1.as_ref().unwrap().id, result2.as_ref().unwrap().id);
}

#[test]
fn test_multiple_services_different_ports() {
    let mut detector = ServiceDetector::new();

    let postgres = detector.detect(5432, 11111, "postgres", None);
    let redis = detector.detect(6379, 22222, "redis-server", None);
    let nginx = detector.detect(80, 33333, "nginx", None);

    assert!(postgres.is_some());
    assert!(redis.is_some());
    assert!(nginx.is_some());

    // All should have different IDs
    let ids = [postgres.unwrap().id, redis.unwrap().id, nginx.unwrap().id];

    let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(unique_ids.len(), 3, "All services should have unique IDs");
}

#[test]
fn test_service_categories() {
    let mut detector = ServiceDetector::new();

    let web = detector.detect(3000, 1, "node", Some("next dev"));
    assert_eq!(web.unwrap().category, ServiceCategory::WebFramework);

    let db = detector.detect(5432, 2, "postgres", None);
    assert_eq!(db.unwrap().category, ServiceCategory::Database);

    let cache = detector.detect(6379, 3, "redis-server", None);
    assert_eq!(cache.unwrap().category, ServiceCategory::Cache);

    let proxy = detector.detect(80, 4, "nginx", None);
    assert_eq!(proxy.unwrap().category, ServiceCategory::Proxy);

    let dev = detector.detect(5173, 5, "node", Some("vite"));
    assert_eq!(dev.unwrap().category, ServiceCategory::Development);
}

#[test]
fn test_express_detection() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(3000, 12345, "node", Some("express"));

    assert!(result.is_some());
    let service = result.unwrap();
    assert_eq!(service.name, "Express");
    assert_eq!(service.icon, "express");
}

#[test]
fn test_django_detection() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(8000, 12345, "python", Some("manage.py runserver"));

    assert!(result.is_some());
    let service = result.unwrap();
    assert_eq!(service.name, "Django");
    assert_eq!(service.icon, "django");
}

#[test]
fn test_flask_detection() {
    let mut detector = ServiceDetector::new();

    let result = detector.detect(5000, 12345, "python", Some("flask run"));

    assert!(result.is_some());
    let service = result.unwrap();
    assert_eq!(service.name, "Flask");
    assert_eq!(service.icon, "flask");
}
