<script lang="ts">
  import { Search, Filter, ChevronDown, ChevronUp } from 'lucide-svelte';

  interface Props {
    searchQuery: string;
    onSearchChange: (value: string) => void;
  }

  let { searchQuery, onSearchChange }: Props = $props();

  let isExpanded = $state(false);
</script>

<div class="filter-section" role="search" aria-label="Port filtering options">
  <!-- Toggle Button -->
  <button
    class="toggle-btn"
    onclick={() => (isExpanded = !isExpanded)}
    aria-expanded={isExpanded}
    aria-controls="filter-content"
    aria-label={isExpanded ? 'Hide filter options' : 'Show filter options'}
  >
    <Filter size={16} aria-hidden="true" />
    <span>{isExpanded ? 'Hide Filters' : 'Show Filters'}</span>
    {#if isExpanded}
      <ChevronUp size={14} class="chevron" aria-hidden="true" />
    {:else}
      <ChevronDown size={14} class="chevron" aria-hidden="true" />
    {/if}
  </button>

  <!-- Collapsible Content -->
  {#if isExpanded}
    <div
      class="filter-content"
      id="filter-content"
      role="group"
      aria-label="Filter controls"
    >
      <!-- Search -->
      <div class="search-wrapper">
        <Search size={16} class="search-icon" aria-hidden="true" />
        <label for="port-search" class="visually-hidden"
          >Search ports, processes, or addresses</label
        >
        <input
          id="port-search"
          type="search"
          class="search-input"
          placeholder="Search ports, processes, addresses..."
          value={searchQuery}
          oninput={(e) => onSearchChange(e.currentTarget.value)}
          aria-describedby="search-help"
        />
        <span id="search-help" class="visually-hidden">
          Filter the port list by typing port numbers, process names, or IP
          addresses
        </span>
      </div>
    </div>
  {/if}
</div>

<style>
  .filter-section {
    background: transparent;
    overflow: hidden;
  }

  .toggle-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 0;
    background: transparent;
    border: none;
    color: var(--muted-foreground);
    font-size: 0.8125rem;
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .toggle-btn:hover {
    color: var(--foreground);
  }

  .toggle-btn span {
    flex: 1;
    text-align: left;
  }

  .chevron {
    color: var(--muted-foreground);
    transition: all var(--transition-fast);
  }

  .filter-content {
    padding: 12px 0 0 0;
    animation: slideDown 0.2s ease-out;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .search-wrapper {
    position: relative;
  }

  :global(.search-icon) {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--muted-foreground);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    height: 36px;
    padding: 0 12px 0 36px;
    background: var(--muted);
    border: 1px solid transparent;
    border-radius: var(--radius-lg);
    color: var(--foreground);
    font-size: 0.875rem;
    transition: all var(--transition-fast);
  }

  .search-input:hover {
    border-color: var(--border);
  }

  .search-input:focus {
    outline: none;
    background: var(--background);
    border-color: var(--foreground);
    box-shadow: 0 0 0 1px var(--foreground);
  }

  .search-input::placeholder {
    color: var(--muted-foreground);
  }

  @media (max-width: 768px) {
    .filter-controls {
      flex-direction: column;
    }

    .filter-group {
      min-width: 100%;
    }
  }
</style>
