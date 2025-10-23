<!--
  @file Dashboard View
  @glinr/sentinel

  Main dashboard showing process list and system metrics.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import {
    processes,
    systemStats,
    startPolling,
    startProcess,
    stopProcess,
    restartProcess
  } from '../stores/processes';
  import { navigateToProcess } from '../stores/navigation';
  import { settings } from '../stores/settings';
  import { metricsHistory } from '../stores/metricsHistory';
  import GlinrProcessCard from '../components/GlinrProcessCard.svelte';
  import GlinrSystemMetrics from '../components/GlinrSystemMetrics.svelte';
  import PollingControl from '../components/PollingControl.svelte';
  import ProcessSearch from '../components/ProcessSearch.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import { Play, Square, Package, LayoutDashboard } from 'lucide-svelte';

  let stopPolling: (() => void) | null = null;
  let isPerformingAction = $state(false);
  let systemUptime = $state<number>(0);
  let searchQuery = $state('');

  // Fetch system uptime on mount and periodically
  onMount(() => {
    const fetchUptime = async () => {
      try {
        const info = (await invoke('get_system_info')) as { uptime: number };
        systemUptime = info.uptime;
      } catch (err) {
        console.error('Failed to fetch system uptime:', err);
      }
    };

    fetchUptime();
    const uptimeInterval = setInterval(fetchUptime, 60000); // Update every minute

    return () => clearInterval(uptimeInterval);
  });

  // Add system stats to metrics history whenever it updates
  $effect(() => {
    if ($systemStats) {
      metricsHistory.addDataPoint($systemStats);
    }
  });

  // Restart polling when interval changes
  $effect(() => {
    if (stopPolling) {
      stopPolling();
    }
    stopPolling = startPolling($settings.pollingInterval);

    return () => {
      if (stopPolling) {
        stopPolling();
      }
    };
  });

  // Filter processes based on search query
  let filteredProcesses = $derived(
    searchQuery.trim()
      ? $processes.filter((p) =>
          p.name.toLowerCase().includes(searchQuery.toLowerCase())
        )
      : $processes
  );

  async function handleStartAll() {
    isPerformingAction = true;
    try {
      for (const process of $processes) {
        if (process.state === 'stopped') {
          await startProcess(process.name);
        }
      }
    } finally {
      isPerformingAction = false;
    }
  }

  async function handleStopAll() {
    isPerformingAction = true;
    try {
      for (const process of $processes) {
        if (process.state === 'running') {
          await stopProcess(process.name);
        }
      }
    } finally {
      isPerformingAction = false;
    }
  }

  async function handleProcessAction(
    processName: string,
    action: 'start' | 'stop' | 'restart'
  ) {
    isPerformingAction = true;
    try {
      if (action === 'start') {
        await startProcess(processName);
      } else if (action === 'stop') {
        await stopProcess(processName);
      } else if (action === 'restart') {
        await restartProcess(processName);
      }
    } catch (error) {
      console.error(`Failed to ${action} process:`, error);
      // TODO: Show error notification
    } finally {
      isPerformingAction = false;
    }
  }

  function handleProcessClick(processName: string) {
    navigateToProcess(processName);
  }
</script>

