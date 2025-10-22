<!--
  @file Glinr Process Card Component
  @glinr/sentinel-core

  Process information card component.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import type { ProcessInfo } from '../types';
  import GlinrButton from './GlinrButton.svelte';
  import { Play, RotateCw, Square } from 'lucide-svelte';

  interface Props {
    process: ProcessInfo;
    onAction?: (action: 'start' | 'stop' | 'restart') => void;
    onClick?: () => void;
    selected?: boolean;
    onSelect?: (selected: boolean) => void;
    selectionMode?: boolean;
  }

  let {
    process,
    onAction,
    onClick,
    selected = false,
    onSelect,
    selectionMode = false
  }: Props = $props();

  type ProcessState =
    | 'running'
    | 'stopped'
    | 'starting'
    | 'stopping'
    | { crashed: { exit_code: number } }
    | { failed: { reason: string } };

  function getStateColor(state: ProcessState): string {
    if (state === 'running') return 'success';
    if (state === 'stopped') return 'secondary';
    if (state === 'starting' || state === 'stopping') return 'info';
    if (typeof state === 'object' && ('crashed' in state || 'failed' in state))
      return 'error';
    return 'secondary';
  }

  function getStateLabel(state: ProcessState): string {
    if (state === 'running') return 'Running';
    if (state === 'stopped') return 'Stopped';
    if (state === 'starting') return 'Starting...';
    if (state === 'stopping') return 'Stopping...';
    if (typeof state === 'object' && 'crashed' in state) {
      return `Crashed (${state.crashed.exit_code})`;
    }
    if (typeof state === 'object' && 'failed' in state) {
      return `Failed: ${state.failed.reason}`;
    }
    return 'Unknown';
  }

  function formatMemory(bytes: number): string {
    const mb = bytes / (1024 * 1024);
    return mb >= 1024 ? `${(mb / 1024).toFixed(1)} GB` : `${mb.toFixed(1)} MB`;
  }

  function formatUptime(startedAt: string | null): string {
    if (!startedAt) return '-';

    const start = new Date(startedAt);
    const now = new Date();
    const diffMs = now.getTime() - start.getTime();

    const seconds = Math.floor(diffMs / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (days > 0) return `${days}d ${hours % 24}h`;
    if (hours > 0) return `${hours}h ${minutes % 60}m`;
    if (minutes > 0) return `${minutes}m ${seconds % 60}s`;
    return `${seconds}s`;
  }

  let isRunning = $derived(process.state === 'running');
  let isStopped = $derived(process.state === 'stopped');
</script>

<article
  class="glinr-card sentinel-process-card"
  class:selected
  role="button"
  tabindex="0"
  onclick={onClick}
  onkeydown={(e) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      onClick?.();
    }
  }}
>
  <!-- Selection Checkbox -->
  {#if selectionMode}
    <div class="process-checkbox-wrapper">
      <input
        type="checkbox"
        class="process-checkbox"
        checked={selected}
        onchange={(e) => {
          e.stopPropagation();
          onSelect?.(e.currentTarget.checked);
        }}
        onclick={(e) => e.stopPropagation()}
        aria-label="Select {process.name}"
      />
    </div>
  {/if}

  <!-- Header -->
  <header class="glinr-card-header">
    <h3 class="glinr-process-name">{process.name}</h3>
    <span
      class="glinr-status-badge glinr-status-{getStateColor(process.state)}"
    >
      {getStateLabel(process.state)}
    </span>
  </header>

  <!-- Stats -->
  <div class="glinr-process-stats">
    <div class="glinr-stat">
      <span class="glinr-stat-label">PID</span>
      <span class="glinr-stat-value">{process.pid ?? '-'}</span>
    </div>

    <div class="glinr-stat">
      <span class="glinr-stat-label">CPU</span>
      <span class="glinr-stat-value">{process.cpu_usage.toFixed(1)}%</span>
    </div>

    <div class="glinr-stat">
      <span class="glinr-stat-label">Memory</span>
      <span class="glinr-stat-value">{formatMemory(process.memory_usage)}</span>
    </div>

    <div class="glinr-stat">
      <span class="glinr-stat-label">Uptime</span>
      <span class="glinr-stat-value">{formatUptime(process.started_at)}</span>
    </div>
  </div>

  <!-- Actions -->
  <footer class="glinr-card-actions">
    {#if isStopped}
      <GlinrButton
        variant="primary"
        size="sm"
        onclick={(e) => {
          e.stopPropagation();
          onAction?.('start');
        }}
      >
        <Play size={14} />
        Start
      </GlinrButton>
    {:else if isRunning}
      <GlinrButton
        variant="secondary"
        size="sm"
        onclick={(e) => {
          e.stopPropagation();
          onAction?.('restart');
        }}
      >
        <RotateCw size={14} />
        Restart
      </GlinrButton>
      <GlinrButton
        variant="danger"
        size="sm"
        onclick={(e) => {
          e.stopPropagation();
          onAction?.('stop');
        }}
      >
        <Square size={14} />
        Stop
      </GlinrButton>
    {/if}
  </footer>
</article>

<style>
  .glinr-card {
    background: var(--glass-bg);
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
    transition: all var(--transition-base);
    cursor: pointer;
    box-shadow: var(--shadow-xs);
  }

  .glinr-card:hover {
    border-color: var(--accent-primary);
    box-shadow: var(--shadow-md);
    transform: translateY(-2px);
  }

  .glinr-card:focus-visible {
    outline: 2px solid var(--accent-primary);
    outline-offset: 2px;
  }

  .sentinel-process-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-lg);
    position: relative;
  }

  .sentinel-process-card.selected {
    border-color: var(--accent-primary);
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.05),
      rgba(59, 130, 246, 0.02)
    );
    box-shadow:
      0 0 0 2px rgba(59, 130, 246, 0.2),
      var(--shadow-md);
  }

  .process-checkbox-wrapper {
    position: absolute;
    top: 12px;
    left: 12px;
    z-index: 10;
  }

  .process-checkbox {
    width: 20px;
    height: 20px;
    cursor: pointer;
    accent-color: var(--accent-primary);
  }

  /* Header */
  .glinr-card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-md);
  }

  .glinr-process-name {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* Status Badge */
  .glinr-status-badge {
    padding: var(--space-xs) var(--space-md);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: 600;
    flex-shrink: 0;
  }

  .glinr-status-success {
    background: var(--success-bg);
    color: var(--success);
  }

  .glinr-status-error {
    background: var(--error-bg);
    color: var(--error);
  }

  .glinr-status-info {
    background: var(--info-bg);
    color: var(--info);
  }

  .glinr-status-secondary {
    background: var(--bg-tertiary);
    color: var(--text-secondary);
  }

  /* Stats */
  .glinr-process-stats {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-md);
  }

  .glinr-stat {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .glinr-stat-label {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .glinr-stat-value {
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
  }

  /* Actions */
  .glinr-card-actions {
    display: flex;
    gap: var(--space-sm);
    padding-top: var(--space-md);
    border-top: 1px solid var(--border-light);
  }

  .glinr-card-actions > :global(button) {
    flex: 1;
  }
</style>
