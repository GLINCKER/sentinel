<script lang="ts">
  import {
    Play,
    Square,
    RotateCw,
    Settings2,
    Trash2,
    ScrollText,
    CheckCircle2,
    XCircle,
    Clock,
    Loader2
  } from 'lucide-svelte';
  import type {
    ProcessConfig,
    ProcessStatusInfo
  } from '../../stores/processConfig.svelte';
  import { navigateToProcess } from '../../stores/navigation';
  import type { ComponentType } from 'svelte';

  interface Props {
    config: ProcessConfig;
    status: ProcessStatusInfo;
    viewMode?: 'grid' | 'list';
    onStart: () => void;
    onStop: () => void;
    onRestart: () => void;
    onEdit: () => void;
    onDelete: () => void;
  }

  let { config, status, onStart, onStop, onRestart, onEdit, onDelete }: Props =
    $props();

  function getStateColor(state?: string): string {
    switch (state) {
      case 'Running':
        return 'running';
      case 'Starting':
        return 'starting';
      case 'Stopped':
        return 'stopped';
      case 'Crashed':
        return 'crashed';
      default:
        return 'stopped';
    }
  }

  function getStateIcon(state?: string): ComponentType {
    switch (state) {
      case 'Running':
        return CheckCircle2;
      case 'Starting':
        return Loader2;
      case 'Stopped':
        return XCircle;
      case 'Crashed':
        return XCircle;
      default:
        return XCircle;
    }
  }

  let stateColor = $derived(getStateColor(status.status));
  let StateIcon = $derived(getStateIcon(status.status));

  function formatUptime(seconds?: number): string {
    if (!seconds) return '-';
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    if (days > 0) return `${days}d ${hours}h`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    if (minutes > 0) return `${minutes}m`;
    return `${seconds}s`;
  }

  function handleViewLogs() {
    // Navigate using config ID, not runtime processId
    navigateToProcess(config.id);
  }
</script>

