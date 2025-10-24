# 4C-1: Split Terminal View

**Sprint:** Phase 4C
**Duration:** 5 days
**Status:** READY
**Priority:** High
**Dependencies:** Phase 4A-1 (Process Log Viewer) + Phase 4B-1 (Process Management)

---

## Pre-Implementation Research (Required)

**BEFORE writing any code, research and document:**

### 2025 Standards & Best Practices
- [x] xterm.js multiple instances (documented in `4-research-notes.md`)
- [x] CSS Grid layouts for terminal splitting
- [ ] xterm.js fit addon for dynamic resizing
- [ ] xterm.js serialize addon for session persistence
- [ ] ResizeObserver API for responsive terminal sizing
- [ ] Drag handles for resizable panes (react-resizable-panels alternatives)
- [ ] Layout persistence (localStorage vs IndexedDB)
- [ ] WebGL renderer performance benchmarks

### Modern UI Patterns (2025)
- [ ] Terminal splitting patterns (VS Code, iTerm2, tmux)
- [ ] Drag-and-drop terminal reordering
- [ ] Pane focus indicators and keyboard navigation
- [ ] Split/merge animations
- [ ] Maximized pane (temporary fullscreen)
- [ ] Pane title bars with close buttons
- [ ] Layout presets (1x1, 2x1, 1x2, 2x2, 3x1)
- [ ] Saved workspace layouts

### Integration Architecture
- [ ] Attach terminal to managed process (from 4B-1)
- [ ] Attach terminal to shell (from existing Shell system)
- [ ] Switch pane attachment dynamically
- [ ] Detach pane without killing process
- [ ] Share terminal session across panes (tmux-like)

### Documentation Requirements
Add findings to `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4-research-notes.md` under new section

---

## Objective

**Build a multi-pane terminal dashboard with flexible layouts, allowing users to attach panes to managed processes or independent shells, with persistent layout configurations.**

### Success Criteria

- [ ] TerminalGrid component with 5 preset layouts (1x1, 2x1, 1x2, 2x2, 3x1)
- [ ] Each pane can attach to:
  - Managed process (from 4B-1) - streams stdout/stderr
  - Independent shell (new shell instance)
- [ ] Switch pane attachment without killing process
- [ ] Drag handles to resize panes dynamically
- [ ] Focus indicators for active pane
- [ ] Keyboard navigation (Cmd+1-9 to focus pane, Cmd+W to close pane)
- [ ] Pane title bar shows attachment (process name or "Shell")
- [ ] Close pane button (confirm if attached to running process)
- [ ] Save/restore layout configurations (localStorage)
- [ ] Layout presets accessible via dropdown
- [ ] Integration with existing Shell system
- [ ] WCAG 2.2 Level AA compliant

---

## Contract Specification

### Backend API Contract

#### 1. Shell Management (Extend Existing)

```rust
/// Create a new shell instance (extend existing shell.rs)
#[tauri::command]
pub async fn create_terminal_shell(
    initial_dir: Option<String>,
    env_vars: Option<HashMap<String, String>>,
) -> Result<ShellSession, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShellSession {
    pub id: String,              // Unique shell ID
    pub pid: u32,                // Shell process PID
    pub initial_dir: String,     // Working directory
    pub created_at: DateTime<Utc>,
}
```

#### 2. Terminal Attachment

```rust
/// Attach a terminal pane to a process (read-only stdout/stderr)
#[tauri::command]
pub async fn attach_terminal_to_process(
    pane_id: String,
    process_id: String,
) -> Result<(), String>

/// Detach terminal pane from process
#[tauri::command]
pub async fn detach_terminal_pane(pane_id: String) -> Result<(), String>

/// Get current attachment for a pane
#[tauri::command]
pub async fn get_pane_attachment(pane_id: String) -> Result<PaneAttachment, String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PaneAttachment {
    Shell { session_id: String, pid: u32 },
    Process { process_id: String, pid: u32, read_only: bool },
    None,
}
```

#### 3. Layout Persistence

