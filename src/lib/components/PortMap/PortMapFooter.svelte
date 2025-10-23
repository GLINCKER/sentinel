<script lang="ts">
  import { Pause, Play } from 'lucide-svelte';

  interface Props {
    lastScan: Date | null;
    groupCount: number;
    connectionCount: number;
    totalPorts: number;
    isPaused: boolean;
    onTogglePause: () => void;
    currentPage: number;
    totalPages: number;
    itemsPerPage: number;
    totalItems: number;
    onPageChange: (page: number) => void;
    onPageSizeChange: (size: number) => void;
    lastScanDuration?: number;
    averageScanDuration?: number;
  }

  let {
    lastScan,
    groupCount,
    connectionCount,
    totalPorts,
    isPaused,
    onTogglePause,
    currentPage,
    totalPages,
    itemsPerPage,
    totalItems,
    onPageChange,
    onPageSizeChange,
    lastScanDuration = 0,
    averageScanDuration = 0
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

  const pageSizeOptions = [
    { value: 10, label: '10 per page' },
    { value: 20, label: '20 per page' },
    { value: 50, label: '50 per page' },
    { value: 100, label: '100 per page' }
  ];

  let showPageSizeDropdown = $state(false);

  function selectPageSize(size: number) {
    onPageSizeChange(size);
    showPageSizeDropdown = false;
  }

  // Calculate range
  const startIndex = $derived((currentPage - 1) * itemsPerPage + 1);
  const endIndex = $derived(Math.min(currentPage * itemsPerPage, totalItems));
</script>

<div class="footer">
  <div class="footer-left">
    <!-- Stats -->
    <div class="stats">
      {#if lastScan}
        <span class="stat">
          Last scan: <span class="mono">{lastScan.toLocaleTimeString()}</span>
          <span class="time-ago">({relativeTime})</span>
          {#if lastScanDuration > 0}
            <span class="perf-metric" class:slow={lastScanDuration > 1000}>
              {lastScanDuration}ms
            </span>
          {/if}
        </span>
      {/if}
      <span class="stat">
        Showing <span class="mono">{groupCount}</span> groups (<span
          class="mono">{connectionCount}</span
        >
        connections) of <span class="mono">{totalPorts}</span> total ports
      </span>
      {#if averageScanDuration > 0}
        <span class="stat">
          Avg: <span
            class="perf-metric"
            class:slow={averageScanDuration > 1000}
          >
            {averageScanDuration}ms
          </span>
        </span>
      {/if}
    </div>
  </div>

  <div class="footer-right">
    <!-- Pagination -->
    {#if totalPages > 1}
      <nav class="pagination" aria-label="Port list pagination">
        <div class="page-info" aria-live="polite" aria-atomic="true">
          Showing <span class="mono">{startIndex}-{endIndex}</span> of
          <span class="mono">{totalItems}</span>
        </div>

        <!-- Page Size Selector -->
        <div class="page-size-selector">
          <button
            class="page-size-btn"
            onclick={() => (showPageSizeDropdown = !showPageSizeDropdown)}
            aria-expanded={showPageSizeDropdown}
            aria-haspopup="listbox"
            aria-label="Select page size"
          >
            {itemsPerPage} per page
            <svg
              width="12"
              height="12"
              viewBox="0 0 12 12"
              fill="none"
              aria-hidden="true"
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

          {#if showPageSizeDropdown}
            <div
              class="dropdown-menu"
              role="listbox"
              aria-label="Page size options"
            >
              {#each pageSizeOptions as option (option.value)}
                <button
                  class="dropdown-item"
                  class:selected={itemsPerPage === option.value}
                  role="option"
                  aria-selected={itemsPerPage === option.value}
                  onclick={() => selectPageSize(option.value)}
                >
                  {option.label}
                  {#if itemsPerPage === option.value}
                    <svg
                      width="14"
                      height="14"
                      viewBox="0 0 14 14"
                      fill="none"
                      aria-hidden="true"
                    >
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

        <!-- Page Navigation -->
        <div class="page-nav" role="group" aria-label="Page navigation">
          <button
            class="page-btn"
            onclick={() => onPageChange(currentPage - 1)}
            disabled={currentPage === 1}
            aria-label="Go to previous page"
          >
            <svg
              width="16"
              height="16"
              viewBox="0 0 16 16"
              fill="none"
              aria-hidden="true"
            >
              <path
                d="M10 12L6 8L10 4"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </button>

          {#each Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
            if (totalPages <= 5) return i + 1;
            if (currentPage <= 3) return i + 1;
            if (currentPage >= totalPages - 2) return totalPages - 4 + i;
            return currentPage - 2 + i;
          }) as page}
            <button
              class="page-btn"
              class:active={page === currentPage}
              onclick={() => onPageChange(page)}
              aria-label="Go to page {page}"
              aria-current={page === currentPage ? 'page' : undefined}
            >
              {page}
            </button>
          {/each}

          <button
            class="page-btn"
            onclick={() => onPageChange(currentPage + 1)}
            disabled={currentPage === totalPages}
            aria-label="Go to next page"
          >
            <svg
              width="16"
              height="16"
              viewBox="0 0 16 16"
              fill="none"
              aria-hidden="true"
            >
              <path
                d="M6 4L10 8L6 12"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </button>
        </div>
      </nav>
    {/if}

    <!-- Pause Button -->
    <button
      class="control-btn"
      class:paused={isPaused}
      onclick={onTogglePause}
      aria-label={isPaused ? 'Resume auto-refresh' : 'Pause auto-refresh'}
      aria-pressed={isPaused}
    >
      {#if isPaused}
        <Play size={14} aria-hidden="true" />
        <span>Resume</span>
      {:else}
        <Pause size={14} aria-hidden="true" />
        <span>Pause</span>
      {/if}
    </button>
  </div>
</div>

<!-- Click outside to close dropdown -->
{#if showPageSizeDropdown}
  <button class="backdrop" onclick={() => (showPageSizeDropdown = false)}
  ></button>
{/if}

<style>
  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1.5rem;
    border-top: 1px solid var(--border);
    background: rgba(var(--muted-rgb), 0.7);
    backdrop-filter: blur(12px) saturate(180%);
    -webkit-backdrop-filter: blur(12px) saturate(180%);
    gap: 1rem;
    flex-wrap: wrap;
    position: sticky;
    bottom: 0;
    z-index: 10;
  }

  .footer-left {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex: 1;
    min-width: 0;
  }

  .footer-right {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .stats {
    display: flex;
    gap: 1.5rem;
    align-items: center;
    flex-wrap: wrap;
    font-size: 0.75rem;
    color: var(--muted-foreground);
  }

  .stat {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    white-space: nowrap;
  }

  .mono {
    font-family:
      'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-weight: 600;
    color: var(--foreground);
    letter-spacing: -0.01em;
  }

  .time-ago {
    font-size: 0.6875rem;
    opacity: 0.7;
  }

  .perf-metric {
    font-family:
      'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-size: 0.6875rem;
    font-weight: 600;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    background: rgba(34, 197, 94, 0.1);
    color: rgb(34, 197, 94);
  }

  .perf-metric.slow {
    background: rgba(245, 158, 11, 0.1);
    color: rgb(245, 158, 11);
  }

  /* Pagination */
  .pagination {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .page-info {
    font-size: 0.75rem;
    color: var(--muted-foreground);
    white-space: nowrap;
  }

  .page-size-selector {
    position: relative;
  }

  .page-size-btn {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    background: var(--background);
    border: 1px solid var(--border);
    border-radius: 0.375rem;
    color: var(--foreground);
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .page-size-btn:hover {
    border-color: var(--foreground);
  }

  .page-size-btn svg {
    color: var(--muted-foreground);
  }

  .dropdown-menu {
    position: absolute;
    bottom: calc(100% + 4px);
    right: 0;
    min-width: 140px;
    background: var(--background);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    padding: 4px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
    z-index: 100;
    animation: slideUp 0.15s ease-out;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(8px);
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
    padding: 0.5rem 0.75rem;
    background: transparent;
    border: none;
    border-radius: 0.375rem;
    color: var(--muted-foreground);
    font-size: 0.75rem;
    font-weight: 500;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
  }

  .dropdown-item:hover {
    background: var(--accent);
    color: var(--foreground);
  }

  .dropdown-item.selected {
    background: var(--accent);
    color: var(--foreground);
  }

  .dropdown-item svg {
    color: var(--primary);
    flex-shrink: 0;
  }

  .page-nav {
    display: flex;
    gap: 0.25rem;
  }

  .page-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 32px;
    height: 32px;
    padding: 0 0.5rem;
    background: var(--background);
    border: 1px solid var(--border);
    border-radius: 0.375rem;
    color: var(--foreground);
    font-size: 0.8125rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .page-btn:hover:not(:disabled) {
    background: var(--accent);
    border-color: var(--foreground);
  }

  .page-btn.active {
    background: var(--foreground);
    color: var(--background);
    border-color: var(--foreground);
  }

  .page-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
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
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
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

  .backdrop {
    position: fixed;
    inset: 0;
    background: transparent;
    border: none;
    cursor: default;
    z-index: 90;
  }

  @media (max-width: 1024px) {
    .footer {
      flex-direction: column;
      align-items: stretch;
    }

    .footer-left,
    .footer-right {
      justify-content: space-between;
    }

    .stats {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }
  }
</style>
