<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import NetworkGraph from '$lib/components/NetworkMonitor/NetworkGraph.svelte';
  import InterfaceDetailsModal from '$lib/components/NetworkMonitor/InterfaceDetailsModal.svelte';
  import {
    getNetworkStats,
    getNetworkHistory,
    getNetworkInterfaces
  } from '$lib/api/network-monitor';
  import type {
    NetworkSnapshot,
    NetworkInterfaceStats
  } from '$lib/types/network';
  import {
    Activity,
    ArrowUp,
    ArrowDown,
    Package,
    Network,
    Info,
    Info as InfoIcon,
    ArrowUpDown
  } from 'lucide-svelte';

  let historyData: NetworkSnapshot[] = $state([]);
  let latestSnapshot: NetworkSnapshot | null = $state(null);
  let interfaces: NetworkInterfaceStats[] = $state([]);
  let isLoading = $state(true);
  let error: string | null = $state(null);
  let updateInterval: ReturnType<typeof setInterval>;
  let selectedTimeRange = $state(300); // 5 minutes default
  let showInfoModal = $state(false);
  let selectedInterface: NetworkInterfaceStats | null = $state(null);
  let sortActiveFirst = $state(true); // Sort active interfaces first by default

  const timeRanges = [
    { label: '1m', seconds: 60 },
    { label: '5m', seconds: 300 },
    { label: '15m', seconds: 900 },
    { label: '30m', seconds: 1800 }
  ];

  async function loadNetworkData() {
    try {
      const [latest, history, currentInterfaces] = await Promise.all([
        getNetworkStats(),
        getNetworkHistory(selectedTimeRange),
        getNetworkInterfaces()
      ]);

      latestSnapshot = latest;
      historyData = history;
      interfaces = currentInterfaces;

      isLoading = false;
      error = null;
    } catch (err) {
      console.error('Failed to load network data:', err);
      error =
        err instanceof Error ? err.message : 'Failed to load network data';
      isLoading = false;
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }

  function selectTimeRange(seconds: number) {
    selectedTimeRange = seconds;
    loadNetworkData();
  }

  function toggleSort() {
    sortActiveFirst = !sortActiveFirst;
  }

  // Sort interfaces by active status
  let sortedInterfaces = $derived(
    sortActiveFirst
      ? [...interfaces].sort((a, b) => {
          if (a.isUp === b.isUp) return 0;
          return a.isUp ? -1 : 1; // Active first
        })
      : [...interfaces].sort((a, b) => {
          if (a.isUp === b.isUp) return 0;
          return a.isUp ? 1 : -1; // Inactive first
        })
  );

  onMount(() => {
    loadNetworkData();

    // Update every 2 seconds
    updateInterval = setInterval(() => {
      loadNetworkData();
    }, 2000);
  });

  onDestroy(() => {
    if (updateInterval) {
      clearInterval(updateInterval);
    }
  });
</script>

<div class="network-page">
  <PageHeader
    title="Network Monitor"
    subtitle="Cumulative network statistics since system startup"
    icon={Activity}
    showInfoButton={true}
    onInfoClick={() => (showInfoModal = true)}
  >
    <div
      class="time-range-selector"
      role="tablist"
      aria-label="Time range selector"
    >
      {#each timeRanges as range (range.seconds)}
        <button
          class="range-button"
          class:active={selectedTimeRange === range.seconds}
          onclick={() => selectTimeRange(range.seconds)}
          role="tab"
          aria-selected={selectedTimeRange === range.seconds}
          aria-label={`Select ${range.label} time range`}
        >
          {range.label}
        </button>
      {/each}
    </div>
  </PageHeader>

  {#if isLoading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading network data...</p>
    </div>
  {:else if error}
    <div class="error">
      <svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
      >
        <circle cx="12" cy="12" r="10" stroke-width="2" />
        <line x1="12" y1="8" x2="12" y2="12" stroke-width="2" />
        <circle cx="12" cy="16" r="0.5" fill="currentColor" />
      </svg>
      <p>{error}</p>
    </div>
  {:else}
    <div class="stats-grid">
      <div class="stat-card upload">
        <div class="stat-header">
          <div class="stat-icon upload-icon">
            <ArrowUp size={18} />
          </div>
          <div class="stat-label">Upload</div>
        </div>
        <div class="stat-value">
          {latestSnapshot ? formatBytes(latestSnapshot.totalBytesSent) : '0 B'}
        </div>
        <div class="stat-meta">Since startup</div>
      </div>

      <div class="stat-card download">
        <div class="stat-header">
          <div class="stat-icon download-icon">
            <ArrowDown size={18} />
          </div>
          <div class="stat-label">Download</div>
        </div>
        <div class="stat-value">
          {latestSnapshot
            ? formatBytes(latestSnapshot.totalBytesReceived)
            : '0 B'}
        </div>
        <div class="stat-meta">Since startup</div>
      </div>

      <div class="stat-card packets-sent">
        <div class="stat-header">
          <div class="stat-icon packets-icon">
            <Package size={18} />
          </div>
          <div class="stat-label">Packets Out</div>
        </div>
        <div class="stat-value">
          {latestSnapshot?.totalPacketsSent.toLocaleString() || 0}
        </div>
        <div class="stat-meta">Since startup</div>
      </div>

      <div class="stat-card packets-received">
        <div class="stat-header">
          <div class="stat-icon packets-icon">
            <Package size={18} />
          </div>
          <div class="stat-label">Packets In</div>
        </div>
        <div class="stat-value">
          {latestSnapshot?.totalPacketsReceived.toLocaleString() || 0}
        </div>
        <div class="stat-meta">Since startup</div>
      </div>
    </div>

    <div class="graph-section">
      <div class="graph-header">
        <h2>Bandwidth Usage</h2>
        <div class="graph-legend">
          <div class="legend-item">
            <span class="legend-dot upload-dot"></span>
            <span>Upload</span>
          </div>
          <div class="legend-item">
            <span class="legend-dot download-dot"></span>
            <span>Download</span>
          </div>
        </div>
      </div>
      <NetworkGraph data={historyData} height={420} />
    </div>

    <!-- Network Interfaces Table -->
    {#if interfaces.length > 0}
      <div class="interfaces-section glass">
        <div class="section-header">
          <div class="section-title-group">
            <Network size={20} />
            <h2>Network Interfaces</h2>
            <span class="interfaces-count">{interfaces.length} interfaces</span>
          </div>
          <button
            class="sort-button"
            onclick={toggleSort}
            aria-label={sortActiveFirst
              ? 'Sort inactive first'
              : 'Sort active first'}
            title={sortActiveFirst
              ? 'Currently: Active first. Click to show inactive first'
              : 'Currently: Inactive first. Click to show active first'}
          >
            <ArrowUpDown size={16} />
            <span>{sortActiveFirst ? 'Active First' : 'Inactive First'}</span>
          </button>
        </div>
        <div class="interfaces-table-container">
          <div class="table-scroll-wrapper">
            <table class="interfaces-table">
              <thead>
                <tr>
                  <th><span class="th-content">Interface</span></th>
                  <th><span class="th-content">Type</span></th>
                  <th><span class="th-content">Upload</span></th>
                  <th><span class="th-content">Download</span></th>
                  <th class="hide-mobile"
                    ><span class="th-content">Packets</span></th
                  >
                  <th class="hide-mobile"
                    ><span class="th-content">Errors</span></th
                  >
                  <th class="hide-tablet"
                    ><span class="th-content">MAC Address</span></th
                  >
                  <th class="actions-col"
                    ><span class="th-content">Info</span></th
                  >
                </tr>
              </thead>
              <tbody>
                {#each sortedInterfaces as iface (iface.name)}
                  <tr
                    class="interface-row"
                    class:is-active={iface.isUp}
                    role="button"
                    tabindex="0"
                    onclick={() => (selectedInterface = iface)}
                    onkeydown={(e) =>
                      e.key === 'Enter' && (selectedInterface = iface)}
                  >
                    <td>
                      <div class="iface-name-cell">
                        <div
                          class="status-indicator"
                          class:active={iface.isUp}
                        ></div>
                        <span class="iface-name">{iface.name}</span>
                      </div>
                    </td>
                    <td>
                      <span class="interface-type-badge"
                        >{iface.interfaceType}</span
                      >
                    </td>
                    <td class="upload-cell">{formatBytes(iface.bytesSent)}</td>
                    <td class="download-cell"
                      >{formatBytes(iface.bytesReceived)}</td
                    >
                    <td class="packets-cell hide-mobile">
                      <div class="packets-breakdown">
                        <span title="Sent"
                          >{iface.packetsSent.toLocaleString()}</span
                        >
                        <span class="separator">/</span>
                        <span title="Received"
                          >{iface.packetsReceived.toLocaleString()}</span
                        >
                      </div>
                    </td>
                    <td
                      class="error-count hide-mobile"
                      class:has-errors={iface.errorsSent +
                        iface.errorsReceived >
                        0}
                    >
                      {iface.errorsSent + iface.errorsReceived}
                    </td>
                    <td class="mac-addr hide-tablet"
                      >{iface.macAddress || 'N/A'}</td
                    >
                    <td class="actions-col">
                      <button
                        class="info-btn glass"
                        onclick={(e) => {
                          e.stopPropagation();
                          selectedInterface = iface;
                        }}
                        aria-label="View details for {iface.name}"
                      >
                        <InfoIcon size={16} />
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    {/if}
  {/if}

  <!-- Interface Details Modal -->
  <InterfaceDetailsModal
    interface={selectedInterface}
    onClose={() => (selectedInterface = null)}
  />

  <!-- Info Modal -->
  {#if showInfoModal}
    <div
      class="modal-overlay"
      onclick={() => (showInfoModal = false)}
      role="button"
      tabindex="0"
    >
      <div
        class="modal-content"
        onclick={(e) => e.stopPropagation()}
        role="dialog"
        aria-labelledby="modal-title"
      >
        <div class="modal-header">
          <h3 id="modal-title">About Network Statistics</h3>
          <button
            class="modal-close"
            onclick={() => (showInfoModal = false)}
            aria-label="Close">×</button
          >
        </div>
        <div class="modal-body">
          <div class="info-section">
            <h4><Info size={16} /> Data Source</h4>
            <p>
              These statistics show <strong>cumulative network usage</strong> collected
              from all network interfaces since your system was last started.
            </p>
          </div>

          <div class="info-section">
            <h4><Activity size={16} /> Metrics Explained</h4>
            <ul>
              <li>
                <strong>Upload:</strong> Total data sent from your computer to the
                network
              </li>
              <li>
                <strong>Download:</strong> Total data received by your computer from
                the network
              </li>
              <li>
                <strong>Packets Out/In:</strong> Number of data packets transmitted
                and received
              </li>
            </ul>
          </div>

          <div class="info-section">
            <h4><Package size={16} /> Time Range Selection</h4>
            <p>
              The time range buttons (1m, 5m, 15m, 30m) control the bandwidth
              graph view period, showing how your network usage has changed over
              the selected timeframe.
            </p>
          </div>

          <div class="info-note">
            <strong>Note:</strong> These values reset to zero when you restart your
            computer. For real-time bandwidth monitoring, check the graph below.
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .network-page {
    width: 100%;
    height: 100%;
    overflow-y: auto;
    background: var(--bg-primary);
  }

  .time-range-selector {
    display: flex;
    gap: 4px;
    background: var(--bg-secondary);
    padding: 4px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
  }

  .range-button {
    padding: 6px 14px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    font-weight: 600;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: all var(--transition-fast);
    min-width: 50px;
  }

  .range-button:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .range-button.active {
    background: var(--text-primary);
    color: var(--bg-primary);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-xl);
    margin-bottom: var(--space-2xl);
    padding: 0 var(--space-xl);
  }

  @media (max-width: 1200px) {
    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 640px) {
    .stats-grid {
      grid-template-columns: 1fr;
    }
  }

  .stat-card {
    background: var(--bg-secondary);
    padding: var(--space-md) var(--space-lg);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
  }

  .stat-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 3px;
    background: linear-gradient(
      90deg,
      transparent,
      var(--card-accent),
      transparent
    );
    opacity: 0;
    transition: opacity 0.2s;
  }

  .stat-card:hover {
    border-color: var(--card-accent);
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  }

  .stat-card:hover::before {
    opacity: 1;
  }

  .stat-card.upload {
    --card-accent: #3b82f6;
  }

  .stat-card.download {
    --card-accent: #10b981;
  }

  .stat-card.packets-sent {
    --card-accent: #8b5cf6;
  }

  .stat-card.packets-received {
    --card-accent: #f59e0b;
  }

  .stat-header {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    margin-bottom: var(--space-sm);
  }

  .stat-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-sm);
    background: var(--icon-bg);
    flex-shrink: 0;
  }

  .upload-icon {
    --icon-bg: rgba(59, 130, 246, 0.1);
    color: #3b82f6;
  }

  .download-icon {
    --icon-bg: rgba(16, 185, 129, 0.1);
    color: #10b981;
  }

  .packets-icon {
    --icon-bg: rgba(139, 92, 246, 0.1);
    color: #8b5cf6;
  }

  .stat-card.packets-received .packets-icon {
    --icon-bg: rgba(245, 158, 11, 0.1);
    color: #f59e0b;
  }

  .stat-label {
    font-size: 11px;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    font-weight: 600;
  }

  .stat-value {
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
    font-family: var(--font-mono);
    letter-spacing: -0.5px;
    margin-bottom: var(--space-xs);
    line-height: 1.2;
  }

  .stat-meta {
    font-size: 11px;
    color: var(--text-tertiary);
    font-weight: 500;
    opacity: 0.7;
  }

  .graph-section {
    background: var(--bg-secondary);
    padding: var(--space-xl);
    margin: 0 var(--space-xl);
    margin-bottom: var(--space-xl);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-color);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  .graph-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  @media (max-width: 640px) {
    .graph-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 12px;
    }
  }

  .graph-header h2 {
    font-size: 16px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
    letter-spacing: -0.2px;
  }

  .graph-legend {
    display: flex;
    gap: var(--spacing-lg);
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    font-size: 13px;
    color: var(--text-secondary);
    font-weight: 600;
  }

  .legend-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
  }

  .upload-dot {
    background: #3b82f6;
    box-shadow: 0 0 8px rgba(59, 130, 246, 0.5);
  }

  .download-dot {
    background: #10b981;
    box-shadow: 0 0 8px rgba(16, 185, 129, 0.5);
  }

  .loading,
  .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-xl);
    gap: var(--spacing-md);
    color: var(--text-secondary);
    min-height: 400px;
  }

  .spinner {
    width: 48px;
    height: 48px;
    border: 4px solid var(--border-color);
    border-top-color: var(--accent-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .error {
    color: #ef4444;
  }

  .error svg {
    width: 56px;
    height: 56px;
  }

  .error p {
    font-size: 16px;
    font-weight: 500;
  }

  /* Modal Styles - Matching Port Map */
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

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .modal-content {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    box-shadow:
      0 0 0 1px var(--border-color),
      0 20px 50px rgba(0, 0, 0, 0.3),
      0 10px 30px rgba(0, 0, 0, 0.15);
    max-width: 600px;
    width: 90%;
    animation: slideUp 0.25s cubic-bezier(0.16, 1, 0.3, 1);
    overflow: hidden;
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

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 1.25rem 1rem;
    border-bottom: 1px solid var(--border-color);
  }

  .modal-header h3 {
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--text-primary);
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
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .modal-close:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .modal-body {
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
    max-height: 70vh;
    overflow-y: auto;
  }

  .info-section {
    padding: var(--space-md);
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
  }

  .info-section h4 {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    font-size: 0.875rem;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 var(--space-sm) 0;
  }

  .info-section p {
    font-size: 0.8125rem;
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.6;
  }

  .info-section ul {
    margin: var(--space-sm) 0 0 0;
    padding-left: var(--space-lg);
    list-style: none;
  }

  .info-section li {
    font-size: 0.8125rem;
    color: var(--text-secondary);
    margin-bottom: var(--space-xs);
    line-height: 1.6;
    position: relative;
  }

  .info-section li::before {
    content: '•';
    position: absolute;
    left: calc(-1 * var(--space-md));
    color: #3b82f6;
    font-weight: bold;
  }

  .info-note {
    padding: 0.75rem;
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: var(--radius-sm);
    font-size: 0.8125rem;
    color: var(--text-secondary);
    line-height: 1.6;
  }

  .info-note strong {
    color: #3b82f6;
    font-weight: 700;
  }

  /* Network Interfaces Table */
  .interfaces-section {
    margin: var(--space-2xl) var(--space-xl) 0;
    background: var(--bg-secondary);
    padding: var(--space-xl);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-color);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-md);
    margin-bottom: var(--space-lg);
    padding-bottom: var(--space-md);
    border-bottom: 1px solid var(--border-color);
  }

  .section-title-group {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .section-header h2 {
    font-size: 17px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
    letter-spacing: -0.3px;
  }

  .interfaces-table-container {
    overflow-x: auto;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    background: var(--bg-primary);
  }

  .interfaces-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }

  .interfaces-table thead {
    background: linear-gradient(
      to bottom,
      var(--bg-tertiary),
      var(--bg-secondary)
    );
    border-bottom: 2px solid var(--border-color);
  }

  .interfaces-table th {
    padding: var(--space-md) var(--space-lg);
    text-align: left;
    font-size: 11px;
    font-weight: 700;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.6px;
    white-space: nowrap;
  }

  .interfaces-table tbody tr {
    border-bottom: 1px solid var(--border-color);
    transition: all 0.2s ease;
  }

  .interfaces-table tbody tr:last-child {
    border-bottom: none;
  }

  .interfaces-table tbody tr:hover {
    background: rgba(59, 130, 246, 0.08);
    transform: translateX(2px);
  }

  .interfaces-table td {
    padding: var(--space-md) var(--space-lg);
    color: var(--text-secondary);
    white-space: nowrap;
    vertical-align: middle;
  }

  .iface-name {
    font-weight: 700;
    color: var(--text-primary);
    font-family: var(--font-mono);
    font-size: 13px;
    background: rgba(59, 130, 246, 0.1);
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    display: inline-block;
  }

  .upload-cell {
    color: #3b82f6;
    font-family: var(--font-mono);
    font-weight: 600;
  }

  .download-cell {
    color: #10b981;
    font-family: var(--font-mono);
    font-weight: 600;
  }

  .packets-cell {
    font-family: var(--font-mono);
    font-size: 12px;
  }

  .packets-breakdown {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .packets-breakdown .separator {
    color: var(--text-tertiary);
    opacity: 0.4;
    font-weight: 300;
  }

  .error-count {
    font-family: var(--font-mono);
    color: var(--text-tertiary);
    text-align: center;
  }

  .error-count.has-errors {
    color: #fff;
    background: linear-gradient(135deg, #f59e0b, #d97706);
    padding: 4px 10px;
    border-radius: var(--radius-sm);
    font-weight: 700;
    box-shadow: 0 2px 8px rgba(245, 158, 11, 0.3);
  }

  .mac-addr {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-tertiary);
    opacity: 0.9;
  }

  /* Additional interface table styles */
  .glass {
    background: var(--glass-bg);
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }

  .table-scroll-wrapper {
    overflow-x: auto;
  }

  .interfaces-count {
    font-size: 13px;
    color: var(--text-tertiary);
    font-weight: 600;
    padding: var(--space-xs) var(--space-md);
    background: rgba(59, 130, 246, 0.1);
    border-radius: var(--radius-full);
  }

  .interface-row {
    cursor: pointer;
  }

  .interface-row.is-active {
    background: rgba(16, 185, 129, 0.03);
  }

  .iface-name-cell {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #6b7280;
    flex-shrink: 0;
  }

  .status-indicator.active {
    background: #10b981;
    box-shadow: 0 0 12px rgba(16, 185, 129, 0.6);
    animation: pulse 2s infinite;
  }

  .interface-type-badge {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    padding: 3px 8px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-sm);
    border: 1px solid var(--border-color);
  }

  .th-content {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .actions-col {
    width: 60px;
    text-align: center;
  }

  .info-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    border: 1px solid var(--border-color);
    background: var(--glass-bg);
    backdrop-filter: blur(8px);
    color: var(--text-tertiary);
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: all 0.2s;
  }

  .info-btn:hover {
    background: rgba(59, 130, 246, 0.1);
    border-color: rgba(59, 130, 246, 0.5);
    color: #3b82f6;
    transform: scale(1.1);
  }

  @media (max-width: 1024px) {
    .hide-tablet {
      display: none;
    }

    .interfaces-table {
      font-size: 12px;
    }

    .interfaces-table th,
    .interfaces-table td {
      padding: var(--space-sm) var(--space-md);
    }

    .iface-name {
      font-size: 12px;
    }
  }

  @media (max-width: 768px) {
    .hide-mobile {
      display: none;
    }

    .interfaces-section {
      padding: var(--space-lg);
    }

    .section-header {
      flex-wrap: wrap;
    }
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.6;
    }
  }

  /* Sort Button */
  .sort-button {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .sort-button:hover {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    color: var(--text-primary);
  }

  @media (max-width: 768px) {
    .section-header {
      flex-direction: column;
      align-items: flex-start;
    }

    .sort-button {
      width: 100%;
      justify-content: center;
    }
  }
</style>
