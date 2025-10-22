<!--
  @file Polling Control Component
  @glinr/sentinel

  Modern polling interval control with pause/play functionality

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.
-->

<script lang="ts">
  import { settings, POLLING_INTERVALS } from '../stores/settings';
  import { RefreshCw, Pause, Play } from 'lucide-svelte';

  let isPaused = $state(false);
  let showDropdown = $state(false);

  function togglePause() {
    isPaused = !isPaused;
    if (isPaused) {
      // Set to a very high interval (effectively paused)
      settings.update((s) => ({ ...s, pollingInterval: 999999999 }));
    } else {
      // Restore to 2 seconds default or last selected
      const lastInterval = parseInt(
        localStorage.getItem('last-poll-interval') || '2000'
      );
      settings.update((s) => ({ ...s, pollingInterval: lastInterval }));
    }
  }

  function selectInterval(interval: number) {
    localStorage.setItem('last-poll-interval', interval.toString());
    isPaused = false;
    settings.update((s) => ({ ...s, pollingInterval: interval }));
    showDropdown = false;
  }

  const currentLabel = $derived(() => {
    if (isPaused) return 'Paused';
    const interval = POLLING_INTERVALS.find(
      (i) => i.value === $settings.pollingInterval
    );
    return interval?.label || '2 seconds';
  });
</script>

<div class="polling-control">
  <!-- Pause/Play Button -->
  <button
    class="control-btn pause-btn"
    class:paused={isPaused}
    onclick={togglePause}
    title={isPaused ? 'Resume polling' : 'Pause polling'}
  >
    {#if isPaused}
      <Play size={14} />
    {:else}
      <Pause size={14} />
    {/if}
  </button>

  <!-- Interval Selector -->
  <div class="interval-selector">
    <button
      class="selector-btn"
      onclick={() => (showDropdown = !showDropdown)}
      class:active={showDropdown}
    >
      <RefreshCw
        size={14}
        class={isPaused ? 'refresh-icon' : 'refresh-icon spinning'}
      />
      <span class="interval-label">{currentLabel()}</span>
      <svg
        class="chevron"
        width="12"
        height="12"
        viewBox="0 0 12 12"
        fill="none"
      >
        <path
          d="M3 4.5L6 7.5L9 4.5"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    </button>

    {#if showDropdown}
      <div class="dropdown">
        {#each POLLING_INTERVALS as interval (interval.value)}
          <button
            class="dropdown-item"
            class:selected={$settings.pollingInterval === interval.value &&
              !isPaused}
            onclick={() => selectInterval(interval.value)}
          >
            <span>{interval.label}</span>
            {#if $settings.pollingInterval === interval.value && !isPaused}
              <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                <path
                  d="M11.667 3.5L5.25 9.917L2.333 7"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
              </svg>
            {/if}
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Click outside to close -->
{#if showDropdown}
  <button class="backdrop" onclick={() => (showDropdown = false)}></button>
{/if}

<style>
  .polling-control {
    display: flex;
    align-items: center;
    gap: 4px;
    position: relative;
    z-index: 50;
  }

  .control-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    padding: 0;
    background: var(--glass-bg);
    backdrop-filter: blur(12px) saturate(180%);
    -webkit-backdrop-filter: blur(12px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    color: var(--text-primary);
    cursor: pointer;
    transition: all var(--transition-fast);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  }

  .control-btn:hover {
    background: var(--accent-primary);
    border-color: var(--accent-primary);
    color: white;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }

  .control-btn.paused {
    background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
    border-color: transparent;
    color: white;
    box-shadow: 0 4px 12px rgba(245, 158, 11, 0.3);
  }

  .control-btn.paused:hover {
    background: linear-gradient(135deg, #fcd34d 0%, #fbbf24 100%);
    transform: translateY(-1px);
    box-shadow: 0 6px 16px rgba(245, 158, 11, 0.4);
  }

  .interval-selector {
    position: relative;
  }

  .selector-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: var(--glass-bg);
    backdrop-filter: blur(12px) saturate(180%);
    -webkit-backdrop-filter: blur(12px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    font-weight: 600;
    font-family: inherit;
    cursor: pointer;
    transition: all var(--transition-fast);
    white-space: nowrap;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  }

  .selector-btn:hover {
    border-color: var(--accent-primary);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  }

  .selector-btn.active {
    border-color: var(--accent-primary);
    box-shadow: 0 4px 16px rgba(59, 130, 246, 0.2);
  }

  :global(.refresh-icon) {
    flex-shrink: 0;
    transition: transform 0.6s ease;
  }

  :global(.refresh-icon.spinning) {
    animation: spin 3s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .interval-label {
    font-size: var(--font-size-xs);
  }

  .chevron {
    flex-shrink: 0;
    transition: transform var(--transition-fast);
  }

  .selector-btn.active .chevron {
    transform: rotate(180deg);
  }

  .dropdown {
    position: absolute;
    top: calc(100% + 4px);
    right: 0;
    min-width: 140px;
    background: var(--glass-bg);
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-lg);
    padding: 4px;
    animation: slideDown 0.2s ease;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 8px 12px;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    font-weight: 500;
    font-family: inherit;
    cursor: pointer;
    transition: all var(--transition-fast);
    text-align: left;
  }

  .dropdown-item:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .dropdown-item.selected {
    background: var(--accent-primary);
    color: white;
  }

  .dropdown-item svg {
    flex-shrink: 0;
  }

  .backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: transparent;
    border: none;
    padding: 0;
    cursor: default;
    z-index: 40;
  }
</style>
