//! Integration tests for Sentinel.
//!
//! These tests verify that different components work together correctly.

use sentinel::core::{ConfigManager, ProcessManager, SystemMonitor};
use sentinel::models::ProcessConfig;
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;
use tokio::time::{sleep, Duration};

/// Helper function to create a test process config.
fn test_config(name: &str, command: &str) -> ProcessConfig {
    ProcessConfig {
        name: name.to_string(),
        command: command.to_string(),
        cwd: None,
        env: HashMap::new(),
        auto_restart: false,
        restart_limit: 0,
        restart_delay: 100,
        depends_on: vec![],
    }
}

#[tokio::test]
async fn test_process_lifecycle_integration() {
    let mut manager = ProcessManager::new();

    // Start a process
    let config = test_config("test-process", "sleep 2");
    let info = manager.start(config).await.unwrap();

    assert_eq!(info.name, "test-process");
    assert!(info.is_running());
    assert!(info.pid.is_some());

    // List processes
    let list = manager.list();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "test-process");

    // Stop the process
    manager.stop("test-process").await.unwrap();

    let info = manager.get("test-process").unwrap();
    assert!(!info.is_running());
    assert!(info.pid.is_none());
}

#[tokio::test]
async fn test_multiple_processes() {
    let mut manager = ProcessManager::new();

    // Start multiple processes
    manager.start(test_config("proc1", "echo 1")).await.unwrap();
    manager.start(test_config("proc2", "echo 2")).await.unwrap();
    manager.start(test_config("proc3", "echo 3")).await.unwrap();

    // Verify all are tracked
    let list = manager.list();
    assert_eq!(list.len(), 3);

    // Stop all
    manager.stop_all().await.unwrap();

    // Verify all stopped
    for name in &["proc1", "proc2", "proc3"] {
        let info = manager.get(name).unwrap();
        assert!(!info.is_running());
    }
}

#[tokio::test]
async fn test_restart_process() {
    let mut manager = ProcessManager::new();

    let config = test_config("restart-test", "echo hello");
    manager.start(config).await.unwrap();

    let old_pid = manager.get("restart-test").unwrap().pid;

    sleep(Duration::from_millis(200)).await;

    manager.restart("restart-test").await.unwrap();

    let new_pid = manager.get("restart-test").unwrap().pid;

    // PIDs should be different (new process spawned)
    assert_ne!(old_pid, new_pid);
}

#[test]
fn test_config_load_and_validate() {
    let yaml = r#"
processes:
  - name: frontend
    command: npm start
    cwd: ./frontend
    autoRestart: true
    restartLimit: 3

  - name: backend
    command: cargo run
    cwd: ./backend
    dependsOn:
      - database

  - name: database
    command: docker-compose up postgres
    autoRestart: true

settings:
  logLevel: debug
  maxLogSize: 5242880
"#;

    let mut file = NamedTempFile::new().unwrap();
    file.write_all(yaml.as_bytes()).unwrap();

    let config = ConfigManager::load_from_file(file.path()).unwrap();

    assert_eq!(config.processes.len(), 3);
    assert_eq!(config.settings.log_level, "debug");

    // Find backend process
    let backend = config
        .processes
        .iter()
        .find(|p| p.name == "backend")
        .unwrap();

    assert_eq!(backend.depends_on, vec!["database"]);
}

#[test]
fn test_config_validation_detects_circular_deps() {
    let yaml = r#"
processes:
  - name: A
    command: echo a
    dependsOn: [B]

  - name: B
    command: echo b
    dependsOn: [A]
"#;

    let mut file = NamedTempFile::new().unwrap();
    file.write_all(yaml.as_bytes()).unwrap();

    let result = ConfigManager::load_from_file(file.path());
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.to_string().contains("cycle"));
}

#[tokio::test]
async fn test_system_monitor_integration() {
    let mut monitor = SystemMonitor::new();

    // Wait for initial measurement
    sleep(Duration::from_millis(200)).await;
    monitor.refresh();

    let stats = monitor.get_stats();

    // CPU stats
    assert!(stats.cpu.overall >= 0.0);
    assert!(stats.cpu.core_count > 0);
    assert_eq!(stats.cpu.cores.len(), stats.cpu.core_count);

    // Memory stats
    assert!(stats.memory.total > 0);
    assert!(stats.memory.used <= stats.memory.total);
    assert!(stats.memory.usage_percent >= 0.0);
    assert!(stats.memory.usage_percent <= 100.0);

    // Timestamp
    assert!(stats.timestamp > 0);
}

#[tokio::test]
async fn test_process_stats_tracking() {
    let mut manager = ProcessManager::new();
    let mut monitor = SystemMonitor::new();

    // Start a process
    let config = test_config("tracked", "sleep 5");
    let info = manager.start(config).await.unwrap();
    let pid = info.pid.unwrap();

    // Wait for process to be registered in sysinfo
    sleep(Duration::from_millis(500)).await;
    monitor.refresh();

    // Get stats for the process
    let result = monitor.get_process_stats(pid);

    if result.is_some() {
        let (cpu, memory) = result.unwrap();
        assert!(cpu >= 0.0);
        assert!(memory > 0);
    }

    // Clean up
    manager.stop("tracked").await.unwrap();
}

#[test]
fn test_config_save_and_reload() {
    let original = ConfigManager::default_config();

    let mut file = NamedTempFile::new().unwrap();
    let path = file.path().to_path_buf();
    drop(file); // Close file

    // Save
    ConfigManager::save_to_file(&original, &path).unwrap();

    // Reload
    let reloaded = ConfigManager::load_from_file(&path).unwrap();

    assert_eq!(reloaded.processes.len(), original.processes.len());
    assert_eq!(
        reloaded.processes[0].name,
        original.processes[0].name
    );
    assert_eq!(
        reloaded.processes[0].command,
        original.processes[0].command
    );
}

#[tokio::test]
async fn test_process_error_handling() {
    let mut manager = ProcessManager::new();

    // Try to start process with invalid command
    let config = test_config("invalid", "/nonexistent/command");
    let result = manager.start(config).await;
    assert!(result.is_err());

    // Try to stop non-existent process
    let result = manager.stop("nonexistent").await;
    assert!(result.is_err());

    // Try to restart non-existent process
    let result = manager.restart("nonexistent").await;
    assert!(result.is_err());
}

#[test]
fn test_system_info_retrieval() {
    let monitor = SystemMonitor::new();

    let os_name = monitor.os_name();
    assert!(os_name.is_some());

    let hostname = monitor.hostname();
    assert!(hostname.is_some());

    let uptime = monitor.uptime();
    assert!(uptime > 0);

    let process_count = monitor.process_count();
    assert!(process_count > 0);
}
