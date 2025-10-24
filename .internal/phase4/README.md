# Phase 4: Process Log Monitoring & Multi-Shell Dashboard

**Status:** 🟡 Planning
**Priority:** ⭐⭐⭐ Critical (Core Feature)
**Duration:** 10-14 days
**Dependencies:** Phase 3 Complete (Port Discovery, Shell Management)

---

## 📋 Overview

Phase 4 delivers the **core productivity feature** of Sentinel: solving the "lost terminal" problem for developers running multiple background services (Spring Boot, FastAPI, Next.js, etc.).

### The Problem We're Solving

Developers face these daily frustrations:
1. **Lost Logs**: Background processes lose their terminal → can't see stdout/stderr
2. **Port Conflicts**: "Port 3001 already in use" → app starts on 3002 → breaks everything
3. **No Visibility**: Can't tell which PID owns which port or service
4. **Context Switching**: Need to monitor 4+ services simultaneously
5. **Time Waste**: 15-30 min/day finding PIDs, killing processes, debugging port issues

### Our Solution

**4A: Process Log Viewer** - Attach logs to any Sentinel-managed process
**4B: Process Management Dashboard** - Start/stop/monitor dev servers with log capture
**4C: Split Terminal View** - 2x2 grid showing multiple shells/logs side-by-side
**4D: Log Aggregation** - Unified multi-tail viewer with filtering

---

## 🎯 Phase Goals

1. **Eliminate Lost Logs** - 100% log visibility for Sentinel-managed processes
2. **Port Conflict Resolution** - Instant identification of port owners with 1-click kill
3. **Multi-Service Monitoring** - View 4+ service logs simultaneously
4. **Developer Productivity** - Save 15+ min/day on process management

---

## 📂 Sprint Breakdown

### [4A-1: Process Log Viewer](./4A-1_PROCESS_LOG_VIEWER.md)
**Duration:** 3-4 days
**Goal:** Real-time log streaming for Sentinel-managed processes

- Capture stdout/stderr from spawned processes
- Display logs in modal or dedicated pane
- Search, filter, export functionality
- Integration with Port Map (view logs button)

### [4B-1: Process Management Dashboard](./4B-1_PROCESS_MANAGEMENT.md)
**Duration:** 3-4 days
**Goal:** Dedicated view for managing development servers

- Add/configure dev servers (port, command, env vars, working dir)
- Auto-detect common frameworks (Spring Boot, FastAPI, Next.js)
- Start/stop/restart with automatic log capture
- Template configurations for common stacks

### [4C-1: Split Terminal View](./4C-1_SPLIT_TERMINAL_VIEW.md)
**Duration:** 2-3 days
**Goal:** Multi-pane terminal dashboard

- Grid layouts (1x1, 2x1, 1x2, 2x2, 3x1)
- Attach pane to specific managed process OR independent shell
- Save/restore layout configurations
- Synchronized scroll (optional)

### [4D-1: Log Aggregation](./4D-1_LOG_AGGREGATION.md)
**Duration:** 2-3 days
**Goal:** Unified log viewer for multiple processes

- Multi-tail functionality (combine logs from N processes)
- Timestamp normalization
- Color coding by source
- Filter by process, log level, text search
- Export combined logs

---

## 🔬 Feasibility Research

### ✅ CONFIRMED FEASIBLE

#### 1. **Log Capture for NEW Processes** (Primary Focus)
**Method**: Tauri Command API with stdio piping
```rust
let (mut rx, child) = Command::new("node")
    .args(["server.js"])
    .spawn()?;

while let Some(event) = rx.recv().await {
    match event {
        CommandEvent::Stdout(line) => emit_to_frontend(line),
        CommandEvent::Stderr(line) => emit_to_frontend(line),
        _ => {}
    }
}
```

**Status**: ✅ Well-documented, production-ready

#### 2. **Split Terminal UI**
**Method**: Multiple xterm.js instances + CSS Grid
```svelte
<div class="terminal-grid grid-2x2">
  {#each terminals as term}
    <TerminalPane terminal={term} />
  {/each}
</div>
```

**Status**: ✅ Standard pattern, used by VS Code, Hyper

### ⚠️ LIMITED FEASIBILITY

#### 3. **Attaching to EXISTING Running Processes**
**Challenge**: Cannot easily attach to stdout/stderr of already-running processes
**Why**: Process stdout/stderr are bound to original terminal or /dev/null

**Workarounds** (Practical Approach):
1. **Option A**: Check if process is Sentinel-managed → show logs
2. **Option B**: Offer "Adopt Process" → restart under Sentinel management
3. **Option C**: Detect log files (`/proc/<PID>/fd/*`) → tail if found
4. **Option D**: Show helpful message: "Logs unavailable - restart to enable logging"

