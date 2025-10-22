# Changelog

All notable changes to Sentinel will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-10-21

### Added

#### Backend & Core
- ✨ Process lifecycle management with start/stop/restart
- ✨ Log capture with circular buffer (10,000 lines per process)
- ✨ Auto-restart with exponential backoff and retry limits
- ✨ Graceful shutdown (SIGTERM → 5s timeout → SIGKILL)
- ✨ Health check system with process monitoring
- ✨ YAML and JSON configuration support
- ✨ Environment variable interpolation in configs
- ✨ Configuration validation (circular dependencies, duplicates)
- ✨ System metrics collection (CPU, RAM, Disk I/O)
- ✨ Per-process resource tracking
- ✨ Historical metrics buffer with circular storage

#### Frontend & UI
- ✨ Modern glassmorphism design system
- ✨ Inter font family for clean typography
- ✨ Lucide icons replacing emojis
- ✨ Theme switcher (Light/Dark/System) with OS preference detection
- ✨ System tray integration
- ✨ Keyboard shortcuts (Cmd+K, Cmd+Shift+P)
- ✨ Terminal log viewer with ANSI color support
- ✨ Process dashboard with real-time updates
- ✨ System metrics visualization
- ✨ Settings page with theme controls
- ✨ 8 reusable component library (IconButton, NavButton, Toggle, etc.)

#### Testing & Quality
- ✨ 100 tests passing (76 unit + 24 doc tests)
- ✨ GitHub Actions CI/CD pipeline
- ✨ Multi-OS testing (Linux, Windows, macOS)
- ✨ Vitest configuration for frontend
- ✨ Code coverage reporting
- ✨ ESLint, Prettier, and Clippy enforcement

#### Performance
- ⚡ Startup time: ~1.2s
- ⚡ Memory usage: ~35MB idle
- ⚡ CPU overhead: 2-5% while monitoring
- ⚡ Bundle size: 3-5MB (vs 80-120MB for Electron)

### Changed
- 🎨 Refactored Settings page (80% code reduction: 565 → 112 lines)
- 🎨 Extracted reusable components to `/src/lib/components/`
- 🎨 Improved dark mode contrast (white bg + black text)
- 🔧 Updated ProcessConfig with `args` and `health_check` fields
- 🔧 Enhanced error handling with proper TypeScript types

### Fixed
- 🐛 Fixed 4 doc test failures in ProcessConfig examples
- 🐛 Fixed all TypeScript errors (0 errors)
- 🐛 Fixed ESLint issues
- 🐛 Fixed dark mode color inconsistencies
- 🐛 Fixed SVG icon color inheritance
- 🐛 Fixed system theme detection for "system" mode
- 🐛 Fixed MouseEvent handler types in GlinrButton

### Technical
- 🛠️ Rust 1.88+ backend with Tauri 2.0
- 🛠️ Svelte 5 with TypeScript
- 🛠️ Vite 7 build system
- 🛠️ pnpm 9 package manager
- 🛠️ sysinfo 0.37.2 for system metrics
- 🛠️ tokio 1.41 async runtime

---

## [Unreleased]

### Planned for v0.2.0
- [ ] Port-based process discovery
- [ ] Process attachment (monitor existing processes)
- [ ] Health check HTTP endpoints
- [ ] Config hot-reload (file watcher)
- [ ] Process grouping and bulk operations
- [ ] Log export (download as .txt)
- [ ] Performance benchmarks in CI

---

**Legend:**
- ✨ Added
- 🎨 Changed
- 🐛 Fixed
- ⚡ Performance
- 🛠️ Technical
- 🔧 Configuration

[0.1.0]: https://github.com/glincker/sentinel/releases/tag/v0.1.0
