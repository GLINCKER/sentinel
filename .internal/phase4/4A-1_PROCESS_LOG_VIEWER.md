# 4A-1: Process Log Viewer

**Sprint:** Phase 4A
**Duration:** 5 days
**Status:** READY
**Priority:** Critical
**Dependencies:** Phase 3A-1 (Port Discovery - COMPLETED)

---

## Pre-Implementation Research (Required)

**BEFORE writing any code, research and document:**

### 2025 Standards & Best Practices
- [x] Tauri Command API for process spawning (documented in `4-research-notes.md`)
- [x] stdout/stderr capture patterns with event streaming
- [x] Circular buffer implementation for log management
- [x] Performance considerations for high-throughput logging
- [ ] ANSI color code parsing libraries (Rust + TypeScript)
- [ ] xterm.js performance benchmarks for log rendering
- [ ] Virtual scrolling libraries compatible with Svelte 5
- [ ] Modern regex search patterns for log filtering

### Modern UI Patterns (2025)
- [x] Log viewer component patterns (VS Code, iTerm2, Warp)
- [x] Virtual scrolling for 10k+ lines
- [x] Search & highlight with regex support
- [x] Auto-scroll "follow mode" toggle
- [ ] ANSI escape sequence rendering
- [ ] Line wrapping strategies
- [ ] Copy/paste handling in log views
- [ ] Keyboard shortcuts for log navigation

### Performance & Scalability
- [x] Circular buffer (10k lines per process) - see research notes
- [x] 50ms batch updates for high-throughput logs
- [ ] Memory profiling for long-running processes
- [ ] WebWorker consideration for log parsing
- [ ] IndexedDB for persistent log storage (optional)
- [ ] Lazy loading of historical logs

### Documentation Requirements
All research findings documented in `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4-research-notes.md`

---

## Objective

**Build a real-time log viewer that captures stdout/stderr from Sentinel-managed processes and displays them in a searchable, scrollable interface with ANSI color support.**

### Success Criteria

- [ ] Process stdout/stderr captured in real-time (< 50ms latency)
- [ ] Circular buffer maintains last 10k lines per process
- [ ] Virtual scrolling handles 10k+ lines without lag
- [ ] Search with regex support highlights matches
- [ ] Auto-scroll "follow mode" toggle works smoothly
- [ ] ANSI color codes rendered correctly
- [ ] "View Logs" button integrated into Port Map
- [ ] Log viewer opens in modal or side panel
- [ ] Copy log selection to clipboard
- [ ] Clear logs button resets buffer
- [ ] WCAG 2.2 Level AA compliant

---

## Contract Specification

### Backend API Contract

#### 1. Process Spawning with Log Capture

```rust
/// Spawn a process with stdout/stderr capture
#[tauri::command]
pub async fn spawn_managed_process(
    command: String,
    args: Vec<String>,
    working_dir: Option<String>,
    process_name: Option<String>,
) -> Result<ManagedProcess, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManagedProcess {
    pub id: String,              // Unique process ID (UUID)
    pub name: String,            // User-friendly name
    pub command: String,         // Executable command
    pub args: Vec<String>,       // Command arguments
    pub pid: u32,                // OS process ID
    pub status: ProcessStatus,   // Running, Stopped, Crashed
    pub started_at: DateTime<Utc>,
    pub port: Option<u16>,       // Auto-detected port
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProcessStatus {
    Starting,
    Running,
    Stopped,
    Crashed,
}
```

#### 2. Log Streaming

```rust
/// Get buffered logs for a process
#[tauri::command]
pub async fn get_process_logs(
    process_id: String,
    offset: usize,
    limit: usize,
) -> Result<LogBatch, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogBatch {
    pub lines: Vec<LogLine>,
    pub total_lines: usize,       // Total in buffer
    pub has_more: bool,           // More lines available
    pub buffer_full: bool,        // Circular buffer wrapped
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogLine {
    pub timestamp: DateTime<Utc>,
    pub stream: LogStream,        // Stdout or Stderr
    pub content: String,          // Raw log line (may contain ANSI codes)
    pub line_number: usize,       // Sequential line number
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogStream {
    Stdout,
    Stderr,
}
```

#### 3. Log Management