```rust
/// Save terminal layout configuration
#[tauri::command]
pub async fn save_terminal_layout(
    name: String,
    layout: TerminalLayout,
) -> Result<(), String>

/// Load saved layout configurations
#[tauri::command]
pub async fn load_terminal_layouts() -> Result<Vec<SavedLayout>, String>

/// Delete a saved layout
#[tauri::command]
pub async fn delete_terminal_layout(name: String) -> Result<(), String>

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TerminalLayout {
    pub grid_type: GridType,
    pub panes: Vec<PaneConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GridType {
    Single,        // 1x1
    TwoColumn,     // 2x1
    TwoRow,        // 1x2
    TwoByTwo,      // 2x2
    ThreeColumn,   // 3x1
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaneConfig {
    pub position: usize,           // 0-based index in grid
    pub attachment: PaneAttachment,
    pub size_ratio: f32,           // For custom sizes (future)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SavedLayout {
    pub name: String,
    pub layout: TerminalLayout,
    pub created_at: DateTime<Utc>,
}
```

---

### Frontend UI Contract

#### 1. TerminalGrid Component

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/TerminalGrid.svelte`

```typescript
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Terminal } from 'xterm';
  import { FitAddon } from 'xterm-addon-fit';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  interface Props {
    gridType: GridType;
    onLayoutChange?: (layout: TerminalLayout) => void;
  }

  type GridType = 'Single' | 'TwoColumn' | 'TwoRow' | 'TwoByTwo' | 'ThreeColumn';

  interface TerminalPane {
    id: string;
    terminal: Terminal;
    fitAddon: FitAddon;
    element: HTMLDivElement;
    attachment: PaneAttachment;
    focused: boolean;
  }

  interface PaneAttachment {
    type: 'Shell' | 'Process' | 'None';
    sessionId?: string;
    processId?: string;
    pid?: number;
    readOnly?: boolean;
  }

  let { gridType = 'Single', onLayoutChange }: Props = $props();

  let panes = $state<TerminalPane[]>([]);
  let activePaneId = $state<string | null>(null);

  const gridLayouts = {
    Single: 'grid-1x1',
    TwoColumn: 'grid-2x1',
    TwoRow: 'grid-1x2',
    TwoByTwo: 'grid-2x2',
    ThreeColumn: 'grid-3x1',
  };

  let gridClass = $derived(gridLayouts[gridType]);
  let paneCount = $derived(getPaneCount(gridType));

  function getPaneCount(type: GridType): number {
    switch (type) {
      case 'Single': return 1;
      case 'TwoColumn': return 2;
      case 'TwoRow': return 2;
      case 'TwoByTwo': return 4;
      case 'ThreeColumn': return 3;
    }
  }

  onMount(() => {
    initializePanes();
    window.addEventListener('resize', handleResize);
    document.addEventListener('keydown', handleKeyboardShortcuts);

    return () => {
      window.removeEventListener('resize', handleResize);
      document.removeEventListener('keydown', handleKeyboardShortcuts);
      panes.forEach(pane => pane.terminal.dispose());
    };
  });

  async function initializePanes() {
    panes = [];
    for (let i = 0; i < paneCount; i++) {
      const pane = await createPane(i);
      panes.push(pane);
    }
    if (panes.length > 0) {
      focusPane(panes[0].id);
    }
  }

  async function createPane(index: number): Promise<TerminalPane> {
    const id = `pane-${Date.now()}-${index}`;
    const terminal = new Terminal({
      theme: {
        background: 'rgba(20, 20, 25, 0.6)',
        foreground: '#e0e0e0',
        cursor: '#3b82f6',
      },
      fontFamily: 'Monaco, Menlo, Consolas, monospace',
      fontSize: 13,
      cursorBlink: true,
    });

    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);

    return {
      id,
      terminal,
      fitAddon,
      element: null as any,
      attachment: { type: 'None' },
      focused: false,
    };
  }

  async function attachToShell(paneId: string) {
    const pane = panes.find(p => p.id === paneId);
    if (!pane) return;

    const session = await invoke<ShellSession>('create_terminal_shell');
    pane.attachment = {
      type: 'Shell',
      sessionId: session.id,
      pid: session.pid,
    };

    // Connect terminal to shell (use existing shell integration)
    // This would use the existing shell PTY system
  }

  async function attachToProcess(paneId: string, processId: string) {
    const pane = panes.find(p => p.id === paneId);
    if (!pane) return;

    await invoke('attach_terminal_to_process', { paneId, processId });

    pane.attachment = {
      type: 'Process',
      processId,
      readOnly: true,
    };

    // Subscribe to process logs and write to terminal
    const unlisten = await listen('process-log', (event: any) => {
      if (event.payload.processId === processId) {
        event.payload.lines.forEach((line: any) => {
          pane.terminal.writeln(line.content);
        });
      }
    });
  }

  async function detachPane(paneId: string) {
    const pane = panes.find(p => p.id === paneId);
    if (!pane) return;

    if (pane.attachment.type !== 'None') {
      await invoke('detach_terminal_pane', { paneId });
      pane.terminal.clear();
      pane.attachment = { type: 'None' };
    }
  }

  function focusPane(paneId: string) {
    panes.forEach(p => p.focused = (p.id === paneId));
    activePaneId = paneId;
    const pane = panes.find(p => p.id === paneId);
    if (pane) pane.terminal.focus();
  }

  function handleResize() {
    panes.forEach(pane => {
      if (pane.fitAddon) {
        setTimeout(() => pane.fitAddon.fit(), 100);
      }
    });
  }

  function handleKeyboardShortcuts(e: KeyboardEvent) {
    // Cmd+1 to Cmd+9: Focus pane
    if ((e.metaKey || e.ctrlKey) && e.key >= '1' && e.key <= '9') {
      const index = parseInt(e.key) - 1;
      if (index < panes.length) {
        e.preventDefault();
        focusPane(panes[index].id);
      }
    }

    // Cmd+W: Close active pane (if not the only one)
    if ((e.metaKey || e.ctrlKey) && e.key === 'w') {
      if (panes.length > 1 && activePaneId) {
        e.preventDefault();
        closePane(activePaneId);
      }
    }
  }

  async function closePane(paneId: string) {
    const pane = panes.find(p => p.id === paneId);
    if (!pane) return;

    // Confirm if attached to running process
    if (pane.attachment.type === 'Process') {
      if (!confirm('Detach from running process?')) return;
    }

    await detachPane(paneId);
    // For multi-pane layouts, just detach; don't remove pane
  }
