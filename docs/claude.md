# Sentinel Development Standards

**Version**: 1.0  
**Last Updated**: 2025-01-XX  
**Purpose**: Define code, documentation, and testing standards for Sentinel

---

## 1. Code Standards

### Naming Conventions
```
Files:       snake_case.rs, kebab-case.tsx
Functions:   camelCase() or snake_case() (language dependent)
Classes:     PascalCase
Constants:   SCREAMING_SNAKE_CASE
Private:     _leadingUnderscore
```

### File Organization
```
src/
├── core/          # Core business logic
├── cli/           # CLI interface
├── gui/           # Desktop GUI
├── utils/         # Shared utilities
└── types/         # Type definitions
```

### Comments
- **What needs comments**: Complex algorithms, non-obvious decisions, workarounds
- **What doesn't**: Self-explanatory code
- **Format**: `// Single line` or `/* Multi-line */`

### Error Handling
```typescript
// ✅ Good: Descriptive errors
throw new Error("Failed to start process 'api': Port 3000 already in use");

// ❌ Bad: Generic errors
throw new Error("Error");
```

---

## 2. Documentation Standards

### Function Documentation
Every public function must have:
- Description (what it does)
- Parameters (type + description)
- Return value
- Example usage
- Errors thrown

Example:
```rust
/// Starts a process with the given configuration.
///
/// # Arguments
/// * `config` - Process configuration including name, command, and options
///
/// # Returns
/// * `Ok(ProcessHandle)` - Handle to the running process
/// * `Err(ProcessError)` - If process fails to start
///
/// # Example
/// ```
/// let handle = start_process(&config)?;
/// ```
pub fn start_process(config: &ProcessConfig) -> Result<ProcessHandle> {
    // ...
}
```

### Module Documentation
- Top of each module: Purpose, responsibilities, usage example

### API Documentation
- Auto-generate from inline docs
- Keep docs close to code

---

## 3. Testing Standards

### Test Coverage
- **Minimum**: 90% line coverage
- **Target**: 95% line coverage
- **Critical paths**: 100% coverage (process lifecycle, config parsing)

### Test Organization
```
tests/
├── unit/          # Test individual functions
├── integration/   # Test component interactions
└── e2e/           # Test full workflows
```

### Test Naming
```rust
#[test]
fn test_process_starts_successfully() { }

#[test]
fn test_process_fails_when_port_in_use() { }

#[test]
fn test_config_validation_rejects_invalid_yaml() { }
```

### What to Test
✅ Public APIs  
✅ Error conditions  
✅ Edge cases  
✅ Integration points  
❌ Private implementation details  
❌ External dependencies (mock them)

### Running Tests
```bash
# Run all tests
make test

# Run with coverage
make test-coverage

# Run specific test
make test-unit NAME=process_manager
```

---

## 4. Git Workflow

### Branch Naming
```
feature/process-manager
fix/memory-leak
docs/api-reference
refactor/config-system
```

### Commit Messages
```
feat: Add process auto-restart functionality
fix: Resolve memory leak in log viewer
docs: Update installation instructions
test: Add integration tests for CLI
refactor: Simplify config validation logic
```

### PR Requirements
- ✅ All tests pass
- ✅ Code coverage ≥ 90%
- ✅ Documentation updated
- ✅ No linter warnings
- ✅ Changelog entry added

---

## 5. Security Checklist

### Input Validation
- [ ] Validate all user input (config files, CLI args)
- [ ] Sanitize process commands (prevent injection)
- [ ] Validate file paths (no traversal)

### Privilege Management
- [ ] Request minimum necessary privileges
- [ ] Drop privileges when not needed
- [ ] Never run processes as root

### Dependencies
- [ ] Audit dependencies weekly (`cargo audit`, `npm audit`)
- [ ] Use known-good versions
- [ ] Lock dependencies (Cargo.lock, package-lock.json)

### Data Handling
- [ ] No secrets in logs
- [ ] Secure temp file creation
- [ ] Clean up resources on exit

---

## 6. Performance Requirements

### Startup Time
- **Target**: < 1.5 seconds
- **Max**: 2 seconds
- **Test**: `time sentinel start`

### Memory Usage
- **Idle**: < 50MB
- **10 processes**: < 100MB
- **100 processes**: < 250MB

### CPU Usage
- **Idle**: < 2%
- **Active monitoring**: < 5%

---

## 7. Build & Release

### Version Numbering
- Follow Semantic Versioning (semver)
- Format: `MAJOR.MINOR.PATCH`
- Example: `1.0.0`, `1.1.0`, `1.1.1`

### Release Checklist
- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] Version bumped
- [ ] Binaries built for all platforms
- [ ] Release notes written
- [ ] GitHub release created
- [ ] Packages published (brew, npm, cargo)

---

## 8. Code Review Guidelines

### Reviewers Check
- [ ] Code follows standards
- [ ] Tests are comprehensive
- [ ] Documentation is clear
- [ ] No security issues
- [ ] Performance is acceptable
- [ ] Changes are necessary

### Authors Provide
- Clear PR description
- Link to related issue
- Screenshots (if UI change)
- Performance benchmarks (if applicable)

---

**Remember**: These standards ensure Sentinel remains maintainable, secure, and performant. When in doubt, prioritize security and user experience.