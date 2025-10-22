<script lang="ts">
  import {
    Info,
    Wifi,
    Globe,
    AlertTriangle,
    RefreshCw,
    X
  } from 'lucide-svelte';

  interface Props {
    show: boolean;
    onClose: () => void;
  }

  let { show = $bindable(), onClose }: Props = $props();
</script>

{#if show}
  <div class="modal-overlay" onclick={onClose}>
    <div class="modal-content info-modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <div class="modal-icon-wrapper info">
          <Info size={24} />
        </div>
        <h3 class="modal-title">Why Does Port Count Change?</h3>
        <button class="modal-close" onclick={onClose} aria-label="Close modal">
          <X size={20} />
        </button>
      </div>

      <div class="modal-body">
        <div class="info-section">
          <h4 class="info-subtitle">This is completely normal behavior</h4>
          <p class="info-text">
            Port counts fluctuate between scans due to the dynamic nature of
            network connections. Here's why:
          </p>
        </div>

        <div class="info-list">
          <div class="info-item">
            <div class="info-icon">
              <Wifi size={16} />
            </div>
            <div class="info-content">
              <strong>Ephemeral Connections</strong>
              <p>
                Short-lived connections (browser requests, API calls) that open
                and close within seconds
              </p>
            </div>
          </div>

          <div class="info-item">
            <div class="info-icon">
              <Globe size={16} />
            </div>
            <div class="info-content">
              <strong>Connection States</strong>
              <p>
                Connections transition through states: Listen → Established →
                TimeWait → Closed
              </p>
            </div>
          </div>

          <div class="info-item">
            <div class="info-icon">
              <AlertTriangle size={16} />
            </div>
            <div class="info-content">
              <strong>Dynamic Port Allocation</strong>
              <p>
                Applications use temporary ports (32768-65535) that are released
                after use
              </p>
            </div>
          </div>

          <div class="info-item">
            <div class="info-icon">
              <RefreshCw size={16} />
            </div>
            <div class="info-content">
              <strong>Background Services</strong>
              <p>
                System processes continuously open/close ports for updates,
                sync, and maintenance
              </p>
            </div>
          </div>
        </div>

        <div class="info-tip">
          <strong>Pro Tip:</strong> Use the quick filters above to focus on persistent
          ports like "Listening" or "Development" to see more stable counts.
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
    max-width: 560px;
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

  .info-section {
    margin-bottom: 1.25rem;
  }

  .info-subtitle {
    font-size: 0.9375rem;
    font-weight: 700;
    color: var(--foreground);
    margin: 0 0 0.5rem 0;
  }

  .info-text {
    font-size: 0.875rem;
    line-height: 1.6;
    color: var(--muted-foreground);
    margin: 0;
  }

  .info-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 1.25rem;
  }

  .info-item {
    display: flex;
    gap: 0.875rem;
    align-items: flex-start;
  }

  .info-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 0.5rem;
    background: var(--muted);
    color: var(--foreground);
    flex-shrink: 0;
  }

  .info-content {
    flex: 1;
  }

  .info-content strong {
    display: block;
    font-size: 0.875rem;
    font-weight: 700;
    color: var(--foreground);
    margin-bottom: 0.25rem;
  }

  .info-content p {
    font-size: 0.8125rem;
    line-height: 1.5;
    color: var(--muted-foreground);
    margin: 0;
  }

  .info-tip {
    padding: 0.875rem;
    background: rgba(59, 130, 246, 0.08);
    border: 1px solid rgba(59, 130, 246, 0.2);
    border-radius: 0.5rem;
    font-size: 0.8125rem;
    line-height: 1.5;
    color: var(--foreground);
  }

  .info-tip strong {
    color: #3b82f6;
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