<div class="process-card">
  <div class="process-header">
    <div class="process-title">
      <div class="status-indicator status-{stateColor}">
        <svelte:component this={StateIcon} size={14} />
      </div>
      <div class="name-group">
        <h4 class="process-name">{config.name}</h4>
        <span class="process-framework"
          >{config.frameworkType || 'Unknown'}</span
        >
      </div>
    </div>
    <div class="process-actions">
      {#if status.running}
        <button
          class="action-btn restart-btn"
          onclick={onRestart}
          title="Restart process"
        >
          <RotateCw size={12} />
        </button>
        <button
          class="action-btn stop-btn"
          onclick={onStop}
          title="Stop process"
        >
          <Square size={12} />
        </button>
      {:else}
        <button
          class="action-btn start-btn"
          onclick={onStart}
          title="Start process"
        >
          <Play size={12} />
        </button>
      {/if}
    </div>
  </div>

  <div class="process-details">
    <div class="detail-row">
      <span class="detail-label">Command</span>
      <span class="detail-value">{config.command} {config.args.join(' ')}</span>
    </div>
    <div class="detail-row">
      <span class="detail-label">Status</span>
      <span class="detail-value status-{stateColor}">
        {status.status || 'Stopped'}
      </span>
    </div>
    {#if config.port}
      <div class="detail-row">
        <span class="detail-label">Port</span>
        <span class="detail-value">{config.port}</span>
      </div>
    {/if}
    {#if status.pid}
      <div class="detail-row">
        <span class="detail-label">PID</span>
        <span class="detail-value">{status.pid}</span>
      </div>
    {/if}
  </div>

  {#if status.running}
    <div class="process-stats">
      <div class="stat-item">
        <div class="stat-icon">
          <Clock size={14} />
        </div>
        <div class="stat-details">
          <div class="stat-label">Uptime</div>
          <div class="stat-value">{formatUptime(status.uptimeSeconds)}</div>
        </div>
      </div>
    </div>
  {/if}

  <div class="process-footer">
    {#if status.running}
      <button class="footer-btn" onclick={handleViewLogs} title="View logs">
        <ScrollText size={14} />
        Logs
      </button>
    {/if}
    <button class="footer-btn" onclick={onEdit} title="Edit configuration">
      <Settings2 size={14} />
      Edit
    </button>
    <button
      class="footer-btn danger"
      onclick={onDelete}
      title="Delete configuration"
    >
      <Trash2 size={14} />
      Delete
    </button>
  </div>

  <div class="process-path" title={config.workingDir}>
    {config.workingDir}
  </div>
</div>

<style>
  .process-card {
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    padding: var(--space-md);
    transition: all var(--transition-base);
    box-shadow:
      0 2px 8px rgba(0, 0, 0, 0.04),
      0 1px 2px rgba(0, 0, 0, 0.02);
  }

  .process-card:hover {
    transform: translateY(-1px);
    border-color: var(--accent-primary);
    box-shadow:
      0 4px 12px rgba(0, 0, 0, 0.08),
      0 2px 4px rgba(0, 0, 0, 0.04);
  }

  .process-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-md);
  }

  .process-title {
    display: flex;
    align-items: flex-start;
    gap: var(--space-sm);
    flex: 1;
    min-width: 0;
  }

  .status-indicator {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: var(--radius-md);
    flex-shrink: 0;
  }

  .status-indicator.status-running {
    background: rgba(34, 197, 94, 0.15);
    color: rgb(34, 197, 94);
  }

  .status-indicator.status-starting {
    background: rgba(251, 191, 36, 0.15);
    color: rgb(251, 191, 36);
  }

  .status-indicator.status-stopped {
    background: rgba(100, 116, 139, 0.15);
    color: rgb(100, 116, 139);
  }

  .status-indicator.status-crashed {
    background: rgba(239, 68, 68, 0.15);
    color: rgb(239, 68, 68);
  }

  .name-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .process-name {
    font-size: var(--font-size-base);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
    letter-spacing: -0.01em;
  }

  .process-framework {
    font-size: 10px;
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
    color: var(--text-tertiary);
    background: rgba(59, 130, 246, 0.08);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    width: fit-content;
  }

  .process-actions {
    display: flex;
    gap: 4px;
  }

  .action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 0;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .action-btn:hover {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
  }

  .action-btn.start-btn:hover {
    background: rgba(34, 197, 94, 0.1);
    border-color: rgb(34, 197, 94);
    color: rgb(34, 197, 94);
  }

  .action-btn.stop-btn:hover {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgb(239, 68, 68);
    color: rgb(239, 68, 68);
  }

  .action-btn.restart-btn:hover {
    background: rgba(59, 130, 246, 0.1);
    border-color: rgb(59, 130, 246);
    color: rgb(59, 130, 246);
  }

  .process-details {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: var(--space-md);
    padding: var(--space-sm) 0;
    border-top: 1px solid var(--border-color);
    border-bottom: 1px solid var(--border-color);
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-sm);
  }

  .detail-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .detail-value {
    font-size: 12px;
    color: var(--text-primary);
    text-align: right;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
  }

  .detail-value.status-running {
    color: rgb(34, 197, 94);
  }

  .detail-value.status-starting {
    color: rgb(251, 191, 36);
  }

  .detail-value.status-stopped {
    color: rgb(100, 116, 139);
  }

  .detail-value.status-crashed {
    color: rgb(239, 68, 68);
  }

  .process-stats {
    display: flex;
    gap: var(--space-sm);
    margin-bottom: var(--space-md);
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    flex: 1;
  }

  .stat-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    color: var(--accent-primary);
  }

  .stat-details {
    flex: 1;
    min-width: 0;
  }

  .stat-label {
    font-size: 9px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 2px;
  }

  .stat-value {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
  }

  .process-footer {
    display: flex;
    gap: 6px;
    margin-bottom: var(--space-sm);
  }

  .footer-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .footer-btn:hover {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .footer-btn.danger:hover {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgb(239, 68, 68);
    color: rgb(239, 68, 68);
  }

  .process-path {
    font-size: 10px;
    color: var(--text-tertiary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
  }
</style>
