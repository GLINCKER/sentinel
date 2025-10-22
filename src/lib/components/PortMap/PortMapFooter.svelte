<script lang="ts">
  import { Pause, Play } from 'lucide-svelte';

  interface Props {
    lastScan: Date | null;
    groupCount: number;
    connectionCount: number;
    totalPorts: number;
    isPaused: boolean;
    onTogglePause: () => void;
  }

  let {
    lastScan,
    groupCount,
    connectionCount,
    totalPorts,
    isPaused,
    onTogglePause
  }: Props = $props();

  // Calculate relative time
  let relativeTime = $state('');
  let updateInterval: number | null = null;

  function updateRelativeTime() {
    if (!lastScan) {
      relativeTime = '';
      return;
    }

    const now = new Date();
    const diff = Math.floor((now.getTime() - lastScan.getTime()) / 1000);

    if (diff < 60) {
      relativeTime = `${diff}s ago`;
    } else if (diff < 3600) {
      const minutes = Math.floor(diff / 60);
      relativeTime = `${minutes}m ago`;
    } else {
      const hours = Math.floor(diff / 3600);
      relativeTime = `${hours}h ago`;
    }
  }

  $effect(() => {
    updateRelativeTime();
    updateInterval = window.setInterval(updateRelativeTime, 1000);

    return () => {
      if (updateInterval) {
        clearInterval(updateInterval);
      }
    };
  });
</script>

<div class="footer">
  <div class="footer-info">
    {#if lastScan}
      <span class="footer-text">
        Last scan: <span class="mono">{lastScan.toLocaleTimeString()}</span>
        <span class="relative-time">({relativeTime})</span>
      </span>
    {/if}
    <span class="footer-text footer-stats">
      Showing <span class="mono">{groupCount}</span> groups (<span class="mono"
        >{connectionCount}</span
      >
      connections) of <span class="mono">{totalPorts}</span> total ports
    </span>
  </div>

  <div class="footer-controls">
    <button
      class="control-btn"
      class:paused={isPaused}
      onclick={onTogglePause}
      title={isPaused ? 'Resume auto-refresh' : 'Pause auto-refresh'}
    >
      {#if isPaused}
        <Play size={14} />
        <span>Resume</span>
      {:else}
        <Pause size={14} />
        <span>Pause</span>
      {/if}
    </button>
  </div>
</div>

<style>
  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1.5rem;
    border-top: 1px solid var(--border);
    background: var(--muted);
    gap: 1rem;
    flex-wrap: wrap;
  }

  .footer-info {
    display: flex;
    gap: 1.5rem;
    align-items: center;
    flex-wrap: wrap;
    flex: 1;
  }

  .footer-text {
    font-size: 0.8125rem;
    color: var(--muted-foreground);
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .footer-stats {
    font-weight: 500;
  }

  .mono {
    font-family:
      'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-weight: 600;
    color: var(--foreground);
    letter-spacing: -0.01em;
  }

  .relative-time {
    font-size: 0.75rem;
    color: var(--muted-foreground);
    opacity: 0.7;
  }

  .footer-controls {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .control-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.875rem;
    background: var(--background);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    color: var(--foreground);
    font-size: 0.8125rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .control-btn:hover {
    background: var(--accent);
    border-color: var(--foreground);
  }

  .control-btn.paused {
    background: rgba(245, 158, 11, 0.1);
    border-color: rgba(245, 158, 11, 0.3);
    color: #f59e0b;
  }

  .control-btn.paused:hover {
    background: rgba(245, 158, 11, 0.2);
    border-color: rgba(245, 158, 11, 0.5);
  }

  @media (max-width: 768px) {
    .footer {
      flex-direction: column;
      align-items: flex-start;
    }

    .footer-info {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }

    .footer-controls {
      width: 100%;
      justify-content: flex-end;
    }
  }
</style>
