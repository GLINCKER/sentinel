<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { X, Terminal, Download, Trash2, AlertCircle } from 'lucide-svelte';

  interface Props {
    pid: number;
    port: number;
    processName: string;
    onClose: () => void;
  }

  let { pid, port, processName, onClose }: Props = $props();

  interface LogLineEvent {
    attachment_id: string;
    timestamp: string;
    line: string;
    stream: string;
  }

  interface ProcessAttachment {
    pid: number;
    port: number | null;
    name: string;
    command: string;
    log_source: LogSource;
  }

  type LogSource =
    | { type: 'DockerLogs'; container_id: string }
    | { type: 'File'; path: string }
    | { type: 'DTrace'; pid: number }
    | { type: 'Manual'; instructions: string };

  let logs = $state<LogLineEvent[]>([]);
  let attachment = $state<ProcessAttachment | null>(null);
  let attachmentId = $state<string | null>(null);
  let error = $state<string | null>(null);
  let isLoading = $state(true);
  let unlistenFn: UnlistenFn | null = null;
  let autoScroll = $state(true);
  let logContainer: HTMLDivElement | null = null;

  onMount(async () => {
    try {
      // Attach to the external process
      attachment = await invoke<ProcessAttachment>(
        'attach_to_external_process',
        {
          pid,
          port
        }
      );

      // Check the log source type
      const logSource = attachment.log_source;

      if (logSource.type === 'File') {
        // Start tailing the log file
        attachmentId = await invoke<string>('tail_log_file', {
          path: logSource.path,
          app: null
        });
      } else if (logSource.type === 'DTrace') {
        // Start dtrace capture
        attachmentId = await invoke<string>('capture_with_dtrace', {
          pid: logSource.pid,
          app: null
        });
      } else if (logSource.type === 'Manual') {
        // Show manual instructions
        error = logSource.instructions;
        isLoading = false;
        return;
      } else if (logSource.type === 'DockerLogs') {
        // TODO: Implement Docker logs streaming
        error = 'Docker log streaming not yet implemented';
        isLoading = false;
        return;
      }

      // Listen for log-line events
      unlistenFn = await listen<LogLineEvent>('log-line', (event) => {
        if (attachmentId && event.payload.attachment_id === attachmentId) {
          logs = [...logs, event.payload];

          // Auto-scroll if enabled
          if (autoScroll && logContainer) {
            setTimeout(() => {
              logContainer!.scrollTop = logContainer!.scrollHeight;
            }, 0);
          }
        }
      });

      isLoading = false;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      isLoading = false;
    }
  });

  onDestroy(async () => {
    // Detach from logs
    if (attachmentId) {
      try {
        await invoke('detach_external_logs', { attachmentId });
      } catch (err) {
        console.error('Failed to detach logs:', err);
      }
    }

    // Unlisten from events
    if (unlistenFn) {
      unlistenFn();
    }
  });

  function clearLogs() {
    logs = [];
  }

  function downloadLogs() {
    const logText = logs
      .map((log) => `[${log.timestamp}] ${log.line}`)
      .join('\n');
    const blob = new Blob([logText], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${processName}-${pid}-logs.txt`;
    a.click();
    URL.revokeObjectURL(url);
  }

  function toggleAutoScroll() {
    autoScroll = !autoScroll;
  }
</script>

<div class="modal-overlay" onclick={onClose}>
  <div
    class="modal-content external-log-viewer"
    onclick={(e) => e.stopPropagation()}
  >
    <!-- Header -->
    <div class="modal-header">
      <div class="header-left">
        <div class="icon-wrapper">
          <Terminal size={20} />
        </div>
        <div class="header-info">
          <h3 class="modal-title">External Process Logs</h3>
          <p class="process-info">
            {processName} (PID: {pid}, Port: {port})
          </p>
        </div>
      </div>
      <button class="modal-close" onclick={onClose} aria-label="Close">
        <X size={20} />
      </button>
    </div>

    <!-- Log Source Info -->
    {#if attachment}
      <div class="log-source-info">
        {#if attachment.log_source.type === 'File'}
          <span class="source-label">Log File:</span>
          <span class="source-value">{attachment.log_source.path}</span>
        {:else if attachment.log_source.type === 'DTrace'}
          <span class="source-label">Capture Method:</span>
          <span class="source-value">dtrace (stdout/stderr intercept)</span>
        {:else if attachment.log_source.type === 'DockerLogs'}
          <span class="source-label">Docker Container:</span>
          <span class="source-value">{attachment.log_source.container_id}</span>
        {/if}
      </div>
    {/if}

    <!-- Controls -->
    <div class="controls">
      <button
        class="btn-control"
        onclick={toggleAutoScroll}
        class:active={autoScroll}
        title={autoScroll ? 'Disable auto-scroll' : 'Enable auto-scroll'}
      >
        Auto-scroll {autoScroll ? 'ON' : 'OFF'}
      </button>
      <button
        class="btn-control"
        onclick={downloadLogs}
        disabled={logs.length === 0}
      >
        <Download size={14} />
        Download
      </button>
      <button
        class="btn-control"
        onclick={clearLogs}
        disabled={logs.length === 0}
      >
        <Trash2 size={14} />
        Clear
      </button>
    </div>

    <!-- Log Content -->
    <div class="log-content" bind:this={logContainer}>
      {#if isLoading}
        <div class="loading-state">
          <div class="spinner"></div>
          <p>Attaching to process...</p>
        </div>
      {:else if error}
        <div class="error-state">
          <AlertCircle size={48} />
          <h4>Cannot Auto-Detect Logs</h4>
          <p>{error}</p>
        </div>
      {:else if logs.length === 0}
        <div class="empty-state">
          <Terminal size={48} />
          <p>Waiting for log output...</p>
        </div>
      {:else}
        <div class="log-lines">
          {#each logs as log (log.timestamp + log.line)}
            <div class="log-line">
              <span class="log-timestamp"
                >{new Date(log.timestamp).toLocaleTimeString()}</span
              >
              <span class="log-text">{log.line}</span>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.15s ease;
  }

  .modal-content {
    background: var(--background);
    border: 1px solid var(--border);
    border-radius: 0.75rem;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
    max-width: 900px;
    width: 90%;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    animation: slideUp 0.25s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid var(--border);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 0.875rem;
  }

  .icon-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: 0.625rem;
    background: rgba(59, 130, 246, 0.1);
    color: #3b82f6;
  }

  .header-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .modal-title {
    font-size: 1rem;
    font-weight: 700;
    color: var(--foreground);
    margin: 0;
  }

  .process-info {
    font-size: 0.75rem;
    color: var(--muted-foreground);
    margin: 0;
    font-family: 'SF Mono', 'JetBrains Mono', monospace;
  }

  .modal-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: transparent;
    border: none;
    border-radius: 0.375rem;
    color: var(--muted-foreground);
    cursor: pointer;
    transition: all 0.2s;
  }

  .modal-close:hover {
    background: var(--accent);
    color: var(--foreground);
  }

  .log-source-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: var(--muted);
    border-bottom: 1px solid var(--border);
    font-size: 0.8125rem;
  }

  .source-label {
    font-weight: 600;
    color: var(--muted-foreground);
  }

  .source-value {
    font-family: 'SF Mono', 'JetBrains Mono', monospace;
    color: var(--foreground);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .controls {
    display: flex;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    border-bottom: 1px solid var(--border);
    background: var(--background);
  }

  .btn-control {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    background: var(--muted);
    border: 1px solid var(--border);
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--foreground);
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-control:hover:not(:disabled) {
    background: var(--accent);
    border-color: var(--primary);
  }

  .btn-control.active {
    background: rgba(59, 130, 246, 0.1);
    border-color: #3b82f6;
    color: #3b82f6;
  }

  .btn-control:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .log-content {
    flex: 1;
    overflow-y: auto;
    background: #1e1e1e;
    padding: 1rem;
    font-family: 'SF Mono', 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.8125rem;
    line-height: 1.6;
  }

  .loading-state,
  .error-state,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem 2rem;
    color: var(--muted-foreground);
    text-align: center;
    gap: 1rem;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--muted);
    border-top-color: var(--primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .error-state {
    color: #ef4444;
  }

  .error-state h4 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .error-state p {
    margin: 0;
    font-size: 0.875rem;
    max-width: 500px;
    line-height: 1.6;
  }

  .log-lines {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .log-line {
    display: flex;
    gap: 0.75rem;
    padding: 0.25rem 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .log-timestamp {
    flex-shrink: 0;
    color: #666;
    font-size: 0.75rem;
  }

  .log-text {
    color: #d4d4d4;
    white-space: pre-wrap;
    word-break: break-word;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
