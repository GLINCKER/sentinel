<script lang="ts">
  import {
    X,
    Lock,
    AlertTriangle,
    Globe,
    Wifi,
    Shield,
    Eye,
    Network
  } from 'lucide-svelte';

  interface Props {
    show: boolean;
    onClose: () => void;
    badgeType?:
      | 'public'
      | 'local'
      | 'network'
      | 'http'
      | 'listen'
      | 'established'
      | 'tcp'
      | 'udp'
      | null;
  }

  let { show = $bindable(), onClose, badgeType = null }: Props = $props();

  const badgeInfo = {
    public: {
      icon: AlertTriangle,
      title: 'Public Exposure',
      color: '#f59e0b',
      description:
        'This port is bound to 0.0.0.0 or *, meaning it accepts connections from ANY network interface.',
      risks: [
        'Accessible from the internet if not behind a firewall',
        'Can be reached by other devices on your local network',
        'Potential security risk if the service is not properly secured'
      ],
      recommendations: [
        'Verify this service needs to be publicly accessible',
        'Ensure firewall rules are properly configured',
        'Use strong authentication and encryption',
        'Consider binding to 127.0.0.1 (localhost) if only local access is needed'
      ],
      technical:
        'Binding to 0.0.0.0 means the socket listens on all available network interfaces. This is the default for many server applications but should be used cautiously.'
    },
    local: {
      icon: Lock,
      title: 'Local Only',
      color: '#22c55e',
      description:
        'This port is bound to 127.0.0.1 (localhost), meaning it only accepts connections from this machine.',
      risks: [
        'Minimal security risk - not accessible from external networks',
        'Safe for development and local services'
      ],
      recommendations: [
        'Ideal for development servers and databases',
        'Use this binding when services should only communicate locally',
        'No firewall configuration needed for external protection'
      ],
      technical:
        'The loopback interface (127.0.0.1) creates a network connection that stays within the local machine, never reaching external networks.'
    },
    network: {
      icon: Globe,
      title: 'Network Connection',
      color: '#3b82f6',
      description:
        'This port is connected to a specific network address, indicating an active network connection.',
      risks: [
        'Connection is to a specific remote host',
        'Security depends on the remote service and encryption'
      ],
      recommendations: [
        'Verify the remote address is trusted',
        'Check if the connection should be encrypted (HTTPS, TLS)',
        'Monitor for unexpected connections'
      ],
      technical:
        'Network connections show active TCP/UDP sessions with specific remote endpoints, including both local and remote IP:port pairs.'
    },
    http: {
      icon: Wifi,
      title: 'HTTP Service',
      color: '#3b82f6',
      description: 'This port is commonly used for HTTP/HTTPS web traffic.',
      risks: [
        'HTTP (80, 8080) sends data in plain text',
        'HTTPS (443) encrypts traffic but certificate validity matters',
        'Common ports for web-based attacks'
      ],
      recommendations: [
        'Always use HTTPS (443) for production web services',
        'Keep web frameworks and dependencies updated',
        'Implement proper authentication and authorization',
        'Use Content Security Policy (CSP) headers'
      ],
      technical:
        'Common HTTP ports: 80 (HTTP), 443 (HTTPS), 3000-3002 (development), 4200 (Angular), 5000/5173 (Vite), 8000/8080/8888 (various web servers), 9000 (dev tools).'
    },
    listen: {
      icon: Wifi,
      title: 'Listening State',
      color: '#3b82f6',
      description:
        'The port is actively listening for incoming connections but has no active connections yet.',
      risks: [
        'Port is open and waiting for connections',
        'Visible to port scanners',
        'Attack surface depends on the service'
      ],
      recommendations: [
        'Ensure only necessary services are listening',
        'Close unused ports to reduce attack surface',
        'Monitor for unexpected listening ports',
        'Keep listening services updated and patched'
      ],
      technical:
        'LISTEN state means the socket has been bound to an address/port and is waiting for connection requests. This is the normal state for server applications.'
    },
    established: {
      icon: Network,
      title: 'Established Connection',
      color: '#22c55e',
      description:
        'An active connection is currently established and data can flow in both directions.',
      risks: [
        'Active data transfer is occurring',
        'Connection security depends on encryption',
        'Unexpected connections may indicate compromise'
      ],
      recommendations: [
        'Verify the remote address is expected',
        'Check if sensitive data should be encrypted',
        'Monitor connection duration and data volume',
        'Investigate unknown or suspicious connections'
      ],
      technical:
        'ESTABLISHED state means the TCP three-way handshake completed successfully and both endpoints can send/receive data. This is the normal operational state.'
    },
    tcp: {
      icon: Network,
      title: 'TCP Protocol',
      color: '#3b82f6',
      description:
        'Transmission Control Protocol - reliable, connection-oriented communication.',
      risks: [
        'SYN flood attacks can overwhelm servers',
        'Slower than UDP due to reliability guarantees',
        'Connection state must be maintained'
      ],
      recommendations: [
        'Use TCP for applications requiring reliability',
        'Implement connection timeouts',
        'Monitor for excessive connection attempts',
        'Consider rate limiting for public services'
      ],
      technical:
        'TCP provides reliable, ordered, error-checked delivery using acknowledgments, retransmission, and flow control. Most internet traffic uses TCP.'
    },
    udp: {
      icon: Wifi,
      title: 'UDP Protocol',
      color: '#a855f7',
      description:
        'User Datagram Protocol - fast, connectionless communication without reliability guarantees.',
      risks: [
        'No built-in reliability or ordering',
        'Can be used for amplification attacks',
        'Easier to spoof source addresses'
      ],
      recommendations: [
        'Use UDP for real-time applications (video, gaming, VoIP)',
        'Implement application-level reliability if needed',
        'Validate and sanitize all incoming data',
        'Consider rate limiting to prevent abuse'
      ],
      technical:
        'UDP sends datagrams without establishing a connection or guaranteeing delivery. Faster but less reliable than TCP. Common for DNS, streaming, and gaming.'
    }
  };

  let currentBadge = $derived(
    badgeType && badgeInfo[badgeType] ? badgeInfo[badgeType] : null
  );
