use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test that the binary exists and shows help
#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Your Development Guardian"))
        .stdout(predicate::str::contains("Process Manager & System Monitor"));
}

/// Test that the binary shows version
#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0-alpha"));
}

/// Test init command creates a config file
#[test]
fn test_init_command_simple() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("test-config.yaml");

    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.arg("init")
        .arg(&config_path)
        .arg("--template")
        .arg("simple")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created configuration file"));

    // Verify file was created
    assert!(config_path.exists());

    // Verify content
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("my-app"));
    assert!(content.contains("node"));
}

/// Test init command with full-stack template
#[test]
fn test_init_command_fullstack() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("fullstack.yaml");

    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.arg("init")
        .arg(&config_path)
        .arg("--template")
        .arg("full-stack")
        .assert()
        .success();

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("database"));
    assert!(content.contains("backend"));
    assert!(content.contains("frontend"));
}

/// Test init command with microservices template
#[test]
fn test_init_command_microservices() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("microservices.yaml");

    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.arg("init")
        .arg(&config_path)
        .arg("--template")
        .arg("microservices")
        .assert()
        .success();

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("redis"));
    assert!(content.contains("postgres"));
    assert!(content.contains("auth-service"));
    assert!(content.contains("api-gateway"));
    assert!(content.contains("user-service"));
}

/// Test init refuses to overwrite without --force
#[test]
fn test_init_no_overwrite() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("existing.yaml");

    // Create existing file
    fs::write(&config_path, "existing content").unwrap();

    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.arg("init")
        .arg(&config_path)
        .arg("--template")
        .arg("simple")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // Verify original content unchanged
    let content = fs::read_to_string(&config_path).unwrap();
    assert_eq!(content, "existing content");
}

/// Test init with --force overwrites existing file
#[test]
fn test_init_force_overwrite() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("existing.yaml");

    // Create existing file
    fs::write(&config_path, "existing content").unwrap();

    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.arg("init")
        .arg(&config_path)
        .arg("--template")
        .arg("simple")
        .arg("--force")
        .assert()
        .success();

    // Verify content changed
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("my-app"));
}

/// Test list command with no config
#[test]
fn test_list_no_config() {
    let tmp = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.env("HOME", tmp.path()).arg("list").assert().failure();
}

/// Test add command creates config and adds process
#[test]
fn test_add_command() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join(".config/sentinel/config.yaml");

    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.env("HOME", tmp.path())
        .arg("add")
        .arg("my-process")
        .arg("echo hello")
        .arg("--auto-restart")
        .assert()
        .success()
        .stdout(predicate::str::contains("Added process 'my-process'"));

    // Verify config was created
    assert!(config_path.exists());

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("my-process"));
    assert!(content.contains("echo"));
}

/// Test add refuses to add duplicate process
#[test]
fn test_add_duplicate_process() {
    let tmp = TempDir::new().unwrap();
    std::env::set_var("HOME", tmp.path());

    // Add first process
    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.env("HOME", tmp.path())
        .arg("add")
        .arg("my-process")
        .arg("echo hello")
        .assert()
        .success();

    // Try to add duplicate
    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.env("HOME", tmp.path())
        .arg("add")
        .arg("my-process")
        .arg("echo world")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

/// Test remove command with --yes flag
#[test]
fn test_remove_command_yes() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join(".config/sentinel/config.yaml");

    // First add a process
    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.env("HOME", tmp.path())
        .arg("add")
        .arg("test-process")
        .arg("echo test")
        .assert()
        .success();

    // Then remove it
    let mut cmd = Command::cargo_bin("sentinel").unwrap();
    cmd.env("HOME", tmp.path())
        .arg("remove")
        .arg("test-process")
        .arg("--yes")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed process 'test-process'"));

    // Verify it was removed
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(!content.contains("test-process"));
}

/// Test help for each subcommand
#[test]
fn test_subcommand_help() {
    let subcommands = vec![
        "start", "stop", "restart", "status", "logs", "add", "remove", "list", "init",
    ];

    for subcommand in subcommands {
        let mut cmd = Command::cargo_bin("sentinel").unwrap();
        cmd.arg(subcommand)
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage:"));
    }
}