**Decision**: Focus on managing new processes (4A-4B), defer adoption feature to Phase 5

---

## 🏗️ Architecture Overview

### Backend (Rust/Tauri)

**New Modules**:
- `src-tauri/src/process_manager.rs` - Spawn/manage processes with log capture
- `src-tauri/src/log_buffer.rs` - Circular buffer for process logs (10k lines)
- `src-tauri/src/commands/process.rs` - Tauri commands for process mgmt

**Key Dependencies**:
- `tokio::process::Command` - Async process spawning
- `tokio::sync::mpsc` - Log streaming channels
- `parking_lot::RwLock` - Thread-safe log access

### Frontend (Svelte 5)

**New Components**:
- `src/views/ProcessManager.svelte` - Process management dashboard (4B)
- `src/views/SplitTerminal.svelte` - Multi-pane terminal view (4C)
- `src/lib/components/LogViewer.svelte` - Log display component (4A)
- `src/lib/components/ProcessConfigModal.svelte` - Add/edit process config (4B)
- `src/lib/components/TerminalGrid.svelte` - Grid layout manager (4C)

**State Management**:
- `src/lib/stores/process.svelte.ts` - Process state (Svelte 5 runes)
- `src/lib/stores/logs.svelte.ts` - Log buffer state

---

## 🎨 UI Standards

Following existing Dashboard patterns:

### Glass Morphism
```css
.component {
  background: var(--card);
  backdrop-filter: blur(10px);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
}
```

### Component Structure
- Use `PageHeader` component for all views
- Follow grid layout patterns from Dashboard
- Use existing `Button`, `IconButton`, `StatusBadge` components
- Maintain consistent spacing (`var(--space-*)`)

### Color Coding
- **Running**: `var(--success)` (green)
- **Stopped**: `var(--muted)` (gray)
- **Failed**: `var(--destructive)` (red)
- **Starting**: `var(--warning)` (yellow)

---

## 🧪 Testing Requirements

### Unit Tests (Rust)
- `process_manager.rs` - spawn, kill, restart
- `log_buffer.rs` - circular buffer, line limits
- **Target**: 20+ tests, 80%+ coverage

### Integration Tests (Rust)
- End-to-end process lifecycle
- Log capture and retrieval
- **Target**: 5+ tests

### Component Tests (Vitest)
- `LogViewer.svelte` - rendering, scrolling
- `TerminalGrid.svelte` - layout switching
- `ProcessConfigModal.svelte` - form validation
- **Target**: 10+ tests

### E2E Scenarios
1. Add process → start → view logs → stop
2. Port conflict detection and resolution
3. Split terminal layout save/restore
4. Multi-tail log aggregation

---

## 📈 Success Metrics

### Performance
- Process spawn time: < 200ms
- Log streaming latency: < 50ms
- Support 10+ concurrent processes
- UI remains responsive with 1000+ log lines/sec

### Developer Experience
- Reduce "port already in use" debug time by 90%
- Eliminate "lost logs" scenarios
- 1-click access to any managed process logs
- Save 15+ min/day on process management

---

## 🚧 Implementation Order

### Week 1: Core Log Infrastructure (4A)
- Days 1-2: Backend process spawning + log capture
- Days 3-4: Frontend log viewer component + Port Map integration

### Week 2: Process Management (4B)
- Days 5-6: Process config UI + auto-detection
- Days 7-8: Dashboard view + start/stop/restart

### Week 3: Split Terminal & Aggregation (4C + 4D)
- Days 9-10: Terminal grid layouts (4C)
- Days 11-12: Multi-tail log aggregation (4D)

### Week 3: Polish & Testing
- Days 13-14: Integration tests, bug fixes, documentation

---

## 📝 Notes

### Design Decisions
- **Focus on New Processes**: Don't try to attach to arbitrary PIDs (complex, platform-specific)
- **Circular Log Buffer**: Last 10k lines per process (prevents memory bloat)
- **Event-Driven Streaming**: Use Tauri events for real-time log updates (no polling)
- **SQLite for Config**: Store process configurations in SQLite (src-tauri/sentinel.db)

### Future Enhancements (Phase 5)
- Process adoption (attach to existing PIDs with reptyr)
- Log file detection and tailing
- Cloud log export (S3, GCS)
- Log alerting (regex patterns → notifications)

---

## 🔗 Related Documentation

- [Port Discovery (3A)](../phase3/3A-1_PORT_DISCOVERY.md)
- [Shell Management (3B)](../phase3/3B-1_SHELL_MANAGEMENT.md)
- [Service Detection (3C)](../phase3/3C-1_SERVICE_DETECTION.md)

---

**Created:** 2025-10-24
**Status**: Ready for Sprint Planning
**Next Step**: Review README → Create detailed sprint contracts (4A-4D)
