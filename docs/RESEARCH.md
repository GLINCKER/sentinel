# Sentinel - Technical Research Report
**Research Date:** October 20, 2025
**Purpose:** Tech stack selection and architecture design for an open-source desktop process manager and system monitor

---

## Table of Contents
1. [Executive Summary](#executive-summary)
2. [Desktop Framework Comparison](#desktop-framework-comparison)
3. [Backend Language Analysis](#backend-language-analysis)
4. [Frontend Framework Comparison](#frontend-framework-comparison)
5. [Process Manager Architecture Analysis](#process-manager-architecture-analysis)
6. [System Monitoring Libraries](#system-monitoring-libraries)
7. [Security Considerations](#security-considerations)
8. [Recommended Tech Stack](#recommended-tech-stack)
9. [Proposed Architecture](#proposed-architecture)
10. [Directory Structure](#directory-structure)
11. [References](#references)

---

## Executive Summary

**Recommended Stack:** Tauri 2.0 + Rust + Svelte

This combination provides:
- ✅ **Performance**: 75% smaller bundle, 90% less memory vs Electron
- ✅ **Security**: Memory-safe Rust backend, granular permission system
- ✅ **Speed**: <500ms startup, 3-10MB bundle size
- ✅ **Developer Experience**: Strong tooling, active 2025 ecosystem
- ✅ **Cross-platform**: Single codebase for Mac/Linux/Windows

---

## Desktop Framework Comparison

### Tauri vs Electron vs Native (2025)

| Metric | Tauri | Electron | Native (Swift/Kotlin/C++) |
|--------|-------|----------|---------------------------|
| **Bundle Size** | 3-10 MB | 80-120 MB | 5-15 MB |
| **Memory Usage (Idle)** | 30-40 MB | 100+ MB | 20-30 MB |
| **Startup Time** | <500ms | 1-2s | <300ms |
| **Cross-platform** | ✅ Single codebase | ✅ Single codebase | ❌ Per-platform |
| **Development Speed** | Fast | Fast | Slow |
| **Security Model** | Granular permissions | Limited | OS-dependent |
| **2025 Adoption Growth** | +35% YoY | Stable (60% market) | Declining |

**Sources:**
- [Tauri vs Electron 2025 Comparison - Codeology](https://codeology.co.nz/articles/tauri-vs-electron-2025-desktop-development.html)
- [Tauri vs Electron Performance - Markaicode](https://markaicode.com/tauri-vs-electron-desktop-app-framework-comparison/)
- [Real-world Tauri vs Electron - Levminer](https://www.levminer.com/blog/tauri-vs-electron)

### Key Findings

**Tauri Advantages:**
- Uses OS native WebView (WebView2/WebKit/WebKitGTK) instead of bundling Chromium
- Rust backend provides memory safety and performance
- 90% reduction in memory usage, 75% smaller bundles
- Battery-efficient (fewer background processes)
- Built-in security audit process for major releases
- Comprehensive permission/capability system in v2.0

**Tauri Challenges:**
- WebView fragmentation across platforms (different feature sets and bugs)
- Smaller ecosystem than Electron (but growing rapidly)
- Requires Rust knowledge for backend customization

**Electron Trade-offs:**
- Mature ecosystem with extensive libraries
- Consistent behavior across platforms (bundled Chromium)
- Higher resource usage (50MB+ disk, 100MB+ RAM)
- Slower startup times

**Verdict:** Tauri meets all Sentinel requirements (<15MB size, <2s startup, low memory) while Electron cannot.

---

## Backend Language Analysis

### Rust vs Go vs C++ (2025)

| Criteria | Rust | Go | C++ |
|----------|------|-----|-----|
| **Memory Safety** | ✅ Compile-time | ⚠️ GC-based | ❌ Manual |
| **Performance** | Excellent | Very Good | Excellent |
| **Concurrency** | Async/await (Tokio) | Goroutines | Threads/async |
| **System Access** | Native | Native | Native |
| **Learning Curve** | Steep | Moderate | Very Steep |
| **Cross-compilation** | Excellent | Excellent | Complex |
| **Ecosystem (2025)** | Rapidly growing | Mature | Very mature |

**Sources:**
- [Rust vs Go 2025 - JetBrains RustRover Blog](https://blog.jetbrains.com/rust/2025/06/12/rust-vs-go/)
- [Building System Monitor in Rust - The New Stack](https://thenewstack.io/building-a-real-time-system-monitor-in-rust-terminal/)

### Key Rust Advantages for Sentinel

1. **Memory Safety**: Zero-cost abstractions prevent crashes
2. **Performance**: Comparable to C++, no GC pauses like Go
3. **System Monitoring Libraries**:
   - `sysinfo` crate (78M+ downloads, actively maintained)
   - Native process/CPU/memory/disk access
   - Cross-platform (Windows/macOS/Linux)
4. **Process Management**:
   - `tokio::process` for async process handling
   - `subprocess` crate for advanced pipelines
5. **Tauri Integration**: First-class support, same language for backend

**Example System Monitoring Tools in Rust:**
- **bottom** (btm) - Feature-rich, resource-efficient system monitor
- **monitor-rs** - Real-time terminal dashboard
- **Parseable** - Log analytics with Apache Arrow/Parquet

**Verdict:** Rust is the optimal choice for system-level desktop apps requiring security and performance.

---

## Frontend Framework Comparison

### React vs Vue vs Svelte (2025)

| Metric | Svelte | Vue | React |
|--------|--------|-----|-------|
| **Bundle Size (Hello World)** | ~1.6 KB | ~20 KB | ~40 KB |
| **Runtime Overhead** | None (compiled) | Virtual DOM | Virtual DOM |
| **Performance** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Learning Curve** | Easy | Easy | Moderate |
| **Desktop GUI Suitability** | Excellent (snappy UIs) | Good | Good |
| **2025 Ecosystem** | Growing | Mature | Very mature |

**Sources:**
- [React vs Vue vs Svelte 2025 - Medium Performance Comparison](https://medium.com/@jessicajournal/react-vs-vue-vs-svelte-the-ultimate-2025-frontend-performance-comparison-5b5ce68614e2)
- [Svelte vs React - Strapi Comparison](https://strapi.io/blog/svelte-vs-react-comparison)
- [Why Learn Svelte in 2025 - DEV Community](https://dev.to/a1guy/why-learn-svelte-in-2025-the-value-proposition-svelte-vs-react-vue-1bhc)

### Why Svelte for Sentinel

**Performance:**
- Compiles to vanilla JS at build time (no runtime framework overhead)
- 10x smaller bundle than React (~1.6KB vs ~40KB)
- No virtual DOM = faster DOM updates
- "Unbeaten for speed, performance, and getting to a blank slate"

**Developer Experience:**
- Reactive by default (no `useState` boilerplate)
- Less code than React/Vue for same functionality
- Built-in animations and transitions
- Scoped styles by default

**Desktop App Benefits:**
- Extremely snappy user experiences
- Excellent performance on low-end devices
- Small bundle complements Tauri's small binary

**Verdict:** Svelte's compiled approach and minimal overhead align perfectly with Sentinel's performance goals.

---

## Process Manager Architecture Analysis

### Existing Solutions Review

| Tool | Language | Architecture | Key Features |
|------|----------|--------------|--------------|
| **PM2** | Node.js | Cluster mode, daemon | Load balancing, startup scripts, process files (JSON/YAML) |
| **Supervisor** | Python | Client/server | Unix-only, simple config, auto-restart |
| **systemd** | C | Init system | Linux native, robust, complex config |

**Sources:**
- [PM2 GitHub](https://github.com/Unitech/pm2)
- [Process Managers for Node.js - TecMint](https://www.tecmint.com/process-managers-for-node-js-applications-in-linux/)
- [PM2 vs Supervisor vs systemd Comparison](https://mrvautin.com/running-nodejs-applications-in-production-forever-vs-supervisord-vs-pm2/)

### Key Architectural Patterns Learned

**1. Configuration-Driven Design**
- Support YAML/JSON process definitions
- Schema: `{ name, command, cwd, env, restart_policy, dependencies }`

**2. Process Lifecycle Management**
```
States: stopped → starting → running → stopping → stopped
Events: crash, restart, exit
```

**3. Auto-Restart Strategies**
- On-crash (PM2 approach)
- Exponential backoff
- Max restart limits
- Dependency ordering (start DB before API)

**4. Log Aggregation**
- Combine stdout/stderr from multiple processes
- Timestamped, color-coded per process
- Streaming vs buffered modes

**5. IPC Mechanisms**
- CLI ↔ Daemon communication (Unix sockets/named pipes)
- RESTful API for GUI integration
- Event broadcasting for real-time updates

**Verdict:** Adopt PM2's configuration approach with Rust's type safety. Use Tokio for async process management.

---

## System Monitoring Libraries

### sysinfo Crate (Rust)

**Stats:**
- 78M+ downloads
- Version: 0.37.2 (actively maintained as of Oct 2025)
- License: MIT
- Minimum Rust: 1.88

**Capabilities:**
- ✅ CPU usage (per-core breakdown)
- ✅ Memory (RAM/swap)
- ✅ Disk I/O
- ✅ Process-level resource tracking
- ✅ Cross-platform (Windows/macOS/Linux)

**Performance Best Practices:**
1. Reuse System instance (works on diffs)
2. Call `.update()` before reading data
3. Use `refresh_specifics()` for selective updates
4. Disable threading with `default-features = false` (lower memory)

**Sources:**
- [sysinfo - crates.io](https://crates.io/crates/sysinfo)
- [sysinfo Documentation](https://docs.rs/sysinfo/latest/sysinfo/)

### Alternative Libraries
- `sys-info` - Lower-level, less active
- `systemstat` - Good for network stats
- `psutil` (if Python binding needed)

**Verdict:** `sysinfo` is production-ready and aligns with Sentinel's requirements.

---

## Security Considerations

### Tauri Security Model (v2.0)

**Permission System:**
- **Capabilities**: Define which permissions are granted per window/webview
- **Scopes**: Limit filesystem/shell/HTTP access to specific paths/commands
- **Platform-specific**: Granular control per OS (macOS/Linux/Windows/iOS/Android)

**Best Practices from Research:**

1. **Principle of Least Privilege**
   - Only grant necessary capabilities to each window
   - Use allowlists for file paths and shell commands
   - Disable features like filesystem/shell/HTTP by default

2. **Security Audits**
   - Tauri conducts audits for major releases (including dependencies)
   - [Tauri 2.0 Audit Report](https://v2.tauri.app/security/) available

3. **Code Separation**
   - Never expose Rust backend directly to frontend
   - Use Tauri Commands (RPC-like interface)
   - Validate all inputs from frontend

4. **System-Level App Hardening**
   - Keep OS and dependencies updated (auto-update mechanism)
   - Implement strong authentication for sensitive operations
   - Monitor and log security events
   - Use Content Security Policy (CSP)

**Sources:**
- [Tauri Security - Capabilities](https://v2.tauri.app/security/capabilities/)
- [Tauri Permissions](https://v2.tauri.app/security/permissions/)
- [Desktop Security Best Practices 2025](https://www.bitsight.com/blog/5-things-to-consider-building-continuous-security-monitoring-strategy)

### Security Risks for Sentinel

| Risk | Mitigation |
|------|------------|
| Arbitrary process execution | Allowlist of safe commands, sanitize inputs |
| File system access abuse | Scope permissions to specific directories |
| Resource exhaustion (fork bomb) | Rate limiting, process count limits |
| Log injection attacks | Sanitize log output, structured logging |
| Privilege escalation | Run with minimum permissions, avoid sudo prompts |

**Verdict:** Tauri v2's capability system provides necessary security controls. Implement strict allowlists and input validation.

---

## Recommended Tech Stack

### Final Selection

```
Frontend:  Svelte 5
Backend:   Rust (stable 1.88+)
Framework: Tauri 2.0
Runtime:   Tokio (async)
State:     Svelte stores + Tauri state management
Styling:   TailwindCSS
Logging:   tracing (Rust), console (frontend)
Testing:   cargo test, Vitest (Svelte)
```

### Rationale Matrix

| Requirement | How Stack Satisfies |
|-------------|-------------------|
| **< 15MB bundle** | Tauri: 3-10MB typical, Svelte: minimal JS |
| **< 2s startup** | Tauri: <500ms, Rust: zero runtime, Svelte: compiled |
| **< 50MB RAM idle** | Tauri: 30-40MB measured in 2025 benchmarks |
| **Cross-platform** | Tauri: single codebase for Win/Mac/Linux |
| **Memory-safe** | Rust: compile-time safety, no GC pauses |
| **Developer experience** | Cargo + npm, hot reload, strong typing |
| **Security** | Tauri v2 capabilities, Rust safety, audited |
| **Community** | Tauri +35% growth, Rust top-loved language |

### Dependency List

**Rust (Cargo.toml):**
```toml
[dependencies]
tauri = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
sysinfo = "0.37"
subprocess = "0.2"
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
```

**Frontend (package.json):**
```json
{
  "dependencies": {
    "svelte": "^5.0.0"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^4.0.0",
    "@tauri-apps/api": "^2.0.0",
    "vite": "^6.0.0",
    "tailwindcss": "^3.4.0",
    "vitest": "^2.0.0"
  }
}
```

---

## Proposed Architecture

### High-Level System Design

```
┌─────────────────────────────────────────────────────────┐
│                    Sentinel Desktop App                  │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────────┐        ┌──────────────────┐       │
│  │   Frontend (UI)  │◄──────►│  Backend (Rust)   │       │
│  │                  │  Tauri │                   │       │
│  │  - Svelte 5      │ Commands│ - Process Manager │       │
│  │  - TailwindCSS   │   IPC   │ - System Monitor  │       │
│  │  - Charts/Graphs │        │ - Config Parser   │       │
│  └──────────────────┘        └──────────────────┘       │
│                                        ▲                  │
│                                        │                  │
│                                        ▼                  │
│                              ┌──────────────────┐        │
│                              │   OS Layer       │        │
│                              │ - Process API    │        │
│                              │ - sysinfo crate  │        │
│                              │ - File System    │        │
│                              └──────────────────┘        │
└─────────────────────────────────────────────────────────┘

External:
┌──────────────┐         ┌──────────────┐         ┌──────────────┐
│   CLI Tool   │◄───────►│  Config File │         │  System Tray │
│ (sentinel)   │  Reads  │(sentinel.yml)│         │   Integration│
└──────────────┘         └──────────────┘         └──────────────┘
```

### Component Breakdown

#### 1. Frontend Layer (Svelte)
**Responsibilities:**
- Render process list, logs, system graphs
- Handle user interactions (start/stop/restart)
- Real-time updates via Tauri events
- Dark/light theme switching
- Keyboard shortcuts

**Key Files:**
- `App.svelte` - Main layout
- `ProcessManager.svelte` - Process list and controls
- `SystemMonitor.svelte` - CPU/RAM/Disk graphs
- `LogViewer.svelte` - Combined log output
- `Settings.svelte` - Configuration UI

#### 2. Backend Layer (Rust)
**Responsibilities:**
- Parse YAML/JSON configuration
- Manage process lifecycle (spawn, kill, restart)
- Monitor system resources (via `sysinfo`)
- Aggregate logs from child processes
- Expose Tauri commands for frontend

**Key Modules:**
- `process_manager.rs` - Core process lifecycle
- `system_monitor.rs` - Resource tracking
- `config.rs` - Config parsing (serde)
- `logger.rs` - Log aggregation
- `commands.rs` - Tauri command handlers
- `state.rs` - Application state management

#### 3. CLI Tool
**Responsibilities:**
- Start/stop/restart processes from terminal
- Query status
- Tail logs

**Implementation:**
- Separate binary using same Rust backend code
- Communicate with GUI via IPC (if running) or standalone mode

#### 4. System Tray
**Responsibilities:**
- Quick access to start/stop
- Show resource usage
- Open main window

**Implementation:**
- Tauri's system tray API
- Platform-native icons

### Data Flow Example: Starting a Process

```
User clicks "Start" button
         │
         ▼
Frontend calls Tauri command: start_process("api-server")
         │
         ▼
Backend receives command
         │
         ▼
ProcessManager spawns tokio::process::Command
         │
         ▼
Capture stdout/stderr streams
         │
         ▼
Emit Tauri event: process_started { name, pid }
         │
         ▼
Frontend updates UI to show "Running" status
         │
         ▼
SystemMonitor refreshes sysinfo for new PID
         │
         ▼
Frontend displays real-time CPU/RAM for process
```

### State Management

**Backend State (Rust):**
```rust
struct AppState {
    processes: Arc<Mutex<HashMap<String, ProcessInfo>>>,
    config: Arc<RwLock<Config>>,
    system: Arc<Mutex<System>>, // sysinfo
}
```

**Frontend State (Svelte):**
```javascript
// stores.js
import { writable } from 'svelte/store';

export const processes = writable([]);
export const systemStats = writable({
  cpu: [],
  memory: { used: 0, total: 0 },
  disk: []
});
```

### Error Handling Strategy

1. **Backend (Rust):**
   - Use `anyhow::Result` for error propagation
   - Log errors with `tracing::error!`
   - Return user-friendly error messages to frontend

2. **Frontend (Svelte):**
   - Display toast notifications for errors
   - Fallback UI states (e.g., "Failed to start process")
   - Retry mechanisms for transient failures

### Testing Strategy

**Unit Tests:**
- `cargo test` for Rust backend logic
- Vitest for Svelte components

**Integration Tests:**
- Spawn test processes and verify lifecycle
- Mock `sysinfo` for deterministic tests

**End-to-End:**
- Tauri's testing framework (WebDriver)
- Test critical flows (start process, view logs, restart on crash)

---

## Directory Structure

### Recommended Layout

```
sentinel/
├── src/                          # Frontend (Svelte)
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ProcessList.svelte
│   │   │   ├── ProcessCard.svelte
│   │   │   ├── SystemMonitor.svelte
│   │   │   ├── CPUGraph.svelte
│   │   │   ├── MemoryGraph.svelte
│   │   │   ├── DiskGraph.svelte
│   │   │   ├── LogViewer.svelte
│   │   │   ├── ConfigEditor.svelte
│   │   │   └── Settings.svelte
│   │   ├── stores/
│   │   │   ├── processes.js
│   │   │   ├── systemStats.js
│   │   │   └── settings.js
│   │   ├── utils/
│   │   │   ├── formatters.js      # Format bytes, timestamps
│   │   │   └── validators.js
│   │   └── styles/
│   │       └── theme.css
│   ├── App.svelte
│   └── main.js
│
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── main.rs               # Desktop entry point
│   │   ├── lib.rs                # Shared library code
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── process.rs        # start_process, stop_process, etc.
│   │   │   └── system.rs         # get_system_stats
│   │   ├── core/
│   │   │   ├── mod.rs
│   │   │   ├── process_manager.rs  # Core process lifecycle
│   │   │   ├── system_monitor.rs   # sysinfo integration
│   │   │   ├── config.rs           # YAML/JSON parsing
│   │   │   └── logger.rs           # Log aggregation
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── process.rs        # ProcessInfo, ProcessState
│   │   │   ├── config.rs         # Config structs
│   │   │   └── system.rs         # SystemStats
│   │   ├── state.rs              # AppState management
│   │   ├── error.rs              # Error types
│   │   └── utils.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json           # Tauri configuration
│   ├── build.rs
│   └── icons/
│
├── cli/                          # CLI tool (optional)
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
│
├── tests/                        # Integration tests
│   ├── process_lifecycle.rs
│   └── system_monitor.rs
│
├── docs/                         # Internal documentation (gitignored)
│   ├── RESEARCH.md
│   └── ARCHITECTURE.md
│
├── examples/                     # Example configs
│   ├── simple.yml
│   └── full-stack.yml
│
├── .github/
│   └── workflows/
│       ├── ci.yml                # Build and test
│       └── release.yml           # Publish binaries
│
├── .gitignore
├── package.json                  # Frontend dependencies
├── vite.config.js
├── tailwind.config.js
├── README.md
├── LICENSE
└── CONTRIBUTING.md
```

### Key Design Choices

**1. Separation of Concerns:**
- `src/` for frontend only
- `src-tauri/` for backend only
- Clear API boundary via Tauri commands

**2. Rust Module Organization:**
- `commands/` - Tauri command handlers (thin layer)
- `core/` - Business logic (process manager, monitor)
- `models/` - Data structures (shared across modules)
- `state.rs` - Centralized state management

**3. Frontend Component Structure:**
- `components/` - Reusable UI elements
- `stores/` - Reactive state (Svelte stores)
- `utils/` - Pure functions (formatters, validators)

**4. Configuration:**
- `tauri.conf.json` - App metadata, permissions, build settings
- `vite.config.js` - Frontend bundler config
- `Cargo.toml` - Rust dependencies

**5. Testing:**
- Unit tests alongside source files (`mod tests`)
- Integration tests in top-level `tests/`
- E2E tests in future phase

---

## Implementation Roadmap (Week 1-2)

### Phase 1.1: Project Setup (Days 1-2)
- [ ] Initialize Tauri project: `npm create tauri-app`
- [ ] Configure Svelte + TailwindCSS
- [ ] Set up Rust workspace structure
- [ ] Add core dependencies (sysinfo, tokio, serde)
- [ ] Configure CI/CD (GitHub Actions)

### Phase 1.2: Core Backend (Days 3-5)
- [ ] Implement config parser (`config.rs`)
- [ ] Build process manager (`process_manager.rs`)
  - Spawn processes via `tokio::process`
  - Track state (stopped, running, crashed)
  - Implement auto-restart logic
- [ ] Integrate sysinfo (`system_monitor.rs`)
  - CPU, memory, disk I/O
  - Per-process resource tracking
- [ ] Create Tauri commands (`commands/`)

### Phase 1.3: Frontend Foundation (Days 6-7)
- [ ] Build process list UI (`ProcessList.svelte`)
- [ ] Create system monitor graphs (`SystemMonitor.svelte`)
- [ ] Implement log viewer (`LogViewer.svelte`)
- [ ] Set up Svelte stores for state
- [ ] Connect to Tauri commands

### Phase 1.4: Polish & Testing (Days 8-10)
- [ ] Add keyboard shortcuts
- [ ] Implement dark/light theme
- [ ] Write unit tests (>80% coverage goal)
- [ ] Test on Mac/Linux/Windows
- [ ] Optimize bundle size

---

## References

### Official Documentation
1. [Tauri v2 Documentation](https://v2.tauri.app/)
2. [Tauri Security Guide](https://v2.tauri.app/security/)
3. [Svelte 5 Documentation](https://svelte.dev/)
4. [Tokio Documentation](https://tokio.rs/)
5. [sysinfo Crate](https://docs.rs/sysinfo/latest/sysinfo/)

### Research Articles (2025)
1. [Tauri vs Electron 2025 Comparison - Codeology](https://codeology.co.nz/articles/tauri-vs-electron-2025-desktop-development.html)
2. [React vs Vue vs Svelte Performance 2025 - Medium](https://medium.com/@jessicajournal/react-vs-vue-vs-svelte-the-ultimate-2025-frontend-performance-comparison-5b5ce68614e2)
3. [Building System Monitor in Rust - The New Stack](https://thenewstack.io/building-a-real-time-system-monitor-in-rust-terminal/)
4. [Desktop Security Best Practices 2025 - BitsLight](https://www.bitsight.com/blog/5-things-to-consider-building-continuous-security-monitoring-strategy)
5. [Rust vs Go 2025 - JetBrains](https://blog.jetbrains.com/rust/2025/06/12/rust-vs-go/)

### GitHub Repositories
1. [PM2 - Process Manager](https://github.com/Unitech/pm2)
2. [bottom - System Monitor](https://github.com/ClementTsang/bottom)
3. [Tauri Apps](https://github.com/tauri-apps/tauri)

### Benchmarks & Comparisons
1. [Tauri vs Electron Bundle Size](https://www.levminer.com/blog/tauri-vs-electron)
2. [Svelte vs React Performance](https://strapi.io/blog/svelte-vs-react-comparison)
3. [Tokio Best Practices](https://tokio.rs/tokio/tutorial)

---

## Appendix: Competitive Analysis

### Similar Projects (Open Source)

| Project | Stack | Stars | Active | Notes |
|---------|-------|-------|--------|-------|
| PM2 | Node.js | 41k+ | ✅ | Industry standard, Node-only |
| bottom (btm) | Rust | 10k+ | ✅ | Terminal-only, no process manager |
| htop | C | 6k+ | ✅ | Terminal-only, monitoring focus |
| Supervisor | Python | 8k+ | ✅ | Unix-only, dated UI |

**Sentinel's Differentiation:**
- ✅ Modern GUI (Tauri + Svelte)
- ✅ Cross-platform desktop app
- ✅ Combined process manager + system monitor
- ✅ Developer-focused (not sys-admin tool)
- ✅ Sub-15MB bundle size

---

**Report compiled by:** Claude (Anthropic AI)
**Last updated:** October 20, 2025
**Next steps:** Review with team, initialize project structure (Week 1)
