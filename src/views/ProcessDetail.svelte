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
  import { processConfigStore } from '../stores/processConfig.svelte';
  import type { ProcessConfig } from '../stores/processConfig.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { toast } from 'svelte-sonner';
  import GlinrButton from '../components/GlinrButton.svelte';
  import GlinrTerminal from '../components/GlinrTerminal.svelte';

  // Find process by ID instead of name
  let processId = $derived($selectedProcess);
  let config = $derived(
    processConfigStore.configs.find((c) => c.id === processId)
  ) as ProcessConfig | undefined;
  let status = $derived(
    config ? processConfigStore.statuses.get(config.id) : undefined
  );

  let logs = $state<
    Array<{ timestamp: string; line: string; stream: 'stdout' | 'stderr' }>
  >([]);
  let autoScroll = $state(true);

  // Calculate uptime from started_at timestamp
  function formatUptime(startedAt: string | null | undefined): string {
    if (!startedAt) return 'N/A';

    try {
      const start = new Date(startedAt);
      const now = new Date();
      const diffMs = now.getTime() - start.getTime();

      if (diffMs < 0) return 'N/A';

      const seconds = Math.floor(diffMs / 1000);
      const minutes = Math.floor(seconds / 60);
      const hours = Math.floor(minutes / 60);
      const days = Math.floor(hours / 24);

      if (days > 0) return `${days}d ${hours % 24}h`;
      if (hours > 0) return `${hours}h ${minutes % 60}m`;
      if (minutes > 0) return `${minutes}m ${seconds % 60}s`;
      return `${seconds}s`;
    } catch {
      return 'N/A';
    }
  }

  async function fetchLogs() {
    if (!config?.id) return;

    try {
      const result = await invoke<
        Array<{ timestamp: string; stream: 'stdout' | 'stderr'; line: string }>
      >('get_managed_process_logs', {
        configId: config.id,
        count: 1000
      });

      logs = result.map((logLine) => ({
        timestamp: new Date(logLine.timestamp)
          .toISOString()
          .split('T')[1]
          .substring(0, 8),
        line: logLine.line,
        stream: logLine.stream
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
    if (!config?.id) return;
    try {
      await processConfigStore.stopProcess(config.id);
      toast.success('Process stopped');
    } catch (e) {
      console.error('Failed to stop process:', e);
      toast.error(`Failed to stop: ${e}`);
    }
  }

  async function handleRestart() {
    if (!config?.id) return;
    try {
      await processConfigStore.restartProcess(config.id);
      toast.success('Process restarted');
      await fetchLogs();
    } catch (e) {
      console.error('Failed to restart process:', e);
      toast.error(`Failed to restart: ${e}`);
    }
  }

  async function handleRemoveFromConfig() {
    if (!config) return;

    const confirmed = confirm(
      `Remove "${config.name}" from config?\n\nThis will:\n- Stop the process if running\n- Remove it from configuration\n- Prevent auto-start on next launch\n\nThis action cannot be undone.`
    );

    if (!confirmed) return;

    try {
      // Stop process first if running
      if (status?.running) {
        await processConfigStore.stopProcess(config.id);
      }

      // Delete config
      await processConfigStore.deleteConfig(config.id);

      toast.success(`Process "${config.name}" removed from config`, {
        description: 'It will not auto-start on next app launch'
      });

      // Navigate back to dashboard
      navigateBack();
    } catch (e) {
      console.error('Failed to remove process from config:', e);
      toast.error('Failed to remove process from config', {
        description: String(e)
      });
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
      <h1 class="glinr-detail-title">{config?.name ?? 'Process'}</h1>
      <span class="glinr-detail-subtitle">Process Details & Logs</span>
    </div>
  </header>

  <div class="glinr-detail-content">
    {#if !config}
      <div class="glinr-empty-state">
        <p>Process not found</p>
      </div>
    {:else}
      <div class="glinr-process-info">
        <div class="info-card">
          <h3>Status</h3>
          <p class="status-{status?.running ? 'running' : 'stopped'}">
            {status?.running ? 'running' : 'stopped'}
          </p>
        </div>

        <div class="info-card">
          <h3>PID</h3>
          <p>{status?.processId ?? 'N/A'}</p>
        </div>

        <div class="info-card">
          <h3>Uptime</h3>
          <p>{formatUptime(status?.startedAt)}</p>
        </div>

        <div class="info-card">
          <h3>Restarts</h3>
          <p>{status?.restartCount ?? 0}</p>
        </div>

        <div class="info-actions">
          <GlinrButton
            variant="primary"
            onclick={handleRestart}
            disabled={!status?.running}>Restart</GlinrButton
          >
          <GlinrButton
            variant="danger"
            onclick={handleStop}
            disabled={!status?.running}>Stop</GlinrButton
          >
          <GlinrButton variant="secondary" onclick={handleRemoveFromConfig}
            >Remove from Config</GlinrButton
          >
        </div>
      </div>

      <div class="glinr-logs-section">
        <h3>Process Logs</h3>
        <div class="terminal-wrapper">
          <GlinrTerminal {logs} bind:autoScroll processName={config.name} />
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
    flex-wrap: wrap;
  }

  .info-actions :global(button) {
    flex: 1 1 auto;
    min-width: 120px;
    white-space: nowrap;
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

  /* Responsive Design */
  @media (max-width: 1024px) {
    .glinr-process-info {
      grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    }
  }

  @media (max-width: 768px) {
    .glinr-detail-header {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--space-md);
      padding: var(--space-lg);
    }

    .glinr-detail-title {
      font-size: var(--font-size-2xl);
    }

    .glinr-detail-content {
      padding: var(--space-lg);
    }

    .glinr-process-info {
      grid-template-columns: repeat(2, 1fr);
      gap: var(--space-md);
    }

    .info-actions {
      width: 100%;
    }

    .info-actions :global(button) {
      flex: 1 1 100%;
      max-width: none;
    }
  }

  @media (max-width: 480px) {
    .glinr-detail-header {
      padding: var(--space-md);
    }

    .glinr-detail-title {
      font-size: var(--font-size-xl);
    }

    .glinr-detail-content {
      padding: var(--space-md);
      gap: var(--space-md);
    }

    .glinr-process-info {
      grid-template-columns: 1fr;
    }

    .info-card {
      padding: var(--space-md);
    }
  }
</style>
