<script lang="ts">
  import type { NetworkInterfaceStats } from '$lib/types/network';
  import {
    Info,
    X,
    Wifi,
    Cable,
    Link,
    Shield,
    AlertCircle
  } from 'lucide-svelte';

  interface Props {
    interface: NetworkInterfaceStats | null;
    onClose: () => void;
  }

  let { interface: iface, onClose }: Props = $props();

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  }

  function getInterfaceDescription(type: string): string {
    switch (type) {
      case 'Loopback':
        return 'Virtual interface for local traffic within your computer. Used by applications to communicate with each other without going through the network.';
      case 'Ethernet':
        return 'Physical wired network connection. Provides stable, high-speed connectivity through ethernet cables.';
      case 'Wi-Fi':
        return 'Wireless network connection using radio waves. Provides flexible connectivity without physical cables.';
      case 'VPN Tunnel':
        return 'Encrypted virtual network tunnel. Routes your internet traffic through a secure connection to protect privacy and access remote networks.';
      case 'Apple Wireless Direct Link':
        return 'Apple proprietary peer-to-peer mesh networking protocol. Used for AirDrop, AirPlay, and other Apple device-to-device features.';
      case 'Apple Network Provider Interface':
        return 'Apple system network provider. Handles network routing and connectivity management for macOS.';
      case 'Generic Tunnel Interface':
        return 'Virtual network interface for tunneling traffic. Used for various VPN protocols and network overlays.';
      case 'Network Bridge':
        return 'Virtual interface that connects multiple network segments. Allows different networks to communicate as if they were one.';
      default:
        return 'Network interface for data transmission.';
    }
  }

  function getInterfaceIcon(type: string) {
    switch (type) {
      case 'Wi-Fi':
        return Wifi;
      case 'Ethernet':
        return Cable;
      case 'VPN Tunnel':
      case 'Generic Tunnel Interface':
        return Shield;
      case 'Network Bridge':
      case 'Apple Wireless Direct Link':
      case 'Apple Network Provider Interface':
        return Link;
      default:
        return Info;
    }
  }
</script>

