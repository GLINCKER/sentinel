# 4B-1: Process Management Dashboard

**Sprint:** Phase 4B
**Duration:** 6 days
**Status:** READY
**Priority:** High
**Dependencies:** Phase 4A-1 (Process Log Viewer - REQUIRED)

---

## Pre-Implementation Research (Required)

**BEFORE writing any code, research and document:**

### 2025 Standards & Best Practices
- [x] Framework detection patterns (documented in `4-research-notes.md`)
- [x] Common dev server configurations (Next.js, Vite, FastAPI, etc.)
- [ ] Process environment variable management in Rust
- [ ] SQLite schema design for process configurations
- [ ] Rust SQLite libraries (rusqlite vs sqlx) - performance comparison
- [ ] Process health check patterns (HTTP probes, TCP checks)
- [ ] Graceful shutdown sequences for different frameworks
- [ ] Cross-platform process tree management

### Modern UI Patterns (2025)
- [ ] Process dashboard layouts (VS Code, Docker Desktop, PM2)
- [ ] Configuration modal design patterns
- [ ] Framework-specific icon libraries
- [ ] Process status visualization (color coding, badges)
- [ ] Real-time status updates without polling
- [ ] Drag-and-drop process reordering
- [ ] Bulk actions (start all, stop all)
- [ ] Process grouping/tagging patterns

### Data Persistence & Schema
- [ ] SQLite schema versioning/migration strategies
- [ ] Process configuration data model
- [ ] Environment variable encryption (sensitive data)
- [ ] JSON schema for framework templates
- [ ] Database indexing for fast queries
- [ ] Backup/restore strategies

### Documentation Requirements
Add findings to `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4-research-notes.md` under new section

---

## Objective

**Build a comprehensive process management dashboard that allows users to configure, start, stop, and monitor development processes with framework-aware templates and persistent storage.**

### Success Criteria

- [ ] Process configurations persisted in SQLite
- [ ] Framework auto-detection for 7+ common frameworks
- [ ] Process templates for quick setup (Next.js, Vite, FastAPI, etc.)
- [ ] ProcessManager view with list/grid toggle
- [ ] ProcessConfigModal for creating/editing configurations
- [ ] Start/stop/restart controls for each process
- [ ] Real-time status indicators (Starting, Running, Crashed)
- [ ] Auto-detect ports and display in UI
- [ ] Environment variable management per process
- [ ] Working directory configuration
- [ ] Process health checks (HTTP/TCP)
- [ ] Integration with 4A-1 Log Viewer (seamless navigation)
- [ ] Export/import process configurations
- [ ] WCAG 2.2 Level AA compliant

---

## Contract Specification

### Backend API Contract

#### 1. Process Configuration Storage

```rust
/// SQLite schema for process configurations
CREATE TABLE process_configs (
    id TEXT PRIMARY KEY,                 -- UUID
    name TEXT NOT NULL,                  -- User-friendly name
    command TEXT NOT NULL,               -- Executable (npm, python, java, etc.)
    args TEXT NOT NULL,                  -- JSON array ["run", "dev"]
    working_dir TEXT NOT NULL,           -- Absolute path
    env_vars TEXT,                       -- JSON object {"PORT": "3000"}
    framework_type TEXT,                 -- "NextJs", "Vite", "FastAPI", etc.
    port INTEGER,                        -- Expected port (for health checks)
    auto_start BOOLEAN DEFAULT 0,        -- Start on Sentinel launch
    health_check_url TEXT,               -- Optional HTTP health check endpoint
    created_at TEXT NOT NULL,            -- ISO 8601 timestamp
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_name ON process_configs(name);
CREATE INDEX idx_auto_start ON process_configs(auto_start);
```

#### 2. Configuration Management Commands

