<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import Toggle from '$lib/components/Toggle.svelte';
  import {
    getDockerInfo,
    listDockerContainers,
    listDockerImages,
    getDockerContainerStats,
    startDockerContainer,
    stopDockerContainer,
    restartDockerContainer,
    pauseDockerContainer,
    unpauseDockerContainer
  } from '$lib/api/docker';
  import type {
    ContainerInfo,
    DockerInfo,
    ContainerStats,
    ImageInfo
  } from '$lib/types/docker';
  import {
    Container,
    Play,
    Square,
    RotateCw,
    Pause,
    PlayCircle,
    Cpu,
    HardDrive,
    RefreshCw,
    CheckCircle2,
    XCircle,
    Clock,
    Box,
    LayoutGrid,
    List
  } from 'lucide-svelte';

  let dockerInfo: DockerInfo | null = $state(null);
  let containers: ContainerInfo[] = $state([]);
  let images: ImageInfo[] = $state([]);
  let containerStats = $state<Map<string, ContainerStats>>(new Map());
  let isLoading = $state(true);
  let error: string | null = $state(null);
  let updateInterval: ReturnType<typeof setInterval>;
  let pollingInterval = $state(5000); // 5 seconds default
  let showStopped = $state(false); // Default: hide stopped containers
  let isRefreshing = $state(false);
  let activeTab = $state<'containers' | 'images'>('containers');
  let viewMode = $state<'grid' | 'list'>('grid');

  // Filtered and sorted containers
  let filteredContainers = $derived.by(() => {
    let result = [...containers];
    if (!showStopped) {
      result = result.filter((c) => c.state === 'running');
    }
    // Sort: running first, then by name
    return result.sort((a, b) => {
      if (a.state === 'running' && b.state !== 'running') return -1;
      if (a.state !== 'running' && b.state === 'running') return 1;
      return a.name.localeCompare(b.name);
    });
  });

  async function loadDockerData() {
    try {
      const [info, containerList, imageList] = await Promise.all([
        getDockerInfo(),
        listDockerContainers(true), // Always fetch all, we'll filter in UI
        listDockerImages()
      ]);

      dockerInfo = info;
      containers = containerList;
      images = imageList;

      // Load stats for running containers
      if (containerList.length > 0) {
        const statsPromises = containerList
          .filter((c) => c.state === 'running')
          .map(async (c) => {
            const stats = await getDockerContainerStats(c.id);
            return { id: c.id, stats };
          });

        const statsResults = await Promise.all(statsPromises);
        // eslint-disable-next-line svelte/prefer-svelte-reactivity
        const newStats = new Map<string, ContainerStats>();
        for (const result of statsResults) {
          if (result.stats) {
            newStats.set(result.id, result.stats);
          }
        }
        containerStats = newStats;
      }

      isLoading = false;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load Docker data';
      isLoading = false;
      dockerInfo = { available: false };
    }
  }

  async function handleRefresh() {
    isRefreshing = true;
    await loadDockerData();
    setTimeout(() => {
      isRefreshing = false;
    }, 600);
  }

  async function handleStart(containerId: string) {
    try {
      await startDockerContainer(containerId);
      await loadDockerData();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to start container';
    }
  }

  async function handleStop(containerId: string) {
    try {
      await stopDockerContainer(containerId);
      await loadDockerData();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to stop container';
    }
  }

  async function handleRestart(containerId: string) {
    try {
      await restartDockerContainer(containerId);
      await loadDockerData();
    } catch (err) {
      error =
        err instanceof Error ? err.message : 'Failed to restart container';
    }
  }

  async function handlePause(containerId: string) {
    try {
      await pauseDockerContainer(containerId);
      await loadDockerData();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to pause container';
    }
  }

  async function handleUnpause(containerId: string) {
    try {
      await unpauseDockerContainer(containerId);
      await loadDockerData();
    } catch (err) {
      error =
        err instanceof Error ? err.message : 'Failed to unpause container';
    }
  }

  function formatBytes(bytes: number | undefined): string {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }

  function formatPercent(value: number | undefined): string {
    if (!value) return '0%';
    return `${value.toFixed(1)}%`;
  }

  function formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));

    if (days === 0) return 'Today';
    if (days === 1) return 'Yesterday';
    if (days < 7) return `${days} days ago`;
    if (days < 30) return `${Math.floor(days / 7)} weeks ago`;
    if (days < 365) return `${Math.floor(days / 30)} months ago`;
    return `${Math.floor(days / 365)} years ago`;
  }

  function getStateColor(state: string): string {
    switch (state.toLowerCase()) {
      case 'running':
        return 'running';
      case 'paused':
        return 'paused';
      case 'exited':
      case 'stopped':
        return 'stopped';
      default:
        return 'unknown';
    }
  }

  function getStateIcon(state: string) {
    switch (state.toLowerCase()) {
      case 'running':
        return CheckCircle2;
      case 'paused':
        return Clock;
      case 'exited':
      case 'stopped':
        return XCircle;
      default:
        return XCircle;
    }
  }

  onMount(() => {
    loadDockerData();

    updateInterval = setInterval(() => {
      loadDockerData();
    }, pollingInterval);
  });

  onDestroy(() => {
    if (updateInterval) {
      clearInterval(updateInterval);
    }
  });
