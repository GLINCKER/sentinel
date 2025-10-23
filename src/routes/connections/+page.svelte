<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import {
    getActiveConnections,
    getConnectionSummary,
    getBandwidthConsumers
  } from '$lib/api/connections';
  import type {
    Connection,
    ConnectionSummary,
    ProcessBandwidth
  } from '$lib/types/connections';
  import {
    Link2,
    ArrowUpDown,
    Search,
    Filter,
    Download,
    Upload,
    Activity
  } from 'lucide-svelte';

  let connections: Connection[] = $state([]);
  let summary: ConnectionSummary | null = $state(null);
  let bandwidthConsumers: ProcessBandwidth[] = $state([]);
  let isLoading = $state(true);
  let error: string | null = $state(null);
  let updateInterval: ReturnType<typeof setInterval>;
  let pollingInterval = $state(2000); // 2 seconds default

  // Filters
  let searchQuery = $state('');
  let protocolFilter = $state<'all' | 'TCP' | 'UDP'>('all');
  let stateFilter = $state<'all' | string>('all');
  let processFilter = $state('');

  // Sorting
  let sortColumn = $state<
    'localAddress' | 'remoteAddress' | 'state' | 'protocol' | 'process'
  >('localAddress');
  let sortOrder = $state<'asc' | 'desc'>('asc');

  async function loadData() {
    try {
      const [conns, summ, bandwidth] = await Promise.all([
        getActiveConnections(),
        getConnectionSummary(),
        getBandwidthConsumers(10)
      ]);

      connections = conns;
      summary = summ;
      bandwidthConsumers = bandwidth;

      isLoading = false;
      error = null;
    } catch (err) {
      error =
        err instanceof Error ? err.message : 'Failed to load connection data';
      isLoading = false;
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }

  function formatRate(bytesPerSec: number): string {
    return `${formatBytes(bytesPerSec)}/s`;
  }

  function toggleSort(
    column: 'localAddress' | 'remoteAddress' | 'state' | 'protocol' | 'process'
  ) {
    if (sortColumn === column) {
      sortOrder = sortOrder === 'asc' ? 'desc' : 'asc';
    } else {
      sortColumn = column;
      sortOrder = 'asc';
    }
  }

  function getProcessName(conn: Connection): string {
    return conn.processName || `PID ${conn.pid || 'Unknown'}`;
  }

  // Filtered and sorted connections
  let filteredConnections = $derived.by(() => {
    let filtered = connections;

    // Search filter
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(
        (c) =>
          c.localAddress.toLowerCase().includes(query) ||
          c.remoteAddress.toLowerCase().includes(query) ||
          (c.processName && c.processName.toLowerCase().includes(query)) ||
          c.state.toLowerCase().includes(query)
      );
    }

    // Protocol filter
    if (protocolFilter !== 'all') {
      filtered = filtered.filter((c) => c.protocol === protocolFilter);
    }

    // State filter
    if (stateFilter !== 'all') {
      filtered = filtered.filter((c) => c.state === stateFilter);
    }

    // Process filter
    if (processFilter.trim()) {
      const query = processFilter.toLowerCase();
      filtered = filtered.filter((c) =>
        c.processName?.toLowerCase().includes(query)
      );
    }

    // Sort
    return [...filtered].sort((a, b) => {
      let comparison = 0;

      switch (sortColumn) {
        case 'localAddress':
          comparison = `${a.localAddress}:${a.localPort}`.localeCompare(
            `${b.localAddress}:${b.localPort}`
          );
          break;
        case 'remoteAddress':
          comparison = `${a.remoteAddress}:${a.remotePort}`.localeCompare(
            `${b.remoteAddress}:${b.remotePort}`
          );
          break;
        case 'state':
          comparison = a.state.localeCompare(b.state);
          break;
        case 'protocol':
          comparison = a.protocol.localeCompare(b.protocol);
          break;
        case 'process':
          comparison = getProcessName(a).localeCompare(getProcessName(b));
          break;
      }

      return sortOrder === 'asc' ? comparison : -comparison;
    });
  });

  // Get unique states for filter dropdown
  let uniqueStates = $derived.by(() => {
    const states = new Set(connections.map((c) => c.state));
    return Array.from(states).sort();
  });

  onMount(() => {
    loadData();

    updateInterval = setInterval(() => {
      loadData();
    }, pollingInterval);
  });

  onDestroy(() => {
    if (updateInterval) {
      clearInterval(updateInterval);
    }
  });
</script>