```rust
/// Create a new process configuration
#[tauri::command]
pub async fn create_process_config(config: ProcessConfig) -> Result<ProcessConfig, String>

/// Update an existing configuration
#[tauri::command]
pub async fn update_process_config(config: ProcessConfig) -> Result<ProcessConfig, String>

/// Delete a configuration
#[tauri::command]
pub async fn delete_process_config(id: String) -> Result<(), String>

/// Get all configurations
#[tauri::command]
pub async fn list_process_configs() -> Result<Vec<ProcessConfig>, String>

/// Get a single configuration by ID
#[tauri::command]
pub async fn get_process_config(id: String) -> Result<ProcessConfig, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessConfig {
    pub id: String,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: String,
    pub env_vars: HashMap<String, String>,
    pub framework_type: Option<FrameworkType>,
    pub port: Option<u16>,
    pub auto_start: bool,
    pub health_check_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FrameworkType {
    NextJs,
    Vite,
    FastAPI,
    SpringBoot,
    Django,
    Express,
    Flask,
    Unknown,
}
```

#### 3. Git Repository & Project Discovery

```rust
/// Browse for Git repositories (similar to GitHub Desktop)
#[tauri::command]
pub async fn browse_git_repositories(start_path: Option<String>) -> Result<Vec<GitRepository>, String>

/// Scan a directory for projects (supports monorepos)
#[tauri::command]
pub async fn scan_directory_for_projects(dir_path: String) -> Result<Vec<DetectedProject>, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitRepository {
    pub path: String,
    pub name: String,
    pub branch: String,
    pub is_dirty: bool,
    pub remote_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DetectedProject {
    pub path: String,
    pub name: String,
    pub framework_type: FrameworkType,
    pub confidence: f32,
    pub suggested_command: String,
    pub suggested_args: Vec<String>,
    pub suggested_port: Option<u16>,
    pub package_manager: Option<String>,  // npm, pnpm, yarn, pip, gradle, maven
    pub detected_files: Vec<String>,
}
```

#### 4. Framework Auto-Detection

```rust
/// Detect framework type from a directory
#[tauri::command]
pub async fn detect_framework(working_dir: String) -> Result<FrameworkDetection, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FrameworkDetection {
    pub framework_type: FrameworkType,
    pub confidence: f32,              // 0.0 - 1.0
    pub detected_files: Vec<String>,  // Files used for detection
    pub suggested_command: String,    // e.g., "npm"
    pub suggested_args: Vec<String>,  // e.g., ["run", "dev"]
    pub suggested_port: Option<u16>,  // e.g., 3000 for Next.js
    pub package_manager: Option<String>,
}
```

#### 4. Process Control

```rust
/// Start a process from a configuration
#[tauri::command]
pub async fn start_process_from_config(config_id: String) -> Result<ManagedProcess, String>

/// Stop a running process by config ID
#[tauri::command]
pub async fn stop_process_by_config_id(config_id: String) -> Result<(), String>

/// Restart a process
#[tauri::command]
pub async fn restart_process(config_id: String) -> Result<ManagedProcess, String>

/// Get process status by config ID
#[tauri::command]
pub async fn get_process_status_by_config(config_id: String) -> Result<ProcessStatusInfo, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessStatusInfo {
    pub config_id: String,
    pub running: bool,
    pub process_id: Option<String>,  // ManagedProcess ID if running
    pub pid: Option<u32>,
    pub status: Option<ProcessStatus>,
    pub uptime_seconds: Option<u64>,
    pub last_health_check: Option<HealthCheckResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HealthCheckResult {
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub response_time_ms: u64,
    pub error: Option<String>,
}
```

#### 5. Templates & Import/Export

