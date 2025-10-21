/**
 * @file Performance Benchmarks
 * @glinr/sentinel
 *
 * Criterion-based performance benchmarks for Sentinel.
 * Tests startup time, memory usage, CPU usage, and stress scenarios.
 *
 * Run with: cargo bench
 *
 * Built by Glincker (A GLINR Product)
 * Copyright (c) 2025 Glincker. All rights reserved.
 *
 * @see https://glincker.com/sentinel
 */
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sentinel::core::{ConfigManager, ProcessManager, SystemMonitor};
use sentinel::models::{Config, ProcessConfig};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::tempdir;

/// Benchmark: Config file loading
fn bench_config_loading(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("bench.yaml");

    // Create a realistic config with 10 processes
    let config = Config {
        processes: (0..10)
            .map(|i| ProcessConfig {
                name: format!("process-{}", i),
                command: "echo".to_string(),
                args: vec!["hello".to_string()],
                cwd: None,
                env: HashMap::new(),
                depends_on: Vec::new(),
                auto_restart: Some(true),
                max_restarts: Some(3),
                restart_delay_ms: Some(1000),
                health_check: None,
            })
            .collect(),
        global_env: HashMap::new(),
    };

    ConfigManager::save_to_file(&config, &config_path).unwrap();

    c.bench_function("config_load_10_processes", |b| {
        b.iter(|| {
            ConfigManager::load_from_file(black_box(&config_path)).unwrap();
        });
    });
}

/// Benchmark: Config validation with different sizes
fn bench_config_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_validation");

    for size in [1, 10, 50, 100].iter() {
        let config = Config {
            processes: (0..*size)
                .map(|i| ProcessConfig {
                    name: format!("process-{}", i),
                    command: "echo".to_string(),
                    args: vec!["test".to_string()],
                    cwd: None,
                    env: HashMap::new(),
                    depends_on: Vec::new(),
                    auto_restart: None,
                    max_restarts: None,
                    restart_delay_ms: None,
                    health_check: None,
                })
                .collect(),
            global_env: HashMap::new(),
        };

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                ConfigManager::validate(black_box(&config)).unwrap();
            });
        });
    }

    group.finish();
}

/// Benchmark: Dependency cycle detection
fn bench_dependency_cycle_detection(c: &mut Criterion) {
    // Create a config with a long dependency chain (no cycle)
    let config = Config {
        processes: (0..20)
            .map(|i| ProcessConfig {
                name: format!("process-{}", i),
                command: "echo".to_string(),
                args: vec![],
                cwd: None,
                env: HashMap::new(),
                depends_on: if i > 0 {
                    vec![format!("process-{}", i - 1)]
                } else {
                    vec![]
                },
                auto_restart: None,
                max_restarts: None,
                restart_delay_ms: None,
                health_check: None,
            })
            .collect(),
        global_env: HashMap::new(),
    };

    c.bench_function("dependency_cycle_check_20_processes", |b| {
        b.iter(|| {
            ConfigManager::validate(black_box(&config)).unwrap();
        });
    });
}

/// Benchmark: System monitor initialization
fn bench_system_monitor_init(c: &mut Criterion) {
    c.bench_function("system_monitor_init", |b| {
        b.iter(|| {
            black_box(SystemMonitor::new());
        });
    });
}

/// Benchmark: System stats collection
fn bench_system_stats_collection(c: &mut Criterion) {
    let mut monitor = SystemMonitor::new();

    c.bench_function("system_stats_refresh", |b| {
        b.iter(|| {
            monitor.refresh();
            black_box(monitor.get_stats());
        });
    });
}

/// Benchmark: Process manager creation
fn bench_process_manager_init(c: &mut Criterion) {
    c.bench_function("process_manager_init", |b| {
        b.iter(|| {
            black_box(ProcessManager::new());
        });
    });
}

/// Benchmark: JSON serialization
fn bench_json_serialization(c: &mut Criterion) {
    let config = Config {
        processes: (0..10)
            .map(|i| ProcessConfig {
                name: format!("process-{}", i),
                command: "echo".to_string(),
                args: vec!["hello".to_string()],
                cwd: None,
                env: HashMap::new(),
                depends_on: Vec::new(),
                auto_restart: Some(true),
                max_restarts: Some(3),
                restart_delay_ms: Some(1000),
                health_check: None,
            })
            .collect(),
        global_env: HashMap::new(),
    };

    c.bench_function("config_to_json", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(&config).unwrap());
        });
    });
}

/// Benchmark: YAML serialization
fn bench_yaml_serialization(c: &mut Criterion) {
    let config = Config {
        processes: (0..10)
            .map(|i| ProcessConfig {
                name: format!("process-{}", i),
                command: "echo".to_string(),
                args: vec!["hello".to_string()],
                cwd: None,
                env: HashMap::new(),
                depends_on: Vec::new(),
                auto_restart: Some(true),
                max_restarts: Some(3),
                restart_delay_ms: Some(1000),
                health_check: None,
            })
            .collect(),
        global_env: HashMap::new(),
    };

    c.bench_function("config_to_yaml", |b| {
        b.iter(|| {
            black_box(serde_yaml::to_string(&config).unwrap());
        });
    });
}

/// Benchmark: Large config file loading (stress test)
fn bench_large_config_loading(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("large.yaml");

    // Create config with 100 processes
    let config = Config {
        processes: (0..100)
            .map(|i| ProcessConfig {
                name: format!("process-{}", i),
                command: "echo".to_string(),
                args: vec!["hello".to_string(), "world".to_string()],
                cwd: Some(PathBuf::from("/tmp")),
                env: {
                    let mut map = HashMap::new();
                    map.insert("VAR1".to_string(), "value1".to_string());
                    map.insert("VAR2".to_string(), "value2".to_string());
                    map
                },
                depends_on: if i > 0 {
                    vec![format!("process-{}", i - 1)]
                } else {
                    vec![]
                },
                auto_restart: Some(true),
                max_restarts: Some(5),
                restart_delay_ms: Some(2000),
                health_check: None,
            })
            .collect(),
        global_env: {
            let mut map = HashMap::new();
            map.insert("GLOBAL_VAR".to_string(), "value".to_string());
            map
        },
    };

    ConfigManager::save_to_file(&config, &config_path).unwrap();

    c.bench_function("config_load_100_processes", |b| {
        b.iter(|| {
            ConfigManager::load_from_file(black_box(&config_path)).unwrap();
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets =
        bench_config_loading,
        bench_config_validation,
        bench_dependency_cycle_detection,
        bench_system_monitor_init,
        bench_system_stats_collection,
        bench_process_manager_init,
        bench_json_serialization,
        bench_yaml_serialization,
        bench_large_config_loading,
}

criterion_main!(benches);
