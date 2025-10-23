<script lang="ts">
  import { ChevronLeft, ChevronRight } from 'lucide-svelte';
  import Dropdown from '../Dropdown.svelte';

  interface Props {
    currentPage: number;
    totalPages: number;
    totalItems: number;
    itemsPerPage: number;
    onPageChange: (page: number) => void;
    onPageSizeChange: (size: number) => void;
  }

  let {
    currentPage,
    totalPages,
    totalItems,
    itemsPerPage,
    onPageChange,
    onPageSizeChange
  }: Props = $props();

  const pageSizeOptions = [
    { value: 10, label: '10 per page' },
    { value: 20, label: '20 per page' },
    { value: 50, label: '50 per page' },
    { value: 100, label: '100 per page' }
  ];

  let startItem = $derived((currentPage - 1) * itemsPerPage + 1);
  let endItem = $derived(Math.min(currentPage * itemsPerPage, totalItems));

  function goToPage(page: number) {
    if (page >= 1 && page <= totalPages) {
      onPageChange(page);
    }
  }
</script>

<div class="pagination">
  <div class="pagination-info">
    <span class="info-text">
      Showing <span class="mono">{startItem}</span>-<span class="mono"
        >{endItem}</span
      >
      of <span class="mono">{totalItems}</span>
    </span>

    <Dropdown
      options={pageSizeOptions}
      selected={itemsPerPage}
      onSelect={onPageSizeChange}
    />
  </div>

  <div class="pagination-controls">
    <button
      class="page-btn"
      onclick={() => goToPage(currentPage - 1)}
      disabled={currentPage === 1}
      aria-label="Previous page"
    >
      <ChevronLeft size={16} />
    </button>

    <div class="page-numbers">
      {#if totalPages <= 7}
        {#each Array(totalPages) as _, i (i)}
          <button
            class="page-number"
            class:active={currentPage === i + 1}
            onclick={() => goToPage(i + 1)}
          >
            {i + 1}
          </button>
        {/each}
      {:else}
        <button
          class="page-number"
          class:active={currentPage === 1}
          onclick={() => goToPage(1)}>1</button
        >

        {#if currentPage > 3}
          <span class="ellipsis">...</span>
        {/if}

        {#each Array(5) as _, i (i)}
          {@const pageNum = Math.max(
            2,
            Math.min(totalPages - 1, currentPage - 2 + i)
          )}
          {#if pageNum > 1 && pageNum < totalPages}
            <button
              class="page-number"
              class:active={currentPage === pageNum}
              onclick={() => goToPage(pageNum)}
            >
              {pageNum}
            </button>
          {/if}
        {/each}

        {#if currentPage < totalPages - 2}
          <span class="ellipsis">...</span>
        {/if}

        <button
          class="page-number"
          class:active={currentPage === totalPages}
          onclick={() => goToPage(totalPages)}>{totalPages}</button
        >
      {/if}
    </div>

    <button
      class="page-btn"
      onclick={() => goToPage(currentPage + 1)}
      disabled={currentPage === totalPages}
      aria-label="Next page"
    >
      <ChevronRight size={16} />
    </button>
  </div>
</div>

<style>
  .pagination {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1.5rem;
    border-top: 1px solid var(--border);
    background: var(--background);
    gap: 1rem;
    flex-wrap: wrap;
  }

  .pagination-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .info-text {
    font-size: 0.8125rem;
    color: var(--muted-foreground);
  }

  .mono {
    font-family:
      'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-weight: 600;
    color: var(--foreground);
  }

  .pagination-controls {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .page-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 0.375rem;
    color: var(--foreground);
    cursor: pointer;
    transition: all 0.2s;
  }

  .page-btn:hover:not(:disabled) {
    background: var(--accent);
    border-color: var(--foreground);
  }

  .page-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .page-numbers {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    margin: 0 0.5rem;
  }

  .page-number {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 32px;
    height: 32px;
    padding: 0 0.5rem;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 600;
    color: var(--foreground);
    cursor: pointer;
    transition: all 0.2s;
  }

  .page-number:hover {
    background: var(--accent);
    border-color: var(--border);
  }

  .page-number.active {
    background: var(--foreground);
    color: var(--background);
    border-color: var(--foreground);
  }

  .ellipsis {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    color: var(--muted-foreground);
    font-size: 0.875rem;
  }

  @media (max-width: 768px) {
    .pagination {
      flex-direction: column;
      align-items: flex-start;
    }

    .pagination-controls {
      width: 100%;
      justify-content: center;
    }

    .page-numbers {
      flex: 1;
      justify-content: center;
    }
  }
</style>
