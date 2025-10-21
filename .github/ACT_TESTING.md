# Testing GitHub Actions Locally with Act

This guide explains how to test GitHub Actions workflows locally before pushing to GitHub.

---

## Installing Act

### macOS (Homebrew)

```bash
brew install act
```

### Linux

```bash
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
```

### Windows (Chocolatey)

```bash
choco install act-cli
```

### Manual Installation

Download from [Releases](https://github.com/nektos/act/releases) and add to PATH.

---

## Prerequisites

**Docker must be installed and running.**

- **macOS/Windows:** Docker Desktop
- **Linux:** Docker Engine

---

## Basic Usage

### Run All Push Workflows

```bash
act push
```

### Run All Pull Request Workflows

```bash
act pull_request
```

### Run Specific Workflow

```bash
act -W .github/workflows/ci.yml
```

### Run Specific Job

```bash
act -j test-rust
```

### List Available Workflows

```bash
act -l
```

---

## Testing Sentinel Workflows

### 1. Test CI Workflow

```bash
# Test all CI jobs
act push -W .github/workflows/ci.yml

# Test specific job
act push -j lint-rust -W .github/workflows/ci.yml
```

### 2. Test PR Checks Workflow

```bash
# Test PR validation
act pull_request -W .github/workflows/pr-checks.yml
```

### 3. Test Coverage Workflow

```bash
act push -W .github/workflows/coverage.yml
```

### 4. Test Release Workflow (Dry Run)

```bash
# Note: This won't actually create a release
act -W .github/workflows/release.yml
```

---

## Configuration

### Use Custom Runner Image

Act uses Docker images to simulate GitHub runners. By default, it uses smaller images.

**Recommended for Sentinel:**

```bash
# Use GitHub's official images (larger, more accurate)
act -P ubuntu-latest=catthehacker/ubuntu:full-latest
```

### Create .actrc Configuration

Create `.actrc` in the project root:

```bash
# Use official images
-P ubuntu-latest=catthehacker/ubuntu:full-latest
-P macos-latest=catthehacker/ubuntu:runner-latest
-P windows-latest=catthehacker/ubuntu:runner-latest

# Reuse existing Docker containers
--reuse

# Bind workspace volume
--bind
```

---

## Secrets

### Set Secrets for Local Testing

Create `.secrets` file (DO NOT COMMIT):

```bash
GITHUB_TOKEN=your_github_token
CODECOV_TOKEN=your_codecov_token
```

Use with:

```bash
act --secret-file .secrets
```

### Individual Secrets

```bash
act -s GITHUB_TOKEN=ghp_xxx
```

---

## Common Workflows

### Test Before Commit

```bash
# Run linting and tests
act push -j lint-rust -j lint-frontend -j test-rust -j test-frontend
```

### Test Full CI Pipeline

```bash
# Run entire CI workflow
act push -W .github/workflows/ci.yml
```

### Debug Workflow

```bash
# Enable verbose output
act push -v

# Enable debugging
act push --verbose
```

---

## Troubleshooting

### Issue: "Error: Cannot connect to Docker daemon"

**Solution:** Ensure Docker is running.

```bash
# macOS/Windows
# Start Docker Desktop

# Linux
sudo systemctl start docker
```

### Issue: "Disk space issues"

**Solution:** Clean up Docker images.

```bash
docker system prune -a
```

### Issue: "Workflow runs different locally vs GitHub"

**Cause:** Act uses different runner images by default.

**Solution:** Use official images.

```bash
act -P ubuntu-latest=catthehacker/ubuntu:full-latest
```

### Issue: "Command not found in workflow"

**Cause:** Tool not installed in Docker image.

**Solution:** Use full image or modify workflow to install dependencies.

---

## Limitations

1. **Act doesn't fully replicate GitHub Actions:**
   - Some GitHub-specific features unavailable
   - Different runner environments
   - Limited support for matrix strategies

2. **Can't test:**
   - External integrations (Codecov uploads, release creation)
   - GitHub API interactions
   - Secrets from GitHub Secrets store

3. **Workarounds:**
   - Mock external services
   - Use dry-run flags
   - Test logic, not integrations

---

## Best Practices

### 1. Test Locally First

```bash
# Before pushing
act push -j lint-rust -j test-rust
```

### 2. Use Dry Runs

```bash
# Preview what would run
act --dry-run
```

### 3. Test Critical Paths

Focus on testing:
- Linting and formatting
- Unit tests
- Build process

Don't test:
- External uploads (Codecov)
- Release creation
- Secret-dependent workflows

### 4. Cache Configuration

```bash
# Speed up subsequent runs
act --reuse
```

---

## Example: Pre-Commit Hook

Create `.git/hooks/pre-push`:

```bash
#!/bin/bash

echo "Running local CI checks with act..."

# Run lint and tests
act push -j lint-rust -j test-rust --quiet

if [ $? -ne 0 ]; then
  echo "❌ CI checks failed. Fix errors before pushing."
  exit 1
fi

echo "✅ CI checks passed"
exit 0
```

Make executable:

```bash
chmod +x .git/hooks/pre-push
```

---

## Workflow-Specific Testing

### CI Workflow

```bash
# Test Rust linting
act push -j lint-rust

# Test frontend linting
act push -j lint-frontend

# Test Rust tests
act push -j test-rust

# Test frontend tests
act push -j test-frontend

# Test build
act push -j build
```

### PR Checks Workflow

```bash
# Test PR validation
act pull_request -j validate-pr

# Test labeling
act pull_request -j label-pr
```

### Coverage Workflow

```bash
# Test coverage (may take time)
act push -j coverage
```

---

## Resources

- **Act Documentation:** https://github.com/nektos/act
- **GitHub Actions Docs:** https://docs.github.com/en/actions
- **Docker Images:** https://github.com/catthehacker/docker_images

---

## Quick Reference

```bash
# Common commands
act                              # Run default event (push)
act push                         # Run push event
act pull_request                 # Run PR event
act -l                           # List workflows
act -j JOB_NAME                  # Run specific job
act -W WORKFLOW_FILE             # Run specific workflow
act --dry-run                    # Preview
act -v                           # Verbose output
act --reuse                      # Reuse containers
act -s SECRET_NAME=value         # Set secret
act --secret-file .secrets       # Load secrets from file
act -P ubuntu-latest=IMAGE       # Use custom image
```

---

Built by Glincker (A GLINR Product)
