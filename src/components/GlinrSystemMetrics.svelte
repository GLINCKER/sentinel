<!--
  @file Glinr System Metrics Component
  @glinr/sentinel-core

  System resource monitoring component showing CPU, RAM, and Disk usage.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import type { SystemStats } from '../types';

  interface Props {
    stats: SystemStats | null;
  }

  let { stats }: Props = $props();

  function formatBytes(bytes: number): string {
    const gb = bytes / (1024 * 1024 * 1024);
    return gb >= 1
      ? `${gb.toFixed(1)} GB`
      : `${(bytes / (1024 * 1024)).toFixed(0)} MB`;
  }

  function formatUptime(seconds: number): string {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (days > 0) return `${days}d ${hours}h`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  }

  function getUsageColor(percentage: number): string {
    if (percentage >= 90) return 'error';
    if (percentage >= 75) return 'warning';
    return 'success';
  }

  let cpuPercentage = $derived(stats?.cpu_usage ?? 0);
  let memoryPercentage = $derived(
    stats ? (stats.memory_used / stats.memory_total) * 100 : 0
  );
  let diskPercentage = $derived(
    stats ? (stats.disk_used / stats.disk_total) * 100 : 0
  );
</script>

<div class="glinr-system-metrics">
  <!-- CPU Usage -->
  <div class="glinr-metric-card">
    <div class="glinr-metric-header">
      <span class="glinr-metric-icon" aria-hidden="true">🖥️</span>
      <h3 class="glinr-metric-title">CPU Usage</h3>
    </div>

    <div class="glinr-metric-value">
      <span class="glinr-metric-number">{cpuPercentage.toFixed(1)}</span>
      <span class="glinr-metric-unit">%</span>
    </div>

    <div
      class="glinr-progress-bar"
      role="progressbar"
      aria-valuenow={cpuPercentage}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-label="CPU usage percentage"
    >
      <div
        class="glinr-progress-fill glinr-progress-{getUsageColor(
          cpuPercentage
        )}"
        style="width: {cpuPercentage}%"
      ></div>
    </div>
  </div>

  <!-- Memory Usage -->
  <div class="glinr-metric-card">
    <div class="glinr-metric-header">
      <span class="glinr-metric-icon" aria-hidden="true">💾</span>
      <h3 class="glinr-metric-title">Memory Usage</h3>
    </div>

    <div class="glinr-metric-value">
      <span class="glinr-metric-number">{memoryPercentage.toFixed(1)}</span>
      <span class="glinr-metric-unit">%</span>
    </div>

    <div
      class="glinr-progress-bar"
      role="progressbar"
      aria-valuenow={memoryPercentage}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-label="Memory usage percentage"
    >
      <div
        class="glinr-progress-fill glinr-progress-{getUsageColor(
          memoryPercentage
        )}"
        style="width: {memoryPercentage}%"
      ></div>
    </div>

    <div class="glinr-metric-detail">
      {stats ? formatBytes(stats.memory_used) : '0'} / {stats
        ? formatBytes(stats.memory_total)
        : '0'}
    </div>
  </div>

  <!-- Disk Usage -->
  <div class="glinr-metric-card">
    <div class="glinr-metric-header">
      <span class="glinr-metric-icon" aria-hidden="true">💿</span>
      <h3 class="glinr-metric-title">Disk Usage</h3>
    </div>

    <div class="glinr-metric-value">
      <span class="glinr-metric-number">{diskPercentage.toFixed(1)}</span>
      <span class="glinr-metric-unit">%</span>
    </div>

    <div
      class="glinr-progress-bar"
      role="progressbar"
      aria-valuenow={diskPercentage}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-label="Disk usage percentage"
    >
      <div
        class="glinr-progress-fill glinr-progress-{getUsageColor(
          diskPercentage
        )}"
        style="width: {diskPercentage}%"
      ></div>
    </div>

    <div class="glinr-metric-detail">
      {stats ? formatBytes(stats.disk_used) : '0'} / {stats
        ? formatBytes(stats.disk_total)
        : '0'}
    </div>
  </div>

  <!-- System Uptime -->
  <div class="glinr-metric-card">
    <div class="glinr-metric-header">
      <span class="glinr-metric-icon" aria-hidden="true">⏱️</span>
      <h3 class="glinr-metric-title">System Uptime</h3>
    </div>

    <div class="glinr-metric-value">
      <span class="glinr-metric-number"
        >{stats ? formatUptime(stats.uptime) : '-'}</span
      >
    </div>
  </div>
</div>

<style>
  .glinr-system-metrics {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: var(--space-lg);
  }

  .glinr-metric-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
    transition: all var(--transition-fast);
  }

  .glinr-metric-card:hover {
    border-color: var(--accent-primary);
    box-shadow: var(--shadow-sm);
  }

  .glinr-metric-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    margin-bottom: var(--space-md);
  }

  .glinr-metric-icon {
    font-size: var(--font-size-xl);
    line-height: 1;
  }

  .glinr-metric-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-secondary);
    margin: 0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .glinr-metric-value {
    display: flex;
    align-items: baseline;
    gap: var(--space-xs);
    margin-bottom: var(--space-md);
  }

  .glinr-metric-number {
    font-size: var(--font-size-3xl);
    font-weight: 700;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
  }

  .glinr-metric-unit {
    font-size: var(--font-size-lg);
    color: var(--text-secondary);
    font-weight: 600;
  }

  /* Progress Bar */
  .glinr-progress-bar {
    height: 8px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-sm);
    overflow: hidden;
    position: relative;
  }

  .glinr-progress-fill {
    height: 100%;
    border-radius: var(--radius-sm);
    transition:
      width var(--transition-base),
      background var(--transition-fast);
  }

  .glinr-progress-success {
    background: var(--success);
  }

  .glinr-progress-warning {
    background: var(--warning);
  }

  .glinr-progress-error {
    background: var(--error);
  }

  .glinr-metric-detail {
    margin-top: var(--space-sm);
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    font-variant-numeric: tabular-nums;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .glinr-system-metrics {
      grid-template-columns: 1fr;
    }
  }

  @media (min-width: 769px) and (max-width: 1024px) {
    .glinr-system-metrics {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>