<div class="connections-page">
  <PageHeader
    title="Network Connections"
    subtitle="Monitor active network connections and bandwidth usage"
    icon={Link2}
  />

  {#if error}
    <div class="error-banner">
      <p>{error}</p>
      <button onclick={loadData}>Retry</button>
    </div>
  {/if}

  <!-- Summary Cards -->
  {#if summary}
    <div class="summary-cards">
      <div class="summary-card">
        <div class="card-icon total">
          <Link2 size={18} />
        </div>
        <div class="card-content">
          <div class="card-label">Total Connections</div>
          <div class="card-value">{summary.totalConnections}</div>
        </div>
      </div>

      <div class="summary-card">
        <div class="card-icon tcp">
          <Activity size={18} />
        </div>
        <div class="card-content">
          <div class="card-label">TCP</div>
          <div class="card-value">{summary.tcpConnections}</div>
        </div>
      </div>

      <div class="summary-card">
        <div class="card-icon udp">
          <Activity size={18} />
        </div>
        <div class="card-content">
          <div class="card-label">UDP</div>
          <div class="card-value">{summary.udpConnections}</div>
        </div>
      </div>

      <div class="summary-card">
        <div class="card-icon established">
          <Link2 size={18} />
        </div>
        <div class="card-content">
          <div class="card-label">Established</div>
          <div class="card-value">{summary.establishedConnections}</div>
        </div>
      </div>

      <div class="summary-card">
        <div class="card-icon listening">
          <Activity size={18} />
        </div>
        <div class="card-content">
          <div class="card-label">Listening</div>
          <div class="card-value">{summary.listeningSockets}</div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Bandwidth Consumers -->
  {#if bandwidthConsumers.length > 0}
    <div class="bandwidth-section">
      <h3 class="section-title">Top Bandwidth Consumers</h3>
      <div class="bandwidth-list">
        {#each bandwidthConsumers.slice(0, 5) as process (process.pid)}
          <div class="bandwidth-item">
            <div class="process-info">
              <span class="process-name">{process.processName}</span>
              <span class="process-pid">PID {process.pid}</span>
            </div>
            <div class="bandwidth-stats">
              <div class="stat">
                <Upload size={16} />
                <span>{formatRate(process.bytesSentPerSec)}</span>
              </div>
              <div class="stat">
                <Download size={16} />
                <span>{formatRate(process.bytesReceivedPerSec)}</span>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Filters -->
  <div class="filters-bar">
    <div class="search-box">
      <Search size={16} />
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search connections..."
      />
    </div>

    <div class="filter-controls">
      <select bind:value={protocolFilter}>
        <option value="all">All Protocols</option>
        <option value="TCP">TCP</option>
        <option value="UDP">UDP</option>
      </select>

      <select bind:value={stateFilter}>
        <option value="all">All States</option>
        {#each uniqueStates as state (state)}
          <option value={state}>{state}</option>
        {/each}
      </select>

      <div class="search-box process-search">
        <Filter size={16} />
        <input
          type="text"
          bind:value={processFilter}
          placeholder="Filter by process..."
        />
      </div>
    </div>
  </div>

  <!-- Connections Table -->
  <div class="table-container">
    <div class="table-header">
      <div
        class="th sortable"
        onclick={() => toggleSort('localAddress')}
        role="button"
        tabindex="0"
      >
        <span>Local Address:Port</span>
        {#if sortColumn === 'localAddress'}
          <ArrowUpDown
            size={12}
            class={sortOrder === 'desc' ? 'rotate-180' : ''}
          />
        {/if}
      </div>
      <div
        class="th sortable"
        onclick={() => toggleSort('remoteAddress')}
        role="button"
        tabindex="0"
      >
        <span>Remote Address:Port</span>
        {#if sortColumn === 'remoteAddress'}
          <ArrowUpDown
            size={12}
            class={sortOrder === 'desc' ? 'rotate-180' : ''}
          />
        {/if}
      </div>
      <div
        class="th sortable"
        onclick={() => toggleSort('state')}
        role="button"
        tabindex="0"
      >
        <span>State</span>
        {#if sortColumn === 'state'}
          <ArrowUpDown
            size={12}
            class={sortOrder === 'desc' ? 'rotate-180' : ''}
          />
        {/if}
      </div>
      <div
        class="th sortable"
        onclick={() => toggleSort('protocol')}
        role="button"
        tabindex="0"
      >
        <span>Protocol</span>
        {#if sortColumn === 'protocol'}
          <ArrowUpDown
            size={12}
            class={sortOrder === 'desc' ? 'rotate-180' : ''}
          />
        {/if}
      </div>
      <div
        class="th sortable"
        onclick={() => toggleSort('process')}
        role="button"
        tabindex="0"
      >
        <span>Process</span>
        {#if sortColumn === 'process'}
          <ArrowUpDown
            size={12}
            class={sortOrder === 'desc' ? 'rotate-180' : ''}
          />
        {/if}
      </div>
      <div class="th">PID</div>
    </div>

    <div class="table-body">
      {#if isLoading}
        {#each Array(10) as _, idx (idx)}
          <div class="skeleton-row"></div>
        {/each}
      {:else if filteredConnections.length === 0}
        <div class="empty-state">
          <Link2 size={40} />
          <h3>No connections found</h3>
          <p>
            {#if searchQuery || protocolFilter !== 'all' || stateFilter !== 'all' || processFilter}
              Try adjusting your filters
            {:else}
              No active network connections detected
            {/if}
          </p>
        </div>
      {:else}
        {#each filteredConnections as conn (conn.timestamp)}
          <div class="table-row">
            <div class="cell">
              <span class="address-text"
                >{conn.localAddress}:{conn.localPort}</span
              >
            </div>
            <div class="cell">
              <span class="address-text"
                >{conn.remoteAddress}:{conn.remotePort}</span
              >
            </div>
            <div class="cell">
              <span class="state-badge state-{conn.state.toLowerCase()}">
                {conn.state}
              </span>
            </div>
            <div class="cell">
              <span class="protocol-badge {conn.protocol.toLowerCase()}">
                {conn.protocol}
              </span>
            </div>
            <div class="cell">
              <span class="process-name">{getProcessName(conn)}</span>
            </div>
            <div class="cell">
              <span class="pid-text">{conn.pid || '-'}</span>
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </div>

  <!-- Footer -->
  <div class="footer">
    <div class="footer-stats">
      Showing {filteredConnections.length} of {connections.length} connections
    </div>
  </div>
</div>

<style>
  .connections-page {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-primary);
    overflow: hidden;
  }

  /* Summary Cards - Compact Design */
  .summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: var(--space-md);
    padding: var(--space-md) var(--space-lg);
    border-bottom: 1px solid var(--border-color);
  }

  .summary-card {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-md);
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-xl);
    transition: all var(--transition-base);
    box-shadow:
      0 4px 16px rgba(0, 0, 0, 0.04),
      0 2px 4px rgba(0, 0, 0, 0.02);
  }

  .summary-card:hover {
    transform: translateY(-2px);
    border-color: var(--accent-primary);
    box-shadow:
      0 8px 24px rgba(0, 0, 0, 0.08),
      0 4px 8px rgba(0, 0, 0, 0.04);
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

  .card-icon.tcp {
    background: rgba(59, 130, 246, 0.1);
    color: rgb(59, 130, 246);
  }

  .card-icon.udp {
    background: rgba(168, 85, 247, 0.1);
    color: rgb(168, 85, 247);
  }

  .card-icon.established {
    background: rgba(34, 197, 94, 0.1);
    color: rgb(34, 197, 94);
  }

  .card-icon.listening {
    background: rgba(59, 130, 246, 0.1);
    color: rgb(59, 130, 246);
  }

  .card-content {
    flex: 1;
  }

  .card-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.8px;
    margin-bottom: 0.25rem;
  }

  .card-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
    line-height: 1;
  }

  /* Bandwidth Section - Compact Design */
  .bandwidth-section {
    padding: var(--space-md) var(--space-lg);
    margin-top: var(--space-lg);
    border-bottom: 1px solid var(--border-color);
  }

  .section-title {
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 var(--space-md) 0;
    letter-spacing: -0.02em;
  }

  .bandwidth-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .bandwidth-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-md);
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-xl);
    transition: all var(--transition-base);
    box-shadow:
      0 4px 16px rgba(0, 0, 0, 0.04),
      0 2px 4px rgba(0, 0, 0, 0.02);
  }

  .bandwidth-item:hover {
    transform: translateY(-2px);
    border-color: var(--accent-primary);
    box-shadow:
      0 8px 24px rgba(0, 0, 0, 0.08),
      0 4px 8px rgba(0, 0, 0, 0.04);
  }

  .process-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .process-name {
    font-weight: 700;
    font-size: 0.95rem;
    color: var(--text-primary);
  }

  .process-pid {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    font-family: monospace;
    background: rgba(59, 130, 246, 0.08);
    padding: 2px 8px;
    border-radius: var(--radius-md);
  }

  .bandwidth-stats {
    display: flex;
    gap: 1.5rem;
  }

  .stat {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--muted-foreground);
  }

  /* Filters - Compact Design */
  .filters-bar {
    display: flex;
    gap: var(--space-sm);
    padding: var(--space-md) var(--space-lg);
    background: var(--bg-primary);
    border-bottom: 1px solid var(--border-color);
  }

  .search-box {
    position: relative;
    flex: 1;
    max-width: 300px;
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-xs) var(--space-sm);
    height: 36px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
  }

  .search-box:hover {
    border-color: var(--accent-primary);
    background: var(--bg-hover);
  }

  .search-box:focus-within {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .search-box input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    font-size: 0.875rem;
    color: var(--text-primary);
    font-family: inherit;
  }

  .search-box input::placeholder {
    color: var(--text-tertiary);
  }

  .process-search {
    max-width: 200px;
  }

  .filter-controls {
    display: flex;
    gap: var(--space-sm);
    align-items: center;
  }

  select {
    padding: var(--space-xs) var(--space-sm);
    height: 36px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary);
    cursor: pointer;
    transition: all var(--transition-fast);
    font-family: inherit;
  }

  select:hover {
    border-color: var(--accent-primary);
    background: var(--bg-hover);
  }

  select:focus {
    outline: none;
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  /* Table - Compact Design */
  .table-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    padding: 0 var(--space-lg) var(--space-lg);
  }

  .table-header {
    display: grid;
    grid-template-columns: 1.5fr 1.5fr 1fr 0.8fr 1.2fr 0.8fr;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .th {
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .th.sortable {
    cursor: pointer;
    user-select: none;
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
  }

  .th.sortable:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .table-body {
    flex: 1;
    overflow-y: auto;
  }

  .table-row {
    display: grid;
    grid-template-columns: 1.5fr 1.5fr 1fr 0.8fr 1.2fr 0.8fr;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    border-bottom: 1px solid var(--border-color);
    transition: all var(--transition-fast);
    font-size: 0.875rem;
  }

  .table-row:hover {
    background: rgba(59, 130, 246, 0.05);
    border-left: 3px solid var(--accent-primary);
    padding-left: calc(var(--space-md) - 3px);
  }

  .cell {
    display: flex;
    align-items: center;
    overflow: hidden;
  }

  .address-text {
    font-family:
      'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas,
      'Courier New', monospace;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .protocol-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px 8px;
    border-radius: 0.5rem;
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    border: 1px solid;
  }

  .protocol-badge.tcp {
    background: rgba(59, 130, 246, 0.15);
    border-color: rgba(59, 130, 246, 0.3);
    color: rgb(96, 165, 250);
  }

  .protocol-badge.udp {
    background: rgba(168, 85, 247, 0.15);
    border-color: rgba(168, 85, 247, 0.3);
    color: rgb(192, 132, 252);
  }

  .state-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px 8px;
    border-radius: 0.5rem;
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    border: 1px solid;
  }

  .state-badge.state-established {
    background: rgba(34, 197, 94, 0.15);
    border-color: rgba(34, 197, 94, 0.3);
    color: rgb(74, 222, 128);
  }

  .state-badge.state-listen {
    background: rgba(59, 130, 246, 0.15);
    border-color: rgba(59, 130, 246, 0.3);
    color: rgb(96, 165, 250);
  }

  .state-badge.state-time_wait {
    background: rgba(251, 191, 36, 0.15);
    border-color: rgba(251, 191, 36, 0.3);
    color: rgb(252, 211, 77);
  }

  .state-badge.state-close_wait {
    background: rgba(239, 68, 68, 0.15);
    border-color: rgba(239, 68, 68, 0.3);
    color: rgb(248, 113, 113);
  }

  .process-name {
    font-weight: 600;
    font-size: 0.875rem;
    color: var(--foreground);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .pid-text {
    font-family: monospace;
    font-size: 0.8125rem;
    color: var(--muted-foreground);
  }

  /* Skeleton Loading */
  .skeleton-row {
    height: 56px;
    margin: 0 1.5rem;
    border-bottom: 1px solid var(--border);
    background: linear-gradient(
      90deg,
      var(--muted) 25%,
      var(--accent) 50%,
      var(--muted) 75%
    );
    background-size: 200% 100%;
    animation: skeleton-loading 1.5s ease-in-out infinite;
  }

  @keyframes skeleton-loading {
    0% {
      background-position: 200% 0;
    }
    100% {
      background-position: -200% 0;
    }
  }

  /* Empty State - Compact Design */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-2xl) var(--space-lg);
    text-align: center;
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border-radius: var(--radius-xl);
    border: 2px dashed var(--border-color);
    margin: var(--space-lg);
    transition: all var(--transition-base);
  }

  .empty-state:hover {
    border-color: var(--accent-primary);
    background: rgba(59, 130, 246, 0.02);
  }

  .empty-state h3 {
    margin-top: var(--space-md);
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .empty-state p {
    margin-top: var(--space-sm);
    font-size: 0.875rem;
    color: var(--text-secondary);
    line-height: 1.6;
  }

  /* Footer */
  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-md) var(--space-2xl);
    border-top: 1px solid var(--border-color);
    background: var(--bg-secondary);
  }

  .footer-stats {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.8px;
  }

  /* Error Banner */
  .error-banner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-lg) var(--space-2xl);
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

  /* Utilities */
  .rotate-180 {
    transform: rotate(180deg);
  }
</style>