</script>

<!-- UI Below -->
```

**UI Layout (2x2 Grid Example):**

```
┌────────────────────────────────────────────────────────────────┐
│ Layout: [2x2 ▼]  [+ New Pane] [Save Layout]                    │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│ ┌──────────────────────────┬──────────────────────────┐        │
│ │ Shell (zsh) - PID 12345 │  next-dev - PID 67890  │        │
│ │ [↻] [×]                  │  [↻] [×]                │        │
│ ├──────────────────────────┼──────────────────────────┤        │
│ │ $ ls -la                 │  ▲ Next.js 14.0.0       │        │
│ │ total 64                 │  - Local: :3000         │        │
│ │ drwxr-xr-x  10 user      │  ✓ Ready in 2.5s        │        │
│ │ -rw-r--r--   1 user      │                         │        │
│ │ $ █                      │                         │        │
│ │                          │                         │        │
│ └──────────────────────────┴──────────────────────────┘        │
│                                                                │
│ ┌──────────────────────────┬──────────────────────────┐        │
│ │ postgres - PID 89012    │  Shell (bash)           │        │
│ │ [↻] [×]                  │  [↻] [×]                │        │
│ ├──────────────────────────┼──────────────────────────┤        │
│ │ LOG: database started    │  $ npm run build        │        │
│ │ LOG: listening on :5432  │  > Building...          │        │
│ │                          │  ✓ Compiled             │        │
│ │                          │  $ █                    │        │
│ │                          │                         │        │
│ └──────────────────────────┴──────────────────────────┘        │
└────────────────────────────────────────────────────────────────┘

Shortcuts: Cmd+1-4 (Focus pane) | Cmd+W (Close) | Cmd+Enter (New)
```

**Component Props:**

```typescript
interface TerminalGridProps {
  gridType: 'Single' | 'TwoColumn' | 'TwoRow' | 'TwoByTwo' | 'ThreeColumn';
  onLayoutChange?: (layout: TerminalLayout) => void;
}

