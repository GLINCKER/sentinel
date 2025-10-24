# 4D-1: Log Aggregation

**Sprint:** Phase 4D
**Duration:** 4 days
**Status:** READY
**Priority:** Medium
**Dependencies:** Phase 4A-1 (Process Log Viewer) + Phase 4B-1 (Process Management)

---

## Pre-Implementation Research (Required)

**BEFORE writing any code, research and document:**

### 2025 Standards & Best Practices
- [x] Multi-tail log patterns (documented in `4-research-notes.md`)
- [x] Timestamp normalization and sorting algorithms
- [ ] Log level detection patterns (regex for INFO, WARN, ERROR)
- [ ] ANSI color stripping for plain text export
- [ ] CSV/JSON export formats for logs
- [ ] Large dataset rendering (100k+ aggregated lines)
- [ ] Binary search for timestamp-based filtering
- [ ] Streaming aggregation vs batch loading

### Modern UI Patterns (2025)
- [ ] Multi-source log viewer UIs (Kibana, Datadog, Splunk)
- [ ] Color coding per source (consistent hashing)
- [ ] Log level badges and icons
- [ ] Timeline visualization for log events
- [ ] Expandable log lines (JSON pretty-print)
- [ ] Source toggling (checkbox per process)
- [ ] Time range filtering (last 1h, 24h, custom)
- [ ] Real-time "follow mode" for aggregated streams

### Performance & Scalability
- [ ] Virtual scrolling for 100k+ aggregated lines
- [ ] Efficient timestamp parsing (chrono vs manual)
- [ ] Merge-sort for streaming log aggregation
- [ ] Memory usage with 10 processes × 10k lines each
- [ ] Debounced search for large datasets
- [ ] WebWorker for log parsing and filtering

### Documentation Requirements
Add findings to `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4-research-notes.md` under new section

---

## Objective

**Build a unified log aggregator that merges logs from multiple managed processes, normalizes timestamps, and displays them in a searchable, filterable interface with color coding and export capabilities.**

### Success Criteria

- [ ] LogAggregator component merges logs from 2+ processes
- [ ] Timestamps normalized to ISO 8601 and sorted chronologically
- [ ] Color coding per source process (consistent color assignment)
- [ ] Filter by:
  - Source process (checkbox toggles)
  - Log level (INFO, WARN, ERROR, DEBUG)
  - Text search (with regex support)
  - Time range (last 1h, 6h, 24h, all time, custom)
- [ ] Auto-scroll "follow mode" for real-time updates
- [ ] Export combined logs to:
  - Plain text (.txt)
  - JSON (.json)
  - CSV (.csv)
- [ ] Virtual scrolling for 100k+ aggregated lines
- [ ] Source process badges in each log line
- [ ] Performance: < 500ms to aggregate 50k lines from 5 processes
- [ ] WCAG 2.2 Level AA compliant

---

## Contract Specification

### Backend API Contract

#### 1. Multi-Process Log Fetching

```rust
/// Get logs from multiple processes, merged and sorted by timestamp
#[tauri::command]
pub async fn get_aggregated_logs(
    process_ids: Vec<String>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<usize>,
) -> Result<AggregatedLogBatch, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AggregatedLogBatch {
    pub lines: Vec<AggregatedLogLine>,
    pub total_lines: usize,
    pub sources: Vec<LogSource>,       // Metadata about each source
    pub time_range: TimeRange,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AggregatedLogLine {
    pub source_process_id: String,
    pub source_process_name: String,
    pub timestamp: DateTime<Utc>,     // Normalized timestamp
    pub stream: LogStream,            // Stdout or Stderr
    pub level: Option<LogLevel>,      // Detected log level
    pub content: String,              // Raw log line
    pub line_number: usize,           // Line number within source process
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogSource {
    pub process_id: String,
    pub process_name: String,
    pub line_count: usize,
    pub first_timestamp: Option<DateTime<Utc>>,
    pub last_timestamp: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
```

#### 2. Log Level Detection

```rust
/// Detect log level from log line content
pub fn detect_log_level(content: &str) -> LogLevel {
    let content_upper = content.to_uppercase();

    if content_upper.contains("FATAL") || content_upper.contains("CRITICAL") {
        return LogLevel::Fatal;
    }
    if content_upper.contains("ERROR") || content_upper.contains("ERR") {
        return LogLevel::Error;
    }
    if content_upper.contains("WARN") || content_upper.contains("WARNING") {
        return LogLevel::Warn;
    }
    if content_upper.contains("INFO") {
        return LogLevel::Info;
    }
    if content_upper.contains("DEBUG") || content_upper.contains("DBG") {
        return LogLevel::Debug;
    }

    LogLevel::Unknown
}
```