```rust
/// Get built-in framework templates
#[tauri::command]
pub async fn get_framework_templates() -> Result<Vec<ProcessTemplate>, String>

/// Export configurations to JSON
#[tauri::command]
pub async fn export_configs() -> Result<String, String>  // Returns JSON string

/// Import configurations from JSON
#[tauri::command]
pub async fn import_configs(json: String) -> Result<Vec<ProcessConfig>, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessTemplate {
    pub name: String,
    pub framework_type: FrameworkType,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub default_port: Option<u16>,
    pub default_env_vars: HashMap<String, String>,
    pub health_check_url: Option<String>,
    pub icon: String,  // Emoji or icon name
}
```

---

### Frontend UI Contract

#### 1. ProcessManager View

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/routes/processes/+page.svelte`

```typescript
<script lang="ts">
  import { onMount } from 'svelte';
  import { processConfigStore } from '$lib/stores/processConfig.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import Button from '$lib/components/Button.svelte';
  import ProcessCard from '$lib/components/ProcessCard.svelte';
  import ProcessConfigModal from '$lib/components/ProcessConfigModal.svelte';
  import { Settings } from 'lucide-svelte';

  let showConfigModal = $state(false);
  let editingConfig = $state<ProcessConfig | null>(null);
  let viewMode = $state<'grid' | 'list'>('grid');

  onMount(() => {
    processConfigStore.loadConfigs();
  });

  function createNew() {
    editingConfig = null;
    showConfigModal = true;
  }

  function editConfig(config: ProcessConfig) {
    editingConfig = config;
    showConfigModal = true;
  }

  async function deleteConfig(id: string) {
    if (!confirm('Delete this process configuration?')) return;
    await processConfigStore.deleteConfig(id);
  }

  async function startProcess(configId: string) {
    await processConfigStore.startProcess(configId);
  }

  async function stopProcess(configId: string) {
    await processConfigStore.stopProcess(configId);
  }
</script>

<!-- UI Below -->
```

**UI Layout (Grid View):**

```
┌────────────────────────────────────────────────────────────────┐
│ ⚙️ Process Manager                    [+ New] [Grid/List]      │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│ ┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐│
│ │ ▲ Next.js Dev    │ │ 🐍 FastAPI       │ │ ☕ Spring Boot   ││
│ │ ─────────────────│ │ ─────────────────│ │ ─────────────────││
│ │ 🟢 Running       │ │ 🔴 Stopped       │ │ 🟡 Starting      ││
│ │ PID: 12345       │ │ -                │ │ PID: 67890       ││
│ │ Port: 3000       │ │ Port: 8000       │ │ Port: 8080       ││
│ │ Uptime: 2h 15m   │ │ -                │ │ Uptime: 5s       ││
│ │                  │ │                  │ │                  ││
│ │ [Stop] [Restart] │ │ [Start] [Edit]   │ │ [Stop] [Logs]    ││
│ │ [Logs] [Delete]  │ │ [Delete]         │ │ [Edit]           ││
│ └──────────────────┘ └──────────────────┘ └──────────────────┘│
│                                                                │
│ ┌──────────────────┐ ┌──────────────────┐                     │
│ │ ⚛️ Vite Dev      │ │ 🐘 PostgreSQL    │                     │
│ │ ─────────────────│ │ ─────────────────│                     │
│ │ 🟢 Running       │ │ 🟢 Running       │                     │
│ │ PID: 34567       │ │ PID: 89012       │                     │
│ │ Port: 5173       │ │ Port: 5432       │                     │
│ │ Uptime: 45m      │ │ Uptime: 3d 12h   │                     │
│ │                  │ │                  │                     │
│ │ [Stop] [Restart] │ │ [Stop] [Restart] │                     │
│ │ [Logs] [Delete]  │ │ [Logs] [Shell]   │                     │
│ └──────────────────┘ └──────────────────┘                     │
└────────────────────────────────────────────────────────────────┘
```

**Component Props:**

```typescript
interface ProcessManagerProps {
  // Top-level view, no props
}

interface ProcessCardProps {
  config: ProcessConfig;
  status: ProcessStatusInfo;
  onStart: () => void;
  onStop: () => void;
  onRestart: () => void;
  onEdit: () => void;
  onDelete: () => void;
  onViewLogs: () => void;
}

