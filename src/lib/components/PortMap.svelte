<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { portStore } from '../stores/port.svelte';
  import type { SortBy, PortInfo } from '../types/port';
  import type { ServiceInfo } from '../types/service';
  import { detectService } from '../api/service-detection';
  import {
    RefreshCw,
    ArrowUpDown,
    Trash2,
    ServerOff,
    Globe,
    Lock,
    AlertTriangle,
    Wifi,
    ChevronDown,
    ChevronRight,
    X,
    Info
  } from 'lucide-svelte';
  import PortCountInfoModal from './PortMap/PortCountInfoModal.svelte';
  import BadgeInfoModal from './PortMap/BadgeInfoModal.svelte';
  import InfoBadge from './PortMap/InfoBadge.svelte';
  import PortMapFooter from './PortMap/PortMapFooter.svelte';
  import FilterSection from './PortMap/FilterSection.svelte';
  import ServiceBadge from './PortMap/ServiceBadge.svelte';
  import PageHeader from './PageHeader.svelte';
  import { Network } from 'lucide-svelte';

  let refreshInterval: number | null = null;
  let expandedGroups = $state<Set<string>>(new Set());
  let deleteModal = $state<{
    show: boolean;
    port: number;
    pid: number;
    processName: string;
    isSystemPort: boolean;
  } | null>(null);
  let isRefreshing = $state(false);
  let isPaused = $state(false);
  let showPortCountInfoModal = $state(false);
  let badgeInfoModal = $state<{
    show: boolean;
    badgeType:
      | 'public'
      | 'local'
      | 'network'
      | 'http'
      | 'listen'
      | 'established'
      | 'tcp'
      | 'udp'
      | null;
  }>({ show: false, badgeType: null });

  // Pagination state
  let currentPage = $state(1);
  let itemsPerPage = $state(20);

  interface PortGroup {
    port: number;
    pid: number;
    processName: string;
    protocol: string;
    state: string;
    category: string;
    connections: PortInfo[];
  }

  // Service detection cache: port -> ServiceInfo
  let detectedServices = $state<Map<number, ServiceInfo>>(new Map());

  type QuickFilter =
    | 'all'
    | 'development'
    | 'database'
    | 'system'
    | 'listen'
    | 'established';

  onMount(async () => {
    // Auto-scan on mount
    await portStore.scanPorts();
    // Detect services for discovered ports
    await detectServicesForPorts(portStore.ports);
  });

  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });

  // Re-detect services when ports change
  $effect(() => {
    if (portStore.ports.length > 0) {
      detectServicesForPorts(portStore.ports);
    }
  });

  let activeQuickFilter = $state<QuickFilter>('all');

  async function detectServicesForPorts(ports: PortInfo[]) {
    // Detect services for all unique ports
    // eslint-disable-next-line svelte/prefer-svelte-reactivity
    const uniquePorts = new Map<number, PortInfo>();
    for (const port of ports) {
      if (!uniquePorts.has(port.port)) {
        uniquePorts.set(port.port, port);
      }
    }

    // Run detection in parallel
    const detectionPromises = Array.from(uniquePorts.values()).map(
      async (portInfo) => {
        const service = await detectService(
          portInfo.port,
          portInfo.pid,
          portInfo.processName,
          undefined // TODO: Get command from process info
        );

        return service ? { port: portInfo.port, service } : null;
      }
    );

    const results = await Promise.all(detectionPromises);

    // Update detectedServices with results
    for (const result of results) {
      if (result) {
        detectedServices.set(result.port, result.service);
      }
    }
  }

  async function handleRefresh() {
    isRefreshing = true;
    await portStore.scanPorts(true);

    // Detect services after port scan
    await detectServicesForPorts(portStore.ports);

    setTimeout(() => {
      isRefreshing = false;
    }, 600);
  }

  function openDeleteModal(port: number, pid: number, processName: string) {
    const isSystemPort = port < 1024;
    deleteModal = { show: true, port, pid, processName, isSystemPort };
  }

  function closeDeleteModal() {
    deleteModal = null;
  }

  async function confirmDelete() {
    if (deleteModal) {
      await portStore.killProcessByPort(deleteModal.port);
      closeDeleteModal();
    }
  }

  function applyQuickFilter(filter: QuickFilter) {
    activeQuickFilter = filter;

    // Reset all filters first
    portStore.resetFilters();

    // Apply the selected quick filter
    switch (filter) {
      case 'development':
        portStore.categoryFilter = 'Development';
        break;
      case 'database':
        portStore.categoryFilter = 'Database';
        break;
      case 'system':
        portStore.categoryFilter = 'System';
        break;
      case 'listen':
        portStore.stateFilter = 'Listen';
        break;
      case 'established':
        portStore.stateFilter = 'Established';
        break;
      case 'all':
      default:
        // All filters already reset
        break;
    }
  }

  function handleSort(column: SortBy) {
    portStore.toggleSort(column);
  }

  function getPortCategory(port: number, processName: string): string {
    // Well-known system ports
    if (port < 1024) return 'System';
    // Common development ports
    if (
      [3000, 3001, 3002, 4200, 5000, 5173, 8000, 8080, 8888, 9000].includes(
        port
      )
    ) {
      return 'Development';
    }
    // Database ports
    if ([5432, 3306, 27017, 6379, 5984].includes(port)) return 'Database';
    // Common service ports
    if (
      processName.includes('postgres') ||
      processName.includes('mongo') ||
      processName.includes('redis')
    ) {
      return 'Database';
    }
    return 'Application';
  }

  // Group ports by port number and PID
  let groupedPorts = $derived.by(() => {
    // eslint-disable-next-line svelte/prefer-svelte-reactivity
    const groupsMap = new Map<string, PortGroup>();

    for (const port of portStore.filteredPorts) {
      const key = `${port.port}-${port.pid}`;

      if (groupsMap.has(key)) {
        groupsMap.get(key)!.connections.push(port);
      } else {
        groupsMap.set(key, {
          port: port.port,
          pid: port.pid,
          processName: port.processName,
          protocol: port.protocol,
          state: port.state,
          category: getPortCategory(port.port, port.processName),
          connections: [port]
        });
      }
    }

    return Array.from(groupsMap.values());
  });

  // Paginated ports
  let paginatedPorts = $derived.by(() => {
    const startIndex = (currentPage - 1) * itemsPerPage;
    const endIndex = startIndex + itemsPerPage;
    return groupedPorts.slice(startIndex, endIndex);
  });

  // Total pages
  let totalPages = $derived(Math.ceil(groupedPorts.length / itemsPerPage));

  // Virtual scrolling state
  let scrollContainer: HTMLDivElement | null = null;
  let visibleStartIndex = $state(0);
  let visibleEndIndex = $state(20);
  const ITEM_HEIGHT = 64; // Minimum height per row
  const BUFFER_SIZE = 5; // Render extra items above/below viewport

  // Calculate visible range based on scroll position
  function handleScroll(e: Event) {
    if (!scrollContainer) return;

    const scrollTop = (e.target as HTMLDivElement).scrollTop;
    const viewportHeight = scrollContainer.clientHeight;

    const start = Math.max(
      0,
      Math.floor(scrollTop / ITEM_HEIGHT) - BUFFER_SIZE
    );
    const end = Math.min(
      paginatedPorts.length,
      Math.ceil((scrollTop + viewportHeight) / ITEM_HEIGHT) + BUFFER_SIZE
    );

    visibleStartIndex = start;
    visibleEndIndex = end;
  }

  // Reset scroll when page changes
  $effect(() => {
    if (currentPage) {
      visibleStartIndex = 0;
      visibleEndIndex = 20;
      if (scrollContainer) {
        scrollContainer.scrollTop = 0;
      }
    }
  });

  // Virtual scrolled ports (only render visible + buffer)
  let virtualPorts = $derived.by(() => {
    // Only use virtual scrolling if there are many items
    if (paginatedPorts.length <= 50) {
      return paginatedPorts;
    }
    return paginatedPorts.slice(visibleStartIndex, visibleEndIndex);
  });

  // Calculate spacer heights for virtual scrolling
  let topSpacerHeight = $derived(visibleStartIndex * ITEM_HEIGHT);
  let bottomSpacerHeight = $derived(
    Math.max(0, (paginatedPorts.length - visibleEndIndex) * ITEM_HEIGHT)
  );

  function toggleGroup(key: string) {
    // eslint-disable-next-line svelte/prefer-svelte-reactivity
    const updated = new Set(expandedGroups);
    if (updated.has(key)) {
      updated.delete(key);
    } else {
      updated.add(key);
    }
    expandedGroups = updated;
  }

  function getGroupKey(group: PortGroup): string {
    return `${group.port}-${group.pid}`;
  }

  function openPortCountInfoModal() {
    showPortCountInfoModal = true;
  }

  function openBadgeInfoModal(
    type:
      | 'public'
      | 'local'
      | 'network'
      | 'http'
      | 'listen'
      | 'established'
      | 'tcp'
      | 'udp'
  ) {
    badgeInfoModal = { show: true, badgeType: type };
  }

  function closeBadgeInfoModal() {
    badgeInfoModal = { show: false, badgeType: null };
  }

  function togglePause() {
    isPaused = !isPaused;
    // You can add logic here to actually pause/resume scanning if needed
  }

  // Pagination handlers
  function handlePageChange(page: number) {
    currentPage = page;
  }

  function handlePageSizeChange(size: number) {
    itemsPerPage = size;
    currentPage = 1; // Reset to first page when changing page size
  }