</script>

{#if show && currentBadge}
  <div class="modal-overlay" onclick={onClose}>
    <div class="modal-content badge-modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div
          class="modal-icon-wrapper"
          style="background: {currentBadge.color}15; color: {currentBadge.color};"
        >
          <svelte:component this={currentBadge.icon} size={24} />
        </div>
        <h3 class="modal-title">{currentBadge.title}</h3>
        <button class="modal-close" onclick={onClose} aria-label="Close modal">
          <X size={20} />
        </button>
      </div>

      <div class="modal-body">
        <div class="info-section">
          <p class="info-description">{currentBadge.description}</p>
        </div>

        <div class="info-block risks">
          <div class="block-header">
            <Shield size={16} />
            <h4>Security Considerations</h4>
          </div>
          <ul class="info-bullet-list">
            {#each currentBadge.risks as risk, idx (idx)}
              <li>{risk}</li>
            {/each}
          </ul>
        </div>

        <div class="info-block recommendations">
          <div class="block-header">
            <Eye size={16} />
            <h4>Best Practices</h4>
          </div>
          <ul class="info-bullet-list">
            {#each currentBadge.recommendations as rec, idx (idx)}
              <li>{rec}</li>
            {/each}
          </ul>
        </div>

        <div class="info-technical">
          <strong>Technical Details:</strong>
          {currentBadge.technical}
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-modal btn-primary" onclick={onClose}>
          Got it
        </button>
      </div>
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
    max-width: 600px;
    width: 90%;
    max-height: 90vh;
    overflow-y: auto;
    animation: slideUp 0.25s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .modal-header {
    display: flex;
    align-items: center;
    gap: 0.875rem;
    padding: 1.25rem 1.25rem 1rem;
    border-bottom: 1px solid var(--border);
    position: sticky;
    top: 0;
    background: var(--background);
    z-index: 10;
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

  .info-section {
    margin-bottom: 1.5rem;
  }

  .info-description {
    font-size: 0.9375rem;
    line-height: 1.6;
    color: var(--foreground);
    margin: 0;
  }

  .info-block {
    margin-bottom: 1.25rem;
    padding: 1rem;
    border-radius: 0.5rem;
    border: 1px solid var(--border);
  }

  .info-block.risks {
    background: rgba(239, 68, 68, 0.05);
    border-color: rgba(239, 68, 68, 0.2);
  }

  .info-block.recommendations {
    background: rgba(34, 197, 94, 0.05);
    border-color: rgba(34, 197, 94, 0.2);
  }

  .block-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
    color: var(--foreground);
  }

  .block-header h4 {
    font-size: 0.875rem;
    font-weight: 700;
    margin: 0;
  }

  .info-bullet-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .info-bullet-list li {
    font-size: 0.8125rem;
    line-height: 1.5;
    color: var(--muted-foreground);
    padding-left: 1.25rem;
    position: relative;
  }

  .info-bullet-list li::before {
    content: '•';
    position: absolute;
    left: 0.5rem;
    color: var(--foreground);
    font-weight: 700;
  }

  .info-technical {
    padding: 0.875rem;
    background: var(--muted);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    font-size: 0.8125rem;
    line-height: 1.6;
    color: var(--muted-foreground);
  }

  .info-technical strong {
    color: var(--foreground);
  }

  .modal-footer {
    display: flex;
    gap: 0.625rem;
    padding: 1rem 1.25rem 1.25rem;
    border-top: 1px solid var(--border);
    position: sticky;
    bottom: 0;
    background: var(--background);
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

  .btn-primary {
    background: var(--foreground);
    border-color: var(--foreground);
    color: var(--background);
  }

  .btn-primary:hover {
    opacity: 0.9;
  }

  .btn-modal:active {
    transform: scale(0.98);
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
</style>