interface ProcessConfig {
  id: string;
  name: string;
  command: string;
  args: string[];
  workingDir: string;
  envVars: Record<string, string>;
  frameworkType?: string;
  port?: number;
  autoStart: boolean;
  healthCheckUrl?: string;
  createdAt: string;
  updatedAt: string;
}

interface ProcessStatusInfo {
  configId: string;
  running: boolean;
  processId?: string;
  pid?: number;
  status?: 'Starting' | 'Running' | 'Stopped' | 'Crashed';
  uptimeSeconds?: number;
  lastHealthCheck?: HealthCheckResult;
}
```

#### 2. ProcessConfigModal Component

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/ProcessConfigModal.svelte`

**UI Layout:**

```
┌────────────────────────────────────────────────────────────────┐
│ ⚙️ New Process Configuration                              [X]  │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│ Framework Template (optional):                                 │
│ [▼ Select template...                                    ]     │
│   • Next.js Development Server                                 │
│   • Vite Development Server                                    │
│   • FastAPI Server                                             │
│   • Spring Boot Application                                    │
│   • Django Development Server                                  │
│   • Express Server                                             │
│   • Flask Application                                          │
│   • Custom (no template)                                       │
│                                                                │
│ ─────────────────────────────────────────────────────────────  │
│                                                                │
│ Process Name: *                                                │
│ [My Next.js App                                         ]      │
│                                                                │
│ Working Directory: *                                           │
│ [/Users/dev/my-project                    ] [Browse...]        │
│ [🔍 Auto-detect framework]                                     │
│                                                                │
│ Command: *                                                     │
│ [npm                                                    ]      │
│                                                                │
│ Arguments: *                                                   │
│ [run, dev                                               ]      │
│ (comma-separated)                                              │
│                                                                │
│ Expected Port (optional):                                      │
│ [3000                                                   ]      │
│                                                                │
│ Health Check URL (optional):                                   │
│ [http://localhost:3000                                  ]      │
│                                                                │
│ Environment Variables:                                         │
│ ┌────────────────────────────────────────────────────────────┐│
│ │ KEY             VALUE                          [+ Add]     ││
│ │ NODE_ENV        development                    [X]         ││
│ │ PORT            3000                           [X]         ││
│ │                                                            ││
│ └────────────────────────────────────────────────────────────┘│
│                                                                │
│ ☑ Auto-start when Sentinel launches                           │
│                                                                │
│                                                                │
│                                    [Cancel] [Save]             │
└────────────────────────────────────────────────────────────────┘
```

**Component Props:**

```typescript
interface ProcessConfigModalProps {
  isOpen: boolean;
  editingConfig?: ProcessConfig | null;  // null for new, ProcessConfig for edit
  onClose: () => void;
  onSave: (config: ProcessConfig) => void;
}
```