interface TerminalPane {
  id: string;
  terminal: Terminal;
  fitAddon: FitAddon;
  element: HTMLDivElement;
  attachment: PaneAttachment;
  focused: boolean;
}

interface PaneAttachment {
  type: 'Shell' | 'Process' | 'None';
  sessionId?: string;
  processId?: string;
  pid?: number;
  readOnly?: boolean;
}
```

#### 2. PaneHeader Component

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/PaneHeader.svelte`

```typescript
<script lang="ts">
  interface Props {
    paneId: string;
    attachment: PaneAttachment;
    focused: boolean;
    onAttach: () => void;
    onDetach: () => void;
    onClose: () => void;
  }

  let { paneId, attachment, focused, onAttach, onDetach, onClose }: Props = $props();

  let title = $derived(getTitle(attachment));
  let subtitle = $derived(getSubtitle(attachment));

  function getTitle(att: PaneAttachment): string {
    if (att.type === 'Shell') return 'Shell';
    if (att.type === 'Process') return 'Process'; // Lookup name from store
    return 'Not Attached';
  }

  function getSubtitle(att: PaneAttachment): string {
    if (att.pid) return `PID ${att.pid}`;
    return '';
  }
</script>

<div class="pane-header" class:focused>
  <div class="pane-title">
    <span class="title">{title}</span>
    {#if subtitle}
      <span class="subtitle">{subtitle}</span>
    {/if}
  </div>
  <div class="pane-actions">
    {#if attachment.type !== 'None'}
      <button onclick={onDetach} aria-label="Detach">↻</button>
    {:else}
      <button onclick={onAttach} aria-label="Attach">+</button>
    {/if}
    <button onclick={onClose} aria-label="Close">×</button>
  </div>
</div>
```

#### 3. LayoutSelector Component

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/LayoutSelector.svelte`

Dropdown to select preset layouts (1x1, 2x1, 1x2, 2x2, 3x1) + saved custom layouts.

```typescript
<script lang="ts">
  import Dropdown from './Dropdown.svelte';

  interface Props {
    currentLayout: GridType;
    savedLayouts: SavedLayout[];
    onSelectLayout: (type: GridType) => void;
    onLoadSaved: (name: string) => void;
  }

  let { currentLayout, savedLayouts, onSelectLayout, onLoadSaved }: Props = $props();

  const presetLayouts = [
    { value: 'Single', label: '1×1 Single' },
    { value: 'TwoColumn', label: '2×1 Two Column' },
    { value: 'TwoRow', label: '1×2 Two Row' },
    { value: 'TwoByTwo', label: '2×2 Four Panes' },
    { value: 'ThreeColumn', label: '3×1 Three Column' },
  ];
</script>

