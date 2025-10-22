<!--
  @file Process Search Component
  @glinr/sentinel

  Search and filter processes with modern UI.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { Search, X } from 'lucide-svelte';

  interface Props {
    value?: string;
    onSearch?: (query: string) => void;
    placeholder?: string;
  }

  let {
    value = $bindable(''),
    onSearch,
    placeholder = 'Search processes...'
  }: Props = $props();

  function handleInput(event: Event) {
    const input = event.target as HTMLInputElement;
    value = input.value;
    onSearch?.(value);
  }

  function clearSearch() {
    value = '';
    onSearch?.('');
  }
</script>

<div class="process-search">
  <Search size={18} class="search-icon" />
  <input
    type="text"
    class="search-input"
    {placeholder}
    {value}
    oninput={handleInput}
    aria-label="Search processes"
  />
  {#if value}
    <button
      class="clear-button"
      onclick={clearSearch}
      aria-label="Clear search"
      type="button"
    >
      <X size={16} />
    </button>
  {/if}
</div>

<style>
  .process-search {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
    max-width: 400px;
  }

  :global(.search-icon) {
    position: absolute;
    left: 14px;
    color: var(--text-tertiary);
    pointer-events: none;
    z-index: 1;
  }

  .search-input {
    width: 100%;
    height: 40px;
    padding: 0 40px 0 44px;
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    font-size: var(--font-size-sm);
    color: var(--text-primary);
    font-family: var(--font-family-base);
    transition: all var(--transition-base);
    outline: none;
  }

  .search-input::placeholder {
    color: var(--text-tertiary);
  }

  .search-input:focus {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .clear-button {
    position: absolute;
    right: 10px;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.05);
    border: none;
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .clear-button:hover {
    background: rgba(0, 0, 0, 0.1);
    color: var(--text-primary);
  }

  .clear-button:active {
    transform: scale(0.95);
  }
</style>