#### 3. ProcessCard Component

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/ProcessCard.svelte`

Glass morphism card with status indicator, quick actions, and framework icon.

```typescript
<script lang="ts">
  interface Props {
    config: ProcessConfig;
    status: ProcessStatusInfo;
    onStart: () => void;
    onStop: () => void;
    onRestart: () => void;
    onEdit: () => void;
    onDelete: () => void;
    onViewLogs: () => void;
  }

  let { config, status, onStart, onStop, onRestart, onEdit, onDelete, onViewLogs }: Props = $props();

  const frameworkIcons = {
    NextJs: '▲',
    Vite: '⚡',
    FastAPI: '🐍',
    SpringBoot: '☕',
    Django: '🎸',
    Express: '🚂',
    Flask: '🌶️',
    Unknown: '⚙️'
  };

  const statusColors = {
    Starting: '#f59e0b',
    Running: '#10b981',
    Stopped: '#6b7280',
    Crashed: '#ef4444'
  };

  let icon = $derived(frameworkIcons[config.frameworkType || 'Unknown']);
  let statusColor = $derived(statusColors[status.status || 'Stopped']);
  let uptimeFormatted = $derived(formatUptime(status.uptimeSeconds));

  function formatUptime(seconds?: number): string {
    if (!seconds) return '-';
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    if (days > 0) return `${days}d ${hours}h`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  }
</script>
```

#### 4. Process Config Store

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/stores/processConfig.svelte.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface ProcessConfig {
  id: string;
  name: string;
  command: string;
  args: string[];
  workingDir: string;
  envVars: Record<string, string>;
  frameworkType?: string;
  port?: number;
  autoStart: boolean;
  healthCheckUrl?: string;
  createdAt: string;
  updatedAt: string;
}

interface ProcessStatusInfo {
  configId: string;
  running: boolean;
  processId?: string;
  pid?: number;
  status?: 'Starting' | 'Running' | 'Stopped' | 'Crashed';
  uptimeSeconds?: number;
  lastHealthCheck?: HealthCheckResult;
}

class ProcessConfigStore {
  configs = $state<ProcessConfig[]>([]);
  statuses = $state<Map<string, ProcessStatusInfo>>(new Map());
  loading = $state(false);
  error = $state<string | null>(null);

  async loadConfigs() {
    try {
      this.loading = true;
      this.configs = await invoke<ProcessConfig[]>('list_process_configs');

      // Load statuses for all configs
      for (const config of this.configs) {
        const status = await invoke<ProcessStatusInfo>('get_process_status_by_config', {
          configId: config.id
        });
        this.statuses.set(config.id, status);
      }
    } catch (err) {
      this.error = String(err);
    } finally {
      this.loading = false;
    }
  }

  async createConfig(config: Omit<ProcessConfig, 'id' | 'createdAt' | 'updatedAt'>) {
    const created = await invoke<ProcessConfig>('create_process_config', { config });
    this.configs = [...this.configs, created];
    return created;
  }

  async updateConfig(config: ProcessConfig) {
    const updated = await invoke<ProcessConfig>('update_process_config', { config });
    this.configs = this.configs.map(c => c.id === updated.id ? updated : c);
    return updated;
  }

  async deleteConfig(id: string) {
    await invoke('delete_process_config', { id });
    this.configs = this.configs.filter(c => c.id !== id);
    this.statuses.delete(id);
  }

  async startProcess(configId: string) {
    const process = await invoke('start_process_from_config', { configId });
    await this.refreshStatus(configId);
  }

  async stopProcess(configId: string) {
    await invoke('stop_process_by_config_id', { configId });
    await this.refreshStatus(configId);
  }

  async restartProcess(configId: string) {
    await invoke('restart_process', { configId });
    await this.refreshStatus(configId);
  }

  async detectFramework(workingDir: string) {
    return await invoke('detect_framework', { workingDir });
  }

  async getTemplates() {
    return await invoke('get_framework_templates');
  }

  async exportConfigs() {
    return await invoke<string>('export_configs');
  }

  async importConfigs(json: string) {
    const imported = await invoke<ProcessConfig[]>('import_configs', { json });
    await this.loadConfigs();
    return imported;
  }

  private async refreshStatus(configId: string) {
    const status = await invoke<ProcessStatusInfo>('get_process_status_by_config', { configId });
    this.statuses.set(configId, status);
  }
}

export const processConfigStore = new ProcessConfigStore();
```

---

## Implementation Tasks

### Backend Implementation

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/process_config.rs`

- [ ] Create SQLite database connection pool
- [ ] Implement database migrations (create `process_configs` table)
- [ ] Implement `create_process_config` command
- [ ] Implement `update_process_config` command
- [ ] Implement `delete_process_config` command
- [ ] Implement `list_process_configs` command
- [ ] Implement `get_process_config` command
- [ ] Add validation for required fields
- [ ] Add unique name constraint handling
- [ ] Serialize/deserialize env_vars and args as JSON

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/framework_detector.rs`