<Dropdown>
  <optgroup label="Presets">
    {#each presetLayouts as layout}
      <option value={layout.value} selected={currentLayout === layout.value}>
        {layout.label}
      </option>
    {/each}
  </optgroup>
  {#if savedLayouts.length > 0}
    <optgroup label="Saved Layouts">
      {#each savedLayouts as saved}
        <option value={saved.name}>{saved.name}</option>
      {/each}
    </optgroup>
  {/if}
</Dropdown>
```

#### 4. Terminal Layout Store

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/stores/terminalLayout.svelte.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';

type GridType = 'Single' | 'TwoColumn' | 'TwoRow' | 'TwoByTwo' | 'ThreeColumn';

interface TerminalLayout {
  gridType: GridType;
  panes: PaneConfig[];
}

interface PaneConfig {
  position: number;
  attachment: PaneAttachment;
  sizeRatio: number;
}

interface SavedLayout {
  name: string;
  layout: TerminalLayout;
  createdAt: string;
}

class TerminalLayoutStore {
  currentLayout = $state<TerminalLayout>({
    gridType: 'Single',
    panes: [],
  });

  savedLayouts = $state<SavedLayout[]>([]);

  async loadSavedLayouts() {
    this.savedLayouts = await invoke<SavedLayout[]>('load_terminal_layouts');
  }

  async saveLayout(name: string) {
    await invoke('save_terminal_layout', {
      name,
      layout: this.currentLayout,
    });
    await this.loadSavedLayouts();
  }

  async deleteLayout(name: string) {
    await invoke('delete_terminal_layout', { name });
    await this.loadSavedLayouts();
  }

  setGridType(type: GridType) {
    this.currentLayout.gridType = type;
  }

  updatePane(position: number, config: PaneConfig) {
    const index = this.currentLayout.panes.findIndex(p => p.position === position);
    if (index >= 0) {
      this.currentLayout.panes[index] = config;
    } else {
      this.currentLayout.panes.push(config);
    }
  }
}

export const terminalLayoutStore = new TerminalLayoutStore();
```

---

## Implementation Tasks

### Backend Implementation

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/terminal_manager.rs`

- [ ] Extend existing shell system to support multiple sessions
- [ ] Implement `create_terminal_shell` command (create new PTY)
- [ ] Implement `attach_terminal_to_process` command
- [ ] Stream process logs to attached terminal pane
- [ ] Implement `detach_terminal_pane` command
- [ ] Implement `get_pane_attachment` command
- [ ] Track pane-to-process/shell mappings in state

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/layout_storage.rs`

- [ ] Create SQLite table for saved layouts (or use JSON file in app data dir)
- [ ] Implement `save_terminal_layout` command
- [ ] Implement `load_terminal_layouts` command
- [ ] Implement `delete_terminal_layout` command
- [ ] Serialize/deserialize `TerminalLayout` to JSON

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/lib.rs`

- [ ] Register all terminal management commands
- [ ] Initialize layout storage on app startup

### Frontend Implementation

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/TerminalGrid.svelte`

- [ ] Create TerminalGrid component with CSS Grid
- [ ] Initialize xterm.js instances for each pane
- [ ] Implement FitAddon for dynamic resizing
- [ ] Handle window resize events
- [ ] Implement keyboard shortcuts (Cmd+1-9, Cmd+W)
- [ ] Implement focus management (click to focus, visual indicator)
- [ ] Attach pane to shell (create new PTY session)
- [ ] Attach pane to process (read-only logs)
- [ ] Detach pane (clear terminal, reset attachment)
- [ ] Close pane confirmation (if attached to process)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/PaneHeader.svelte`

- [ ] Create header component with title and actions
- [ ] Display attachment type (Shell, Process name, or "Not Attached")
- [ ] Display PID if attached
- [ ] Attach button (opens modal to select process/shell)
- [ ] Detach button (confirmation if process running)
- [ ] Close button
- [ ] Focus indicator styling

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/LayoutSelector.svelte`

- [ ] Create dropdown with 5 preset layouts
- [ ] Show saved layouts in separate section
- [ ] Handle layout change
- [ ] Handle load saved layout

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/AttachModal.svelte`

- [ ] Modal to select attachment source
- [ ] Tab 1: New Shell (select shell type: zsh, bash, fish)
- [ ] Tab 2: Managed Process (list from 4B-1)
- [ ] Tab 3: Independent Shell (existing shell sessions)
- [ ] Attach button triggers attachment

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/stores/terminalLayout.svelte.ts`

- [ ] Implement TerminalLayoutStore with Svelte 5 runes
- [ ] Load saved layouts on init
- [ ] Save/delete layout methods
- [ ] Current layout state management

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/routes/terminals/+page.svelte`

- [ ] Create dedicated Terminals page route
- [ ] Use PageHeader component
- [ ] Render TerminalGrid component
- [ ] Add LayoutSelector in header
- [ ] Add "Save Layout" button in header
- [ ] Add keyboard shortcuts help tooltip

### CSS Grid Layouts

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/TerminalGrid.svelte` (styles)

- [ ] Define `.grid-1x1` (single pane, full width/height)
- [ ] Define `.grid-2x1` (two columns, 1fr 1fr)
- [ ] Define `.grid-1x2` (two rows, 1fr 1fr)
- [ ] Define `.grid-2x2` (2x2 grid, 1fr 1fr / 1fr 1fr)
- [ ] Define `.grid-3x1` (three columns, 1fr 1fr 1fr)
- [ ] Add gap between panes (8px)
- [ ] Add resize handles (optional, future enhancement)

### UI/UX Polish

- [ ] Add smooth transitions when changing layouts
- [ ] Add focus ring on active pane
- [ ] Add pane number badges (1, 2, 3, 4) for keyboard shortcuts
- [ ] Add drag-and-drop to reorder panes (future)
- [ ] Add maximize pane button (temporary fullscreen)
- [ ] Add tooltips for all buttons
- [ ] Add keyboard shortcuts cheat sheet (Cmd+?)

---

## Testing Requirements

### Backend Tests (Rust)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/__tests__/terminal_manager_test.rs`

- [ ] Test create multiple shell sessions
- [ ] Test attach pane to process
- [ ] Test detach pane from process
- [ ] Test process log streaming to pane
- [ ] Test concurrent shell sessions (4+ shells)
- [ ] Test pane cleanup on detach

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src-tauri/src/core/__tests__/layout_storage_test.rs`

- [ ] Test save layout to storage
- [ ] Test load saved layouts
- [ ] Test delete layout
- [ ] Test layout name conflicts
- [ ] Test invalid layout data handling

**Target:** 15+ unit tests passing

### Frontend Tests (TypeScript/Vitest)

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/__tests__/TerminalGrid.test.ts`

- [ ] Test renders correct number of panes per layout
- [ ] Test grid layout classes applied correctly
- [ ] Test focus management (click to focus)
- [ ] Test keyboard shortcuts (Cmd+1, Cmd+2, etc.)
- [ ] Test attach/detach pane
- [ ] Test terminal resize on window resize

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/src/lib/components/__tests__/LayoutSelector.test.ts`

- [ ] Test preset layouts render
- [ ] Test saved layouts render
- [ ] Test layout change event

**Target:** 12+ component tests passing

### Integration Tests

**File:** `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/tests/terminal_grid_integration.rs`

- [ ] Create 2x2 layout → attach 2 shells + 2 processes
- [ ] Verify all panes render correctly
- [ ] Change to 1x1 layout → verify panes collapsed
- [ ] Save layout → reload → verify restored
- [ ] Close pane with running process → verify confirmation

**Target:** 5+ integration tests passing

---

## Performance Requirements

- [ ] xterm.js initialization: < 100ms per pane
- [ ] Layout change (1x1 → 2x2): < 200ms transition
- [ ] Terminal resize (FitAddon): < 50ms
- [ ] Keyboard shortcut response: < 10ms
- [ ] Simultaneous log streaming to 4 panes: 60 FPS
- [ ] Memory per terminal pane: < 10MB
- [ ] Support 100k+ lines in scrollback without lag

---

## Accessibility Requirements

WCAG 2.2 Level AA compliance:

- [ ] All panes keyboard accessible (Tab to navigate)
- [ ] Active pane has visible focus indicator
- [ ] Keyboard shortcuts documented (Cmd+? opens help)
- [ ] Pane headers have `aria-label` (e.g., "Terminal Pane 1")
- [ ] Attach/Detach buttons have `aria-label`
- [ ] Close button has confirmation for running processes
- [ ] Terminal content accessible to screen readers (xterm.js screen reader mode)
- [ ] Color contrast ≥ 4.5:1 for pane borders
- [ ] Focus trap in attach modal
- [ ] Esc key closes modals

---

## UI Design Specifications

### Glass Morphism Theme

```css
/* Terminal Grid Container */
.terminal-grid {
  display: grid;
  gap: 8px;
  height: 100%;
  padding: var(--space-md);
  background: rgba(15, 15, 20, 0.3);
}

.grid-1x1 {
  grid-template-columns: 1fr;
  grid-template-rows: 1fr;
}

.grid-2x1 {
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr;
}

.grid-1x2 {
  grid-template-columns: 1fr;
  grid-template-rows: 1fr 1fr;
}

.grid-2x2 {
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
}

.grid-3x1 {
  grid-template-columns: 1fr 1fr 1fr;
  grid-template-rows: 1fr;
}

/* Terminal Pane */
.terminal-pane {
  display: flex;
  flex-direction: column;
  background: rgba(20, 20, 25, 0.8);
  backdrop-filter: blur(12px);
  border: 2px solid rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  overflow: hidden;
  transition: border-color 0.2s;
}

.terminal-pane.focused {
  border-color: #3b82f6;
  box-shadow: 0 0 0 1px #3b82f6;
}

/* Pane Header */
.pane-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: rgba(30, 30, 35, 0.6);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.pane-header.focused {
  background: rgba(59, 130, 246, 0.15);
  border-bottom-color: rgba(59, 130, 246, 0.3);
}

.pane-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary);
}

.pane-subtitle {
  font-size: 11px;
  color: var(--text-tertiary);
  margin-left: 8px;
}

/* Terminal Content */
.terminal-content {
  flex: 1;
  overflow: hidden;
}

/* Pane Number Badge */
.pane-number {
  position: absolute;
  top: 4px;
  left: 4px;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(59, 130, 246, 0.2);
  color: #3b82f6;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 700;
  pointer-events: none;
}
```

### Colors

- Pane background: `rgba(20, 20, 25, 0.8)`
- Pane border (inactive): `rgba(255, 255, 255, 0.05)`
- Pane border (focused): `#3b82f6`
- Header background: `rgba(30, 30, 35, 0.6)`
- Header background (focused): `rgba(59, 130, 246, 0.15)`
- Terminal foreground: `#e0e0e0`
- Terminal cursor: `#3b82f6`

---

## Definition of Done

- [ ] Backend: Terminal manager supports multiple shell sessions
- [ ] Backend: Attach/detach pane to process working
- [ ] Backend: Layout storage (save/load/delete) working
- [ ] Frontend: TerminalGrid component with 5 layouts complete
- [ ] Frontend: xterm.js integration with FitAddon working
- [ ] Frontend: Focus management and visual indicators working
- [ ] Frontend: Keyboard shortcuts (Cmd+1-9, Cmd+W) working
- [ ] Frontend: PaneHeader component complete
- [ ] Frontend: LayoutSelector component complete
- [ ] Frontend: AttachModal for selecting attachment source
- [ ] Store: terminalLayout.svelte.ts fully implemented
- [ ] Integration: Attach to managed processes from 4B-1 working
- [ ] Integration: Attach to independent shells working
- [ ] Tests: 15+ backend unit tests passing
- [ ] Tests: 12+ frontend component tests passing
- [ ] Tests: 5+ integration tests passing
- [ ] Performance: All metrics met
- [ ] Accessibility: WCAG 2.2 Level AA compliant
- [ ] Documentation: User guide with layout screenshots
- [ ] Documentation: Keyboard shortcuts documented
- [ ] Code Review: PR approved
- [ ] Demo: Video showing multi-pane workflow

---

## Notes

### Technical Decisions

1. **CSS Grid over manual positioning**: Simpler, responsive, leverages browser layout engine.

2. **xterm.js over custom terminal**: Mature, well-tested, supports addons (fit, serialize).

3. **Read-only process attachment**: Prevents accidental input to managed processes. Shell input is for independent shells only.

4. **Layout persistence**: Using localStorage initially for simplicity. SQLite option for future sync across devices.

5. **Keyboard shortcuts**: Cmd+1-9 for focus (matches browser tab shortcuts), Cmd+W to close (familiar).

### Future Enhancements (Phase 5)

- Resizable panes with drag handles (use `react-resizable-panels` equivalent)
- Drag-and-drop pane reordering
- Maximize/minimize panes (temporary fullscreen)
- Shared terminal sessions (tmux-like multiplexing)
- Terminal session recording/playback
- Export terminal history to file
- Custom layouts (arbitrary grid sizes)
- Vertical/horizontal split buttons (VS Code style)

### References

- Research notes: `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4-research-notes.md`
- Process Log Viewer (4A-1): `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4A-1_PROCESS_LOG_VIEWER.md`
- Process Management (4B-1): `/Users/gdsks/G-Development/GLINR/GLINR/GLINCKER/sentinel/.internal/phase4/4B-1_PROCESS_MANAGEMENT.md`
- xterm.js: https://xtermjs.org/

---

**Start Date:** TBD
**End Date:** TBD
**Assignee:** TBD

**This is the contract for Phase 4C-1. All requirements must be met before moving to 4D-1.**
