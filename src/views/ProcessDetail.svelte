<!--
  @file Process Detail View
  @glinr/sentinel

  Detailed view for a single process showing logs and resource graphs.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { selectedProcess, navigateBack } from '../stores/navigation';
  import { processes, stopProcess, restartProcess } from '../stores/processes';
  import { invoke } from '@tauri-apps/api/core';
  import GlinrButton from '../components/GlinrButton.svelte';
  import GlinrTerminal from '../components/GlinrTerminal.svelte';

  let process = $derived($processes.find((p) => p.name === $selectedProcess));
  let logs = $state<
    Array<{ timestamp: string; line: string; stream: 'stdout' | 'stderr' }>
  >([]);
  let autoScroll = $state(true);

  async function fetchLogs() {
    if (!$selectedProcess) return;

    try {
      const result = await invoke<string[]>('get_recent_process_logs', {
        name: $selectedProcess,
        lines: 1000
      });

      logs = result.map((line) => ({
        timestamp: new Date().toISOString().split('T')[1].substring(0, 8),
        line,
        stream: 'stdout' as const
      }));
    } catch (e) {
      console.error('Failed to fetch logs:', e);
      logs = [
        {
          timestamp: new Date().toISOString().split('T')[1].substring(0, 8),
          line: `Error fetching logs: ${e}`,
          stream: 'stderr' as const
        }
      ];
    }
  }

  async function handleStop() {
    if (!$selectedProcess) return;
    try {
      await stopProcess($selectedProcess);
    } catch (e) {
      console.error('Failed to stop process:', e);
    }
  }

  async function handleRestart() {
    if (!$selectedProcess) return;
    try {
      await restartProcess($selectedProcess);
      await fetchLogs();
    } catch (e) {
      console.error('Failed to restart process:', e);
    }
  }

  let pollInterval: number;

  onMount(() => {
    fetchLogs();
    pollInterval = setInterval(fetchLogs, 2000) as unknown as number;
  });

  onDestroy(() => {
    clearInterval(pollInterval);
  });
</script>

<div class="glinr-process-detail">
  <header class="glinr-detail-header">
    <GlinrButton variant="ghost" onclick={navigateBack}>← Back</GlinrButton>

    <div class="glinr-detail-title-group">
      <h1 class="glinr-detail-title">{process?.name ?? 'Process'}</h1>
      <span class="glinr-detail-subtitle">Process Details & Logs</span>
    </div>
  </header>

  <div class="glinr-detail-content">
    {#if !process}
      <div class="glinr-empty-state">
        <p>Process not found</p>
      </div>
    {:else}
      <div class="glinr-process-info">
        <div class="info-card">
          <h3>Status</h3>
          <p
            class="status-{typeof process.state === 'string'
              ? process.state
              : 'crashed'}"
          >
            {typeof process.state === 'string' ? process.state : 'crashed'}
          </p>
        </div>

        <div class="info-card">
          <h3>PID</h3>
          <p>{process.pid ?? 'N/A'}</p>
        </div>

        <div class="info-card">
          <h3>Uptime</h3>
          <p>{process.uptime ?? 'N/A'}</p>
        </div>

        <div class="info-card">
          <h3>Restarts</h3>
          <p>{process.restart_count ?? 0}</p>
        </div>

        <div class="info-actions">
          <GlinrButton variant="primary" onclick={handleRestart}
            >Restart</GlinrButton
          >
          <GlinrButton variant="danger" onclick={handleStop}>Stop</GlinrButton>
        </div>
      </div>

      <div class="glinr-logs-section">
        <h3>Process Logs</h3>
        <div class="terminal-wrapper">
          <GlinrTerminal {logs} bind:autoScroll processName={process.name} />
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .glinr-process-detail {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .glinr-detail-header {
    display: flex;
    align-items: center;
    gap: var(--space-lg);
    padding: var(--space-xl) var(--space-2xl);
    border-bottom: 1px solid var(--border-light);
    flex-shrink: 0;
  }

  .glinr-detail-title-group {
    flex: 1;
  }

  .glinr-detail-title {
    font-size: var(--font-size-3xl);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 var(--space-xs) 0;
  }

  .glinr-detail-subtitle {
    font-size: var(--font-size-base);
    color: var(--text-secondary);
  }

  .glinr-detail-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-2xl);
    display: flex;
    flex-direction: column;
    gap: var(--space-xl);
  }

  .glinr-process-info {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: var(--space-lg);
  }

  .info-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
  }

  .info-card h3 {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin: 0 0 var(--space-sm) 0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .info-card p {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .info-card .status-running {
    color: var(--success-color);
  }

  .info-card .status-stopped {
    color: var(--text-tertiary);
  }

  .info-card .status-crashed {
    color: var(--danger-color);
  }

  .info-actions {
    display: flex;
    gap: var(--space-sm);
    align-items: center;
  }

  .glinr-logs-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 400px;
  }

  .glinr-logs-section h3 {
    font-size: var(--font-size-lg);
    color: var(--text-primary);
    margin: 0 0 var(--space-md) 0;
  }

  .terminal-wrapper {
    flex: 1;
    min-height: 0;
  }

  .glinr-empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-tertiary);
  }
</style>
