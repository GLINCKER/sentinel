# GitHub Actions Workflow Testing Guide

**Project:** Sentinel - A GLINR Product by Glincker
**Purpose:** Test GitHub Actions workflows locally before pushing

---

## Quick Start

### 1. Run Quick Tests (Recommended Before Every Commit)

```bash
./scripts/quick-test.sh
```

This runs:
- Rust formatting check
- Rust clippy linting
- Rust unit tests
- Frontend linting
- Frontend tests

**Duration:** ~2-3 minutes

---

### 2. Test Full CI Workflow with Act

```bash
./scripts/test-workflows.sh ci
```

Tests all CI jobs locally using Docker.

**Duration:** ~10-15 minutes
**Requires:** Docker Desktop running

---

### 3. Test Specific Workflows

```bash
# Quick tests only (fastest)
./scripts/test-workflows.sh quick

# CI workflow
./scripts/test-workflows.sh ci

# PR checks workflow
./scripts/test-workflows.sh pr

# Coverage workflow
./scripts/test-workflows.sh coverage

# All workflows
./scripts/test-workflows.sh all
```

---

## Available Scripts

### ./scripts/quick-test.sh

**What it does:**
- Checks Rust formatting (`cargo fmt --check`)
- Runs Rust linting (`cargo clippy`)
- Runs Rust unit tests (`cargo test`)
- Runs frontend linting (`npm run lint`)
- Runs frontend tests (`npm test`)

**When to use:** Before every commit
**Duration:** 2-3 minutes
**Requires:** Rust + Node.js (no Docker needed)

---

### ./scripts/test-workflows.sh

**What it does:**
- Tests GitHub Actions workflows using `act`
- Simulates CI environment with Docker containers
- Validates workflows before pushing to GitHub

**When to use:** Before pushing to GitHub
**Duration:** 5-30 minutes (depending on mode)
**Requires:** Docker Desktop + `act` installed

**Modes:**
- `quick` - Lint + tests only (~5 min)
- `ci` - Full CI pipeline (~10-15 min)
- `pr` - PR validation (~2 min)
- `coverage` - Code coverage (~5-10 min)
- `all` - Everything (~20-30 min)

---

## Prerequisites

### For quick-test.sh

✅ Already have if you can build the project:
- Rust (cargo, clippy, rustfmt)
- Node.js (npm)

### For test-workflows.sh

Need to install:

**1. Install Act:**
```bash
# macOS
brew install act

# Linux
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Windows
choco install act-cli
```

**2. Install Docker:**
- **macOS/Windows:** [Docker Desktop](https://www.docker.com/products/docker-desktop)
- **Linux:** [Docker Engine](https://docs.docker.com/engine/install/)

**3. Ensure Docker is running:**
```bash
docker info
# Should show Docker info, not an error
```

---

## Recommended Workflow

### Before Every Commit

```bash
# Run quick tests (no Docker needed)
./scripts/quick-test.sh
```

If this passes, your code meets basic quality standards.

---

### Before Every Push to GitHub

```bash
# Test workflows with act (requires Docker)
./scripts/test-workflows.sh ci
```

This ensures GitHub Actions will pass.

---

### Full Pre-Push Checklist

```bash
# 1. Quick tests
./scripts/quick-test.sh

# 2. Test workflows (if act + Docker installed)
./scripts/test-workflows.sh ci

# 3. Commit
git add .
git commit -m "feat: your changes"

# 4. Push
git push origin main
```

---

## Manual Act Commands

If you want to test specific jobs:

```bash
# Test Rust linting only
act push -j test-rust

# Test frontend tests only
act push -j test-frontend

# Test build only
act push -j build

# Test with verbose output
act push -j test-rust -v

# Dry run (preview only)
act push --dry-run
```

---

## Configuration

### .actrc File

The `.actrc` file configures act for M1/M2 Macs:

```ini
# Use Linux AMD64 (required for Apple Silicon)
--container-architecture linux/amd64

# Use official GitHub runner images
-P ubuntu-latest=catthehacker/ubuntu:act-latest

# Reuse containers (faster)
--reuse

# Don't pull images every time
--pull=false
```

---

## Troubleshooting

### Quick Test Issues

**Error: "cargo: command not found"**
- Install Rust: https://rustup.rs/

**Error: "npm: command not found"**
- Install Node.js: https://nodejs.org/

**Error: "Cargo.toml not found"**
- Run from project root: `cd /path/to/sentinel && ./scripts/quick-test.sh`

---

### Act/Docker Issues

**Error: "Cannot connect to Docker daemon"**
- Start Docker Desktop application
- Or run: `sudo systemctl start docker` (Linux)

**Error: "act: command not found"**
- Install act: `brew install act` (macOS)

**Slow performance on M1/M2 Mac**
- This is expected (ARM emulation)
- `.actrc` already configured for best performance

---

## What Can/Can't Be Tested Locally

### ✅ Can Test with Act

- Rust formatting, linting, tests
- Frontend linting, tests
- Build process
- Security audits
- Workflow syntax
- Job dependencies

### ❌ Can't Test Locally (Will Fail/Skip)

- Codecov uploads (requires token)
- GitHub release creation (requires GitHub API)
- Some GitHub-specific actions

**This is expected and OK!** Focus on testing your code quality, not external integrations.

---

## Examples

### Example 1: Daily Development

```bash
# Make changes
vim src-tauri/src/lib.rs

# Quick test before commit
./scripts/quick-test.sh

# If passed, commit
git commit -am "fix: bug fix"
```

---

### Example 2: Before Push

```bash
# Test workflows if you have Docker
./scripts/test-workflows.sh ci

# If passed, push
git push origin main
```

---

### Example 3: Debugging CI Failure

If GitHub Actions fails:

```bash
# Test locally to reproduce
act push -j test-rust -v

# Fix the issue
# ...

# Test again
./scripts/quick-test.sh

# Push
git push origin main
```

---

## Performance Tips

1. **Use quick-test.sh for fast feedback** (2-3 min, no Docker)
2. **Use test-workflows.sh before push** (10-15 min, requires Docker)
3. **Reuse containers** (.actrc already configured)
4. **Test only what changed** (act push -j specific-job)

---

## Summary

| Script | Duration | Requires | When to Use |
|--------|----------|----------|-------------|
| quick-test.sh | 2-3 min | Rust, Node | Before every commit |
| test-workflows.sh ci | 10-15 min | + Docker, act | Before every push |
| test-workflows.sh all | 20-30 min | + Docker, act | Before releases |

---

## Resources

- **Act Documentation:** https://github.com/nektos/act
- **Detailed Act Guide:** See [.github/ACT_TESTING.md](.github/ACT_TESTING.md)
- **Workflow Files:** See [.github/workflows/](.github/workflows/)

---

**Built with precision by Glincker (A GLINR Product)**

https://glincker.com/sentinel