```rust
/// Clear all buffered logs for a process
#[tauri::command]
pub async fn clear_process_logs(process_id: String) -> Result<(), String>

/// Stop a managed process
#[tauri::command]
pub async fn stop_managed_process(process_id: String) -> Result<(), String>

/// Get all managed processes
#[tauri::command]
pub async fn list_managed_processes() -> Result<Vec<ManagedProcess>, String>
```

#### 4. Tauri Event Streaming

```rust
// Event emitted on each new log line (batched every 50ms)
#[derive(Serialize, Clone)]
struct LogEvent {
    process_id: String,
    lines: Vec<LogLine>,
}
// Emit via: app.emit_all("process-log", payload)

// Event emitted when process status changes
#[derive(Serialize, Clone)]
struct ProcessStatusEvent {
    process_id: String,
    status: ProcessStatus,
    exit_code: Option<i32>,
}
// Emit via: app.emit_all("process-status", payload)
```

---

### Frontend UI Contract

#### 1. LogViewer Component

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/LogViewer.svelte`

```typescript
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  interface Props {
    processId: string;
    processName: string;
    onClose?: () => void;
  }

  interface LogLine {
    timestamp: string;
    stream: 'Stdout' | 'Stderr';
    content: string;
    lineNumber: number;
  }

  let { processId, processName, onClose }: Props = $props();

  let logs = $state<LogLine[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let searchQuery = $state('');
  let searchRegex = $state(false);
  let autoScroll = $state(true);
  let container: HTMLDivElement;

  // Virtual scrolling state
  let visibleStart = $state(0);
  let visibleEnd = $state(100);
  const LINE_HEIGHT = 20; // pixels

  onMount(async () => {
    // Load initial logs
    await loadLogs();

    // Subscribe to real-time log events
    const unlisten = await listen('process-log', (event: any) => {
      if (event.payload.processId === processId) {
        logs = [...logs, ...event.payload.lines];

        // Auto-scroll if enabled
        if (autoScroll && container) {
          container.scrollTop = container.scrollHeight;
        }
      }
    });

    return () => {
      unlisten();
    };
  });

  async function loadLogs() {
    try {
      loading = true;
      const batch = await invoke<LogBatch>('get_process_logs', {
        processId,
        offset: 0,
        limit: 10000,
      });
      logs = batch.lines;
    } catch (err) {
      error = String(err);
    } finally {
      loading = false;
    }
  }

  async function clearLogs() {
    if (!confirm('Clear all logs for this process?')) return;
    try {
      await invoke('clear_process_logs', { processId });
      logs = [];
    } catch (err) {
      alert(`Failed to clear logs: ${err}`);
    }
  }

  function copyLogs() {
    const text = filteredLogs.map(l => l.content).join('\n');
    navigator.clipboard.writeText(text);
  }

  // Derived: filtered logs
  let filteredLogs = $derived.by(() => {
    if (!searchQuery.trim()) return logs;

    try {
      if (searchRegex) {
        const regex = new RegExp(searchQuery, 'i');
        return logs.filter(l => regex.test(l.content));
      } else {
        const query = searchQuery.toLowerCase();
        return logs.filter(l => l.content.toLowerCase().includes(query));
      }
    } catch {
      return logs; // Invalid regex
    }
  });
</script>

<!-- UI Below -->
```

**UI Layout:**

```
┌────────────────────────────────────────────────────────────────┐
│ 📄 Logs: next-dev (PID 12345)                    [X]           │
├────────────────────────────────────────────────────────────────┤
│ 🔍 [Search...] [☑ Regex] [↓ Follow] [📋 Copy] [🗑️ Clear]     │
├────────────────────────────────────────────────────────────────┤
│ 1  [12:34:56] [stdout] > next-dev@0.0.0 dev                   │
│ 2  [12:34:57] [stdout] > next dev                             │
│ 3  [12:34:58] [stdout]                                        │
│ 4  [12:34:59] [stdout]   ▲ Next.js 14.0.0                     │
│ 5  [12:35:00] [stdout]   - Local:        http://localhost:3000│
│ 6  [12:35:01] [stdout]   - Environments: .env.local            │
│ 7  [12:35:02] [stdout]                                        │
│ 8  [12:35:03] [stdout] ✓ Ready in 2.5s                       │
│ 9  [12:35:04] [stderr] ⚠ Warning: React component hydration   │
│ 10 [12:35:05] [stdout] ○ Compiling /page ...                 │
│                                                                │
│ [Virtual scrolled content - 9,990 more lines...]              │
├────────────────────────────────────────────────────────────────┤
│ 10,000 lines • Auto-scroll: ON • Last update: 2s ago          │
└────────────────────────────────────────────────────────────────┘
```

**Component Props:**

```typescript
interface LogViewerProps {
  processId: string;
  processName: string;
  onClose?: () => void;
}

interface LogLine {
  timestamp: string;
  stream: 'Stdout' | 'Stderr';
  content: string;
  lineNumber: number;
}

interface LogBatch {
  lines: LogLine[];
  totalLines: number;
  hasMore: boolean;
  bufferFull: boolean;
}
```

#### 2. Port Map Integration

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/PortMap.svelte` (modify existing)

Add "View Logs" button to each port row:

```typescript
// In PortRow component
<Button
  size="sm"
  variant="ghost"
  onclick={() => openLogViewer(port.pid)}
  disabled={!isManagedProcess(port.pid)}
>
  View Logs
</Button>

function isManagedProcess(pid: number): boolean {
  // Check if this PID corresponds to a managed process
  return managedProcesses.some(p => p.pid === pid);
}

function openLogViewer(pid: number) {
  // Open LogViewer modal with process ID
  showLogModal = true;
  selectedProcessId = getManagedProcessByPid(pid)?.id;
}
```

#### 3. Process Log Store

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/stores/processLog.svelte.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

interface ManagedProcess {
  id: string;
  name: string;
  command: string;
  args: string[];
  pid: number;
  status: 'Starting' | 'Running' | 'Stopped' | 'Crashed';
  startedAt: string;
  port?: number;
}

interface LogLine {
  timestamp: string;
  stream: 'Stdout' | 'Stderr';
  content: string;
  lineNumber: number;
}

class ProcessLogStore {
  // State
  processes = $state<Map<string, ManagedProcess>>(new Map());
  logs = $state<Map<string, LogLine[]>>(new Map());
  loading = $state(false);
  error = $state<string | null>(null);

  private unlisteners: UnlistenFn[] = [];

  async init() {
    // Load managed processes
    await this.loadProcesses();

    // Subscribe to events
    const logUnlisten = await listen('process-log', (event: any) => {
      const { processId, lines } = event.payload;
      const existing = this.logs.get(processId) || [];
      this.logs.set(processId, [...existing, ...lines]);
    });

    const statusUnlisten = await listen('process-status', (event: any) => {
      const { processId, status, exitCode } = event.payload;
      const process = this.processes.get(processId);
      if (process) {
        process.status = status;
        this.processes.set(processId, process);
      }
    });

    this.unlisteners = [logUnlisten, statusUnlisten];
  }

  async loadProcesses() {
    try {
      this.loading = true;
      const processes = await invoke<ManagedProcess[]>('list_managed_processes');
      this.processes = new Map(processes.map(p => [p.id, p]));
    } catch (err) {
      this.error = String(err);
    } finally {
      this.loading = false;
    }
  }

  async spawnProcess(
    command: string,
    args: string[],
    workingDir?: string,
    name?: string
  ): Promise<string> {
    const process = await invoke<ManagedProcess>('spawn_managed_process', {
      command,
      args,
      workingDir,
      processName: name,
    });
    this.processes.set(process.id, process);
    return process.id;
  }

  async stopProcess(processId: string) {
    await invoke('stop_managed_process', { processId });
  }

  async clearLogs(processId: string) {
    await invoke('clear_process_logs', { processId });
    this.logs.set(processId, []);
  }

  cleanup() {
    this.unlisteners.forEach(fn => fn());
  }
}

export const processLogStore = new ProcessLogStore();
```

---

## Implementation Tasks

### Backend Implementation

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/process_manager.rs`

- [ ] Create `ManagedProcess` struct with circular log buffer (10k lines)
- [ ] Implement `spawn_managed_process` command using `tokio::process::Command`
- [ ] Capture stdout/stderr with `AsyncBufRead` + `lines()` stream
- [ ] Batch log events every 50ms using `tokio::time::interval`
- [ ] Emit `process-log` events via Tauri app handle
- [ ] Implement `get_process_logs` with pagination (offset + limit)
- [ ] Implement `clear_process_logs` to reset buffer
- [ ] Implement `stop_managed_process` with graceful shutdown (SIGTERM then SIGKILL)
- [ ] Implement `list_managed_processes` to return all active processes
- [ ] Track process exit codes and emit `process-status` events
- [ ] Store managed processes in `Arc<Mutex<HashMap<String, ManagedProcess>>>`
- [ ] Add unit tests for circular buffer logic

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/lib.rs`

- [ ] Register all process management commands in Tauri builder
- [ ] Initialize process manager state on app startup

### Frontend Implementation

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/LogViewer.svelte`

- [ ] Create LogViewer component with virtual scrolling (use `svelte-virtual` or custom)
- [ ] Implement search with regex toggle
- [ ] Implement auto-scroll toggle with scroll position detection
- [ ] Render ANSI color codes (use `ansi-to-html` library or similar)
- [ ] Add copy logs to clipboard button
- [ ] Add clear logs button with confirmation
- [ ] Style with glass morphism matching Dashboard
- [ ] Add loading skeleton for initial load
- [ ] Add empty state when no logs
- [ ] Handle error states gracefully

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/PortMap.svelte`

- [ ] Add "View Logs" button to each port row
- [ ] Disable button if process is not Sentinel-managed
- [ ] Open LogViewer modal on button click
- [ ] Pass process ID and name to LogViewer

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/stores/processLog.svelte.ts`

- [ ] Create ProcessLogStore with Svelte 5 runes
- [ ] Implement `init()` to subscribe to Tauri events
- [ ] Implement `loadProcesses()` to fetch managed processes
- [ ] Implement `spawnProcess()` wrapper
- [ ] Implement `stopProcess()` wrapper
- [ ] Implement `clearLogs()` wrapper
- [ ] Add cleanup method to unlisten events

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/LogLine.svelte`

- [ ] Create reusable LogLine component
- [ ] Parse and render ANSI escape codes
- [ ] Style stdout vs stderr differently
- [ ] Show timestamps in human-readable format
- [ ] Add line number display
- [ ] Add text selection support

### UI/UX Polish

- [ ] Add keyboard shortcuts (Cmd+F for search, Cmd+K to clear)
- [ ] Add tooltips for all buttons
- [ ] Add focus indicators for accessibility
- [ ] Add ARIA labels for screen readers
- [ ] Test with 10k+ lines for performance
- [ ] Add smooth scrolling animations
- [ ] Add transition effects for modal open/close

---

## Testing Requirements

### Backend Tests (Rust)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/__tests__/process_manager_test.rs`

- [ ] Test circular buffer wraps at 10k lines
- [ ] Test log batching (50ms intervals)
- [ ] Test process spawning with stdout/stderr capture
- [ ] Test graceful process shutdown
- [ ] Test process crash detection
- [ ] Test concurrent log writes from multiple processes
- [ ] Test memory usage with 10k lines per process
- [ ] Test log clearing
- [ ] Test pagination (offset + limit)
- [ ] Test edge cases (empty logs, malformed commands)

**Target:** 20+ unit tests passing

### Frontend Tests (TypeScript/Vitest)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/__tests__/LogViewer.test.ts`

- [ ] Test component renders with mock data
- [ ] Test search filtering (text + regex)
- [ ] Test auto-scroll toggle
- [ ] Test copy logs to clipboard
- [ ] Test clear logs confirmation
- [ ] Test virtual scrolling with 10k lines
- [ ] Test log event subscription
- [ ] Test error states
- [ ] Test empty state
- [ ] Test ANSI color rendering

**Target:** 15+ component tests passing

### Integration Tests

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/tests/log_viewer_integration.rs`

- [ ] Spawn test process (e.g., `echo` loop)
- [ ] Verify logs appear in frontend
- [ ] Verify real-time updates
- [ ] Verify logs persist after UI refresh
- [ ] Verify process stop clears from list

**Target:** 5+ integration tests passing

---

## Performance Requirements

- [ ] Process spawn latency: < 200ms
- [ ] Log capture latency: < 50ms (from process output to UI)
- [ ] Virtual scrolling: 60 FPS with 10k lines
- [ ] Search performance: < 100ms for 10k lines
- [ ] Memory per process: < 5MB for 10k lines
- [ ] CPU usage: < 5% while streaming logs
- [ ] UI remains responsive during high-throughput logging (1000+ lines/sec)

---

## Accessibility Requirements

WCAG 2.2 Level AA compliance:

- [ ] All interactive elements keyboard accessible (Tab navigation)
- [ ] Search input has `aria-label="Search logs"`
- [ ] Clear button has confirmation dialog
- [ ] Auto-scroll toggle has `aria-pressed` state
- [ ] Log lines have `role="log"` and `aria-live="polite"`
- [ ] Focus indicators visible on all controls
- [ ] Color contrast ratio ≥ 4.5:1 for text
- [ ] Keyboard shortcuts documented and non-conflicting
- [ ] Screen reader announces new log lines (when auto-scroll on)
- [ ] Error messages announced to screen readers

---

## UI Design Specifications

### Glass Morphism Theme

Match existing Dashboard and Port Map components:

```css
/* LogViewer Modal */
.log-viewer-modal {
  background: rgba(15, 15, 20, 0.9);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

/* Log Container */
.log-container {
  background: rgba(20, 20, 25, 0.6);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
  font-size: 13px;
  line-height: 20px;
}

/* Log Line */
.log-line {
  padding: 2px 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.03);
}

.log-line-stdout {
  color: #e0e0e0;
}

.log-line-stderr {
  color: #ff6b6b;
  background: rgba(255, 107, 107, 0.05);
}

/* Search Input */
.search-input {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
  border-radius: 6px;
  padding: 8px 12px;
}

.search-input:focus {
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}
```

### Colors

- Background: `rgba(15, 15, 20, 0.9)`
- Log container: `rgba(20, 20, 25, 0.6)`
- Stdout text: `#e0e0e0`
- Stderr text: `#ff6b6b` with `rgba(255, 107, 107, 0.05)` background
- Timestamp: `#888888`
- Search highlight: `#fbbf24` background, `#000000` text
- Auto-scroll active: `#10b981`

### Typography

- Font family: `'Monaco', 'Menlo', 'Consolas', 'Courier New', monospace`
- Font size: `13px`
- Line height: `20px`
- Timestamp: `12px`, bold

---

## Definition of Done

- [ ] Backend: `process_manager.rs` fully implemented with all commands
- [ ] Backend: Circular buffer (10k lines) working correctly
- [ ] Backend: Real-time event streaming (50ms batching) working
- [ ] Frontend: `LogViewer.svelte` component complete
- [ ] Frontend: Virtual scrolling handles 10k+ lines smoothly
- [ ] Frontend: Search with regex support working
- [ ] Frontend: Auto-scroll toggle working
- [ ] Frontend: ANSI color rendering working
- [ ] Integration: "View Logs" button in Port Map functional
- [ ] Store: `processLog.svelte.ts` implemented with event subscriptions
- [ ] Tests: 20+ backend unit tests passing
- [ ] Tests: 15+ frontend component tests passing
- [ ] Tests: 5+ integration tests passing
- [ ] Performance: All metrics met (< 50ms latency, 60 FPS scrolling)
- [ ] Accessibility: WCAG 2.2 Level AA compliant
- [ ] Documentation: API docs generated
- [ ] Documentation: User guide updated with screenshots
- [ ] Code Review: PR approved
- [ ] Demo: Screen recording showing log streaming from a Next.js app

---

## Notes

### Technical Decisions

1. **Tauri Command API vs OS-level attach**: Command API is more reliable and cross-platform. Attaching to existing processes deferred to Phase 5.

2. **Circular buffer size**: 10k lines balances memory usage (~5MB per process) with sufficient history for debugging.

3. **50ms batching**: Prevents UI lag from chatty processes (1000+ lines/sec) while maintaining perceived real-time updates.

4. **Virtual scrolling**: Essential for 10k+ lines. Consider `svelte-virtual-list` or custom implementation with Intersection Observer.

5. **ANSI rendering**: Use `ansi-to-html` NPM package or write custom parser for color codes.

### Future Enhancements (Phase 5)

- Attach to existing processes via `/proc` filesystem
- Export logs to file (JSON, plain text)
- Log level detection and filtering (INFO, WARN, ERROR)
- Syntax highlighting for common log formats (JSON, Apache, nginx)
- Multi-process log aggregation (see 4D-1)
- Persistent log storage in SQLite
- Log retention policies (auto-delete after 7 days)

### References

- Research notes: `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4-research-notes.md`
- Tauri Shell API: https://v1.tauri.app/v1/api/js/shell/
- Port Map component: `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/PortMap.svelte`

---

**Start Date:** TBD
**End Date:** TBD
**Assignee:** TBD

**This is the contract for Phase 4A-1. All requirements must be met before moving to 4B-1.**
