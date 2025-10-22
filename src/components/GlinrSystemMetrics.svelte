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
  import {
    Cpu,
    MemoryStick,
    HardDrive,
    Activity,
    ArrowUpCircle,
    ArrowDownCircle
  } from 'lucide-svelte';
  import AnimatedNumber from './AnimatedNumber.svelte';
  import MiniSparkline from './MiniSparkline.svelte';
  import {
    cpuHistory,
    memoryHistory,
    diskReadHistory,
    diskWriteHistory
  } from '../stores/metricsHistory';

  interface Props {
    stats: SystemStats | null;
    uptime?: number;
    onCardClick?: (metric: 'cpu' | 'memory' | 'disk') => void;
  }

  let { stats, uptime = 0, onCardClick }: Props = $props();

  function formatBytes(bytes: number): string {
    const gb = bytes / (1024 * 1024 * 1024);
    return gb >= 1
      ? `${gb.toFixed(1)} GB`
      : `${(bytes / (1024 * 1024)).toFixed(0)} MB`;
  }

  function formatUptime(seconds: number): string {
    const years = Math.floor(seconds / 31536000);
    const months = Math.floor((seconds % 31536000) / 2592000);
    const weeks = Math.floor((seconds % 2592000) / 604800);
    const days = Math.floor((seconds % 604800) / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (years > 0) return `${years}y ${months}mo`;
    if (months > 0) return `${months}mo ${weeks}w`;
    if (weeks > 0) return `${weeks}w ${days}d`;
    if (days > 0) return `${days}d ${hours}h`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  }

  function formatDiskRate(bytesPerSec: number): string {
    const mbPerSec = bytesPerSec / (1024 * 1024);
    if (mbPerSec >= 1000) {
      return `${(mbPerSec / 1024).toFixed(1)} GB/s`;
    } else if (mbPerSec >= 1) {
      return `${mbPerSec.toFixed(1)} MB/s`;
    } else {
      return `${(bytesPerSec / 1024).toFixed(0)} KB/s`;
    }
  }

  function getUsageColor(percentage: number): string {
    if (percentage >= 90) return 'error';
    if (percentage >= 75) return 'warning';
    return 'success';
  }

  let cpuPercentage = $derived(stats?.cpu.overall ?? 0);
  let memoryPercentage = $derived(stats?.memory.usage_percent ?? 0);
  let diskPercentage = $derived(
    stats && stats.disk.total_space > 0
      ? ((stats.disk.total_space - stats.disk.available_space) /
          stats.disk.total_space) *
          100
      : 0
  );
</script>

<div class="glinr-system-metrics">
  <!-- CPU Usage -->
  <button
    class="glinr-metric-card"
    onclick={() => onCardClick?.('cpu')}
    type="button"
  >
    <div class="glinr-metric-header">
      <Cpu size={20} class="glinr-metric-icon" />
      <h3 class="glinr-metric-title">CPU USAGE</h3>
      <span class="glinr-metric-uptime">
        {formatUptime(uptime)}
      </span>
    </div>

    <div class="glinr-metric-value">
      <AnimatedNumber
        value={cpuPercentage}
        precision={1}
        class="glinr-metric-number"
      />
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

    <div class="glinr-metric-detail">
      {stats ? stats.cpu.core_count : '0'} Cores
    </div>

    <!-- Embedded sparkline with glassmorphic overlay -->
    <div class="glinr-metric-chart-overlay">
      <MiniSparkline data={$cpuHistory} color="#3b82f6" height={40} />
    </div>
  </button>

  <!-- Memory Usage -->
  <button
    class="glinr-metric-card"
    onclick={() => onCardClick?.('memory')}
    type="button"
  >
    <div class="glinr-metric-header">
      <MemoryStick size={20} class="glinr-metric-icon" />
      <h3 class="glinr-metric-title">MEMORY USAGE</h3>
    </div>

    <div class="glinr-metric-value">
      <AnimatedNumber
        value={memoryPercentage}
        precision={1}
        class="glinr-metric-number"
      />
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
      {stats ? formatBytes(stats.memory.used) : '0'} / {stats
        ? formatBytes(stats.memory.total)
        : '0'}
    </div>

    <!-- Embedded sparkline with glassmorphic overlay -->
    <div class="glinr-metric-chart-overlay">
      <MiniSparkline data={$memoryHistory} color="#8b5cf6" height={40} />
    </div>
  </button>

  <!-- Disk Usage -->
  <button
    class="glinr-metric-card"
    onclick={() => onCardClick?.('disk')}
    type="button"
  >
    <div class="glinr-metric-header">
      <HardDrive size={20} class="glinr-metric-icon" />
      <h3 class="glinr-metric-title">DISK USAGE</h3>
    </div>

    <div class="glinr-metric-value">
      <AnimatedNumber
        value={diskPercentage}
        precision={1}
        class="glinr-metric-number"
      />
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
      {stats
        ? formatBytes(stats.disk.total_space - stats.disk.available_space)
        : '0'} / {stats ? formatBytes(stats.disk.total_space) : '0'}
    </div>

    <!-- Embedded sparkline - using memory data as placeholder for disk I/O -->
    <div class="glinr-metric-chart-overlay">
      <MiniSparkline data={$memoryHistory} color="#10b981" height={40} />
    </div>
  </button>

  <!-- Disk I/O -->
  <button
    class="glinr-metric-card"
    onclick={() => onCardClick?.('disk')}
    type="button"
  >
    <div class="glinr-metric-header">
      <Activity size={20} class="glinr-metric-icon glinr-icon-disk-io" />
      <h3 class="glinr-metric-title">DISK I/O</h3>
    </div>

    <div class="glinr-disk-io-values">
      <div class="glinr-io-metric">
        <ArrowDownCircle size={16} class="glinr-io-icon glinr-io-read" />
        <div class="glinr-io-content">
          <span class="glinr-io-label-small">READ</span>
          <span class="glinr-io-value-large">
            {stats ? formatDiskRate(stats.disk.read_bytes_per_sec) : '0 KB/s'}
          </span>
        </div>
      </div>
      <div class="glinr-io-metric">
        <ArrowUpCircle size={16} class="glinr-io-icon glinr-io-write" />
        <div class="glinr-io-content">
          <span class="glinr-io-label-small">WRITE</span>
          <span class="glinr-io-value-large">
            {stats ? formatDiskRate(stats.disk.write_bytes_per_sec) : '0 KB/s'}
          </span>
        </div>
      </div>
    </div>

    <!-- Embedded dual sparkline for read/write -->
    <div class="glinr-metric-chart-overlay glinr-disk-io-chart">
      <div class="glinr-io-chart-item">
        <MiniSparkline data={$diskReadHistory} color="#3b82f6" height={18} />
      </div>
      <div class="glinr-io-chart-item">
        <MiniSparkline data={$diskWriteHistory} color="#f59e0b" height={18} />
      </div>
    </div>
  </button>
</div>

<style>
  .glinr-system-metrics {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: var(--space-lg);
  }

  .glinr-metric-card {
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-xl);
    padding: 20px 20px 12px 20px;
    transition: all var(--transition-base);
    box-shadow:
      0 4px 16px rgba(0, 0, 0, 0.04),
      0 2px 4px rgba(0, 0, 0, 0.02);
    position: relative;
    overflow: hidden;
    cursor: pointer;
    text-align: left;
    width: 100%;
  }

  .glinr-metric-card:hover {
    transform: translateY(-2px);
    box-shadow:
      0 8px 24px rgba(0, 0, 0, 0.08),
      0 4px 8px rgba(0, 0, 0, 0.04);
    border-color: var(--accent-primary);
  }

  .glinr-metric-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 3px;
    background: linear-gradient(
      90deg,
      var(--accent-primary),
      var(--accent-secondary)
    );
    opacity: 0;
    transition: opacity var(--transition-base);
  }

  .glinr-metric-card:hover {
    border-color: rgba(59, 130, 246, 0.3);
    box-shadow:
      0 8px 24px rgba(0, 0, 0, 0.08),
      0 4px 8px rgba(59, 130, 246, 0.12);
    transform: translateY(-4px);
  }

  .glinr-metric-card:hover::before {
    opacity: 1;
  }

  .glinr-metric-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    margin-bottom: 12px;
    min-height: 28px;
  }

  :global(.glinr-metric-icon) {
    color: var(--accent-primary);
    flex-shrink: 0;
  }

  .glinr-metric-title {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--text-secondary);
    margin: 0;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    flex: 1;
  }

  .glinr-metric-uptime {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--accent-primary);
    background: rgba(59, 130, 246, 0.08);
    padding: 4px 10px;
    border-radius: var(--radius-md);
    font-variant-numeric: tabular-nums;
    letter-spacing: 0.3px;
  }

  .glinr-metric-value {
    display: flex;
    align-items: baseline;
    gap: var(--space-xs);
    margin-bottom: 10px;
  }

  .glinr-metric-number {
    font-size: 2.25rem; /* 36px - Large and bold like modern dashboards */
    font-weight: 700;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
    line-height: 1;
  }

  .glinr-metric-unit {
    font-size: 1.125rem; /* 18px - Proportional to number size */
    color: var(--text-secondary);
    font-weight: 600;
  }

  /* Progress Bar - Modern Gradient */
  .glinr-progress-bar {
    height: 12px;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: var(--radius-full);
    overflow: hidden;
    position: relative;
    margin-bottom: 0;
  }

  @media (prefers-color-scheme: light) {
    .glinr-progress-bar {
      background: linear-gradient(
        to right,
        rgba(0, 0, 0, 0.05),
        rgba(0, 0, 0, 0.02)
      );
      border: none;
      box-shadow: inset 0 2px 6px rgba(0, 0, 0, 0.08);
    }
  }

  .glinr-progress-fill {
    height: 100%;
    border-radius: var(--radius-full);
    transition: width 0.8s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
  }

  /* Shimmer animation */
  .glinr-progress-fill::after {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.4),
      transparent
    );
    animation: shimmer 3s ease-in-out infinite;
  }

  @keyframes shimmer {
    0% {
      left: -100%;
    }
    50% {
      left: 100%;
    }
    100% {
      left: 100%;
    }
  }

  .glinr-progress-success {
    background: linear-gradient(135deg, #10b981 0%, #34d399 50%, #6ee7b7 100%);
    box-shadow:
      0 4px 12px rgba(16, 185, 129, 0.4),
      inset 0 1px 0 rgba(255, 255, 255, 0.3);
  }

  .glinr-progress-warning {
    background: linear-gradient(135deg, #f59e0b 0%, #fbbf24 50%, #fcd34d 100%);
    box-shadow:
      0 4px 12px rgba(245, 158, 11, 0.4),
      inset 0 1px 0 rgba(255, 255, 255, 0.3);
  }

  .glinr-progress-error {
    background: linear-gradient(135deg, #ef4444 0%, #f87171 50%, #fca5a5 100%);
    box-shadow:
      0 4px 12px rgba(239, 68, 68, 0.4),
      inset 0 1px 0 rgba(255, 255, 255, 0.3);
  }

  .glinr-metric-detail {
    margin-top: 8px;
    margin-bottom: 12px;
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    font-variant-numeric: tabular-nums;
  }

  /* Disk I/O Styles */
  .glinr-disk-io-values {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 12px;
  }

  .glinr-io-metric {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .glinr-io-content {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  :global(.glinr-io-icon) {
    flex-shrink: 0;
  }

  :global(.glinr-io-read) {
    color: #3b82f6;
  }

  :global(.glinr-io-write) {
    color: #f59e0b;
  }

  :global(.glinr-icon-disk-io) {
    color: #8b5cf6;
  }

  .glinr-io-label-small {
    font-size: 9px;
    color: var(--text-tertiary);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    line-height: 1;
  }

  .glinr-io-value-large {
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
    line-height: 1.1;
  }

  .glinr-disk-io-chart {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 8px 0 0 0;
  }

  .glinr-io-chart-item {
    height: 18px;
    display: flex;
    align-items: center;
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

  /* Chart Overlay - Positioned Below Content */
  .glinr-metric-chart-overlay {
    margin-top: 8px;
    margin-left: -20px;
    margin-right: -20px;
    margin-bottom: -12px;
    height: 48px;
    padding: 6px 0 0 0;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.015), transparent);
    backdrop-filter: blur(1px);
    -webkit-backdrop-filter: blur(1px);
    pointer-events: none;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    opacity: 0.85;
    transition: opacity var(--transition-fast);
    border-bottom-left-radius: var(--radius-xl);
    border-bottom-right-radius: var(--radius-xl);
  }

  .glinr-metric-chart-overlay :global(svg) {
    width: 100%;
    height: 40px;
  }

  .glinr-metric-card:hover .glinr-metric-chart-overlay {
    opacity: 1;
  }
</style>
