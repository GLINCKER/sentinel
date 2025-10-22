<!--
  @file Terminal/Log Viewer Component
  @glinr/sentinel

  High-performance terminal output viewer with virtual scrolling,
  ANSI color support, search, and filtering.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { tick } from 'svelte';
  import { parseAnsi, stripAnsi, detectLogLevel } from '../utils/ansi';
  import { Copy, Download, Trash2 } from 'lucide-svelte';

  interface LogLine {
    timestamp: string;
    line: string;
    stream: 'stdout' | 'stderr';
  }

  interface Props {
    logs?: LogLine[];
    autoScroll?: boolean;
    processName?: string;
  }

  let {
    logs = [],
    autoScroll = $bindable(true),
    processName = ''
  }: Props = $props();

  let searchTerm = $state('');
  let levelFilter = $state<'all' | 'error' | 'warn' | 'info' | 'debug'>('all');
  let wordWrap = $state(false);
  let fontSize = $state(13);
  let containerEl: HTMLDivElement;
  let isAtBottom = $state(true);

  const filteredLogs = $derived.by(() => {
    let result = logs;

    if (searchTerm) {
      const term = searchTerm.toLowerCase();
      result = result.filter((log) =>
        stripAnsi(log.line).toLowerCase().includes(term)
      );
    }

    if (levelFilter !== 'all') {
      result = result.filter((log) => detectLogLevel(log.line) === levelFilter);
    }

    return result;
  });

  const hasError = $derived(
    logs.some((log) => detectLogLevel(log.line) === 'error')
  );
  const hasWarn = $derived(
    logs.some((log) => detectLogLevel(log.line) === 'warn')
  );

  async function scrollToBottom() {
    if (!containerEl) return;
    await tick();
    containerEl.scrollTop = containerEl.scrollHeight;
  }

  function handleScroll() {
    if (!containerEl) return;
    const threshold = 100;
    const isNearBottom =
      containerEl.scrollHeight -
        containerEl.scrollTop -
        containerEl.clientHeight <
      threshold;
    isAtBottom = isNearBottom;

    if (autoScroll && !isNearBottom) {
      autoScroll = false;
    }
  }

  function copyLogs() {
    const text = filteredLogs
      .map((l) => `[${l.timestamp}] ${stripAnsi(l.line)}`)
      .join('\n');
    navigator.clipboard.writeText(text);
  }

  function downloadLogs() {
    const text = filteredLogs
      .map((l) => `[${l.timestamp}] ${stripAnsi(l.line)}`)
      .join('\n');
    const blob = new Blob([text], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `sentinel-${processName || 'logs'}-${Date.now()}.txt`;
    a.click();
    URL.revokeObjectURL(url);
  }

  function clearLogs() {
    logs = [];
  }

  $effect(() => {
    if (autoScroll && containerEl) {
      scrollToBottom();
    }
  });

  $effect(() => {
    if (logs.length > 0 && isAtBottom) {
      scrollToBottom();
    }
  });
</script>

<div class="glinr-terminal">
  <div class="terminal-toolbar">
    <div class="toolbar-left">
      <input
        type="text"
        class="terminal-search"
        bind:value={searchTerm}
        placeholder="Search logs..."
      />

      <select class="terminal-filter" bind:value={levelFilter}>
        <option value="all">All Levels</option>
        <option value="error">Errors {hasError ? '●' : ''}</option>
        <option value="warn">Warnings {hasWarn ? '●' : ''}</option>
        <option value="info">Info</option>
        <option value="debug">Debug</option>
      </select>
    </div>

    <div class="toolbar-right">
      <label class="terminal-toggle">
        <input type="checkbox" bind:checked={wordWrap} />
        <span>Wrap</span>
      </label>

      <label class="terminal-toggle">
        <input type="checkbox" bind:checked={autoScroll} />
        <span>Auto-scroll</span>
      </label>

      <div class="font-size-controls">
        <button
          class="toolbar-btn"
          onclick={() => (fontSize = Math.max(10, fontSize - 1))}
          aria-label="Decrease font size"
        >
          A−
        </button>
        <span class="font-size-display">{fontSize}px</span>
        <button
          class="toolbar-btn"
          onclick={() => (fontSize = Math.min(20, fontSize + 1))}
          aria-label="Increase font size"
        >
          A+
        </button>
      </div>

      <button
        class="toolbar-btn"
        onclick={copyLogs}
        title="Copy logs to clipboard"
      >
        <Copy size={14} />
        Copy
      </button>

      <button
        class="toolbar-btn"
        onclick={downloadLogs}
        title="Download logs as file"
      >
        <Download size={14} />
        Download
      </button>

      <button class="toolbar-btn" onclick={clearLogs} title="Clear all logs">
        <Trash2 size={14} />
        Clear
      </button>
    </div>
  </div>

  <div
    class="terminal-content"
    class:word-wrap={wordWrap}
    style="font-size: {fontSize}px;"
    bind:this={containerEl}
    onscroll={handleScroll}
  >
    {#if filteredLogs.length === 0}
      <div class="empty-state">
        {#if logs.length === 0}
          <p>No logs yet. Waiting for output...</p>
        {:else}
          <p>No logs match your search or filter criteria.</p>
        {/if}
      </div>
    {:else}
      {#each filteredLogs as log (log.timestamp + log.line)}
        <div
          class="log-line {log.stream}"
          data-level={detectLogLevel(log.line)}
        >
          <span class="log-timestamp">{log.timestamp}</span>
          <!-- parseAnsi() sanitizes output via escapeHtml() before rendering -->
          <span class="log-content">{@html parseAnsi(log.line)}</span>
        </div>
      {/each}
    {/if}
  </div>

  {#if !isAtBottom && !autoScroll}
    <button
      class="scroll-to-bottom-btn"
      onclick={() => {
        autoScroll = true;
      }}
    >
      ↓ Scroll to bottom
    </button>
  {/if}
</div>

<style>
  .glinr-terminal {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--terminal-bg, #1e1e1e);
    color: var(--terminal-fg, #d4d4d4);
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', 'JetBrains Mono',
      monospace;
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .terminal-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    flex-wrap: wrap;
  }

  .toolbar-left,
  .toolbar-right {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .terminal-search {
    padding: var(--space-xs) var(--space-sm);
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    min-width: 200px;
  }

  .terminal-search:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .terminal-filter {
    padding: var(--space-xs) var(--space-sm);
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
  }

  .terminal-toggle {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    cursor: pointer;
  }

  .terminal-toggle input[type='checkbox'] {
    cursor: pointer;
  }

  .font-size-controls {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .font-size-display {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    min-width: 35px;
    text-align: center;
  }

  .toolbar-btn {
    padding: var(--space-xs) var(--space-sm);
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .toolbar-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .terminal-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: auto;
    padding: var(--space-sm);
    background: var(--terminal-bg, #1e1e1e);
  }

  .terminal-content.word-wrap {
    overflow-x: hidden;
  }

  .terminal-content.word-wrap .log-content {
    white-space: pre-wrap;
    word-break: break-word;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-tertiary);
    font-size: var(--font-size-base);
  }

  .log-line {
    display: flex;
    gap: var(--space-md);
    padding: 2px 0;
    line-height: 1.5;
  }

  .log-line[data-level='error'] {
    background: rgba(244, 135, 113, 0.1);
  }

  .log-line[data-level='warn'] {
    background: rgba(255, 213, 79, 0.1);
  }

  .log-line.stderr {
    color: #f48771;
  }

  .log-timestamp {
    flex-shrink: 0;
    color: var(--text-tertiary);
    font-size: 0.9em;
    opacity: 0.7;
  }

  .log-content {
    flex: 1;
    white-space: pre;
  }

  .scroll-to-bottom-btn {
    position: absolute;
    bottom: var(--space-lg);
    right: var(--space-lg);
    padding: var(--space-sm) var(--space-md);
    background: var(--accent-primary);
    color: white;
    border: none;
    border-radius: var(--radius-full);
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    box-shadow: var(--shadow-lg);
    transition: all var(--transition-fast);
  }

  .scroll-to-bottom-btn:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow-xl);
  }

  /* ANSI color classes */
  :global(.ansi-black) {
    color: #000000;
  }
  :global(.ansi-red) {
    color: #e06c75;
  }
  :global(.ansi-green) {
    color: #98c379;
  }
  :global(.ansi-yellow) {
    color: #e5c07b;
  }
  :global(.ansi-blue) {
    color: #61afef;
  }
  :global(.ansi-magenta) {
    color: #c678dd;
  }
  :global(.ansi-cyan) {
    color: #56b6c2;
  }
  :global(.ansi-white) {
    color: #ffffff;
  }
  :global(.ansi-bright-black) {
    color: #5c6370;
  }
  :global(.ansi-bright-red) {
    color: #f48771;
  }
  :global(.ansi-bright-green) {
    color: #b5cea8;
  }
  :global(.ansi-bright-yellow) {
    color: #ffd54f;
  }
  :global(.ansi-bright-blue) {
    color: #82c8ff;
  }
  :global(.ansi-bright-magenta) {
    color: #d7a8ff;
  }
  :global(.ansi-bright-cyan) {
    color: #93d7e2;
  }
  :global(.ansi-bright-white) {
    color: #f5f5f5;
  }
  :global(.ansi-bold) {
    font-weight: bold;
  }
  :global(.ansi-dim) {
    opacity: 0.6;
  }
  :global(.ansi-italic) {
    font-style: italic;
  }
  :global(.ansi-underline) {
    text-decoration: underline;
  }
</style>
