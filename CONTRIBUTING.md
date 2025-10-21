# Contributing to Sentinel

First off, thank you for considering contributing to Sentinel! 🎉

We appreciate all contributions, whether they're bug reports, feature requests, documentation improvements, or code contributions. This guide will help you get started.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

---

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [conduct@sentinel.dev](mailto:conduct@sentinel.dev).

---

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, please check the [existing issues](https://github.com/GLINCKER/sentinel/issues) to avoid duplicates.

**To submit a good bug report:**

1. Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md)
2. Include a clear title and description
3. Provide steps to reproduce the issue
4. Include your environment details (OS, version, etc.)
5. Add screenshots or logs if relevant

### Suggesting Features

We love new ideas! To suggest a feature:

1. Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md)
2. Explain the problem your feature solves
3. Describe your proposed solution
4. Consider alternative solutions

### Improving Documentation

Documentation improvements are always welcome:
- Fix typos or unclear explanations
- Add examples or clarifications
- Translate documentation
- Write tutorials or guides

### Contributing Code

See the sections below for detailed guidance on code contributions.

---

## Development Setup

### Prerequisites

- **Rust** 1.88+ ([Install](https://rustup.rs/))
- **Node.js** 20+ and npm ([Install](https://nodejs.org/))
- **Git** ([Install](https://git-scm.com/))

**Platform-specific dependencies:**

- **macOS**:
  ```bash
  xcode-select --install
  ```

- **Linux (Debian/Ubuntu)**:
  ```bash
  sudo apt update
  sudo apt install build-essential libgtk-3-dev libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev librsvg2-dev
  ```

- **Windows**:
  Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/sentinel.git
cd sentinel

# Add upstream remote
git remote add upstream https://github.com/GLINCKER/sentinel.git
```

### Install Dependencies

```bash
# Install Node.js dependencies
npm install

# Fetch Rust dependencies
cargo fetch
```

### Run Development Build

```bash
# Start the app in development mode (hot reload)
npm run tauri:dev
```

The app will launch with hot reloading enabled. Changes to Svelte files will update automatically.

### Run Tests

```bash
# Run all tests
cargo test && npm test

# Run Rust tests only
cargo test

# Run Rust tests with output
cargo test -- --nocapture

# Run JavaScript/Svelte tests
npm test

# Run tests in watch mode
npm run test:watch

# Generate coverage reports
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
npm run test:coverage
```

---

## Project Structure

```
sentinel/
├── src/                    # Frontend (Svelte)
│   ├── lib/
│   │   ├── components/     # Reusable UI components
│   │   ├── stores/         # Svelte stores (state)
│   │   └── utils/          # Helper functions
│   ├── App.svelte          # Root component
│   └── main.js             # Entry point
├── src-tauri/              # Backend (Rust)
│   ├── src/
│   │   ├── commands/       # Tauri command handlers
│   │   ├── core/           # Business logic
│   │   │   ├── process_manager.rs
│   │   │   ├── system_monitor.rs
│   │   │   └── config.rs
│   │   ├── models/         # Data structures
│   │   ├── main.rs         # Entry point
│   │   └── lib.rs          # Public API
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── tests/                  # Integration tests
├── examples/               # Example configurations
├── docs/                   # Documentation (internal)
└── .github/                # GitHub templates
```

---

## Coding Standards

We follow strict coding standards to maintain consistency and quality. Please read [docs/claude.md](docs/claude.md) for detailed standards.

### Quick Reference

**Rust:**
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy -- -D warnings`
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Naming: `snake_case` for functions/variables, `UpperCamelCase` for types

**Svelte/JavaScript:**
- Use Prettier for formatting: `npm run format`
- Use ESLint for linting: `npm run lint`
- Component files: `kebab-case.svelte`
- Component names in code: `PascalCase`

**Documentation:**
- Document all public functions with Rustdoc (`///`)
- Include examples in documentation
- Update README if changing user-facing features

---

## Testing Guidelines

### Coverage Requirements

- **Overall coverage**: 90%+
- **Core business logic**: 95%+
- **UI components**: 80%+
- **Error handling paths**: 100%

### What to Test

**Unit Tests (Rust):**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test happy path
        assert_eq!(function(valid_input), expected_output);
    }

    #[test]
    fn test_error_handling() {
        // Test error cases
        assert!(function(invalid_input).is_err());
    }
}
```

**Component Tests (Svelte):**
```javascript
import { render, fireEvent } from '@testing-library/svelte';
import MyComponent from './my-component.svelte';

test('renders correctly', () => {
    const { getByText } = render(MyComponent, { props: { name: 'Test' } });
    expect(getByText('Test')).toBeInTheDocument();
});
```

**Integration Tests:**
- Test process lifecycle (start, stop, restart)
- Test configuration parsing and validation
- Test system monitoring data collection

---

## Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/) for clear, semantic commit history.

### Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, no logic change)
- `refactor`: Code refactor (no feature/fix)
- `perf`: Performance improvement
- `test`: Add or update tests
- `build`: Build system or dependencies
- `ci`: CI/CD changes
- `chore`: Other changes (tooling, config)

### Examples

```bash
# Feature
feat(process): add auto-restart on crash

# Bug fix
fix(ui): prevent CPU graph overflow on high values

# Documentation
docs: add installation guide for Windows

# Breaking change
feat(config)!: change config format to YAML v2

BREAKING CHANGE: Old TOML configs no longer supported.
Run `sentinel migrate-config` to convert.
```

### Commit Hooks

We use Husky and commitlint to enforce commit message standards:

```bash
# Install hooks
npm install

# Commits will be automatically linted
git commit -m "feat: add new feature"
```

---

## Pull Request Process

### Before Submitting

**Checklist:**

- [ ] Code follows the style guidelines (run `cargo fmt` and `npm run format`)
- [ ] Linting passes (`cargo clippy` and `npm run lint`)
- [ ] All tests pass (`cargo test && npm test`)
- [ ] New tests added for new features
- [ ] Coverage maintained at 90%+
- [ ] Documentation updated (Rustdoc, README, etc.)
- [ ] Commits follow Conventional Commits format

### Submitting a PR

1. **Create a feature branch:**
   ```bash
   git checkout -b feat/my-feature
   ```

2. **Make your changes and commit:**
   ```bash
   git add .
   git commit -m "feat: add my feature"
   ```

3. **Keep your branch up to date:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

4. **Push to your fork:**
   ```bash
   git push origin feat/my-feature
   ```

5. **Open a Pull Request:**
   - Use the [PR template](.github/pull_request_template.md)
   - Link related issues (e.g., `Closes #123`)
   - Provide a clear description of changes
   - Add screenshots for UI changes

### PR Review Process

1. **Automated checks**: CI must pass (build, test, lint)
2. **Code review**: At least 1 approval from a maintainer
3. **Address feedback**: Make requested changes
4. **Merge**: Maintainer will squash-merge your PR

**Response times:**
- First review: Usually within 2-3 business days
- Follow-up reviews: Within 1-2 business days

---

## Community

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions, ideas, general discussion
- **Discord**: Real-time chat with the community
- **Twitter**: [@SentinelDev](https://twitter.com/SentinelDev)

### Getting Help

If you're stuck or have questions:

1. Check the [documentation](docs/)
2. Search [existing issues](https://github.com/GLINCKER/sentinel/issues)
3. Ask in [GitHub Discussions](https://github.com/GLINCKER/sentinel/discussions)
4. Join our [Discord server](https://discord.gg/sentinel)

### Recognition

Contributors are recognized in:
- Our [Contributors page](https://github.com/GLINCKER/sentinel/graphs/contributors)
- Release notes (when your PR is included)
- The README (for significant contributions)

---

## Development Tips

### Useful Commands

```bash
# Format all code
cargo fmt && npm run format

# Lint all code
cargo clippy -- -D warnings && npm run lint

# Fix linting issues automatically
npm run lint:fix

# Run tests with coverage
cargo tarpaulin --out Html && npm run test:coverage

# Build for production
npm run tauri:build

# Clean build artifacts
cargo clean && rm -rf node_modules dist
```

### Debugging

**Rust:**
```bash
# Run with debug output
RUST_LOG=debug npm run tauri:dev

# Use rust-lldb or rust-gdb
cargo build
rust-lldb target/debug/sentinel
```

**Frontend:**
```bash
# Open DevTools in Tauri
# Run with devtools feature enabled (already in tauri.conf.json)
npm run tauri:dev
# Press Cmd+Option+I (macOS) or F12 (Windows/Linux)
```

### Common Issues

**Build fails on macOS:**
```bash
# Reinstall Xcode Command Line Tools
sudo rm -rf /Library/Developer/CommandLineTools
xcode-select --install
```

**Build fails on Linux:**
```bash
# Install missing dependencies
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev
```

---

## License

By contributing to Sentinel, you agree that your contributions will be licensed under the [MIT License](LICENSE).

---

## Questions?

Don't hesitate to ask! We're here to help:
- Open a [Discussion](https://github.com/GLINCKER/sentinel/discussions)
- Join our [Discord](https://discord.gg/sentinel)
- Email: [contribute@sentinel.dev](mailto:contribute@sentinel.dev)

**Thank you for contributing! 🚀**