</script>

<div class="docker-page">
  <PageHeader
    title="Docker"
    subtitle="Manage containers and images"
    icon={Container}
  >
    <div class="header-actions">
      <button
        class="btn-refresh"
        onclick={handleRefresh}
        disabled={isRefreshing}
      >
        <RefreshCw size={16} class={isRefreshing ? 'animate-spin' : ''} />
        Refresh
      </button>
    </div>
  </PageHeader>

  {#if error}
    <div class="error-banner">
      <p>{error}</p>
      <button onclick={loadDockerData}>Retry</button>
    </div>
  {/if}

  {#if dockerInfo && !dockerInfo.available}
    <div class="unavailable-state">
      <Container size={64} />
      <h2>Docker is not available</h2>
      <p>
        Docker is not running or not installed on this system. Please start
        Docker or install it to use this feature.
      </p>
    </div>
  {:else}
    <!-- Docker Info Cards -->
    {#if dockerInfo}
      <div class="info-cards">
        <div class="info-card">
          <div class="card-icon version">
            <Container size={18} />
          </div>
          <div class="card-content">
            <div class="card-label">Docker Version</div>
            <div class="card-value">{dockerInfo.version || 'N/A'}</div>
          </div>
        </div>

        <div class="info-card">
          <div class="card-icon total">
            <Container size={18} />
          </div>
          <div class="card-content">
            <div class="card-label">Total Containers</div>
            <div class="card-value">{dockerInfo.containersCount || 0}</div>
          </div>
        </div>

        <div class="info-card">
          <div class="card-icon running">
            <CheckCircle2 size={18} />
          </div>
          <div class="card-content">
            <div class="card-label">Running</div>
            <div class="card-value">{dockerInfo.containersRunning || 0}</div>
          </div>
        </div>

        <div class="info-card">
          <div class="card-icon stopped">
            <XCircle size={18} />
          </div>
          <div class="card-content">
            <div class="card-label">Stopped</div>
            <div class="card-value">{dockerInfo.containersStopped || 0}</div>
          </div>
        </div>

        <div class="info-card">
          <div class="card-icon images">
            <Box size={18} />
          </div>
          <div class="card-content">
            <div class="card-label">Images</div>
            <div class="card-value">{dockerInfo.imagesCount || 0}</div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Tabs and Controls -->
    <div class="tabs-section">
      <div class="tabs">
        <button
          class="tab"
          class:active={activeTab === 'containers'}
          onclick={() => (activeTab = 'containers')}
        >
          <Container size={16} />
          Containers ({filteredContainers.length})
        </button>
        <button
          class="tab"
          class:active={activeTab === 'images'}
          onclick={() => (activeTab = 'images')}
        >
          <Box size={16} />
          Images ({images.length})
        </button>
      </div>

      <div class="controls">
        {#if activeTab === 'containers'}
          <div class="toggle-group">
            <Toggle
              bind:checked={showStopped}
              ariaLabel="Show stopped containers"
            />
            <span class="toggle-label">Show Stopped</span>
          </div>
        {/if}
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
      {#if activeTab === 'containers'}
        {#if isLoading}
          {#each Array(3) as _, idx (idx)}
            <div class="skeleton-card"></div>
          {/each}
        {:else if filteredContainers.length === 0}
          <div class="empty-state">
            <Container size={48} />
            <h3>No containers found</h3>
            <p>
              {#if !showStopped}
                No running containers. Toggle "Show Stopped" to see stopped
                containers.
              {:else}
                No Docker containers exist on this system.
              {/if}
            </p>
          </div>
        {:else if viewMode === 'grid'}
          <div class="containers-grid">
            {#each filteredContainers as container (container.id)}
              {@const stats = containerStats.get(container.id)}
              {@const StateIcon = getStateIcon(container.state)}
              <div class="container-card">
                <div class="container-header">
                  <div class="container-title">
                    <div
                      class="status-indicator status-{getStateColor(
                        container.state
                      )}"
                    >
                      <svelte:component this={StateIcon} size={14} />
                    </div>
                    <div class="name-group">
                      <h4 class="container-name">{container.name}</h4>
                      <span class="container-id">{container.id}</span>
                    </div>
                  </div>
                  <div class="container-actions">
                    {#if container.state === 'running'}
                      <button
                        class="action-btn pause-btn"
                        onclick={() => handlePause(container.id)}
                        title="Pause container"
                      >
                        <Pause size={12} />
                      </button>
                      <button
                        class="action-btn restart-btn"
                        onclick={() => handleRestart(container.id)}
                        title="Restart container"
                      >
                        <RotateCw size={12} />
                      </button>
                      <button
                        class="action-btn stop-btn"
                        onclick={() => handleStop(container.id)}
                        title="Stop container"
                      >
                        <Square size={12} />
                      </button>
                    {:else if container.state === 'paused'}
                      <button
                        class="action-btn start-btn"
                        onclick={() => handleUnpause(container.id)}
                        title="Unpause container"
                      >
                        <PlayCircle size={12} />
                      </button>
                      <button
                        class="action-btn stop-btn"
                        onclick={() => handleStop(container.id)}
                        title="Stop container"
                      >
                        <Square size={12} />
                      </button>
                    {:else}
                      <button
                        class="action-btn start-btn"
                        onclick={() => handleStart(container.id)}
                        title="Start container"
                      >
                        <Play size={12} />
                      </button>
                    {/if}
                  </div>
                </div>

                <div class="container-details">
                  <div class="detail-row">
                    <span class="detail-label">Image</span>
                    <span class="detail-value">{container.image}</span>
                  </div>
                  <div class="detail-row">
                    <span class="detail-label">Status</span>
                    <span
                      class="detail-value status-{getStateColor(
                        container.state
                      )}"
                    >
                      {container.status}
                    </span>
                  </div>
                  {#if container.ports.length > 0}
                    <div class="detail-row">
                      <span class="detail-label">Ports</span>
                      <span class="detail-value">
                        {container.ports
                          .map((p) => {
                            if (p.hostPort) {
                              return `${p.hostPort}:${p.containerPort}`;
                            }
                            return p.containerPort.toString();
                          })
                          .join(', ')}
                      </span>
                    </div>
                  {/if}
                </div>

                {#if stats}
                  <div class="container-stats">
                    <div class="stat-item">
                      <div class="stat-icon">
                        <Cpu size={12} />
                      </div>
                      <div class="stat-content">
                        <span class="stat-label">CPU</span>
                        <span class="stat-value"
                          >{formatPercent(stats.cpuPercent)}</span
                        >
                      </div>
                    </div>
                    <div class="stat-item">
                      <div class="stat-icon">
                        <HardDrive size={12} />
                      </div>
                      <div class="stat-content">
                        <span class="stat-label">Memory</span>
                        <span class="stat-value">
                          {formatBytes(stats.memoryUsage)} / {formatBytes(
                            stats.memoryLimit
                          )}
                        </span>
                      </div>
                    </div>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {:else}
          <!-- List View -->
          <div class="containers-list">
            {#each filteredContainers as container (container.id)}
              {@const stats = containerStats.get(container.id)}
              {@const StateIcon = getStateIcon(container.state)}
              <div class="container-list-item">
                <div
                  class="status-indicator status-{getStateColor(
                    container.state
                  )}"
                >
                  <svelte:component this={StateIcon} size={14} />
                </div>
                <div class="list-name">
                  <div class="container-name">{container.name}</div>
                  <div class="container-image">{container.image}</div>
                </div>
                <div class="list-id">{container.id}</div>
                <div
                  class="list-status status-{getStateColor(container.state)}"
                >
                  {container.status}
                </div>
                {#if stats}
                  <div class="list-stats">
                    <span>CPU: {formatPercent(stats.cpuPercent)}</span>
                    <span>Mem: {formatBytes(stats.memoryUsage)}</span>
                  </div>
                {:else}
                  <div class="list-stats">
                    <span>-</span>
                  </div>
                {/if}
                <div class="list-actions">
                  {#if container.state === 'running'}
                    <button
                      class="action-btn pause-btn"
                      onclick={() => handlePause(container.id)}
                      title="Pause"
                    >
                      <Pause size={12} />
                    </button>
                    <button
                      class="action-btn restart-btn"
                      onclick={() => handleRestart(container.id)}
                      title="Restart"
                    >
                      <RotateCw size={12} />
                    </button>
                    <button
                      class="action-btn stop-btn"
                      onclick={() => handleStop(container.id)}
                      title="Stop"
                    >
                      <Square size={12} />
                    </button>
                  {:else if container.state === 'paused'}
                    <button
                      class="action-btn start-btn"
                      onclick={() => handleUnpause(container.id)}
                      title="Unpause"
                    >
                      <PlayCircle size={12} />
                    </button>
                    <button
                      class="action-btn stop-btn"
                      onclick={() => handleStop(container.id)}
                      title="Stop"
                    >
                      <Square size={12} />
                    </button>
                  {:else}
                    <button
                      class="action-btn start-btn"
                      onclick={() => handleStart(container.id)}
                      title="Start"
                    >
                      <Play size={12} />
                    </button>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        {/if}
      {:else}
        <!-- Images Tab -->
        {#if isLoading}
          {#each Array(3) as _, idx (idx)}
            <div class="skeleton-card"></div>
          {/each}
        {:else if images.length === 0}
          <div class="empty-state">
            <Box size={48} />
            <h3>No images found</h3>
            <p>No Docker images exist on this system.</p>
          </div>
        {:else if viewMode === 'grid'}
          <div class="images-grid">
            {#each images as image (image.id)}
              <div class="image-card">
                <div class="image-header">
                  <div class="image-icon">
                    <Box size={16} />
                  </div>
                  <div class="image-info">
                    <h4 class="image-name">
                      {image.repoTags.length > 0
                        ? image.repoTags[0]
                        : 'Untagged'}
                    </h4>
                    <span class="image-id">{image.id}</span>
                  </div>
                </div>
                <div class="image-details">
                  {#if image.repoTags.length > 1}
                    <div class="detail-row">
                      <span class="detail-label">Tags</span>
                      <span class="detail-value">{image.repoTags.length}</span>
                    </div>
                  {/if}
                  <div class="detail-row">
                    <span class="detail-label">Size</span>
                    <span class="detail-value">{formatBytes(image.size)}</span>
                  </div>
                  <div class="detail-row">
                    <span class="detail-label">Created</span>
                    <span class="detail-value">{formatDate(image.created)}</span
                    >
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <!-- Images List View -->
          <div class="images-list">
            {#each images as image (image.id)}
              <div class="image-list-item">
                <div class="image-icon">
                  <Box size={16} />
                </div>
                <div class="list-name">
                  <div class="image-name">
                    {image.repoTags.length > 0 ? image.repoTags[0] : 'Untagged'}
                  </div>
                  {#if image.repoTags.length > 1}
                    <div class="image-tags">
                      +{image.repoTags.length - 1} more
                    </div>
                  {/if}
                </div>
                <div class="list-id">{image.id}</div>
                <div class="list-size">{formatBytes(image.size)}</div>
                <div class="list-date">{formatDate(image.created)}</div>
              </div>
            {/each}
          </div>
        {/if}
      {/if}
    </div>
  {/if}
</div>

<style>
  .docker-page {
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

  .btn-refresh:active:not(:disabled) {
    transform: translateY(0);
  }

  .btn-refresh:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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

  /* Info Cards - Compact Design */
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

  .card-icon.version {
    background: rgba(59, 130, 246, 0.1);
    color: rgb(59, 130, 246);
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

  .card-icon.images {
    background: rgba(168, 85, 247, 0.1);
    color: rgb(168, 85, 247);
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

  /* Tabs Section */
  .tabs-section {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-md) var(--space-lg);
    border-bottom: 1px solid var(--border-color);
    gap: var(--space-md);
  }

  .tabs {
    display: flex;
    gap: var(--space-xs);
  }

  .tab {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    padding: var(--space-sm) var(--space-md);
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .tab:hover {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    color: var(--text-primary);
  }

  .tab.active {
    background: var(--accent-primary);
    border-color: var(--accent-primary);
    color: white;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .toggle-group {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .toggle-label {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-secondary);
    user-select: none;
  }

  .view-toggle {
    display: flex;
    gap: 2px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    padding: 2px;
  }

  .view-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px 8px;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
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
    flex: 1;
    padding: var(--space-lg);
  }

  /* Containers Grid - Compact */
  .containers-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: var(--space-md);
  }

  .container-card {
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    padding: var(--space-md);
    transition: all var(--transition-base);
    box-shadow:
      0 2px 8px rgba(0, 0, 0, 0.04),
      0 1px 2px rgba(0, 0, 0, 0.02);
  }

  .container-card:hover {
    transform: translateY(-1px);
    border-color: var(--accent-primary);
    box-shadow:
      0 4px 12px rgba(0, 0, 0, 0.08),
      0 2px 4px rgba(0, 0, 0, 0.04);
  }

  .container-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-md);
  }

  .container-title {
    display: flex;
    align-items: flex-start;
    gap: var(--space-sm);
    flex: 1;
    min-width: 0;
  }

  .status-indicator {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: var(--radius-md);
    flex-shrink: 0;
  }

  .status-indicator.status-running {
    background: rgba(34, 197, 94, 0.15);
    color: rgb(34, 197, 94);
  }

  .status-indicator.status-paused {
    background: rgba(251, 191, 36, 0.15);
    color: rgb(251, 191, 36);
  }

  .status-indicator.status-stopped {
    background: rgba(100, 116, 139, 0.15);
    color: rgb(100, 116, 139);
  }

  .name-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .container-name {
    font-size: var(--font-size-base);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
    letter-spacing: -0.01em;
  }

  .container-id {
    font-size: 10px;
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
    color: var(--text-tertiary);
    background: rgba(59, 130, 246, 0.08);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    width: fit-content;
  }

  .container-actions {
    display: flex;
    gap: 4px;
  }

  .action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 0;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    background: var(--bg-secondary);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .action-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }

  .action-btn:active {
    transform: translateY(0);
  }

  .start-btn:hover {
    background: var(--success-bg);
    border-color: var(--success);
    color: var(--success);
  }

  .stop-btn:hover {
    background: var(--error-bg);
    border-color: var(--error);
    color: var(--error);
  }

  .restart-btn:hover {
    background: rgba(59, 130, 246, 0.15);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .pause-btn:hover {
    background: rgba(251, 191, 36, 0.15);
    border-color: #f59e0b;
    color: #f59e0b;
  }

  .container-details {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    padding-bottom: var(--space-md);
    border-bottom: 1px solid var(--border-color);
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-md);
  }

  .detail-label {
    font-size: 10px;
    font-weight: 700;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.6px;
  }

  .detail-value {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--text-primary);
    text-align: right;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .detail-value.status-running {
    color: rgb(34, 197, 94);
  }

  .detail-value.status-paused {
    color: rgb(251, 191, 36);
  }

  .detail-value.status-stopped {
    color: rgb(100, 116, 139);
  }

  .container-stats {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-md);
    margin-top: var(--space-md);
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .stat-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: var(--radius-md);
    background: rgba(59, 130, 246, 0.1);
    color: var(--accent-primary);
    flex-shrink: 0;
  }

  .stat-content {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .stat-label {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.6px;
  }

  .stat-value {
    font-size: var(--font-size-xs);
    font-weight: 700;
    color: var(--text-primary);
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
    font-variant-numeric: tabular-nums;
  }

  /* List View for Containers */
  .containers-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .container-list-item {
    display: grid;
    grid-template-columns: auto 1fr auto auto auto auto;
    gap: var(--space-md);
    align-items: center;
    padding: var(--space-md);
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    transition: all var(--transition-fast);
  }

  .container-list-item:hover {
    border-color: var(--accent-primary);
    transform: translateY(-1px);
  }

  .list-name {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .container-image {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
  }

  .list-id {
    font-size: 10px;
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
    color: var(--text-tertiary);
    background: rgba(59, 130, 246, 0.08);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    width: fit-content;
  }

  .list-status {
    font-size: var(--font-size-xs);
    font-weight: 600;
  }

  .list-status.status-running {
    color: rgb(34, 197, 94);
  }

  .list-status.status-paused {
    color: rgb(251, 191, 36);
  }

  .list-status.status-stopped {
    color: rgb(100, 116, 139);
  }

  .list-stats {
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--text-secondary);
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
  }

  .list-actions {
    display: flex;
    gap: 4px;
  }

  /* Images Grid */
  .images-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: var(--space-md);
  }

  .image-card {
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    padding: var(--space-md);
    transition: all var(--transition-base);
  }

  .image-card:hover {
    transform: translateY(-1px);
    border-color: var(--accent-primary);
  }

  .image-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    margin-bottom: var(--space-md);
    padding-bottom: var(--space-md);
    border-bottom: 1px solid var(--border-color);
  }

  .image-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-md);
    background: rgba(168, 85, 247, 0.1);
    color: rgb(168, 85, 247);
    flex-shrink: 0;
  }

  .image-info {
    flex: 1;
    min-width: 0;
  }

  .image-name {
    font-size: var(--font-size-base);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 4px 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .image-id {
    font-size: 10px;
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
    color: var(--text-tertiary);
    background: rgba(168, 85, 247, 0.08);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
  }

  .image-details {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  /* Images List View */
  .images-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .image-list-item {
    display: grid;
    grid-template-columns: auto 1fr auto auto auto;
    gap: var(--space-md);
    align-items: center;
    padding: var(--space-md);
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    transition: all var(--transition-fast);
  }

  .image-list-item:hover {
    border-color: var(--accent-primary);
    transform: translateY(-1px);
  }

  .image-tags {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
  }

  .list-size,
  .list-date {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--text-secondary);
  }

  /* Skeleton Loading */
  .skeleton-card {
    height: 150px;
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-lg);
    animation: skeleton-loading 1.5s ease-in-out infinite;
  }

  @keyframes skeleton-loading {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-4xl) var(--space-2xl);
    text-align: center;
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    border-radius: var(--radius-lg);
    border: 2px dashed var(--border-color);
    transition: all var(--transition-base);
  }

  .empty-state:hover {
    border-color: var(--accent-primary);
    background: rgba(59, 130, 246, 0.02);
  }

  .empty-state h3 {
    margin-top: var(--space-md);
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .empty-state p {
    margin-top: var(--space-sm);
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    line-height: 1.6;
  }

  /* Unavailable State */
  .unavailable-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-4xl) var(--space-2xl);
    text-align: center;
    color: var(--text-secondary);
    flex: 1;
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    border-radius: var(--radius-lg);
    border: 2px dashed var(--border-color);
    margin: var(--space-lg);
  }

  .unavailable-state h2 {
    margin-top: var(--space-md);
    font-size: var(--font-size-2xl);
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .unavailable-state p {
    margin-top: var(--space-sm);
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    max-width: 400px;
    line-height: 1.6;
  }

  /* Error Banner */
  .error-banner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-md) var(--space-lg);
    background: var(--error-bg);
    border-bottom: 1px solid var(--error);
    color: var(--error);
  }

  .error-banner button {
    padding: 6px 12px;
    background: var(--error);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-xs);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .error-banner button:hover {
    background: #dc2626;
    transform: translateY(-1px);
  }
</style>
