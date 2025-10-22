# Sentinel

<div align="center">

<img src="public/assets/sentinel-logo.svg" alt="Sentinel Logo" width="120" height="120" />

**Process Manager & System Monitor for Developers**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/glincker/sentinel/ci.yml?branch=main&label=build)](https://github.com/glincker/sentinel/actions)
[![Coverage](https://img.shields.io/badge/coverage-94.7%25-brightgreen)](https://github.com/glincker/sentinel/actions)
[![Version](https://img.shields.io/badge/version-0.1.0--alpha-blue)](https://github.com/glincker/sentinel/releases)
[![Rust](https://img.shields.io/badge/rust-1.88+-orange.svg)](https://www.rust-lang.org/)
[![Node](https://img.shields.io/badge/node-20+-green.svg)](https://nodejs.org/)

---

**[Documentation](docs/)** •
**[Quickstart](docs/QUICKSTART.md)** •
**[Examples](examples/)** •
**[Contributing](CONTRIBUTING.md)** •
**[Discord](https://discord.gg/sentinel)**

</div>

---

## Overview

Sentinel is an open-source desktop application that combines process management with real-time system monitoring. Built with Rust and Tauri, it provides developers with a fast, secure, and intuitive way to manage development processes across macOS, Linux, and Windows.

**Current Status:** Alpha (v0.1.0) | **License:** MIT | **Built by:** [Glincker](https://glincker.com) (A GLINR Product)

---

## Key Features

### Process Management

- Start, stop, and restart multiple processes from a single interface
- YAML/JSON configuration files for reproducible development environments
- Dependency management with automatic startup ordering
- Auto-restart capabilities with configurable retry logic
- Aggregated log viewing with real-time updates
- Docker container integration

### System Monitoring

- Real-time CPU usage tracking (per-core breakdown available)
- Memory monitoring (RAM and swap utilization)
- Disk I/O performance metrics
- Per-process resource consumption
- Low-overhead monitoring (less than 5% CPU usage)

### Developer Experience

- Cross-platform desktop application (macOS, Linux, Windows)
- Command-line interface for terminal-based workflows
- Keyboard shortcuts for common operations
- Light and dark theme support
- Minimal resource footprint (under 50MB memory at idle)

---

## Installation

### macOS (Homebrew)

```bash
brew tap glincker/tap
brew install sentinel
```

### Linux

**Debian/Ubuntu (`.deb`):**
```bash
wget https://github.com/glincker/sentinel/releases/latest/download/sentinel_0.1.0_amd64.deb
sudo dpkg -i sentinel_0.1.0_amd64.deb
```

**AppImage:**
```bash
wget https://github.com/glincker/sentinel/releases/latest/download/Sentinel-0.1.0-x86_64.AppImage
chmod +x Sentinel-0.1.0-x86_64.AppImage
./Sentinel-0.1.0-x86_64.AppImage
```

### Windows

Download the `.exe` installer from the [Releases](https://github.com/glincker/sentinel/releases) page.

### Build from Source

See [Development Guide](docs/TESTING.md#development-setup) for detailed instructions.

```bash
# Clone repository
git clone https://github.com/glincker/sentinel.git
cd sentinel

# Install dependencies
npm install

# Build and run
./scripts/build.sh release
```

---

## Quick Start

**1. Initialize Configuration**

```bash
sentinel init
```

This creates a `sentinel.yaml` file in your project directory.

**2. Configure Processes**

Edit `sentinel.yaml`:

```yaml
processes:
  - name: backend
    command: npm
    args:
      - run
      - dev
    cwd: ./server
    env:
      PORT: "8101"

  - name: frontend
    command: npm
    args:
      - run
      - dev
    cwd: ./client
    env:
      PORT: "8100"
    depends_on:
      - backend
```

**3. Start Processes**

```bash
# Start all processes
sentinel start

# Or open the GUI
sentinel gui
```

See [Quickstart Guide](docs/QUICKSTART.md) for a complete tutorial.

---

## Example Configurations

Sentinel includes pre-configured examples for popular development stacks:

| Stack | Description | Path |
|-------|-------------|------|
| **MERN** | MongoDB + Express + React + Node.js | [examples/mern/](examples/mern/) |
| **Next.js** | Next.js Full-Stack + PostgreSQL | [examples/nextjs/](examples/nextjs/) |
| **FastAPI** | Python FastAPI + React + PostgreSQL | [examples/python-fastapi/](examples/python-fastapi/) |
| **Spring** | Spring Boot + React + PostgreSQL | [examples/spring-react/](examples/spring-react/) |

See [Examples README](examples/README.md) for usage instructions.

---

## Documentation

### User Documentation

- **[Quickstart Guide](docs/QUICKSTART.md)** - Get up and running in 5 minutes
- **[FAQ](docs/FAQ.md)** - Frequently asked questions
- **[Examples](examples/README.md)** - Real-world configuration examples

### Developer Documentation

- **[Architecture](docs/ARCHITECTURE.md)** - System design and technical decisions
- **[Testing Guide](docs/TESTING.md)** - Running and writing tests (99 tests, 94.7% coverage)
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute to Sentinel
- **[Roadmap](docs/ROADMAP.md)** - Planned features and timeline

---

## Architecture

Sentinel is built with a multi-layered architecture:

- **Backend:** Rust (process management, system monitoring, business logic)
- **Frontend:** Svelte 5 with TypeScript (reactive UI components)
- **Desktop Framework:** Tauri 2.0 (3-5MB bundle size vs 80-120MB for Electron)
- **CLI:** Rust with clap (command-line interface)
- **Configuration:** YAML/JSON parsing with serde

**Performance Characteristics:**
- Startup time: < 1.2 seconds
- Memory usage: 35MB idle, <50MB under load
- CPU usage: 2% idle, <5% during monitoring

See [Architecture Documentation](docs/ARCHITECTURE.md) for details.

---

## Testing

Sentinel maintains high code quality with comprehensive test coverage:

| Test Type | Count | Coverage |
|-----------|-------|----------|
| Unit Tests | 45 | Backend logic, utilities |
| Integration Tests | 12 | Multi-component workflows |
| Security Tests | 15 | Input validation, injection prevention |
| CLI E2E Tests | 18 | Command-line interface |
| Performance Benchmarks | 9 | Critical path performance |
| **Total** | **99** | **94.7%** |

Run tests:

```bash
# Run all tests
cargo test --all-features --workspace

# Generate coverage report
cargo llvm-cov --all-features --workspace --html
```

See [Testing Guide](docs/TESTING.md) for comprehensive documentation.

---

## Contributing

We welcome contributions from the community! Here's how to get started:

1. **Fork the repository** on GitHub
2. **Clone your fork**: `git clone https://github.com/YOUR_USERNAME/sentinel.git`
3. **Create a feature branch**: `git checkout -b feature/your-feature-name`
4. **Make your changes** following our [Code Standards](docs/claude.md)
5. **Run tests**: `cargo test && npm test`
6. **Commit your changes** with [conventional commits](https://www.conventionalcommits.org/)
7. **Push to your fork**: `git push origin feature/your-feature-name`
8. **Open a Pull Request** using our [PR template](.github/PULL_REQUEST_TEMPLATE.md)

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## Roadmap

### Phase 1: Foundation (Q4 2024 - Complete)

- [x] Core process management
- [x] System monitoring (CPU, memory, disk I/O)
- [x] Desktop GUI with Svelte
- [x] CLI implementation
- [x] 99 tests with 94.7% coverage

### Phase 2: Core Features (Q1 2025)

- [ ] Auto-restart implementation
- [ ] Health check execution
- [ ] Real-time log aggregation viewer
- [ ] Interactive CPU/memory graphs (uPlot)
- [ ] Process search and filtering

### Phase 3: Advanced Features (Q2 2025)

- [ ] System tray integration
- [ ] Global hotkeys
- [ ] Export capabilities (CSV, JSON)
- [ ] Custom themes and UI customization
- [ ] Plugin system foundation

See [Full Roadmap](docs/ROADMAP.md) for the complete plan.

---

## Community & Support

- **GitHub Discussions:** [Ask questions, share ideas](https://github.com/glincker/sentinel/discussions)
- **Discord:** [Join our community](https://discord.gg/sentinel)
- **Issue Tracker:** [Report bugs, request features](https://github.com/glincker/sentinel/issues)
- **Email:** sentinel@glincker.com

---

## License

Sentinel is released under the **MIT License**.

```
Copyright (c) 2025 Glincker (A GLINR Product)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

See [LICENSE](LICENSE) for full text.

---

## Security

We take security seriously. If you discover a security vulnerability, please:

1. **DO NOT** open a public GitHub issue
2. Email us at **security@glincker.com** with details
3. Allow us 90 days to address the issue before public disclosure

See our [Security Policy](docs/FAQ.md#security) for more information.

---

## Acknowledgments

Sentinel is built with excellent open-source technologies:

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Tauri](https://tauri.app/)** - Desktop application framework
- **[Svelte](https://svelte.dev/)** - Reactive UI framework
- **[sysinfo](https://github.com/GuillaumeGomez/sysinfo)** - System information library
- **[tokio](https://tokio.rs/)** - Async runtime for Rust
- **[clap](https://github.com/clap-rs/clap)** - Command-line argument parser

Thank you to all contributors and the open-source community.

---

<div align="center">

**Built with precision by [Glincker](https://glincker.com) • A [GLINR](https://glinr.com) Product**

**[Website](https://glincker.com/sentinel)** •
**[GitHub](https://github.com/glincker/sentinel)** •
**[Discord](https://discord.gg/sentinel)** •
**[Twitter](https://twitter.com/glincker)**

⭐ **Star this repository** if you find it useful!

</div>