<div class="glinr-dashboard">
  <PageHeader
    title="Dashboard"
    subtitle="Monitor and manage your development processes"
    icon={LayoutDashboard}
  >
    <button
      class="glinr-action-btn glinr-action-btn-start"
      disabled={isPerformingAction}
      onclick={handleStartAll}
      title="Start all stopped processes"
    >
      <Play size={14} />
      <span>Start All</span>
    </button>

    <button
      class="glinr-action-btn glinr-action-btn-stop"
      disabled={isPerformingAction}
      onclick={handleStopAll}
      title="Stop all running processes"
    >
      <Square size={14} />
      <span>Stop All</span>
    </button>
  </PageHeader>

  <!-- Main Content -->
  <div class="glinr-dashboard-content">
    <!-- System Metrics -->
    <section class="glinr-metrics-section" aria-label="System metrics">
      <div class="glinr-section-header">
        <h2 class="glinr-section-title">System Metrics</h2>
        <PollingControl />
      </div>
      <GlinrSystemMetrics stats={$systemStats} uptime={systemUptime} />
    </section>

    <!-- Process List -->
    <section class="glinr-processes-section" aria-label="Process list">
      <div class="glinr-section-header">
        <h2 class="glinr-section-title">Processes</h2>
        <span class="glinr-process-count"
          >{filteredProcesses.length} of {$processes.length}</span
        >
      </div>

      <div class="glinr-process-controls">
        <ProcessSearch bind:value={searchQuery} />
      </div>

      {#if $processes.length === 0}
        <div class="glinr-empty-state">
          <Package size={48} class="glinr-empty-icon" />
          <h3 class="glinr-empty-title">No Processes Configured</h3>
          <p class="glinr-empty-text">
            Add processes to your configuration file to get started.
          </p>
        </div>
      {:else if filteredProcesses.length === 0}
        <div class="glinr-empty-state">
          <Package size={48} class="glinr-empty-icon" />
          <h3 class="glinr-empty-title">No Processes Found</h3>
          <p class="glinr-empty-text">
            No processes match "{searchQuery}"
          </p>
        </div>
      {:else}
        <div class="glinr-process-grid">
          {#each filteredProcesses as process (process.name)}
            <GlinrProcessCard
              {process}
              onAction={(action) => handleProcessAction(process.name, action)}
              onClick={() => handleProcessClick(process.name)}
            />
          {/each}
        </div>
      {/if}
    </section>
  </div>
</div>

<style>
  .glinr-dashboard {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    background: var(--bg-primary);
  }

  /* Action Buttons */
  .glinr-action-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    font-size: var(--font-size-xs);
    font-weight: 600;
    font-family: inherit;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .glinr-action-btn:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
    transform: translateY(-1px);
  }

  .glinr-action-btn:active:not(:disabled) {
    transform: translateY(0);
  }

  .glinr-action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .glinr-action-btn-start:hover:not(:disabled) {
    background: var(--success-bg);
    border-color: var(--success);
    color: var(--success);
  }

  .glinr-action-btn-stop:hover:not(:disabled) {
    background: var(--error-bg);
    border-color: var(--error);
    color: var(--error);
  }

  /* Main Content */
  .glinr-dashboard-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: var(--space-2xl);
  }

  .glinr-metrics-section {
    margin-bottom: var(--space-3xl);
  }

  .glinr-metrics-charts {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
    gap: var(--space-lg);
    margin-top: var(--space-lg);
  }

  .glinr-processes-section {
    margin-bottom: var(--space-2xl);
  }

  .glinr-section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-lg);
  }

  .glinr-process-controls {
    margin-bottom: var(--space-xl);
  }

  .glinr-section-title {
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
    letter-spacing: -0.02em;
  }

  .glinr-process-count {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--accent-primary);
    background: rgba(59, 130, 246, 0.08);
    padding: 6px 12px;
    border-radius: var(--radius-full);
    border: 1px solid rgba(59, 130, 246, 0.2);
  }

  /* Process Grid */
  .glinr-process-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: var(--space-lg);
  }

  /* Empty State */
  .glinr-empty-state {
    text-align: center;
    padding: var(--space-4xl) var(--space-2xl);
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border-radius: var(--radius-xl);
    border: 2px dashed var(--border-color);
    transition: all var(--transition-base);
  }

  .glinr-empty-state:hover {
    border-color: var(--accent-primary);
    background: rgba(59, 130, 246, 0.02);
  }

  :global(.glinr-empty-icon) {
    color: var(--text-tertiary);
    margin-bottom: var(--space-xl);
    opacity: 0.6;
    transition: all var(--transition-base);
  }

  .glinr-empty-state:hover :global(.glinr-empty-icon) {
    color: var(--accent-primary);
    opacity: 1;
    transform: scale(1.05);
  }

  .glinr-empty-title {
    font-size: var(--font-size-2xl);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 var(--space-md) 0;
    letter-spacing: -0.02em;
  }

  .glinr-empty-text {
    font-size: var(--font-size-base);
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.6;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .glinr-process-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
