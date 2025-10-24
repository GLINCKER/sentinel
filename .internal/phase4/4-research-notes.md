# Phase 4: Research Notes

**Created:** 2025-10-24
**Research Focus:** Process log capture, multi-terminal UI, log aggregation patterns

---

## 🔍 Research Completed

### 1. Process stdout/stderr Capture in Rust/Tauri

#### Tauri Command API (RECOMMENDED)
**Source**: [Tauri v1 Shell API](https://v1.tauri.app/v1/api/js/shell/)

```javascript
const cmd = Command.create('ffmpeg', ['-i', 'input.mp4', 'output.mp4'])
cmd.on('close', (data) => { /* ... */ })
cmd.on('error', (error) => console.error(`error: "${error}"`))
cmd.stdout.on('data', (line) => console.log(`stdout: "${line}"`))
cmd.stderr.on('data', (line) => console.log(`stderr: "${line}"`))
const child = await cmd.spawn()
```

**Pros**:
- Built-in Tauri API (no external deps)
- Event-driven streaming
- Non-blocking
- Cross-platform

**Cons**:
- Only works for NEW processes spawned by Sentinel
- Cannot attach to existing processes

**Decision**: ✅ Use for all Sentinel-managed processes

---

### 2. Attaching to Existing Processes

#### Method A: /proc filesystem (Linux/macOS)
**Source**: [Unix StackExchange](https://unix.stackexchange.com/questions/58550)

```bash
tail -f /proc/<PID>/fd/1  # stdout
tail -f /proc/<PID>/fd/2  # stderr
```

**Limitations**:
- Only works if process has allocated a tty
- Many background processes redirect to /dev/null
- Platform-specific (no Windows support)

#### Method B: strace (Linux only)
```bash
strace -p <PID> -e write -s 9999
```

**Limitations**:
- Requires root/ptrace permissions
- Performance overhead
- Linux only
- Security concerns

#### Method C: reptyr (Advanced)
**Source**: [GitHub - nelhage/reptyr](https://github.com/nelhage/reptyr)

Reparent process to new terminal

**Limitations**:
- Requires ptrace permissions
- Complex implementation
- Platform-specific
- Risky (can crash process)

**Decision**: ⚠️ Defer to Phase 5 (Future Enhancement)

**Practical Approach for Phase 4**:
1. Check if process is Sentinel-managed → show logs ✅
2. If not managed → show "Adopt Process" button → restart under Sentinel
3. Or detect log files and tail them

---

### 3. xterm.js Multi-Terminal Patterns

#### Multiple Terminal Instances
**Source**: [xterm.js GitHub Issues #564](https://github.com/xtermjs/xterm.js/issues/564)

```javascript
// Create separate Terminal instance per pane
const terminal1 = new Terminal()
const terminal2 = new Terminal()

terminal1.open(document.getElementById('pane-1'))
terminal2.open(document.getElementById('pane-2'))
```

**Layout with CSS Grid**:
```css
.terminal-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
  gap: 8px;
  height: 100%;
}
```

**Pros**:
- Simple, well-documented
- Each terminal is independent
- Easy resize handling with fit addon
- Used by VS Code, Hyper

**Decision**: ✅ Use CSS Grid + multiple Terminal instances

---

### 4. Log Aggregation Patterns

#### Multi-Tail Approach
**Concept**: Merge logs from multiple sources with timestamps

```typescript
interface LogLine {
  source: string;        // process name/ID
  timestamp: Date;
  level: 'info' | 'warn' | 'error' | 'debug';
  content: string;
  color: string;         // color code for source
}

// Merge and sort by timestamp
const aggregatedLogs = [
  ...process1Logs,
  ...process2Logs,
  ...process3Logs
].sort((a, b) => a.timestamp - b.timestamp);
```

**UI Pattern**:
```
[12:34:56] [nextjs]  ▸ Ready on http://localhost:3000
[12:34:57] [fastapi] INFO: Started server on 0.0.0.0:8000
[12:34:58] [spring]  2025-10-24 12:34:58 INFO  Application started
```

**Decision**: ✅ Implement in 4D with color coding per source

---

### 5. Log Buffer Management

#### Circular Buffer Pattern
**Problem**: Unbounded log growth → memory issues
**Solution**: Fixed-size ring buffer (10k lines)

```rust
use std::collections::VecDeque;

pub struct LogBuffer {
    lines: VecDeque<String>,
    max_lines: usize,
}

impl LogBuffer {
    pub fn new(max_lines: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(max_lines),
            max_lines,
        }
    }

    pub fn push(&mut self, line: String) {
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front(); // Remove oldest
        }
        self.lines.push_back(line);
    }
}
```

**Decision**: ✅ 10k lines per process, configurable in settings

---

### 6. Performance Considerations

#### Log Streaming Rate Limiting
**Problem**: Chatty processes (1000s of lines/sec) → UI lag
**Solution**: Batch updates

```rust
// Batch log lines every 50ms
const LOG_BATCH_INTERVAL_MS: u64 = 50;

tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_millis(LOG_BATCH_INTERVAL_MS));
    let mut batch = Vec::new();

    loop {
        tokio::select! {
            Some(line) = log_rx.recv() => {
                batch.push(line);
            }
            _ = interval.tick() => {
                if !batch.is_empty() {
                    emit_batch(&batch);
                    batch.clear();
                }
            }
        }
    }
});
```

**Decision**: ✅ 50ms batching for high-throughput processes

---

### 7. UI Patterns Research (2025)

#### Modern Log Viewer Features
**Researched**: VS Code terminal, iTerm2, Hyper, Warp

**Best Practices Identified**:
1. **Virtual Scrolling** - Only render visible lines (1000+ lines)
2. **Search & Highlight** - Cmd+F with regex support
3. **Follow Mode** - Auto-scroll to bottom (toggle)
4. **Copy Selection** - Standard text selection
5. **Font Customization** - Monospace font settings
6. **Line Numbers** - Optional toggle
7. **Syntax Highlighting** - ANSI color support
8. **Timestamps** - Relative or absolute

**Decision**: ✅ Implement 1-5 in Phase 4, defer 6-8 to Phase 5

---

### 8. Process Auto-Detection

#### Framework Detection Patterns
**Research**: Common dev server patterns

| Framework | Detection | Default Port | Start Command |
|-----------|-----------|--------------|---------------|
| Next.js | `package.json` → `"next"` | 3000 | `npm run dev` |
| React (Vite) | `package.json` → `"vite"` | 5173 | `npm run dev` |
| FastAPI | `main.py` + `fastapi` import | 8000 | `uvicorn main:app --reload` |
| Spring Boot | `pom.xml` or `build.gradle` | 8080 | `./mvnw spring-boot:run` |
| Django | `manage.py` | 8000 | `python manage.py runserver` |
| Express | `package.json` → `"express"` | 3000 | `npm start` |
| Flask | `app.py` + `flask` import | 5000 | `flask run` |

**Implementation**:
```rust
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

pub async fn detect_framework(working_dir: &Path) -> FrameworkType {
    // Check for package.json, pom.xml, requirements.txt, etc.
}
```

**Decision**: ✅ Implement in 4B for common frameworks

---

## 🎯 Key Takeaways

1. **Focus on New Processes**: Tauri Command API is mature and reliable
2. **Defer Process Adoption**: Too complex for Phase 4, plan for Phase 5
3. **Multi-Terminal UI**: Standard CSS Grid + xterm.js pattern works well
4. **Log Performance**: 50ms batching + 10k line buffer prevents lag
5. **Auto-Detection**: 7 common frameworks cover 90% of use cases

---

## 📚 Resources Used

1. Tauri Shell API - https://v1.tauri.app/v1/api/js/shell/
2. xterm.js Documentation - https://xtermjs.org/docs/
3. Attaching to Process Output - https://unix.stackexchange.com/questions/58550
4. Rust tokio::process - https://docs.rs/tokio/latest/tokio/process/
5. VS Code Terminal Source - https://github.com/microsoft/vscode/tree/main/src/vs/workbench/contrib/terminal

---

**Research Completed:** 2025-10-24
**Ready for Implementation:** ✅ Yes
**Next Step:** Begin 4A-1 implementation
