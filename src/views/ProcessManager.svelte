<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Plus,
    LayoutGrid,
    List,
    RefreshCw,
    Package,
    PlayCircle
  } from 'lucide-svelte';
  import {
    processConfigStore,
    type ProcessConfig
  } from '../stores/processConfig.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import ProcessCard from '$lib/components/ProcessCard.svelte';
  import ProcessConfigModal from '$lib/components/ProcessConfigModal.svelte';
  import { toast } from 'svelte-sonner';
  import { useVisibilityPolling } from '$lib/hooks/useVisibilityPolling.svelte';

  let viewMode = $state<'grid' | 'list'>('grid');
  let showConfigModal = $state(false);
  let editingConfig = $state<ProcessConfig | null>(null);
  let isRefreshing = $state(false);

  onMount(() => {
    processConfigStore.loadConfigs();
  });

  // Auto-refresh every 5 seconds
  useVisibilityPolling({
    interval: 5000,
    callback: async () => {
      if (!processConfigStore.loading) {
        await processConfigStore.loadConfigs();
      }
    },
    immediate: false
  });

  async function handleRefresh() {
    isRefreshing = true;
    try {
      await processConfigStore.loadConfigs();
    } catch (err) {
      console.error('Refresh error:', err);
    } finally {
      setTimeout(() => {
        isRefreshing = false;
      }, 600);
    }
  }

  async function handleStart(configId: string) {
    try {
      await processConfigStore.startProcess(configId);
      toast.success('Process started successfully');
    } catch (err) {
      toast.error(`Failed to start process: ${err}`);
    }
  }

  async function handleStop(configId: string) {
    try {
      await processConfigStore.stopProcess(configId);
      toast.success('Process stopped successfully');
    } catch (err) {
      toast.error(`Failed to stop process: ${err}`);
    }
  }

  async function handleRestart(configId: string) {
    try {
      await processConfigStore.restartProcess(configId);
      toast.success('Process restarted successfully');
    } catch (err) {
      toast.error(`Failed to restart process: ${err}`);
    }
  }

  async function handleEdit(config: ProcessConfig) {
    editingConfig = config;
    showConfigModal = true;
  }

  async function handleDelete(configId: string, name: string) {
    if (!confirm(`Delete process configuration "${name}"?`)) return;

    try {
      await processConfigStore.deleteConfig(configId);
      toast.success('Process configuration deleted');
    } catch (err) {
      toast.error(`Failed to delete configuration: ${err}`);
    }
  }

  function handleCreateNew() {
    editingConfig = null;
    showConfigModal = true;
  }

  async function handleSaved() {
    showConfigModal = false;
    await processConfigStore.loadConfigs();
  }

  // Stats for header cards
  let stats = $derived.by(() => {
    const total = processConfigStore.configs.length;
    const running = Array.from(processConfigStore.statuses.values()).filter(
      (s) => s.running
    ).length;
    const stopped = total - running;
    return { total, running, stopped };
  });
</script>