#### 3. Export Functionality

```rust
/// Export aggregated logs to file
#[tauri::command]
pub async fn export_aggregated_logs(
    process_ids: Vec<String>,
    format: ExportFormat,
    file_path: String,
) -> Result<ExportResult, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ExportFormat {
    PlainText,
    Json,
    Csv,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExportResult {
    pub file_path: String,
    pub line_count: usize,
    pub file_size_bytes: u64,
}
```

---

### Frontend UI Contract

#### 1. LogAggregator Component

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/LogAggregator.svelte`

```typescript
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { processConfigStore } from '$lib/stores/processConfig.svelte';

  interface Props {
    processIds?: string[];  // If provided, filter to these processes
    onClose?: () => void;
  }

  interface AggregatedLogLine {
    sourceProcessId: string;
    sourceProcessName: string;
    timestamp: string;
    stream: 'Stdout' | 'Stderr';
    level?: 'Debug' | 'Info' | 'Warn' | 'Error' | 'Fatal' | 'Unknown';
    content: string;
    lineNumber: number;
  }

  interface LogSource {
    processId: string;
    processName: string;
    lineCount: number;
    firstTimestamp?: string;
    lastTimestamp?: string;
  }

  let { processIds, onClose }: Props = $props();

  let logs = $state<AggregatedLogLine[]>([]);
  let sources = $state<LogSource[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  // Filters
  let searchQuery = $state('');
  let searchRegex = $state(false);
  let enabledSources = $state<Set<string>>(new Set());
  let levelFilter = $state<Set<string>>(new Set(['Debug', 'Info', 'Warn', 'Error', 'Fatal']));
  let timeRange = $state<'1h' | '6h' | '24h' | 'all' | 'custom'>('all');
  let customStartTime = $state<string>('');
  let customEndTime = $state<string>('');
  let autoScroll = $state(true);

  // Virtual scrolling
  let visibleStart = $state(0);
  let visibleEnd = $state(100);
  let container: HTMLDivElement;

  onMount(async () => {
    await loadLogs();

    // Subscribe to real-time log events
    const unlisten = await listen('process-log', async (event: any) => {
      // If this process is in our aggregation, add the new lines
      if (enabledSources.has(event.payload.processId)) {
        const newLines = event.payload.lines.map((line: any) => ({
          sourceProcessId: event.payload.processId,
          sourceProcessName: getProcessName(event.payload.processId),
          ...line,
        }));

        logs = mergeSortedLogs(logs, newLines);

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

      // Determine which processes to aggregate
      const pids = processIds || Array.from(processConfigStore.configs.values()).map(c => c.id);
      enabledSources = new Set(pids);

      // Calculate time range
      const { startTime, endTime } = calculateTimeRange();

      const batch = await invoke('get_aggregated_logs', {
        processIds: pids,
        startTime,
        endTime,
        limit: 100000,
      });

      logs = batch.lines;
      sources = batch.sources;
    } catch (err) {
      error = String(err);
    } finally {
      loading = false;
    }
  }

  function calculateTimeRange() {
    const now = new Date();
    let startTime = null;
    let endTime = null;

    switch (timeRange) {
      case '1h':
        startTime = new Date(now.getTime() - 3600000);
        break;
      case '6h':
        startTime = new Date(now.getTime() - 21600000);
        break;
      case '24h':
        startTime = new Date(now.getTime() - 86400000);
        break;
      case 'custom':
        startTime = customStartTime ? new Date(customStartTime) : null;
        endTime = customEndTime ? new Date(customEndTime) : null;
        break;
    }

    return { startTime, endTime };
  }

  function mergeSortedLogs(existing: AggregatedLogLine[], newLines: AggregatedLogLine[]): AggregatedLogLine[] {
    // Merge two sorted arrays by timestamp
    const merged = [...existing];
    newLines.forEach(line => {
      const index = merged.findIndex(l => new Date(l.timestamp) > new Date(line.timestamp));
      if (index === -1) {
        merged.push(line);
      } else {
        merged.splice(index, 0, line);
      }
    });
    return merged;
  }

  function getProcessName(processId: string): string {
    return processConfigStore.configs.find(c => c.id === processId)?.name || 'Unknown';
  }

  function getSourceColor(processId: string): string {
    // Consistent color assignment via hash
    const hash = processId.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
    const colors = [
      '#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6',
      '#ec4899', '#14b8a6', '#f97316', '#06b6d4', '#84cc16'
    ];
    return colors[hash % colors.length];
  }

  function toggleSource(processId: string) {
    if (enabledSources.has(processId)) {
      enabledSources.delete(processId);
    } else {
      enabledSources.add(processId);
    }
    enabledSources = new Set(enabledSources); // Trigger reactivity
  }

  function toggleLevel(level: string) {
    if (levelFilter.has(level)) {
      levelFilter.delete(level);
    } else {
      levelFilter.add(level);
    }
    levelFilter = new Set(levelFilter); // Trigger reactivity
  }

  async function exportLogs(format: 'PlainText' | 'Json' | 'Csv') {
    try {
      const pids = Array.from(enabledSources);
      const result = await invoke('export_aggregated_logs', {
        processIds: pids,
        format,
        filePath: '', // Will open save dialog
      });
      alert(`Exported ${result.lineCount} lines to ${result.filePath}`);
    } catch (err) {
      alert(`Export failed: ${err}`);
    }
  }

  // Derived: filtered logs
  let filteredLogs = $derived.by(() => {
    let filtered = logs;

    // Filter by enabled sources
    filtered = filtered.filter(l => enabledSources.has(l.sourceProcessId));

    // Filter by log level
    filtered = filtered.filter(l => {
      if (!l.level) return true; // Always show unknown levels
      return levelFilter.has(l.level);
    });

    // Filter by search query
    if (searchQuery.trim()) {
      try {
        if (searchRegex) {
          const regex = new RegExp(searchQuery, 'i');
          filtered = filtered.filter(l => regex.test(l.content));
        } else {
          const query = searchQuery.toLowerCase();
          filtered = filtered.filter(l => l.content.toLowerCase().includes(query));
        }
      } catch {
        // Invalid regex, skip filtering
      }
    }

    return filtered;
  });
</script>

<!-- UI Below -->
```

**UI Layout:**

```
┌────────────────────────────────────────────────────────────────┐
│ 🔗 Log Aggregator                                      [X]     │
├────────────────────────────────────────────────────────────────┤
│ Sources: [☑ next-dev] [☑ fastapi] [☐ postgres] [☑ vite-dev]   │
│                                                                │
│ Levels: [☑ DEBUG] [☑ INFO] [☑ WARN] [☑ ERROR] [☑ FATAL]      │
│                                                                │
│ Time: [Last 24h ▼] 🔍 [Search...] [☑ Regex] [↓ Follow]       │
│                                                                │
│ [📄 Export TXT] [📊 Export JSON] [📋 Export CSV]              │
├────────────────────────────────────────────────────────────────┤
│ 1  [12:34:56] [next-dev]   INFO  ▸ Ready on :3000            │
│ 2  [12:34:57] [fastapi]    INFO  Application startup          │
│ 3  [12:34:58] [vite-dev]   INFO  ⚡ Vite dev server running   │
│ 4  [12:34:59] [next-dev]   WARN  Missing .env file            │
│ 5  [12:35:00] [fastapi]    DEBUG SQLAlchemy connection pool   │
│ 6  [12:35:01] [vite-dev]   ERROR Failed to load config        │
│ 7  [12:35:02] [next-dev]   INFO  Compiled successfully        │
│ 8  [12:35:03] [fastapi]    INFO  Uvicorn running on :8000     │
│ 9  [12:35:04] [vite-dev]   WARN  HMR update failed            │
│ 10 [12:35:05] [next-dev]   INFO  GET / 200 in 45ms            │
│                                                                │
│ [Virtual scrolled content - 99,990 more lines...]             │
├────────────────────────────────────────────────────────────────┤
│ 100,000 lines from 3 sources • Last update: 2s ago            │
└────────────────────────────────────────────────────────────────┘
```

**Component Props:**

```typescript
interface LogAggregatorProps {
  processIds?: string[];  // Optional filter to specific processes
  onClose?: () => void;
}

interface AggregatedLogLine {
  sourceProcessId: string;
  sourceProcessName: string;
  timestamp: string;
  stream: 'Stdout' | 'Stderr';
  level?: 'Debug' | 'Info' | 'Warn' | 'Error' | 'Fatal' | 'Unknown';
  content: string;
  lineNumber: number;
}

interface LogSource {
  processId: string;
  processName: string;
  lineCount: number;
  firstTimestamp?: string;
  lastTimestamp?: string;
}

interface AggregatedLogBatch {
  lines: AggregatedLogLine[];
  totalLines: number;
  sources: LogSource[];
  timeRange: TimeRange;
}
```

#### 2. SourceBadge Component

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/SourceBadge.svelte`

```typescript
<script lang="ts">
  interface Props {
    processName: string;
    color: string;
  }

  let { processName, color }: Props = $props();
</script>

<span class="source-badge" style="--badge-color: {color}">
  {processName}
</span>

<style>
  .source-badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
    background: var(--badge-color);
    color: white;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
  }
</style>
```

#### 3. LevelBadge Component

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/LevelBadge.svelte`

```typescript
<script lang="ts">
  interface Props {
    level: 'Debug' | 'Info' | 'Warn' | 'Error' | 'Fatal' | 'Unknown';
  }

  let { level }: Props = $props();

  const levelColors = {
    Debug: '#6b7280',
    Info: '#3b82f6',
    Warn: '#f59e0b',
    Error: '#ef4444',
    Fatal: '#991b1b',
    Unknown: '#4b5563',
  };

  let color = $derived(levelColors[level]);
</script>

<span class="level-badge" style="--level-color: {color}">
  {level.toUpperCase()}
</span>

<style>
  .level-badge {
    display: inline-block;
    padding: 2px 6px;
    border-radius: 3px;
    font-size: 10px;
    font-weight: 700;
    background: var(--level-color);
    color: white;
    min-width: 50px;
    text-align: center;
  }
</style>
```

---

## Implementation Tasks

### Backend Implementation

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/log_aggregator.rs`

- [ ] Implement `get_aggregated_logs` command
- [ ] Fetch logs from multiple processes (via process_manager.rs)
- [ ] Normalize timestamps to ISO 8601 (parse various formats)
- [ ] Merge logs from all sources and sort by timestamp
- [ ] Detect log level using regex patterns
- [ ] Apply time range filtering (start_time, end_time)
- [ ] Apply limit (return max N lines)
- [ ] Calculate LogSource metadata (line count, time range per source)
- [ ] Handle processes with no logs gracefully
- [ ] Optimize for performance (use BinaryHeap for merge-sort)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/log_level_detector.rs`

- [ ] Implement `detect_log_level` function
- [ ] Regex patterns for DEBUG, INFO, WARN, ERROR, FATAL
- [ ] Support multiple formats (JSON logs, structured logs, plain text)
- [ ] Case-insensitive matching
- [ ] Return `Unknown` for unmatched lines

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/log_exporter.rs`

- [ ] Implement `export_aggregated_logs` command
- [ ] Plain text format: `[timestamp] [source] [level] content`
- [ ] JSON format: Array of log objects
- [ ] CSV format: `timestamp,source,level,stream,content`
- [ ] Open native save dialog (Tauri dialog API)
- [ ] Write to file asynchronously (tokio::fs)
- [ ] Return ExportResult with file size and line count

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/lib.rs`

- [ ] Register `get_aggregated_logs` command
- [ ] Register `export_aggregated_logs` command

### Frontend Implementation

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/LogAggregator.svelte`

- [ ] Create LogAggregator component with virtual scrolling
- [ ] Load initial aggregated logs on mount
- [ ] Subscribe to real-time log events for enabled sources
- [ ] Merge new logs into existing array (maintain sort order)
- [ ] Implement source toggles (checkboxes)
- [ ] Implement level toggles (checkboxes)
- [ ] Implement time range selector (dropdown)
- [ ] Implement custom time range picker (date inputs)
- [ ] Implement search with regex toggle
- [ ] Implement auto-scroll toggle
- [ ] Render SourceBadge and LevelBadge per line
- [ ] Implement export buttons (TXT, JSON, CSV)
- [ ] Style with glass morphism
- [ ] Add loading skeleton
- [ ] Add empty state

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/SourceBadge.svelte`

- [ ] Create badge component with dynamic color
- [ ] Truncate long process names

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/LevelBadge.svelte`

- [ ] Create badge component with level-specific colors
- [ ] Use consistent colors (DEBUG=gray, INFO=blue, WARN=amber, ERROR=red, FATAL=dark red)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/routes/logs/+page.svelte`

- [ ] Create dedicated Logs page route
- [ ] Use PageHeader component
- [ ] Render LogAggregator component
- [ ] Add "All Logs" subtitle in header

### UI/UX Polish

- [ ] Add smooth transitions for filter changes
- [ ] Add tooltips for all buttons and badges
- [ ] Add keyboard shortcuts (Cmd+F for search, Cmd+E for export)
- [ ] Add copy log line on click
- [ ] Add "Clear filters" button
- [ ] Add stats footer (X lines from Y sources)
- [ ] Add loading indicator during export

---

## Testing Requirements

### Backend Tests (Rust)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/__tests__/log_aggregator_test.rs`

- [ ] Test merge logs from 2 processes with different timestamps
- [ ] Test timestamp normalization (various formats)
- [ ] Test log sorting (chronological order)
- [ ] Test time range filtering (start_time, end_time)
- [ ] Test limit parameter (max N lines)
- [ ] Test empty process (no logs)
- [ ] Test performance: 50k lines from 5 processes in < 500ms

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/__tests__/log_level_detector_test.rs`

- [ ] Test detect DEBUG level
- [ ] Test detect INFO level
- [ ] Test detect WARN level
- [ ] Test detect ERROR level
- [ ] Test detect FATAL level
- [ ] Test case-insensitive matching
- [ ] Test unknown level fallback

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/__tests__/log_exporter_test.rs`

- [ ] Test export to plain text format
- [ ] Test export to JSON format
- [ ] Test export to CSV format
- [ ] Test file creation and content validation
- [ ] Test large export (100k lines)

**Target:** 20+ unit tests passing

### Frontend Tests (TypeScript/Vitest)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/__tests__/LogAggregator.test.ts`

- [ ] Test renders with mock aggregated logs
- [ ] Test source toggle filters logs
- [ ] Test level toggle filters logs
- [ ] Test search filtering
- [ ] Test time range filtering
- [ ] Test auto-scroll toggle
- [ ] Test export button click

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/__tests__/SourceBadge.test.ts`

- [ ] Test renders with correct color
- [ ] Test truncates long names

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/__tests__/LevelBadge.test.ts`

- [ ] Test renders correct color per level
- [ ] Test all level types

**Target:** 15+ component tests passing

### Integration Tests

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/tests/log_aggregation_integration.rs`

- [ ] Start 3 processes → verify logs aggregated correctly
- [ ] Filter by source → verify correct subset
- [ ] Filter by level → verify correct subset
- [ ] Export to file → verify file content
- [ ] Real-time updates → verify new logs appear

**Target:** 5+ integration tests passing

---

## Performance Requirements

- [ ] Aggregate 50k lines from 5 processes: < 500ms
- [ ] Timestamp normalization: < 1ms per line
- [ ] Merge-sort 100k lines: < 1s
- [ ] Virtual scrolling: 60 FPS with 100k lines
- [ ] Search 100k lines: < 200ms
- [ ] Export 100k lines to file: < 3s
- [ ] UI remains responsive during aggregation
- [ ] Memory usage: < 50MB for 100k lines

---

## Accessibility Requirements

WCAG 2.2 Level AA compliance:

- [ ] All filter controls keyboard accessible
- [ ] Source checkboxes have `aria-label` (e.g., "Include next-dev logs")
- [ ] Level checkboxes have `aria-label` (e.g., "Show INFO level")
- [ ] Search input has `aria-label="Search logs"`
- [ ] Export buttons have `aria-label`
- [ ] Log lines have `role="log"` and `aria-live="polite"`
- [ ] Color not sole indicator (use text + icons)
- [ ] Focus indicators visible on all controls
- [ ] Keyboard shortcuts documented
- [ ] Screen reader announces filter changes

---

## UI Design Specifications

### Glass Morphism Theme

```css
/* Log Aggregator Container */
.log-aggregator {
  background: rgba(15, 15, 20, 0.9);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  height: 100%;
}

/* Filter Bar */
.filter-bar {
  padding: var(--space-md);
  background: rgba(20, 20, 25, 0.6);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-sm);
}

/* Source Toggle */
.source-toggle {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.source-toggle:hover {
  background: rgba(255, 255, 255, 0.1);
}

.source-toggle input[type="checkbox"] {
  accent-color: #3b82f6;
}

/* Log Line */
.log-line {
  display: flex;
  gap: 12px;
  padding: 4px 12px;
  font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
  font-size: 13px;
  line-height: 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.03);
}

.log-line:hover {
  background: rgba(255, 255, 255, 0.03);
}

.log-line-timestamp {
  color: #888888;
  font-size: 11px;
  min-width: 80px;
}

.log-line-source {
  min-width: 100px;
}

.log-line-level {
  min-width: 60px;
}

.log-line-content {
  flex: 1;
  color: #e0e0e0;
  word-break: break-word;
}
```

### Colors

- Background: `rgba(15, 15, 20, 0.9)`
- Filter bar: `rgba(20, 20, 25, 0.6)`
- Log line hover: `rgba(255, 255, 255, 0.03)`
- Timestamp: `#888888`
- Level DEBUG: `#6b7280`
- Level INFO: `#3b82f6`
- Level WARN: `#f59e0b`
- Level ERROR: `#ef4444`
- Level FATAL: `#991b1b`

### Source Color Palette

Consistent colors via hash:
- `#3b82f6` (blue)
- `#10b981` (green)
- `#f59e0b` (amber)
- `#ef4444` (red)
- `#8b5cf6` (purple)
- `#ec4899` (pink)
- `#14b8a6` (teal)
- `#f97316` (orange)
- `#06b6d4` (cyan)
- `#84cc16` (lime)

---

## Definition of Done

- [ ] Backend: `get_aggregated_logs` command fully implemented
- [ ] Backend: Timestamp normalization working for common formats
- [ ] Backend: Log level detection working
- [ ] Backend: Export to TXT/JSON/CSV working
- [ ] Frontend: LogAggregator component complete
- [ ] Frontend: Virtual scrolling handles 100k+ lines
- [ ] Frontend: Source and level filters working
- [ ] Frontend: Time range filtering working
- [ ] Frontend: Search with regex support working
- [ ] Frontend: Auto-scroll toggle working
- [ ] Frontend: Export buttons functional
- [ ] Components: SourceBadge and LevelBadge components complete
- [ ] Integration: Real-time log updates from multiple processes
- [ ] Tests: 20+ backend unit tests passing
- [ ] Tests: 15+ frontend component tests passing
- [ ] Tests: 5+ integration tests passing
- [ ] Performance: All metrics met (< 500ms aggregation)
- [ ] Accessibility: WCAG 2.2 Level AA compliant
- [ ] Documentation: User guide with screenshots
- [ ] Documentation: API docs generated
- [ ] Code Review: PR approved
- [ ] Demo: Video showing multi-process log aggregation

---

## Notes

### Technical Decisions

1. **Timestamp normalization**: Use `chrono` crate to parse various formats (ISO 8601, RFC3339, Unix timestamps, custom formats). Default to process-generated timestamps if parsing fails.

2. **Merge algorithm**: Use BinaryHeap (min-heap) for efficient merge of pre-sorted logs from N processes. O(N log K) where K is number of processes.

3. **Log level detection**: Regex-based detection for common patterns. Future: ML-based detection for custom log formats.

4. **Virtual scrolling**: Essential for 100k+ lines. Use `svelte-virtual-list` or custom implementation.

5. **Export format**: Plain text is human-readable, JSON is machine-readable, CSV is Excel-compatible.

### Future Enhancements (Phase 5)

- Timeline visualization (chart showing log volume over time)
- Expandable JSON logs (pretty-print in modal)
- Syntax highlighting for log content
- Log correlation (link related logs across processes)
- Advanced filters (custom query language like Lucene)
- Saved filter presets
- Log streaming to external systems (e.g., Elasticsearch)
- Log retention policies (auto-delete old logs)
- Alerting on specific log patterns (e.g., ERROR count > 10)

### References

- Research notes: `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4-research-notes.md`
- Process Log Viewer (4A-1): `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4A-1_PROCESS_LOG_VIEWER.md`
- Process Management (4B-1): `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4B-1_PROCESS_MANAGEMENT.md`

---

**Start Date:** TBD
**End Date:** TBD
**Assignee:** TBD

**This is the contract for Phase 4D-1. All requirements must be met before Phase 4 is considered complete.**