- [ ] Implement `detect_framework` command
- [ ] Next.js detection (check for `next` in `package.json`)
- [ ] Vite detection (check for `vite` in `package.json`)
- [ ] FastAPI detection (check for `fastapi` in `requirements.txt` or `main.py`)
- [ ] Spring Boot detection (check for `pom.xml` or `build.gradle`)
- [ ] Django detection (check for `manage.py`)
- [ ] Express detection (check for `express` in `package.json`)
- [ ] Flask detection (check for `flask` in `requirements.txt` or `app.py`)
- [ ] Return confidence score based on file matches
- [ ] Suggest default command, args, and port per framework

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/process_templates.rs`

- [ ] Define built-in templates for 7 frameworks
- [ ] Implement `get_framework_templates` command
- [ ] Include default env vars per template
- [ ] Include health check URLs per template

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/process_control.rs`

- [ ] Implement `start_process_from_config` command
- [ ] Load config from DB, spawn process via `process_manager.rs`
- [ ] Map config ID to managed process ID
- [ ] Implement `stop_process_by_config_id` command
- [ ] Implement `restart_process` command (stop then start)
- [ ] Implement `get_process_status_by_config` command
- [ ] Track uptime by comparing current time to `started_at`
- [ ] Implement basic HTTP health check (optional)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/import_export.rs`

- [ ] Implement `export_configs` command (serialize to JSON)
- [ ] Implement `import_configs` command (deserialize and insert)
- [ ] Handle ID conflicts on import (regenerate UUIDs)
- [ ] Validate imported data schema

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/lib.rs`

- [ ] Initialize SQLite database on app startup
- [ ] Run migrations
- [ ] Register all new commands in Tauri builder
- [ ] Implement auto-start logic (start processes with `auto_start: true` on launch)

### Frontend Implementation

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/routes/processes/+page.svelte`

- [ ] Create ProcessManager page route
- [ ] Use PageHeader component with Settings icon
- [ ] Implement grid/list view toggle
- [ ] Load configs on mount via store
- [ ] Render ProcessCard components in grid layout
- [ ] Handle create/edit/delete actions
- [ ] Show empty state when no configs
- [ ] Add loading skeleton

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/ProcessCard.svelte`

- [ ] Create glass morphism card component
- [ ] Display framework icon, name, status badge
- [ ] Display PID, port, uptime
- [ ] Add Start/Stop/Restart buttons
- [ ] Add View Logs button (opens 4A-1 LogViewer)
- [ ] Add Edit/Delete buttons
- [ ] Color-code status indicator (green/red/yellow/gray)
- [ ] Add hover effects
- [ ] Add tooltips for all buttons

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/ProcessConfigModal.svelte`

- [ ] Create modal component with form
- [ ] Template selector dropdown
- [ ] Auto-populate fields when template selected
- [ ] Working directory input with file browser
- [ ] Auto-detect framework button
- [ ] Command and args inputs
- [ ] Port and health check URL inputs (optional)
- [ ] Environment variable key-value editor
- [ ] Auto-start checkbox
- [ ] Form validation (required fields)
- [ ] Save handler (create or update)
- [ ] Cancel button closes modal

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/stores/processConfig.svelte.ts`