<div class="process-manager-page">
  <PageHeader
    title="Process Manager"
    subtitle="Manage development processes with framework templates"
  >
    <div class="header-actions">
      <button
        class="btn-refresh"
        onclick={handleRefresh}
        disabled={isRefreshing}
        title="Refresh"
      >
        <RefreshCw size={14} class={isRefreshing ? 'animate-spin' : ''} />
        Refresh
      </button>
      <button
        class="btn-new"
        onclick={handleCreateNew}
        title="Create new process"
      >
        <Plus size={16} />
        New Process
      </button>
    </div>
  </PageHeader>

  {#if !processConfigStore.loading || processConfigStore.configs.length > 0}
    <!-- Info Cards -->
    <div class="info-cards">
      <div class="info-card">
        <div class="card-icon total">
          <Package size={18} />
        </div>
        <div class="card-content">
          <div class="card-label">Total</div>
          <div class="card-value">{stats.total}</div>
        </div>
      </div>

      <div class="info-card">
        <div class="card-icon running">
          <PlayCircle size={18} />
        </div>
        <div class="card-content">
          <div class="card-label">Running</div>
          <div class="card-value">{stats.running}</div>
        </div>
      </div>

      <div class="info-card">
        <div class="card-icon stopped">
          <Package size={18} />
        </div>
        <div class="card-content">
          <div class="card-label">Stopped</div>
          <div class="card-value">{stats.stopped}</div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Controls Section -->
  <div class="controls-section">
    <div class="controls-right">
      <div class="view-toggle">
        <button
          class="view-btn"
          class:active={viewMode === 'grid'}
          onclick={() => (viewMode = 'grid')}
          title="Grid view"
        >
          <LayoutGrid size={18} />
        </button>
        <button
          class="view-btn"
          class:active={viewMode === 'list'}
          onclick={() => (viewMode = 'list')}
          title="List view"
        >
          <List size={18} />
        </button>
      </div>
    </div>
  </div>

  <!-- Content Section -->
  <div class="content-section">
    {#if processConfigStore.loading}
      {#each Array(3) as _, idx (idx)}
        <div class="skeleton-card"></div>
      {/each}
    {:else if processConfigStore.error}
      <div class="empty-state">
        <Package size={48} />
        <h3>Error Loading Processes</h3>
        <p>{processConfigStore.error}</p>
      </div>
    {:else if processConfigStore.configs.length === 0}
      <div class="empty-state">
        <Package size={48} />
        <h3>No Process Configurations Yet</h3>
        <p>
          Create your first process configuration to start managing development
          processes.
        </p>
        <button class="btn-create-first" onclick={handleCreateNew}>
          <Plus size={20} />
          Create First Process
        </button>
      </div>
    {:else if viewMode === 'grid'}
      <div class="processes-grid">
        {#each processConfigStore.configs as config (config.id)}
          {@const status = processConfigStore.getStatus(config.id)}
          {#if status}
            <ProcessCard
              {config}
              {status}
              onStart={() => handleStart(config.id)}
              onStop={() => handleStop(config.id)}
              onRestart={() => handleRestart(config.id)}
              onEdit={() => handleEdit(config)}
              onDelete={() => handleDelete(config.id, config.name)}
            />
          {/if}
        {/each}
      </div>
    {:else}
      <div class="processes-list">
        {#each processConfigStore.configs as config (config.id)}
          {@const status = processConfigStore.getStatus(config.id)}
          {#if status}
            <ProcessCard
              {config}
              {status}
              viewMode="list"
              onStart={() => handleStart(config.id)}
              onStop={() => handleStop(config.id)}
              onRestart={() => handleRestart(config.id)}
              onEdit={() => handleEdit(config)}
              onDelete={() => handleDelete(config.id, config.name)}
            />
          {/if}
        {/each}
      </div>
    {/if}
  </div>
</div>

<ProcessConfigModal
  show={showConfigModal}
  {editingConfig}
  onClose={() => (showConfigModal = false)}
  onSaved={handleSaved}
/>

<style>
  .process-manager-page {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-primary);
    overflow-y: auto;
  }

  .header-actions {
    display: flex;
    gap: var(--space-sm);
    align-items: center;
  }

  .btn-refresh {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: var(--bg-secondary);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    font-size: var(--font-size-xs);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .btn-refresh:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
    transform: translateY(-1px);
  }

  .btn-refresh:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-new {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 14px;
    background: var(--accent-primary);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-xs);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .btn-new:hover {
    background: var(--accent-hover);
    transform: translateY(-1px);
  }

  :global(.animate-spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  /* Info Cards */
  .info-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: var(--space-md);
    padding: var(--space-md) var(--space-lg);
    border-bottom: 1px solid var(--border-color);
  }

  .info-card {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-md);
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    transition: all var(--transition-base);
    box-shadow:
      0 2px 8px rgba(0, 0, 0, 0.04),
      0 1px 2px rgba(0, 0, 0, 0.02);
  }

  .info-card:hover {
    transform: translateY(-1px);
    border-color: var(--accent-primary);
    box-shadow:
      0 4px 12px rgba(0, 0, 0, 0.08),
      0 2px 4px rgba(0, 0, 0, 0.04);
  }

  .card-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-md);
    flex-shrink: 0;
  }

  .card-icon.total {
    background: rgba(100, 116, 139, 0.1);
    color: rgb(100, 116, 139);
  }

  .card-icon.running {
    background: rgba(34, 197, 94, 0.1);
    color: rgb(34, 197, 94);
  }

  .card-icon.stopped {
    background: rgba(239, 68, 68, 0.1);
    color: rgb(239, 68, 68);
  }

  .card-content {
    flex: 1;
    min-width: 0;
  }

  .card-label {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.6px;
    margin-bottom: 2px;
  }

  .card-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
    line-height: 1;
  }

  /* Controls Section */
  .controls-section {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding: var(--space-md) var(--space-lg);
    border-bottom: 1px solid var(--border-color);
  }

  .controls-right {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .view-toggle {
    display: flex;
    gap: 4px;
    padding: 4px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
  }

  .view-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    background: transparent;
    color: var(--text-secondary);
    border: none;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .view-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .view-btn.active {
    background: var(--accent-primary);
    color: white;
  }

  /* Content Section */
  .content-section {
    padding: var(--space-lg);
    flex: 1;
  }

  .processes-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: var(--space-md);
  }

  .processes-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-2xl);
    color: var(--text-secondary);
    text-align: center;
  }

  .empty-state :global(svg) {
    margin-bottom: var(--space-md);
    color: var(--text-tertiary);
    opacity: 0.5;
  }

  .empty-state h3 {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--space-sm) 0;
  }

  .empty-state p {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    max-width: 400px;
    margin: 0 0 var(--space-lg) 0;
  }

  .btn-create-first {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 10px 20px;
    background: var(--accent-primary);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .btn-create-first:hover {
    background: var(--accent-hover);
    transform: translateY(-1px);
  }

  .skeleton-card {
    height: 200px;
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }
</style>