{#if iface}
  <div
    class="modal-overlay glass"
    onclick={onClose}
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-title"
  >
    <div class="modal-content glass" onclick={(e) => e.stopPropagation()}>
      <!-- Header -->
      <div class="modal-header">
        <div class="header-title-group">
          {#if iface.interfaceType}
            {@const Icon = getInterfaceIcon(iface.interfaceType)}
            <div class="interface-icon" class:is-active={iface.isUp}>
              <Icon size={24} />
            </div>
          {/if}
          <div>
            <h3 id="modal-title">{iface.name}</h3>
            <span class="interface-type">{iface.interfaceType}</span>
          </div>
        </div>
        <button class="close-button" onclick={onClose} aria-label="Close modal">
          <X size={20} />
        </button>
      </div>

      <!-- Status Badge -->
      <div class="status-badge" class:active={iface.isUp}>
        <div class="status-dot"></div>
        <span>{iface.isUp ? 'Active' : 'Inactive'}</span>
      </div>

      <!-- Description -->
      <div class="description-card glass">
        <div class="description-header">
          <Info size={16} />
          <span>About this interface</span>
        </div>
        <p>{getInterfaceDescription(iface.interfaceType)}</p>
      </div>

      <!-- Stats Grid -->
      <div class="stats-section">
        <h4>Statistics</h4>
        <div class="stats-grid">
          <div class="stat-item glass">
            <div class="stat-label">Upload</div>
            <div class="stat-value upload">{formatBytes(iface.bytesSent)}</div>
            <div class="stat-meta">
              {iface.packetsSent.toLocaleString()} packets
            </div>
          </div>

          <div class="stat-item glass">
            <div class="stat-label">Download</div>
            <div class="stat-value download">
              {formatBytes(iface.bytesReceived)}
            </div>
            <div class="stat-meta">
              {iface.packetsReceived.toLocaleString()} packets
            </div>
          </div>

          <div class="stat-item glass">
            <div class="stat-label">Errors</div>
            <div
              class="stat-value"
              class:has-errors={iface.errorsSent + iface.errorsReceived > 0}
            >
              {iface.errorsSent + iface.errorsReceived}
            </div>
            <div class="stat-meta">
              Sent: {iface.errorsSent} | Recv: {iface.errorsReceived}
            </div>
          </div>

          <div class="stat-item glass">
            <div class="stat-label">MAC Address</div>
            <div class="stat-value mac">{iface.macAddress || 'N/A'}</div>
            <div class="stat-meta">Physical address</div>
          </div>
        </div>
      </div>

      {#if iface.errorsSent + iface.errorsReceived > 0}
        <div class="error-note glass">
          <AlertCircle size={18} />
          <p>
            This interface has {iface.errorsSent + iface.errorsReceived} transmission
            errors. This may indicate network issues or hardware problems.
          </p>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
    animation: fadeIn 0.2s ease;
    padding: var(--space-lg);
  }

  .modal-content {
    max-width: 600px;
    width: 100%;
    max-height: 90vh;
    overflow-y: auto;
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    border-radius: var(--radius-xl);
    box-shadow:
      0 0 0 1px rgba(255, 255, 255, 0.1),
      0 24px 48px rgba(0, 0, 0, 0.4),
      0 12px 24px rgba(0, 0, 0, 0.3);
    animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    padding: var(--space-2xl);
  }

  .glass {
    background: var(--glass-bg);
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-xl);
  }

  .header-title-group {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .interface-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border-radius: var(--radius-md);
    background: rgba(59, 130, 246, 0.1);
    color: #3b82f6;
    transition: all 0.3s;
  }

  .interface-icon.is-active {
    background: linear-gradient(135deg, #3b82f6, #2563eb);
    color: white;
    box-shadow: 0 8px 24px rgba(59, 130, 246, 0.3);
  }

  .modal-header h3 {
    font-size: 22px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
    font-family: var(--font-mono);
    letter-spacing: -0.5px;
  }

  .interface-type {
    font-size: 13px;
    color: var(--text-tertiary);
    font-weight: 500;
  }

  .close-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-md);
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border-color);
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .close-button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text-primary);
    transform: scale(1.05);
  }

  .status-badge {
    display: inline-flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-full);
    background: rgba(107, 114, 128, 0.1);
    border: 1px solid rgba(107, 114, 128, 0.2);
    font-size: 13px;
    font-weight: 600;
    color: #9ca3af;
    margin-bottom: var(--space-xl);
  }

  .status-badge.active {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    color: #10b981;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #9ca3af;
  }

  .status-badge.active .status-dot {
    background: #10b981;
    box-shadow: 0 0 12px rgba(16, 185, 129, 0.6);
    animation: pulse 2s infinite;
  }

  .description-card {
    padding: var(--space-lg);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-color);
    margin-bottom: var(--space-2xl);
  }

  .description-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    font-size: 13px;
    font-weight: 700;
    color: var(--text-primary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: var(--space-sm);
  }

  .description-card p {
    font-size: 14px;
    line-height: 1.6;
    color: var(--text-secondary);
    margin: 0;
  }

  .stats-section h4 {
    font-size: 14px;
    font-weight: 700;
    color: var(--text-primary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin: 0 0 var(--space-md) 0;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-md);
    margin-bottom: var(--space-xl);
  }

  .stat-item {
    padding: var(--space-lg);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    transition: all 0.2s;
  }

  .stat-item:hover {
    border-color: rgba(59, 130, 246, 0.5);
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  }

  .stat-label {
    font-size: 11px;
    font-weight: 700;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: var(--space-xs);
  }

  .stat-value {
    font-size: 18px;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: var(--space-xs);
    word-break: break-all;
  }

  .stat-value.upload {
    color: #3b82f6;
    font-family: var(--font-mono);
  }

  .stat-value.download {
    color: #10b981;
    font-family: var(--font-mono);
  }

  .stat-value.mac {
    font-family: var(--font-mono);
    font-size: 14px;
  }

  .stat-value.has-errors {
    color: #f59e0b;
  }

  .stat-meta {
    font-size: 11px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
  }

  .error-note {
    display: flex;
    gap: var(--space-md);
    padding: var(--space-lg);
    border-radius: var(--radius-md);
    background: rgba(245, 158, 11, 0.1);
    border: 1px solid rgba(245, 158, 11, 0.3);
    color: #f59e0b;
  }

  .error-note p {
    font-size: 13px;
    line-height: 1.5;
    margin: 0;
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
      transform: translateY(20px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
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

  @media (max-width: 640px) {
    .stats-grid {
      grid-template-columns: 1fr;
    }

    .modal-content {
      padding: var(--space-lg);
    }
  }
</style>