</script>

<div class="port-map" role="region" aria-label="Network port discovery">
  <PageHeader
    title="Port Map"
    subtitle="Monitor network ports and service discovery"
    icon={Network}
  >
    <div class="header-stats">
      <span class="stat-badge stat-total">
        {portStore.stats.total} ports
        <button
          class="info-icon-btn"
          onclick={openPortCountInfoModal}
          aria-label="Port count information"
        >
          <Info size={14} />
        </button>
      </span>
      <span class="stat-badge stat-tcp">
        {portStore.stats.tcp} TCP
        <InfoBadge onClick={() => openBadgeInfoModal('tcp')} />
      </span>
      <span class="stat-badge stat-udp">
        {portStore.stats.udp} UDP
        <InfoBadge onClick={() => openBadgeInfoModal('udp')} />
      </span>
    </div>

    <button
      class="btn-refresh"
      onclick={handleRefresh}
      disabled={portStore.loading || isRefreshing}
      aria-label="Refresh port list"
    >
      <RefreshCw
        size={16}
        class={portStore.loading || isRefreshing ? 'animate-spin' : ''}
      />
      Refresh
    </button>
  </PageHeader>

  <!-- Quick Filter Tabs -->
  <div class="quick-filters" role="tablist" aria-label="Port category filters">
    <button
      class="filter-tab {activeQuickFilter === 'all' ? 'active' : ''}"
      role="tab"
      aria-selected={activeQuickFilter === 'all'}
      aria-controls="port-list"
      onclick={() => applyQuickFilter('all')}
    >
      All Ports
    </button>
    <button
      class="filter-tab {activeQuickFilter === 'development' ? 'active' : ''}"
      role="tab"
      aria-selected={activeQuickFilter === 'development'}
      aria-controls="port-list"
      onclick={() => applyQuickFilter('development')}
    >
      Development
    </button>
    <button
      class="filter-tab {activeQuickFilter === 'database' ? 'active' : ''}"
      role="tab"
      aria-selected={activeQuickFilter === 'database'}
      aria-controls="port-list"
      onclick={() => applyQuickFilter('database')}
    >
      Database
    </button>
    <button
      class="filter-tab {activeQuickFilter === 'system' ? 'active' : ''}"
      role="tab"
      aria-selected={activeQuickFilter === 'system'}
      aria-controls="port-list"
      onclick={() => applyQuickFilter('system')}
    >
      System
    </button>
    <button
      class="filter-tab {activeQuickFilter === 'listen' ? 'active' : ''}"
      role="tab"
      aria-selected={activeQuickFilter === 'listen'}
      aria-controls="port-list"
      onclick={() => applyQuickFilter('listen')}
    >
      Listening
    </button>
    <button
      class="filter-tab {activeQuickFilter === 'established' ? 'active' : ''}"
      role="tab"
      aria-selected={activeQuickFilter === 'established'}
      aria-controls="port-list"
      onclick={() => applyQuickFilter('established')}
    >
      Active Connections
    </button>
  </div>

  <!-- Collapsible Filter Section -->
  <div class="filter-section-wrapper">
    <FilterSection
      searchQuery={portStore.searchQuery}
      onSearchChange={(value) => (portStore.searchQuery = value)}
    />
  </div>

  <!-- Error State -->
  {#if portStore.error}
    <div class="error-banner" role="alert">
      <p>{portStore.error}</p>
      <button onclick={handleRefresh}>Retry</button>
    </div>
  {/if}

  <!-- Port Table -->
  <div class="table-container" role="region" aria-label="Port list table">
    <div class="table-header" role="row">
      <div class="th-expand" role="columnheader" aria-label="Expand"></div>
      <div
        class="th th-port sortable"
        role="columnheader"
        aria-sort={portStore.sortBy === 'port'
          ? portStore.sortOrder === 'asc'
            ? 'ascending'
            : 'descending'
          : 'none'}
        onclick={() => handleSort('port')}
        onkeydown={(e) =>
          (e.key === 'Enter' || e.key === ' ') && handleSort('port')}
        tabindex="0"
        aria-label="Port number, sortable"
      >
        <span>Port</span>
        {#if portStore.sortBy === 'port'}
          <ArrowUpDown
            size={12}
            class={portStore.sortOrder === 'desc' ? 'rotate-180' : ''}
            aria-hidden="true"
          />
        {/if}
      </div>
      <div
        class="th th-protocol"
        role="columnheader"
        aria-label="Protocol type"
      >
        Protocol
      </div>
      <div
        class="th th-process sortable"
        role="columnheader"
        aria-sort={portStore.sortBy === 'process'
          ? portStore.sortOrder === 'asc'
            ? 'ascending'
            : 'descending'
          : 'none'}
        onclick={() => handleSort('process')}
        onkeydown={(e) =>
          (e.key === 'Enter' || e.key === ' ') && handleSort('process')}
        tabindex="0"
        aria-label="Process name, sortable"
      >
        <span>Process</span>
        {#if portStore.sortBy === 'process'}
          <ArrowUpDown
            size={12}
            class={portStore.sortOrder === 'desc' ? 'rotate-180' : ''}
            aria-hidden="true"
          />
        {/if}
      </div>
      <div class="th th-pid" role="columnheader" aria-label="Process ID">
        PID
      </div>
      <div
        class="th th-state"
        role="columnheader"
        aria-label="Connection state"
      >
        State
      </div>
      <div
        class="th th-address"
        role="columnheader"
        aria-label="Network address"
      >
        Address
      </div>
      <div
        class="th th-info"
        role="columnheader"
        aria-label="Additional information"
      >
        Info
      </div>
      <div
        class="th th-actions"
        role="columnheader"
        aria-label="Available actions"
      >
        Actions
      </div>
    </div>

    <!-- Scrolling Container -->
    <div
      class="scroll-container"
      bind:this={scrollContainer}
      onscroll={handleScroll}
    >
      {#if portStore.loading && portStore.ports.length === 0}
        <!-- Skeleton Loading -->
        {#each Array(10) as _, idx (idx)}
          <div class="skeleton-row" aria-busy="true" aria-label="Loading ports">
            <div class="skeleton-cell"></div>
            <div class="skeleton-cell"></div>
            <div class="skeleton-cell"></div>
          </div>
        {/each}
      {:else if groupedPorts.length === 0}
        <!-- Empty State -->
        <div class="empty-state">
          <ServerOff size={48} class="text-muted-foreground mb-4" />
          <h3 class="text-xl font-semibold mb-2">No ports found</h3>
          <p class="text-muted-foreground">
            {#if portStore.searchQuery || portStore.protocolFilter !== 'all' || portStore.stateFilter !== 'all'}
              Try adjusting your filters
            {:else}
              No active network ports detected
            {/if}
          </p>
        </div>
      {:else}
        <!-- Port Groups -->
        <div class="port-list" id="port-list" role="tabpanel">
          <!-- Top spacer for virtual scrolling -->
          {#if paginatedPorts.length > 50 && topSpacerHeight > 0}
            <div style="height: {topSpacerHeight}px;" aria-hidden="true"></div>
          {/if}

          {#each virtualPorts as group (getGroupKey(group))}
            {@const groupKey = getGroupKey(group)}
            {@const isExpanded = expandedGroups.has(groupKey)}
            {@const hasMultiple = group.connections.length > 1}
            {@const mainConnection = group.connections[0]}

            <!-- Group Header Row -->
            <div
              class="port-row {hasMultiple ? 'group-header' : ''}"
              role="row"
              aria-label="Port {group.port}, {group.processName}, {group.state}"
            >
              <div class="cell-expand" role="cell">
                {#if hasMultiple}
                  <button
                    class="expand-btn"
                    onclick={() => toggleGroup(groupKey)}
                    aria-expanded={isExpanded}
                    aria-label={isExpanded
                      ? 'Collapse connections'
                      : 'Expand connections'}
                    aria-controls="group-{groupKey}"
                  >
                    {#if isExpanded}
                      <ChevronDown size={14} aria-hidden="true" />
                    {:else}
                      <ChevronRight size={14} aria-hidden="true" />
                    {/if}
                  </button>
                {/if}
              </div>
              <div class="cell-port-main">
                <div class="port-info-wrapper">
                  <div class="port-header">
                    <span class="port-number">{group.port}</span>
                    <span
                      class="category-badge category-{group.category.toLowerCase()}"
                      >{group.category}</span
                    >
                  </div>
                  {#if hasMultiple}
                    <span class="connection-count"
                      >{group.connections.length} connections</span
                    >
                  {/if}
                </div>
              </div>
              <div class="cell-protocol">
                <span class="protocol-badge {group.protocol.toLowerCase()}"
                  >{group.protocol}</span
                >
              </div>
              <div class="cell-process">
                <span class="process-name">{group.processName}</span>
                {#if detectedServices.has(group.port)}
                  <ServiceBadge
                    service={detectedServices.get(group.port)!}
                    size="sm"
                  />
                {/if}
              </div>
              <div class="cell-pid">
                <span class="pid-number">{group.pid}</span>
              </div>
              <div class="cell-state">
                <span class="state-badge state-{group.state.toLowerCase()}">
                  {#if group.state === 'Listen'}
                    <Wifi size={12} />
                    Listen
                    <InfoBadge onClick={() => openBadgeInfoModal('listen')} />
                  {:else if group.state === 'Established'}
                    <Globe size={12} />
                    Established
                    <InfoBadge
                      onClick={() => openBadgeInfoModal('established')}
                    />
                  {:else}
                    <AlertTriangle size={12} />
                    {group.state}
                  {/if}
                </span>
              </div>
              <div class="cell-address">
                {#if hasMultiple && !isExpanded}
                  <div class="address-text">Multiple addresses</div>
                {:else}
                  <div class="address-text">
                    {mainConnection.localAddress === '*'
                      ? '0.0.0.0'
                      : mainConnection.localAddress}:{group.port}
                  </div>
                  {#if mainConnection.remoteAddress}
                    <div class="address-text remote">
                      → {mainConnection.remoteAddress}
                    </div>
                  {/if}
                {/if}
              </div>
              <div class="cell-info">
                <div class="info-badges">
                  {#if mainConnection.localAddress === '0.0.0.0' || mainConnection.localAddress === '*'}
                    <span class="info-badge warning">
                      <AlertTriangle size={10} />
                      Public
                      <InfoBadge onClick={() => openBadgeInfoModal('public')} />
                    </span>
                  {:else if mainConnection.localAddress === '127.0.0.1'}
                    <span class="info-badge safe">
                      <Lock size={10} />
                      Local
                      <InfoBadge onClick={() => openBadgeInfoModal('local')} />
                    </span>
                  {:else}
                    <span class="info-badge">
                      <Globe size={10} />
                      Network
                      <InfoBadge
                        onClick={() => openBadgeInfoModal('network')}
                      />
                    </span>
                  {/if}
                  {#if [80, 443, 3000, 3001, 3002, 4200, 5000, 5173, 8000, 8080, 8888, 9000].includes(group.port)}
                    <span class="info-badge http">
                      <Wifi size={10} />
                      HTTP
                      <InfoBadge onClick={() => openBadgeInfoModal('http')} />
                    </span>
                  {/if}
                </div>
              </div>
              <div class="cell-actions">
                <button
                  class="btn-icon-sm"
                  onclick={() =>
                    openDeleteModal(group.port, group.pid, group.processName)}
                  aria-label="Kill process {group.processName}"
                >
                  <Trash2 size={14} />
                </button>
              </div>
            </div>

            <!-- Expanded Connections -->
            {#if hasMultiple && isExpanded}
              {#each group.connections as connection, idx (idx)}
                <div class="port-row sub-connection">
                  <div class="cell-expand">
                    <div class="sub-indicator"></div>
                  </div>
                  <div class="cell-port-main">
                    <span class="sub-connection-label"
                      >Connection {idx + 1}</span
                    >
                  </div>
                  <div class="cell-protocol">
                    <span
                      class="protocol-badge {connection.protocol.toLowerCase()}"
                      >{connection.protocol}</span
                    >
                  </div>
                  <div class="cell-process">
                    <span class="process-name sub"
                      >{connection.processName}</span
                    >
                  </div>
                  <div class="cell-pid">
                    <span class="pid-number sub">{connection.pid}</span>
                  </div>
                  <div class="cell-state">
                    <span
                      class="state-badge state-{connection.state.toLowerCase()}"
                    >
                      {#if connection.state === 'Listen'}
                        <Wifi size={12} />
                      {:else if connection.state === 'Established'}
                        <Globe size={12} />
                      {:else}
                        <AlertTriangle size={12} />
                      {/if}
                      {connection.state}
                    </span>
                  </div>
                  <div class="cell-address">
                    <div class="address-text">
                      {connection.localAddress === '*'
                        ? '0.0.0.0'
                        : connection.localAddress}:{connection.port}
                    </div>
                    {#if connection.remoteAddress}
                      <div class="address-text remote">
                        → {connection.remoteAddress}
                      </div>
                    {/if}
                  </div>
                  <div class="cell-info">
                    <div class="info-badges">
                      {#if connection.localAddress === '0.0.0.0' || connection.localAddress === '*'}
                        <span class="info-badge warning">
                          <AlertTriangle size={10} />
                          Public
                        </span>
                      {:else if connection.localAddress === '127.0.0.1'}
                        <span class="info-badge safe">
                          <Lock size={10} />
                          Local
                        </span>
                      {:else}
                        <span class="info-badge">
                          <Globe size={10} />
                          Network
                        </span>
                      {/if}
                    </div>
                  </div>
                  <div class="cell-actions">
                    <button
                      class="btn-icon-sm sub"
                      onclick={() =>
                        openDeleteModal(
                          connection.port,
                          connection.pid,
                          connection.processName
                        )}
                      aria-label="Kill connection"
                    >
                      <Trash2 size={14} />
                    </button>
                  </div>
                </div>
              {/each}
            {/if}
          {/each}

          <!-- Bottom spacer for virtual scrolling -->
          {#if paginatedPorts.length > 50 && bottomSpacerHeight > 0}
            <div style="height: {bottomSpacerHeight}px;"></div>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <!-- Footer with Pagination -->
  <PortMapFooter
    lastScan={portStore.lastScan}
    groupCount={groupedPorts.length}
    connectionCount={portStore.filteredPorts.length}
    totalPorts={portStore.stats.total}
    {isPaused}
    onTogglePause={togglePause}
    {currentPage}
    {totalPages}
    totalItems={groupedPorts.length}
    {itemsPerPage}
    onPageChange={handlePageChange}
    onPageSizeChange={handlePageSizeChange}
  />
</div>

<!-- Delete Confirmation Modal -->
{#if deleteModal?.show}
  <div class="modal-overlay" onclick={closeDeleteModal}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div
          class="modal-icon-wrapper {deleteModal.isSystemPort
            ? 'danger'
            : 'warning'}"
        >
          <AlertTriangle size={24} />
        </div>
        <h3 class="modal-title">
          {deleteModal.isSystemPort ? 'Kill System Process?' : 'Kill Process?'}
        </h3>
        <button
          class="modal-close"
          onclick={closeDeleteModal}
          aria-label="Close modal"
        >
          <X size={20} />
        </button>
      </div>

      <div class="modal-body">
        {#if deleteModal.isSystemPort}
          <div class="warning-banner">
            <AlertTriangle size={16} />
            <span
              >Warning: This is a system port (&lt; 1024). Killing this process
              may cause system instability.</span
            >
          </div>
        {/if}

        <div class="process-details">
          <div class="detail-row">
            <span class="detail-label">Process</span>
            <span class="detail-value">{deleteModal.processName}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Port</span>
            <span class="detail-value">{deleteModal.port}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">PID</span>
            <span class="detail-value">{deleteModal.pid}</span>
          </div>
        </div>

        <p class="modal-description">
          {#if deleteModal.isSystemPort}
            This action will terminate a system-level process. This may affect
            critical system services or applications. Are you absolutely sure
            you want to continue?
          {:else}
            This action will terminate the process and free up the port. The
            process may restart automatically if managed by a service manager.
          {/if}
        </p>
      </div>

      <div class="modal-footer">
        <button class="btn-modal btn-cancel" onclick={closeDeleteModal}>
          Cancel
        </button>
        <button
          class="btn-modal btn-delete {deleteModal.isSystemPort
            ? 'danger'
            : ''}"
          onclick={confirmDelete}
        >
          {deleteModal.isSystemPort ? 'Force Kill Process' : 'Kill Process'}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Modals -->
<PortCountInfoModal
  show={showPortCountInfoModal}
  onClose={() => (showPortCountInfoModal = false)}
/>

<BadgeInfoModal
  show={badgeInfoModal.show}
  badgeType={badgeInfoModal.badgeType}
  onClose={closeBadgeInfoModal}
/>

<style>
  .port-map {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--background);
    overflow: hidden;
  }

  /* Header */
  .header-stats {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .stat-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.25rem 0.75rem;
    border-radius: 0.5rem;
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.025em;
  }

  .info-icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    background: transparent;
    border: none;
    color: inherit;
    opacity: 0.6;
    cursor: pointer;
    transition: all 0.2s;
  }

  .info-icon-btn:hover {
    opacity: 1;
    transform: scale(1.1);
  }

  .stat-total {
    background: rgba(100, 116, 139, 0.1);
    color: rgb(100, 116, 139);
  }

  .stat-tcp {
    background: rgba(59, 130, 246, 0.1);
    color: rgb(59, 130, 246);
  }

  .stat-udp {
    background: rgba(168, 85, 247, 0.1);
    color: rgb(168, 85, 247);
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

  .btn-refresh :global(.lucide-refresh-cw) {
    transition: transform 0.6s ease-in-out;
  }

  .btn-refresh :global(.animate-spin) {
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

  /* Quick Filter Tabs */
  .quick-filters {
    display: flex;
    gap: 0.5rem;
    padding: 1rem 1.5rem;
    background: var(--background);
    overflow-x: auto;
    scrollbar-width: none;
    border-bottom: 1px solid var(--border);
  }

  .quick-filters::-webkit-scrollbar {
    display: none;
  }

  .filter-tab {
    display: inline-flex;
    align-items: center;
    padding: 0.5rem 1rem;
    background: transparent;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 600;
    color: var(--muted-foreground);
    cursor: pointer;
    transition: all 0.15s;
    white-space: nowrap;
    position: relative;
  }

  .filter-tab:hover {
    background: var(--accent);
    color: var(--foreground);
  }

  .filter-tab.active {
    background: var(--foreground);
    color: var(--background);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  /* Filter Section Wrapper */
  .filter-section-wrapper {
    padding: 0.5rem 1.5rem 0.75rem 1.5rem;
    background: var(--background);
    border-bottom: 1px solid var(--border);
  }

  /* Toolbar */
  .toolbar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem 1.5rem;
    background: var(--background);
    border-bottom: 1px solid var(--border);
  }

  .search-box {
    position: relative;
    flex: 1;
    max-width: 400px;
  }

  .search-icon {
    position: absolute;
    left: 0.75rem;
    top: 50%;
    transform: translateY(-50%);
    color: var(--muted-foreground);
    pointer-events: none;
    display: flex;
    align-items: center;
  }

  .search-box input {
    width: 100%;
    padding: 0.625rem 0.875rem 0.625rem 2.5rem;
    background: var(--muted);
    border: 1px solid transparent;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    color: var(--foreground);
    transition: all 0.2s;
  }

  .search-box input::placeholder {
    color: var(--muted-foreground);
  }

  .search-box input:hover {
    border-color: var(--border);
  }

  .search-box input:focus {
    outline: none;
    background: var(--background);
    border-color: var(--foreground);
    box-shadow: 0 0 0 1px var(--foreground);
  }

  .filter-controls {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .filter-group {
    position: relative;
  }

  .filter-select {
    padding: 0.625rem 2rem 0.625rem 0.875rem;
    background: var(--muted);
    border: 1px solid transparent;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    color: var(--foreground);
    cursor: pointer;
    transition: all 0.2s;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg width='12' height='12' viewBox='0 0 12 12' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M3 5L6 8L9 5' stroke='%23666' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 0.625rem center;
    min-width: 140px;
  }

  .filter-select:hover {
    border-color: var(--border);
  }

  .filter-select:focus {
    outline: none;
    background: var(--background);
    border-color: var(--foreground);
    box-shadow: 0 0 0 1px var(--foreground);
  }

  .btn-clear {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.625rem 0.875rem;
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--muted-foreground);
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-clear:hover {
    background: var(--background);
    border-color: var(--primary);
    color: var(--primary);
  }

  /* Table */
  .table-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* Table Header */
  .table-header {
    display: grid;
    grid-template-columns:
      40px 100px 80px minmax(150px, 1.5fr)
      80px 140px minmax(180px, 1.2fr) minmax(140px, 1fr) 70px;
    gap: 0.75rem;
    background: var(--muted);
    border-bottom: 1px solid var(--border);
    position: sticky;
    top: 0;
    z-index: 10;
    align-items: center;
    padding: 0 1rem;
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
  }

  .th-expand {
    width: 40px;
  }

  .th {
    padding: 1rem 0.75rem;
    font-size: 0.6875rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--muted-foreground);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .th.sortable {
    cursor: pointer;
    user-select: none;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 0.375rem;
    border-radius: 0.5rem;
    padding: 1rem 0.75rem;
  }

  .th.sortable:hover {
    background: var(--accent);
    color: var(--foreground);
  }

  .th-port {
    justify-content: flex-start;
  }

  .th-pid {
    text-align: right;
    justify-content: flex-end;
  }

  .th-actions {
    text-align: center;
    justify-content: center;
  }

  .scroll-container {
    flex: 1;
    overflow-y: auto;
    position: relative;
  }

  .port-list {
    overflow-y: auto;
  }

  /* Port Rows */
  .port-row {
    display: grid;
    grid-template-columns:
      40px 100px 80px minmax(150px, 1.5fr)
      80px 140px minmax(180px, 1.2fr) minmax(140px, 1fr) 70px;
    gap: 0.75rem;
    align-items: center;
    border-bottom: 1px solid var(--border);
    transition: all 0.2s ease;
    min-height: 64px;
    padding: 0 1rem;
    position: relative;
  }

  .port-row:hover {
    background: rgba(59, 130, 246, 0.05);
    border-left: 2px solid var(--primary);
    padding-left: calc(1rem - 2px);
  }

  .port-row:focus {
    outline: none;
    background: var(--accent);
  }

  .port-row.group-header {
    background: var(--muted);
    border-left: 2px solid var(--border);
    padding-left: calc(1rem - 2px);
  }

  .port-row.group-header:hover {
    background: rgba(59, 130, 246, 0.08);
    border-left: 2px solid var(--primary);
  }

  .port-row.sub-connection {
    background: transparent;
    border-left: 3px solid var(--primary);
    opacity: 0.8;
    padding-left: calc(1rem - 3px);
    min-height: 56px;
  }

  .port-row.sub-connection:hover {
    background: rgba(59, 130, 246, 0.05);
    opacity: 1;
  }

  /* Cell Components */
  .cell-expand {
    width: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .cell-port-main {
    overflow: hidden;
  }

  .cell-protocol {
    display: flex;
    align-items: center;
  }

  .cell-process {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 8px;
    overflow: hidden;
    flex-wrap: wrap;
  }

  .cell-pid {
    text-align: right;
    display: flex;
    justify-content: flex-end;
    align-items: center;
  }

  .cell-state {
    display: flex;
    align-items: center;
  }

  .cell-address {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    padding-left: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .cell-info {
    display: flex;
    align-items: center;
  }

  .cell-actions {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .port-info-wrapper {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.5rem 0;
  }

  .port-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .sub-indicator {
    width: 3px;
    height: 24px;
    background: var(--primary);
    border-radius: 2px;
    opacity: 0.5;
  }

  .sub-connection-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--muted-foreground);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .process-name.sub,
  .pid-number.sub {
    font-size: 0.75rem;
    opacity: 0.8;
  }

  .btn-icon-sm.sub {
    opacity: 0.7;
  }

  .btn-icon-sm.sub:hover {
    opacity: 1;
  }

  /* Interactive Elements */
  .expand-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: transparent;
    border: none;
    color: var(--muted-foreground);
    cursor: pointer;
    border-radius: 0.375rem;
    transition: all 0.2s;
  }

  .expand-btn:hover {
    background: var(--accent);
    color: var(--foreground);
    transform: scale(1.1);
  }

  /* Text Elements */
  .port-number {
    font-family:
      'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-weight: 700;
    font-size: 1rem;
    color: var(--foreground);
    letter-spacing: -0.01em;
  }

  .connection-count {
    font-size: 0.6875rem;
    color: var(--muted-foreground);
    font-weight: 600;
    letter-spacing: 0.025em;
  }

  .pid-number {
    font-family:
      'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-size: 0.8125rem;
    color: var(--muted-foreground);
    font-weight: 600;
    letter-spacing: -0.01em;
  }

  .process-name {
    font-weight: 600;
    font-size: 0.875rem;
    color: var(--foreground);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    letter-spacing: -0.01em;
  }

  .address-text {
    font-family:
      'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-size: 0.8125rem;
    color: var(--foreground);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    letter-spacing: -0.01em;
  }

  .address-text.remote {
    color: var(--muted-foreground);
    font-size: 0.75rem;
    margin-top: 0.25rem;
  }

  .info-badges {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
  }

  .category-badge {
    display: inline-flex;
    padding: 0.25rem 0.625rem;
    border-radius: 0.375rem;
    font-size: 0.625rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border: 1px solid;
    transition: all 0.2s;
  }

  .category-development {
    background: rgba(34, 197, 94, 0.15);
    border-color: rgba(34, 197, 94, 0.3);
    color: rgb(74, 222, 128);
  }

  .category-database {
    background: rgba(168, 85, 247, 0.15);
    border-color: rgba(168, 85, 247, 0.3);
    color: rgb(192, 132, 252);
  }

  .category-system {
    background: rgba(239, 68, 68, 0.15);
    border-color: rgba(239, 68, 68, 0.3);
    color: rgb(248, 113, 113);
  }

  .category-application {
    background: rgba(59, 130, 246, 0.15);
    border-color: rgba(59, 130, 246, 0.3);
    color: rgb(96, 165, 250);
  }

  /* Badges */
  .protocol-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.375rem 0.75rem;
    border-radius: 0.5rem;
    font-size: 0.6875rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    border: 1px solid;
    transition: all 0.2s;
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
    gap: 0.375rem;
    font-size: 0.6875rem;
    padding: 0.375rem 0.75rem;
    border-radius: 0.5rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border: 1px solid;
    transition: all 0.2s;
  }

  .state-badge.state-listen {
    background: rgba(59, 130, 246, 0.15);
    border-color: rgba(59, 130, 246, 0.3);
    color: rgb(96, 165, 250);
  }

  .state-badge.state-established {
    background: rgba(34, 197, 94, 0.15);
    border-color: rgba(34, 197, 94, 0.3);
    color: rgb(74, 222, 128);
  }

  .state-badge.state-timewait {
    background: rgba(251, 191, 36, 0.15);
    border-color: rgba(251, 191, 36, 0.3);
    color: rgb(252, 211, 77);
  }

  .state-badge.state-closewait {
    background: rgba(239, 68, 68, 0.15);
    border-color: rgba(239, 68, 68, 0.3);
    color: rgb(248, 113, 113);
  }

  .state-badge.state-unknown {
    background: rgba(100, 116, 139, 0.15);
    border-color: rgba(100, 116, 139, 0.3);
    color: rgb(148, 163, 184);
  }

  .info-badge {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.625rem;
    border-radius: 0.375rem;
    font-size: 0.625rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    gap: 0.25rem;
    border: 1px solid;
    transition: all 0.2s;
  }

  .info-badge.warning {
    background: rgba(251, 191, 36, 0.15);
    border-color: rgba(251, 191, 36, 0.3);
    color: rgb(252, 211, 77);
  }

  .info-badge.safe {
    background: rgba(34, 197, 94, 0.15);
    border-color: rgba(34, 197, 94, 0.3);
    color: rgb(74, 222, 128);
  }

  .info-badge.http {
    background: rgba(59, 130, 246, 0.15);
    border-color: rgba(59, 130, 246, 0.3);
    color: rgb(96, 165, 250);
  }

  .info-badge {
    background: rgba(100, 116, 139, 0.15);
    border-color: rgba(100, 116, 139, 0.3);
    color: rgb(148, 163, 184);
  }

  /* Buttons */
  .btn-icon-sm {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem;
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.2s ease;
    color: var(--muted-foreground);
  }

  .btn-icon-sm:hover {
    background: rgba(239, 68, 68, 0.15);
    border-color: rgba(239, 68, 68, 0.4);
    color: #ef4444;
    transform: scale(1.05);
  }

  .btn-icon-sm:active {
    transform: scale(0.95);
  }

  /* Skeleton Loading */
  .skeleton-row {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 1rem;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid var(--border);
  }

  .skeleton-cell {
    height: 20px;
    background: linear-gradient(
      90deg,
      var(--muted) 25%,
      var(--accent) 50%,
      var(--muted) 75%
    );
    background-size: 200% 100%;
    border-radius: 0.25rem;
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

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
  }

  /* Error Banner */
  .error-banner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    background: #fee;
    border-bottom: 1px solid #fcc;
    color: #c00;
  }

  /* Footer */
  .footer {
    display: flex;
    justify-content: space-between;
    padding: 0.75rem 1.5rem;
    border-top: 1px solid var(--border);
    background: var(--muted);
  }

  /* Modal Styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.15s ease;
  }

  .modal-content {
    background: var(--background);
    border: 1px solid var(--border);
    border-radius: 0.75rem;
    box-shadow:
      0 0 0 1px var(--border),
      0 20px 50px rgba(0, 0, 0, 0.3),
      0 10px 30px rgba(0, 0, 0, 0.15);
    max-width: 420px;
    width: 90%;
    animation: slideUp 0.25s cubic-bezier(0.16, 1, 0.3, 1);
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: center;
    gap: 0.875rem;
    padding: 1.25rem 1.25rem 1rem;
    border-bottom: 1px solid var(--border);
  }

  .modal-icon-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: 0.625rem;
    flex-shrink: 0;
  }

  .modal-icon-wrapper.warning {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  .modal-icon-wrapper.danger {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
  }

  .modal-icon-wrapper.info {
    background: rgba(59, 130, 246, 0.1);
    color: #3b82f6;
  }

  .modal-title {
    flex: 1;
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--foreground);
    margin: 0;
  }

  .modal-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: transparent;
    border: none;
    border-radius: 0.375rem;
    color: var(--muted-foreground);
    cursor: pointer;
    transition: all 0.2s;
  }

  .modal-close:hover {
    background: var(--accent);
    color: var(--foreground);
  }

  .modal-body {
    padding: 1.25rem;
  }

  .warning-banner {
    display: flex;
    align-items: flex-start;
    gap: 0.625rem;
    padding: 0.75rem;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 0.5rem;
    color: #ef4444;
    font-size: 0.8125rem;
    font-weight: 600;
    margin-bottom: 1rem;
    line-height: 1.5;
  }

  .process-details {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
    padding: 0.875rem;
    background: var(--muted);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    margin-bottom: 0.875rem;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  .detail-label {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--muted-foreground);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .detail-value {
    font-family:
      'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--foreground);
  }

  .modal-description {
    font-size: 0.8125rem;
    line-height: 1.6;
    color: var(--muted-foreground);
    margin: 0;
  }

  .modal-footer {
    display: flex;
    gap: 0.625rem;
    padding: 1rem 1.25rem 1.25rem;
    border-top: 1px solid var(--border);
  }

  .btn-modal {
    flex: 1;
    padding: 0.625rem 1.25rem;
    border: 1px solid;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-cancel {
    background: var(--background);
    border-color: var(--border);
    color: var(--foreground);
  }

  .btn-cancel:hover {
    background: var(--accent);
    border-color: var(--border);
  }

  .btn-delete {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgba(239, 68, 68, 0.3);
    color: #ef4444;
  }

  .btn-delete:hover {
    background: rgba(239, 68, 68, 0.2);
    border-color: rgba(239, 68, 68, 0.5);
  }

  .btn-delete.danger {
    background: #ef4444;
    border-color: #dc2626;
    color: white;
  }

  .btn-delete.danger:hover {
    background: #dc2626;
    border-color: #b91c1c;
  }

  .btn-modal:active {
    transform: scale(0.98);
  }

  .btn-primary {
    background: var(--primary);
    border-color: var(--primary);
    color: var(--primary-foreground);
  }

  .btn-primary:hover {
    background: #2563eb;
    border-color: #2563eb;
  }

  /* Utilities */
  .animate-spin {
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

  .rotate-180 {
    transform: rotate(180deg);
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Add fade-in animation for port rows */
  .port-row {
    animation: fadeInRow 0.3s ease;
  }

  @keyframes fadeInRow {
    from {
      opacity: 0;
      transform: translateX(-10px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  /* Stagger animation for groups */
  .port-list > .port-row:nth-child(1) {
    animation-delay: 0.05s;
  }
  .port-list > .port-row:nth-child(2) {
    animation-delay: 0.1s;
  }
  .port-list > .port-row:nth-child(3) {
    animation-delay: 0.15s;
  }
  .port-list > .port-row:nth-child(4) {
    animation-delay: 0.2s;
  }
  .port-list > .port-row:nth-child(5) {
    animation-delay: 0.25s;
  }
</style>