- [ ] Implement ProcessConfigStore with Svelte 5 runes
- [ ] Implement loadConfigs, createConfig, updateConfig, deleteConfig
- [ ] Implement startProcess, stopProcess, restartProcess
- [ ] Implement detectFramework, getTemplates
- [ ] Implement exportConfigs, importConfigs
- [ ] Subscribe to process status events from 4A-1
- [ ] Auto-refresh statuses every 5 seconds

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/EnvVarEditor.svelte`

- [ ] Create reusable component for env var editing
- [ ] Add/remove key-value pairs
- [ ] Validation (no empty keys)
- [ ] Show/hide sensitive values (password fields)
- [ ] Import from .env file (optional)

### UI/UX Polish

- [ ] Add confirmation dialogs for destructive actions
- [ ] Add toast notifications for success/error
- [ ] Add keyboard shortcuts (Cmd+N for new)
- [ ] Add drag-and-drop for reordering (future)
- [ ] Add bulk actions (start all, stop all)
- [ ] Add search/filter by name or framework
- [ ] Add sort by name, status, uptime

---

## Testing Requirements

### Backend Tests (Rust)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/__tests__/process_config_test.rs`

- [ ] Test CRUD operations (create, read, update, delete)
- [ ] Test unique name constraint
- [ ] Test JSON serialization of env_vars and args
- [ ] Test database migrations
- [ ] Test concurrent access (multiple inserts)
- [ ] Test cascading deletes
- [ ] Test query performance with 100+ configs

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/__tests__/framework_detector_test.rs`

- [ ] Test Next.js detection from sample project
- [ ] Test Vite detection from sample project
- [ ] Test FastAPI detection from sample project
- [ ] Test Spring Boot detection from sample project
- [ ] Test Django detection from sample project
- [ ] Test Express detection from sample project
- [ ] Test Flask detection from sample project
- [ ] Test unknown framework fallback
- [ ] Test confidence scoring accuracy

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/__tests__/import_export_test.rs`

- [ ] Test export to JSON
- [ ] Test import from JSON
- [ ] Test import with ID conflicts
- [ ] Test import validation (reject malformed JSON)

**Target:** 25+ unit tests passing

### Frontend Tests (TypeScript/Vitest)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/routes/processes/__tests__/+page.test.ts`

- [ ] Test page renders with mock configs
- [ ] Test grid/list view toggle
- [ ] Test create new config flow
- [ ] Test edit config flow
- [ ] Test delete config flow
- [ ] Test start/stop actions
- [ ] Test empty state

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/__tests__/ProcessCard.test.ts`

- [ ] Test renders correct status colors
- [ ] Test uptime formatting
- [ ] Test button states (disabled when stopped, etc.)
- [ ] Test framework icon display

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/__tests__/ProcessConfigModal.test.ts`

- [ ] Test template selection populates fields
- [ ] Test form validation
- [ ] Test save creates new config
- [ ] Test save updates existing config
- [ ] Test cancel closes modal

**Target:** 20+ component tests passing

### Integration Tests

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/tests/process_management_integration.rs`

- [ ] Create config → start process → verify running
- [ ] Stop process → verify stopped
- [ ] Restart process → verify PID changed
- [ ] Delete config → verify process stopped
- [ ] Auto-start on app launch
- [ ] Export → import → verify configs restored

**Target:** 8+ integration tests passing

---

## Performance Requirements

- [ ] Config list query: < 50ms for 100+ configs
- [ ] Framework detection: < 200ms per directory
- [ ] Start process from config: < 300ms
- [ ] Database writes: < 20ms per operation
- [ ] UI renders 50+ process cards without lag
- [ ] Real-time status updates: < 1s latency
- [ ] Export/import 100 configs: < 2s

---

## Accessibility Requirements

WCAG 2.2 Level AA compliance:

- [ ] All buttons keyboard accessible (Tab navigation)
- [ ] Modal has focus trap (Tab cycles within modal)
- [ ] Form inputs have `aria-label` or `<label>` elements
- [ ] Required fields marked with `aria-required="true"`
- [ ] Error messages announced to screen readers
- [ ] Status badges have `aria-label` (e.g., "Status: Running")
- [ ] Color not sole indicator (use icons + text)
- [ ] Focus indicators visible on all interactive elements
- [ ] Keyboard shortcuts non-conflicting
- [ ] Grid/List toggle has `aria-pressed` state

---

## UI Design Specifications

### Glass Morphism Theme

Match Dashboard and Port Map:

```css
/* Process Card */
.process-card {
  background: rgba(20, 20, 25, 0.6);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
  transition: all 0.3s;
}

