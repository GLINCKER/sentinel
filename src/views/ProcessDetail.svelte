<!--
  @file Process Detail View
  @glinr/sentinel

  Detailed view for a single process showing logs and resource graphs.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { selectedProcess, navigateBack } from '../stores/navigation';
  import { processes } from '../stores/processes';
  import GlinrButton from '../components/GlinrButton.svelte';

  let process = $derived($processes.find((p) => p.name === $selectedProcess));
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
      <!-- TODO: Implement logs viewer and resource graphs -->
      <div class="glinr-placeholder">
        <h2>Coming Soon</h2>
        <p>Process logs and resource graphs will be implemented here</p>
        <ul>
          <li>Real-time log viewer (last 1000 lines)</li>
          <li>CPU/Memory usage graphs</li>
          <li>Process actions (restart, stop, edit config)</li>
        </ul>
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
  }

  .glinr-placeholder {
    background: var(--bg-secondary);
    border: 2px dashed var(--border-color);
    border-radius: var(--radius-lg);
    padding: var(--space-2xl);
    text-align: center;
  }

  .glinr-placeholder h2 {
    color: var(--text-primary);
    margin: 0 0 var(--space-md) 0;
  }

  .glinr-placeholder p {
    color: var(--text-secondary);
    margin: 0 0 var(--space-lg) 0;
  }

  .glinr-placeholder ul {
    text-align: left;
    display: inline-block;
    color: var(--text-tertiary);
  }
</style>
