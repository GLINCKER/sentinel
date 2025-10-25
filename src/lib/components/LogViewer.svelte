<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { processLogStore, type LogLine } from '$lib/stores/processLog.svelte';
  import { listen } from '@tauri-apps/api/event';
  import {
    X,
    Search,
    Copy,
    Trash2,
    ChevronDown,
    Download,
    Regex as RegexIcon,
    FileText
  } from 'lucide-svelte';
  import { toast } from 'svelte-sonner';
  import Convert from 'ansi-to-html';

  interface ProcessOutputEvent {
    process_id: string;
    output: string;
    stream: string;
    timestamp: string;
  }

  interface Props {
    processName: string;
    onClose?: () => void;
  }

  let { processName, onClose }: Props = $props();

  const convert = new Convert({
    fg: '#e0e0e0',
    bg: 'transparent',
    newline: false,
    escapeXML: true,
    stream: true
  });

  let logs = $state<LogLine[]>([]);
  let filteredLogs = $derived.by(() => {
    if (!searchQuery.trim()) return logs;

    try {
      if (searchRegex) {
        const regex = new RegExp(searchQuery, 'i');
        return logs.filter((l) => regex.test(l.line));
      } else {
        const query = searchQuery.toLowerCase();
        return logs.filter((l) => l.line.toLowerCase().includes(query));
      }
    } catch {
      return logs;
    }
  });

  let loading = $state(true);
  let error = $state<string | null>(null);
  let searchQuery = $state('');
  let searchRegex = $state(false);
  let autoScroll = $state(true);
  let container: HTMLDivElement;

  function renderAnsi(text: string): string {
    try {
      return convert.toHtml(text);
    } catch {
      return text;
    }
  }

  async function scrollToBottom() {
    if (container && autoScroll) {
      await tick();
      container.scrollTop = container.scrollHeight;
    }
  }

  let unlistenOutput: (() => void) | null = null;
  let unlistenExit: (() => void) | null = null;

  onMount(async () => {
    await loadLogs();

    // Listen for real-time PTY output
    unlistenOutput = await listen<ProcessOutputEvent>(
      'process-output',
      (event) => {
        if (event.payload.process_id === processName) {
          const logLine: LogLine = {
            line: event.payload.output,
            timestamp: event.payload.timestamp,
            level: 'info'
          };
          logs = [...logs, logLine];
          scrollToBottom();
        }
      }
    );

    // Listen for process exit
    unlistenExit = await listen<{
      process_id: string;
      exit_code: number | null;
    }>('process-exit', (event) => {
      if (event.payload.process_id === processName) {
        const exitLine: LogLine = {
          line: `[Process exited with code ${event.payload.exit_code}]`,
          timestamp: new Date().toISOString(),
          level: 'info'
        };
        logs = [...logs, exitLine];
        scrollToBottom();
      }
    });

    // Add keyboard event listener
    window.addEventListener('keydown', handleKeydown);
  });

  onDestroy(() => {
    if (unlistenOutput) unlistenOutput();
    if (unlistenExit) unlistenExit();
    window.removeEventListener('keydown', handleKeydown);
  });

  async function loadLogs() {
    try {
      loading = true;
      error = null;
      logs = await processLogStore.getLogs(processName);
      await scrollToBottom();
    } catch (err) {
      error = String(err);
      toast.error('Failed to load logs', { description: String(err) });
    } finally {
      loading = false;
    }
  }

  async function clearLogs() {
    if (!confirm('Clear all logs for this process?')) return;
    try {
      await processLogStore.clearLogs(processName);
      logs = [];
      toast.success('Logs cleared successfully');
    } catch (err) {
      toast.error('Failed to clear logs', { description: String(err) });
    }
  }

  async function copyLogs() {
    try {
      const text = filteredLogs
        .map((l) => `[${formatTimestamp(l.timestamp)}] [${l.stream}] ${l.line}`)
        .join('\n');
      await navigator.clipboard.writeText(text);
      toast.success(`Copied ${filteredLogs.length} log lines to clipboard`);
    } catch (err) {
      toast.error('Failed to copy logs', { description: String(err) });
    }
  }

  async function exportLogs() {
    try {
      const content = logs
        .map((l) => `[${formatTimestamp(l.timestamp)}] [${l.stream}] ${l.line}`)
        .join('\n');
      const blob = new Blob([content], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `${processName}-logs-${new Date().toISOString().slice(0, 10)}.txt`;
      a.click();
      URL.revokeObjectURL(url);
      toast.success('Logs exported successfully');
    } catch (err) {
      toast.error('Failed to export logs', { description: String(err) });
    }
  }

  function formatTimestamp(timestamp: string): string {
    try {
      const date = new Date(timestamp);
      return date.toLocaleTimeString('en-US', { hour12: false });
    } catch {
      return timestamp;
    }
  }

  function handleScroll() {
    if (!container) return;
    const isAtBottom =
      container.scrollHeight - container.scrollTop - container.clientHeight <
      50;
    autoScroll = isAtBottom;
  }

  function toggleAutoScroll() {
    autoScroll = !autoScroll;
    if (autoScroll) {
      scrollToBottom();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    // Cmd/Ctrl + F: Focus search
    if ((e.metaKey || e.ctrlKey) && e.key === 'f') {
      e.preventDefault();
      document.querySelector<HTMLInputElement>('.search-input')?.focus();
    }
    // Cmd/Ctrl + K: Clear logs
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
      e.preventDefault();
      clearLogs();
    }
    // Cmd/Ctrl + S: Export logs
    if ((e.metaKey || e.ctrlKey) && e.key === 's') {
      e.preventDefault();
      exportLogs();
    }
    // Cmd/Ctrl + C: Copy logs (when not selecting text)
    if (
      (e.metaKey || e.ctrlKey) &&
      e.key === 'c' &&
      !window.getSelection()?.toString()
    ) {
      e.preventDefault();
      copyLogs();
    }
    // End key: Jump to bottom
    if (e.key === 'End') {
      e.preventDefault();
      autoScroll = true;
      scrollToBottom();
    }
    // Escape: Close modal
    if (e.key === 'Escape' && onClose) {
      onClose();
    }
  }
</script>

<div class="log-viewer-overlay" onclick={onClose} role="presentation">
  <div
    class="log-viewer-modal"
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-label="Log Viewer"
  >
    <!-- Header -->
    <div class="header">
      <div class="title-container">
        <FileText size={20} class="title-icon" />
        <h2 class="title">Logs: {processName}</h2>
      </div>
      <button class="close-button" onclick={onClose} aria-label="Close">
        <X size={20} />
      </button>
    </div>

    <!-- Toolbar -->
    <div class="toolbar">
      <div class="search-container">
        <Search size={16} class="search-icon" />
        <input
          type="text"
          class="search-input"
          placeholder="Search logs... (⌘F)"
          bind:value={searchQuery}
          aria-label="Search logs"
        />
        <button
          class="regex-toggle"
          class:active={searchRegex}
          onclick={() => (searchRegex = !searchRegex)}
          aria-label="Toggle regex search"
          aria-pressed={searchRegex}
          title="Regular Expression Mode"
        >
          <RegexIcon size={14} />
        </button>
      </div>

      <div class="actions">
        <button
          class="action-button"
          class:active={autoScroll}
          onclick={toggleAutoScroll}
          aria-label="Toggle auto-scroll"
          aria-pressed={autoScroll}
          title="Auto-scroll to bottom (End key)"
        >
          <ChevronDown size={16} />
          Follow
        </button>

        <button
          class="action-button"
          onclick={copyLogs}
          aria-label="Copy logs to clipboard"
          title="Copy logs (⌘C)"
        >
          <Copy size={16} />
          Copy
        </button>

        <button
          class="action-button"
          onclick={exportLogs}
          aria-label="Export logs to file"
          title="Export logs (⌘S)"
        >
          <Download size={16} />
          Export
        </button>

        <button
          class="action-button danger"
          onclick={clearLogs}
          aria-label="Clear all logs"
          title="Clear logs (⌘K)"
        >
          <Trash2 size={16} />
          Clear
        </button>
      </div>
    </div>

    <!-- Log Content -->
    <div
      class="log-container"
      bind:this={container}
      onscroll={handleScroll}
      role="log"
      aria-live={autoScroll ? 'polite' : 'off'}
    >
      {#if loading}
        <div class="status-message">Loading logs...</div>
      {:else if error}
        <div class="status-message error">Error: {error}</div>
      {:else if filteredLogs.length === 0}
        <div class="status-message">No logs to display</div>
      {:else}
        {#each filteredLogs as log, i (i)}
          <div class="log-line" class:stderr={log.stream === 'stderr'}>
            <span class="timestamp">[{formatTimestamp(log.timestamp)}]</span>
            <span class="stream">[{log.stream}]</span>
            <span class="content">{@html renderAnsi(log.line)}</span>
          </div>
        {/each}
      {/if}
    </div>

    <!-- Footer -->
    <div class="footer">
      <span>{filteredLogs.length} lines</span>
      <span>Auto-scroll: {autoScroll ? 'ON' : 'OFF'}</span>
    </div>
  </div>
</div>

<style>
  .log-viewer-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 2rem;
  }

  .log-viewer-modal {
    background: var(--bg-secondary, rgba(15, 15, 20, 0.95));
    backdrop-filter: blur(20px);
    border: 1px solid var(--border-primary, rgba(255, 255, 255, 0.1));
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    width: 90%;
    max-width: 1200px;
    height: 80vh;
    display: flex;
    flex-direction: column;
  }

  :global(.light) .log-viewer-modal {
    background: rgba(255, 255, 255, 0.98);
    border: 1px solid rgba(0, 0, 0, 0.1);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-lg) var(--space-xl);
    border-bottom: 1px solid var(--border);
  }

  .title-container {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .title-icon {
    color: var(--text-secondary);
    flex-shrink: 0;
  }

  .title {
    margin: 0;
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-button {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: var(--space-sm);
    border-radius: var(--radius-sm);
    transition: var(--transition-fast);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-button:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: var(--space-lg);
    padding: var(--space-lg) var(--space-xl);
    border-bottom: 1px solid var(--border);
    flex-wrap: wrap;
  }

  .search-container {
    flex: 1;
    min-width: 250px;
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 0 var(--space-md);
  }

  .search-container:focus-within {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .search-icon {
    color: var(--text-secondary);
    flex-shrink: 0;
  }

  .search-input {
    flex: 1;
    background: none;
    border: none;
    outline: none;
    color: var(--text-primary);
    padding: var(--space-sm) 0;
    font-size: var(--font-size-sm);
  }

  .regex-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-sm);
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-left: none;
    border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
    color: var(--text-secondary);
    cursor: pointer;
    transition: var(--transition-fast);
    flex-shrink: 0;
  }

  .regex-toggle:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .regex-toggle.active {
    background: rgba(59, 130, 246, 0.1);
    border-color: var(--primary);
    color: var(--primary);
  }

  .actions {
    display: flex;
    gap: var(--space-sm);
  }

  .action-button {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-lg);
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    font-weight: 500;
    transition: var(--transition-fast);
    white-space: nowrap;
  }

  .action-button:hover {
    background: var(--bg-hover);
    border-color: var(--border-light);
  }

  .action-button.active {
    background: var(--success-bg);
    border-color: var(--success);
    color: var(--success);
  }

  .action-button.danger:hover {
    background: var(--error-bg);
    border-color: var(--error);
    color: var(--error);
  }

  .log-container {
    flex: 1;
    overflow-y: auto;
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    margin: 0 var(--space-xl) var(--space-lg);
    border-radius: var(--radius-md);
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    line-height: 1.5;
  }

  .log-line {
    padding: var(--space-xs) var(--space-md);
    border-bottom: 1px solid var(--border);
    color: var(--text-primary);
    display: flex;
    gap: var(--space-sm);
  }

  .log-line.stderr {
    background: var(--error-bg);
    color: var(--error);
  }

  .timestamp {
    color: var(--text-tertiary);
    font-size: var(--font-size-xs);
    font-weight: 600;
    flex-shrink: 0;
  }

  .stream {
    color: var(--text-secondary);
    font-size: var(--font-size-xs);
    flex-shrink: 0;
    min-width: 60px;
  }

  .content {
    flex: 1;
    word-break: break-all;
  }

  .status-message {
    padding: var(--space-3xl);
    text-align: center;
    color: var(--text-secondary);
  }

  .status-message.error {
    color: var(--error);
  }

  .footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-md) var(--space-xl);
    border-top: 1px solid var(--border);
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
  }
</style>