.process-card:hover {
  border-color: rgba(59, 130, 246, 0.3);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  transform: translateY(-2px);
}

/* Status Badge */
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
}

.status-running {
  background: rgba(16, 185, 129, 0.15);
  color: #10b981;
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.status-stopped {
  background: rgba(107, 114, 128, 0.15);
  color: #9ca3af;
  border: 1px solid rgba(107, 114, 128, 0.3);
}

/* Config Modal */
.config-modal {
  background: rgba(15, 15, 20, 0.95);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
}
```

### Colors

- Background: `rgba(20, 20, 25, 0.6)`
- Status Running: `#10b981` (green-500)
- Status Starting: `#f59e0b` (amber-500)
- Status Stopped: `#6b7280` (gray-500)
- Status Crashed: `#ef4444` (red-500)
- Accent: `#3b82f6` (blue-500)

### Framework Icons

- Next.js: `▲` or React logo
- Vite: `⚡` or Vite logo
- FastAPI: `🐍` or FastAPI logo
- Spring Boot: `☕` or Spring logo
- Django: `🎸` or Django logo
- Express: `🚂` or Express logo
- Flask: `🌶️` or Flask logo
- Unknown: `⚙️`

---

## Definition of Done

- [ ] Backend: SQLite schema created with migrations
- [ ] Backend: All CRUD commands implemented
- [ ] Backend: Framework auto-detection working for 7 frameworks
- [ ] Backend: Templates for 7 frameworks defined
- [ ] Backend: Process control commands (start/stop/restart) working
- [ ] Backend: Import/export functionality working
- [ ] Frontend: ProcessManager page fully functional
- [ ] Frontend: ProcessCard component complete with all actions
- [ ] Frontend: ProcessConfigModal complete with form validation
- [ ] Frontend: Store implemented with all methods
- [ ] Integration: "View Logs" opens 4A-1 LogViewer
- [ ] Integration: Auto-start processes on app launch
- [ ] Tests: 25+ backend unit tests passing
- [ ] Tests: 20+ frontend component tests passing
- [ ] Tests: 8+ integration tests passing
- [ ] Performance: All metrics met
- [ ] Accessibility: WCAG 2.2 Level AA compliant
- [ ] Documentation: User guide with screenshots
- [ ] Documentation: API docs generated
- [ ] Code Review: PR approved
- [ ] Demo: Video showing create/start/monitor workflow

---

## Notes

### Technical Decisions

1. **SQLite over file-based storage**: Better query performance, ACID compliance, easier migrations.

2. **rusqlite vs sqlx**: Using `rusqlite` for simpler setup. `sqlx` deferred if async queries needed later.

3. **Framework detection confidence**: Multi-file matching increases confidence (e.g., `package.json` + `next.config.js` = 95% confidence).

4. **Health checks**: Optional HTTP GET to verify service is up. Timeout after 5s.

5. **Auto-start**: Implemented in `main.rs` after app initialization, before window shown.

### Future Enhancements (Phase 5)

- Process grouping/tagging (e.g., "Frontend", "Backend", "Database")
- Environment profiles (dev, staging, prod)
- Process dependencies (start order)
- Resource monitoring (CPU, memory per process)
- Log level configuration per process
- Scheduled tasks/cron jobs
- Docker container support (delegate to Phase 3B if applicable)

### References

- Research notes: `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4-research-notes.md`
- Process Log Viewer (4A-1): `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4A-1_PROCESS_LOG_VIEWER.md`

---

**Start Date:** TBD
**End Date:** TBD
**Assignee:** TBD

**This is the contract for Phase 4B-1. All requirements must be met before moving to 4C-1.**
