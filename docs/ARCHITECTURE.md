# Sentinel Architecture

**Version:** 0.1.0
**Last Updated:** October 2025

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Technology Stack](#technology-stack)
3. [High-Level Architecture](#high-level-architecture)
4. [Component Breakdown](#component-breakdown)
5. [Data Flow](#data-flow)
6. [Security Model](#security-model)
7. [Process Lifecycle](#process-lifecycle)
8. [State Management](#state-management)
9. [Error Handling](#error-handling)
10. [Performance Considerations](#performance-considerations)
11. [Design Decisions](#design-decisions)

---

## System Overview

Sentinel is a cross-platform desktop application built using **Tauri 2.0**, combining a **Rust backend** for system-level operations with a **Svelte frontend** for the user interface. The architecture prioritizes:

- **Security**: Memory-safe Rust, minimal permissions, input validation
- **Performance**: Native speed, low memory footprint (<50MB idle)
- **Modularity**: Clear separation of concerns, testable components
- **Cross-platform**: Single codebase for macOS, Linux, Windows

### Design Philosophy

1. **Backend Does Heavy Lifting**: Process management and system monitoring happen in Rust
2. **Frontend Stays Thin**: Svelte UI focuses on rendering and user interaction
3. **Clear API Boundary**: Tauri commands define a strict interface
4. **Fail-Safe**: Errors are handled gracefully, never crash the app

---

## Technology Stack

### Core Technologies

| Layer | Technology | Version | Purpose |
|-------|-----------|---------|---------|
| **Desktop Framework** | Tauri | 2.0 | Cross-platform app shell |
| **Backend** | Rust | 1.88+ | Process management, system monitoring |
| **Frontend** | Svelte | 5.0 | Reactive UI |
| **Async Runtime** | Tokio | 1.35+ | Async process handling |
| **System Monitoring** | sysinfo crate | 0.37 | CPU/RAM/disk metrics |
| **Styling** | TailwindCSS | 3.4 | Utility-first CSS |
| **Build Tool** | Vite | 6.0 | Fast frontend bundling |

### Key Dependencies

**Rust:**
- `serde` / `serde_json` / `serde_yaml` - Serialization
- `anyhow` / `thiserror` - Error handling
- `tracing` - Logging
- `subprocess` - Process spawning

**JavaScript:**
- `@tauri-apps/api` - Tauri bindings
- `@testing-library/svelte` - Component testing
- `vitest` - Test runner

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Sentinel Desktop App                          │
│                                                                   │
│  ┌──────────────────────┐         ┌───────────────────────┐     │
│  │   Frontend (Svelte)   │         │   Backend (Rust)      │     │
│  │                       │◄────────┤                       │     │
│  │  Components:          │ Tauri   │  Modules:             │     │
│  │  - ProcessList       │  IPC    │  - ProcessManager     │     │
│  │  - SystemMonitor     │  (JSON)  │  - SystemMonitor      │     │
│  │  - LogViewer         │         │  - ConfigParser       │     │
│  │  - Settings          │         │  - LogAggregator      │     │
│  │                       │         │                       │     │
│  │  Stores:              │         │  State:               │     │
│  │  - processes.js       │         │  - AppState (Mutex)   │     │
│  │  - systemStats.js     │         │                       │     │
│  └──────────────────────┘         └───────────────────────┘     │
│           ▲                                    │                  │
│           │                                    ▼                  │
│           │                        ┌───────────────────────┐     │
│           │                        │   OS Layer            │     │
│           └────────────────────────┤  - Process API        │     │
│                 Events             │  - sysinfo (metrics)  │     │
│                                    │  - File System        │     │
│                                    └───────────────────────┘     │
└─────────────────────────────────────────────────────────────────┘

External Components:
┌───────────────┐     ┌─────────────────┐     ┌──────────────┐
│  CLI Tool     │────►│  Config File    │     │ System Tray  │
│  (sentinel)   │     │ (sentinel.yaml) │     │ Integration  │
└───────────────┘     └─────────────────┘     └──────────────┘
```

---

## Component Breakdown

### 1. Frontend Layer (Svelte)

**Location:** `src/`

**Responsibilities:**
- Render UI components
- Handle user interactions
- Display real-time data (graphs, logs)
- Manage local UI state
- Invoke Tauri commands

**Key Components:**

| Component | File | Purpose |
|-----------|------|---------|
| ProcessList | `lib/components/process-list.svelte` | Display running processes with controls |
| ProcessCard | `lib/components/process-card.svelte` | Individual process card with metrics |
| SystemMonitor | `lib/components/system-monitor.svelte` | CPU/RAM/disk graphs |
| LogViewer | `lib/components/log-viewer.svelte` | Combined log output with filtering |
| Settings | `lib/components/settings.svelte` | App configuration UI |

**State Management (Svelte Stores):**

```javascript
// stores/processes.js
import { writable } from 'svelte/store';

export const processes = writable([]);
export const systemStats = writable({
  cpu: [],
  memory: { used: 0, total: 0 },
  disk: []
});
```

**Communication with Backend:**

```javascript
import { invoke } from '@tauri-apps/api/core';

// Call Rust function
const result = await invoke('start_process', { name: 'api-server' });

// Listen to events
import { listen } from '@tauri-apps/api/event';
listen('process-started', (event) => {
  console.log('Process started:', event.payload);
});
```

---

### 2. Backend Layer (Rust)

**Location:** `src-tauri/src/`

**Responsibilities:**
- Spawn and manage child processes
- Monitor system resources
- Parse and validate configuration files
- Aggregate logs from multiple processes
- Expose Tauri commands

**Module Structure:**

```
src-tauri/src/
├── main.rs              # App entry point (minimal)
├── lib.rs               # Public API exports
├── commands/            # Tauri command handlers
│   ├── mod.rs
│   ├── process.rs       # start_process, stop_process, etc.
│   └── system.rs        # get_system_stats
├── core/                # Business logic
│   ├── mod.rs
│   ├── process_manager.rs  # ProcessManager struct
│   ├── system_monitor.rs   # SystemMonitor struct
│   ├── config.rs           # Config parsing
│   └── logger.rs           # Log aggregation
├── models/              # Data structures
│   ├── mod.rs
│   ├── process.rs       # ProcessInfo, ProcessState
│   ├── config.rs        # ProcessConfig
│   └── system.rs        # SystemStats
├── state.rs             # AppState (shared state)
└── error.rs             # SentinelError enum
```

**Core Structs:**

```rust
// AppState: Global application state (thread-safe)
pub struct AppState {
    processes: Arc<Mutex<HashMap<String, ProcessInfo>>>,
    config: Arc<RwLock<Config>>,
    system: Arc<Mutex<System>>,  // sysinfo::System
}

// ProcessManager: Handles process lifecycle
pub struct ProcessManager {
    processes: HashMap<String, ProcessHandle>,
    config: Config,
}

// SystemMonitor: Collects system metrics
pub struct SystemMonitor {
    system: System,  // sysinfo::System
    update_interval: Duration,
}
```

---

### 3. Tauri Commands (API Layer)

**Location:** `src-tauri/src/commands/`

Tauri commands are the **public API** exposed to the frontend. They act as a thin layer that:
1. Validates input
2. Calls core business logic
3. Formats responses
4. Handles errors

**Example Command:**

```rust
#[tauri::command]
pub async fn start_process(
    name: String,
    state: State<'_, AppState>
) -> Result<ProcessInfo, String> {
    // Validate input
    if name.is_empty() {
        return Err("Process name cannot be empty".into());
    }

    // Get process manager from state
    let mut processes = state.processes.lock().await;

    // Call core logic
    let info = processes.start(&name)
        .map_err(|e| e.to_string())?;

    Ok(info)
}
```

**Available Commands:**

| Command | Parameters | Returns | Description |
|---------|-----------|---------|-------------|
| `start_process` | `name: String` | `ProcessInfo` | Start a process by name |
| `stop_process` | `name: String` | `()` | Stop a running process |
| `restart_process` | `name: String` | `ProcessInfo` | Restart a process |
| `list_processes` | - | `Vec<ProcessInfo>` | Get all processes |
| `get_process` | `name: String` | `ProcessInfo` | Get single process info |
| `get_system_stats` | - | `SystemStats` | Get CPU/RAM/disk metrics |
| `load_config` | `path: String` | `Config` | Load config from file |
| `save_config` | `config: Config` | `()` | Save config to file |

---

### 4. Process Manager

**Location:** `src-tauri/src/core/process_manager.rs`

**Responsibilities:**
- Spawn processes using `tokio::process::Command`
- Track process state (running, stopped, crashed)
- Capture stdout/stderr streams
- Implement auto-restart with exponential backoff
- Handle graceful shutdown

**Process States:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Crashed { exit_code: i32 },
    Failed { reason: String },
}
```

**Auto-Restart Logic:**

```rust
async fn monitor_process(mut child: Child, config: ProcessConfig) {
    let status = child.wait().await;

    match status {
        Ok(exit_status) if !exit_status.success() => {
            // Process crashed
            if should_restart(&config) {
                let delay = calculate_backoff(config.restart_count);
                sleep(delay).await;
                spawn_process(config).await;
            }
        }
        Err(e) => {
            error!("Process monitoring error: {}", e);
        }
        _ => {}
    }
}
```

---

### 5. System Monitor

**Location:** `src-tauri/src/core/system_monitor.rs`

**Responsibilities:**
- Collect system metrics using `sysinfo` crate
- Track per-process resource usage
- Provide real-time updates to frontend
- Minimize overhead (<5% CPU)

**Metrics Collected:**

- **CPU**: Per-core usage, overall usage, per-process usage
- **Memory**: Total RAM, used RAM, swap, per-process memory
- **Disk**: Read/write bytes per second, per-disk metrics

**Update Strategy:**

```rust
impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn update(&mut self) {
        // Selective refresh for better performance
        self.system.refresh_cpu();
        self.system.refresh_memory();
        self.system.refresh_processes();
    }

    pub fn get_stats(&self) -> SystemStats {
        SystemStats {
            cpu_usage: self.system.global_cpu_usage(),
            memory: MemoryStats {
                total: self.system.total_memory(),
                used: self.system.used_memory(),
                available: self.system.available_memory(),
            },
            // ...
        }
    }
}
```

---

### 6. Configuration System

**Location:** `src-tauri/src/core/config.rs`

**Responsibilities:**
- Parse YAML/JSON configuration files
- Validate configuration schema
- Provide default values
- Support environment variable expansion

**Config Structure:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub processes: Vec<ProcessConfig>,
    pub settings: GlobalSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessConfig {
    pub name: String,
    pub command: String,
    pub cwd: Option<PathBuf>,
    pub env: HashMap<String, String>,
    pub auto_restart: bool,
    pub restart_limit: u32,
    pub restart_delay: u64,  // milliseconds
    pub depends_on: Vec<String>,
}
```

**Validation:**

```rust
pub fn validate_config(config: &Config) -> Result<()> {
    // Check for duplicate names
    let mut names = HashSet::new();
    for process in &config.processes {
        if !names.insert(&process.name) {
            return Err(anyhow!("Duplicate process name: {}", process.name));
        }
    }

    // Validate dependencies
    for process in &config.processes {
        for dep in &process.depends_on {
            if !names.contains(dep) {
                return Err(anyhow!("Unknown dependency: {}", dep));
            }
        }
    }

    Ok(())
}
```

---

## Data Flow

### Example: Starting a Process

```
User clicks "Start" button in UI
         │
         ▼
Svelte component calls invoke('start_process', { name })
         │
         ▼
Tauri IPC layer serializes request → sends to Rust
         │
         ▼
Rust command handler (commands/process.rs)
  - Validates input
  - Acquires lock on AppState.processes
         │
         ▼
ProcessManager.start(name)
  - Reads config for process
  - Spawns tokio::process::Command
  - Captures stdout/stderr streams
  - Returns ProcessInfo
         │
         ▼
Command handler emits event: 'process-started'
         │
         ▼
Svelte component receives event via listen()
  - Updates processes store
  - Re-renders UI
         │
         ▼
SystemMonitor starts tracking process (PID-based)
         │
         ▼
UI displays process as "Running" with live metrics
```

### Event-Driven Updates

**Backend → Frontend Events:**

```rust
// In Rust (emit events)
use tauri::Manager;

app.emit("process-started", ProcessInfo { name, pid, ... })?;
app.emit("process-crashed", ProcessCrash { name, exit_code })?;
app.emit("system-stats", SystemStats { cpu, memory, ... })?;
```

```javascript
// In Svelte (listen to events)
import { listen } from '@tauri-apps/api/event';

onMount(() => {
    const unlisten = listen('process-started', (event) => {
        processes.update(list => [...list, event.payload]);
    });
    return () => unlisten();
});
```

---

## Security Model

### Tauri Permissions

**Configuration:** `src-tauri/tauri.conf.json`

```json
{
  "permissions": {
    "fs": {
      "scope": ["$APPCONFIG/sentinel/**", "$APPDATA/sentinel/**"],
      "deny": ["$HOME/**", "/etc/**"]
    },
    "shell": {
      "scope": {
        "allowed": [
          { "name": "node", "args": true },
          { "name": "npm", "args": true },
          { "name": "cargo", "args": true }
        ]
      }
    }
  }
}
```

### Input Validation

**All Tauri commands validate inputs:**

```rust
fn validate_process_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Process name cannot be empty"));
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(anyhow!("Invalid characters in process name"));
    }
    if name.len() > 64 {
        return Err(anyhow!("Process name too long"));
    }
    Ok(())
}
```

### No Elevated Privileges

Sentinel runs with normal user permissions. No `sudo` required.

---

## Process Lifecycle

```
┌─────────┐
│ Stopped │
└────┬────┘
     │ start()
     ▼
┌──────────┐
│ Starting │
└────┬─────┘
     │ spawned
     ▼
┌─────────┐     stop()     ┌──────────┐
│ Running │────────────────►│ Stopping │
└────┬────┘                 └────┬─────┘
     │                           │
     │ crashed                   │ exited
     ▼                           ▼
┌─────────┐                 ┌─────────┐
│ Crashed │                 │ Stopped │
└────┬────┘                 └─────────┘
     │
     │ auto_restart = true
     ▼
┌──────────────┐
│ Restarting   │
│ (backoff)    │
└──────────────┘
```

---

## State Management

### Rust Backend State

**Shared across all Tauri commands:**

```rust
pub struct AppState {
    processes: Arc<Mutex<HashMap<String, ProcessInfo>>>,
    config: Arc<RwLock<Config>>,
    system: Arc<Mutex<System>>,
}
```

- `Arc<Mutex<T>>` - Thread-safe shared ownership with exclusive access
- `Arc<RwLock<T>>` - Thread-safe with multiple readers or one writer

### Svelte Frontend State

**Reactive stores:**

```javascript
// processes store
export const processes = writable([]);

// Derived store (auto-computed)
export const runningProcesses = derived(
    processes,
    $processes => $processes.filter(p => p.state === 'Running')
);

// Usage in components
$: cpuUsage = $processes.reduce((sum, p) => sum + p.cpu, 0);
```

---

## Error Handling

### Rust Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum SentinelError {
    #[error("Process '{name}' not found")]
    ProcessNotFound { name: String },

    #[error("Failed to spawn process: {source}")]
    SpawnFailed {
        #[from]
        source: std::io::Error,
    },

    #[error("Invalid configuration: {reason}")]
    InvalidConfig { reason: String },
}
```

### Propagation to Frontend

```rust
#[tauri::command]
fn risky_operation() -> Result<String, String> {
    do_something()
        .map_err(|e| e.to_string())  // Convert to String for Tauri
}
```

---

## Performance Considerations

### Memory Footprint

- **Target**: <50MB idle, <200MB with 10+ processes
- **Strategy**:
  - Reuse `sysinfo::System` instance (avoid re-allocation)
  - Limit log buffer size (circular buffer)
  - Use `Arc` for shared data (avoid cloning)

### CPU Usage

- **Target**: <5% CPU for monitoring
- **Strategy**:
  - Poll system metrics every 1-2 seconds (not 60fps)
  - Use selective refresh (`refresh_cpu()` vs `refresh_all()`)
  - Offload heavy work to background threads

### Startup Time

- **Target**: <500ms cold start
- **Strategy**:
  - Tauri (native WebView) vs Electron (bundled Chromium)
  - Lazy load config (only when needed)
  - Minimal dependencies

---

## Design Decisions

### Why Tauri over Electron?

| Reason | Impact |
|--------|--------|
| Bundle size | 3-10MB vs 80-120MB |
| Memory usage | 30-40MB vs 100+MB |
| Security | Granular permissions, no Node.js in frontend |
| Performance | Native WebView, faster startup |

### Why Rust backend?

| Reason | Impact |
|--------|--------|
| Memory safety | No segfaults, no data races |
| Performance | Native speed, no GC pauses |
| Ecosystem | `sysinfo`, `tokio` battle-tested |
| Type safety | Catch errors at compile time |

### Why Svelte frontend?

| Reason | Impact |
|--------|--------|
| Bundle size | 1.6KB runtime vs React 40KB |
| Reactivity | Built-in, no hooks boilerplate |
| Performance | Compiled, no virtual DOM |
| Developer experience | Less code, clearer intent |

### Why not PM2?

PM2 is excellent but:
- Node.js-only (Sentinel is language-agnostic)
- CLI-focused (Sentinel has GUI)
- No system monitoring (Sentinel combines both)

---

## Future Architecture Enhancements

### Phase 2 (Planned)

1. **Plugin System**: Allow custom process handlers and monitors
2. **Historical Data**: Store metrics in SQLite for trends
3. **Network Monitoring**: Track network I/O per process
4. **Distributed Mode**: Manage processes across multiple machines

---

**End of Architecture Document**

For implementation details, see:
- [Code Standards](docs/claude.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Research Report](docs/RESEARCH.md)
