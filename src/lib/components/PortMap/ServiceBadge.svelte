<script lang="ts">
  import * as simpleIcons from 'simple-icons';
  import { open } from '@tauri-apps/plugin-shell';

  interface ServiceInfo {
    name: string;
    category: string;
    version?: string;
    description: string;
    docs_url?: string;
    health_status?: string;
    confidence: number;
    icon: string;
  }

  interface Props {
    service: ServiceInfo;
    size?: 'sm' | 'md' | 'lg';
  }

  let { service, size = 'md' }: Props = $props();

  // Category colors
  const categoryColors: Record<string, string> = {
    WebFramework: '#3b82f6', // blue
    Database: '#10b981', // green
    MessageQueue: '#f59e0b', // amber
    Cache: '#ef4444', // red
    Proxy: '#8b5cf6', // purple
    Development: '#06b6d4' // cyan
  };

  const bgColor = categoryColors[service.category] || '#6b7280';
  const confidencePercent = Math.round(service.confidence * 100);

  // Get icon from simple-icons (icon slug is stored in service.icon)
  const iconKey =
    `si${service.icon.charAt(0).toUpperCase()}${service.icon.slice(1).replace(/-/g, '')}` as keyof typeof simpleIcons;
  const iconData = simpleIcons[iconKey];
  const iconSize = size === 'sm' ? 14 : size === 'lg' ? 20 : 16;

  async function openDocs(e: MouseEvent) {
    e.preventDefault();
    if (service.docs_url) {
      try {
        await open(service.docs_url);
      } catch (error) {
        console.error('Failed to open docs URL:', error);
      }
    }
  }
</script>

<div class="service-badge" class:sm={size === 'sm'} class:lg={size === 'lg'}>
  <div class="badge-content" style="--badge-color: {bgColor}">
    <div class="icon">
      {#if iconData}
        {@html `<svg role="img" viewBox="0 0 24 24" width="${iconSize}" height="${iconSize}" fill="${iconData.hex ? `#${iconData.hex}` : 'currentColor'}" xmlns="http://www.w3.org/2000/svg"><title>${iconData.title}</title><path d="${iconData.path}"/></svg>`}
      {/if}
    </div>
    <div class="info">
      <div class="name-row">
        <span class="name">{service.name}</span>
        {#if service.version}
          <span class="version">v{service.version}</span>
        {/if}
      </div>
      {#if size !== 'sm'}
        <span class="category">{service.category}</span>
      {/if}
    </div>
    {#if size === 'lg'}
      <div class="confidence" title="Detection confidence">
        <div class="confidence-bar">
          <div
            class="confidence-fill"
            style="width: {confidencePercent}%"
          ></div>
        </div>
        <span class="confidence-text">{confidencePercent}%</span>
      </div>
    {/if}
  </div>

  {#if service.docs_url}
    <button onclick={openDocs} class="docs-link" title="View documentation">
      <svg
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <path
          d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6M15 3h6v6M10 14L21 3"
        />
      </svg>
    </button>
  {/if}
</div>

<style>
  .service-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    padding: 6px 10px;
    font-size: 13px;
    transition: all 0.2s ease;
    position: relative;
  }

  .service-badge:hover {
    background: var(--bg-tertiary);
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .service-badge.sm {
    padding: 4px 8px;
    font-size: 11px;
  }

  .service-badge.lg {
    padding: 8px 12px;
    font-size: 14px;
  }

  .badge-content {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
  }

  .icon {
    display: flex;
    align-items: center;
    justify-content: center;
    line-height: 1;
    filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1));
  }

  .icon :global(svg) {
    display: block;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .name-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .name {
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1.2;
  }

  .version {
    font-size: 10px;
    color: var(--text-tertiary);
    background: var(--badge-color);
    padding: 1px 5px;
    border-radius: var(--radius-sm);
    font-weight: 500;
    opacity: 0.8;
  }

  .category {
    font-size: 10px;
    color: var(--text-tertiary);
    text-transform: capitalize;
    line-height: 1;
  }

  .confidence {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-left: auto;
  }

  .confidence-bar {
    width: 40px;
    height: 4px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .confidence-fill {
    height: 100%;
    background: var(--badge-color);
    transition: width 0.3s ease;
  }

  .confidence-text {
    font-size: 10px;
    color: var(--text-tertiary);
    font-weight: 600;
    min-width: 32px;
    text-align: right;
  }

  .docs-link {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border-radius: var(--radius-sm);
    color: var(--text-tertiary);
    transition: all 0.2s ease;
    border: none;
    background: transparent;
    cursor: pointer;
    padding: 0;
  }

  .docs-link:hover {
    background: var(--badge-color);
    color: white;
  }

  .sm .docs-link {
    width: 20px;
    height: 20px;
  }

  .lg .docs-link {
    width: 28px;
    height: 28px;
  }
</style>
