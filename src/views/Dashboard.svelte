<!--
  @file Dashboard View
  @glinr/sentinel

  Main dashboard showing process list and system metrics.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    processes,
    systemStats,
    startPolling,
    startProcess,
    stopProcess,
    restartProcess
  } from '../stores/processes';
  import { navigateToProcess } from '../stores/navigation';
  import type { ProcessInfo } from '../types';
  import GlinrProcessCard from '../components/GlinrProcessCard.svelte';
  import GlinrSystemMetrics from '../components/GlinrSystemMetrics.svelte';
  import GlinrButton from '../components/GlinrButton.svelte';

  let stopPolling: (() => void) | null = null;
  let selectedProcesses = $state<Set<string>>(new Set());
  let isPerformingAction = $state(false);

  onMount(() => {
    stopPolling = startPolling(2000); // Poll every 2 seconds
  });

  onDestroy(() => {
    if (stopPolling) {
      stopPolling();
    }
  });

  function getProcessStateClass(state: any): string {
    if (state === 'running') return 'running';
    if (state === 'stopped') return 'stopped';
    if (typeof state === 'object' && 'crashed' in state) return 'crashed';
    if (typeof state === 'object' && 'failed' in state) return 'failed';
    return 'unknown';
  }

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
  <!-- Header -->
  <header class="glinr-dashboard-header">
    <div class="glinr-header-content">
      <h1 class="glinr-dashboard-title">Dashboard</h1>
      <p class="glinr-dashboard-subtitle">
        Monitor and manage your development processes
      </p>
    </div>

    <div class="glinr-header-actions">
      <GlinrButton
        variant="secondary"
        disabled={isPerformingAction}
        onclick={handleStartAll}
        aria-label="Start all stopped processes"
      >
        ▶️ Start All
      </GlinrButton>

      <GlinrButton
        variant="secondary"
        disabled={isPerformingAction}
        onclick={handleStopAll}
        aria-label="Stop all running processes"
      >
        ⏹️ Stop All
      </GlinrButton>
    </div>
  </header>

  <!-- Main Content -->
  <div class="glinr-dashboard-content">
    <!-- System Metrics -->
    <section class="glinr-metrics-section" aria-label="System metrics">
      <h2 class="glinr-section-title">System Metrics</h2>
      <GlinrSystemMetrics stats={$systemStats} />
    </section>

    <!-- Process List -->
    <section class="glinr-processes-section" aria-label="Process list">
      <div class="glinr-section-header">
        <h2 class="glinr-section-title">Processes</h2>
        <span class="glinr-process-count">{$processes.length} total</span>
      </div>

      {#if $processes.length === 0}
        <div class="glinr-empty-state">
          <p class="glinr-empty-icon">📦</p>
          <h3 class="glinr-empty-title">No Processes Configured</h3>
          <p class="glinr-empty-text">
            Add processes to your configuration file to get started.
          </p>
        </div>
      {:else}
        <div class="glinr-process-grid">
          {#each $processes as process (process.name)}
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

  /* Header */
  .glinr-dashboard-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-xl) var(--space-2xl);
    border-bottom: 1px solid var(--border-light);
    background: var(--bg-primary);
    flex-shrink: 0;
  }

  .glinr-header-content {
    flex: 1;
  }

  .glinr-dashboard-title {
    font-size: var(--font-size-3xl);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 var(--space-xs) 0;
  }

  .glinr-dashboard-subtitle {
    font-size: var(--font-size-base);
    color: var(--text-secondary);
    margin: 0;
  }

  .glinr-header-actions {
    display: flex;
    gap: var(--space-md);
  }

  /* Main Content */
  .glinr-dashboard-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: var(--space-2xl);
  }

  .glinr-metrics-section {
    margin-bottom: var(--space-2xl);
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

  .glinr-section-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--space-lg) 0;
  }

  .glinr-process-count {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    background: var(--bg-secondary);
    padding: var(--space-xs) var(--space-md);
    border-radius: var(--radius-md);
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
    padding: var(--space-2xl);
    background: var(--bg-secondary);
    border-radius: var(--radius-lg);
    border: 2px dashed var(--border-color);
  }

  .glinr-empty-icon {
    font-size: 4rem;
    margin: 0 0 var(--space-lg) 0;
  }

  .glinr-empty-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--space-sm) 0;
  }

  .glinr-empty-text {
    font-size: var(--font-size-base);
    color: var(--text-secondary);
    margin: 0;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .glinr-dashboard-header {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--space-lg);
    }

    .glinr-header-actions {
      width: 100%;
      justify-content: stretch;
    }

    .glinr-header-actions > :global(button) {
      flex: 1;
    }

    .glinr-process-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
