<script lang="ts">
  import type { ComponentType } from 'svelte';
  import { Info } from 'lucide-svelte';
  import TitleBar from './TitleBar.svelte';

  interface Props {
    title: string;
    subtitle?: string;
    icon?: ComponentType;
    showInfoButton?: boolean;
    onInfoClick?: () => void;
    children?: import('svelte').Snippet;
  }

  let {
    title,
    subtitle,
    icon: Icon,
    showInfoButton = false,
    onInfoClick,
    children
  }: Props = $props();
</script>

<TitleBar>
  <header class="page-header" data-tauri-drag-region>
    <div class="header-content" data-tauri-drag-region>
      {#if Icon}
        <div class="header-icon" data-tauri-drag-region>
          <Icon size={18} strokeWidth={2.5} />
        </div>
      {/if}
      <div class="header-text" data-tauri-drag-region>
        <div class="title-row" data-tauri-drag-region>
          <h1 data-tauri-drag-region>{title}</h1>
          {#if showInfoButton && onInfoClick}
            <button
              class="info-button"
              onclick={onInfoClick}
              aria-label="Information"
            >
              <Info size={18} />
            </button>
          {/if}
        </div>
        {#if subtitle}
          <p class="subtitle" data-tauri-drag-region>{subtitle}</p>
        {/if}
      </div>
    </div>

    {#if children}
      <div class="header-actions">
        {@render children()}
      </div>
    {/if}
  </header>
</TitleBar>

<style>
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-lg);
    padding: var(--space-md) var(--space-xl);
    gap: var(--space-lg);
    flex-wrap: wrap;
  }

  @media (max-width: 768px) {
    .page-header {
      flex-direction: column;
      align-items: flex-start;
    }
  }

  .header-content {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .header-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-md);
    background: var(--text-primary) !important;
    color: var(--bg-primary);
    flex-shrink: 0;
  }

  .header-text {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  h1 {
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
    letter-spacing: -0.3px;
  }

  .info-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    padding: 0;
    border: none;
    background: rgba(59, 130, 246, 0.1);
    color: #3b82f6;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: all 0.2s;
  }

  .info-button:hover {
    background: rgba(59, 130, 246, 0.2);
    transform: scale(1.05);
  }

  .subtitle {
    font-size: 12px;
    color: var(--text-tertiary);
    margin: 0;
    margin-top: 1px;
    line-height: 1.3;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  @media (max-width: 768px) {
    .header-actions {
      width: 100%;
    }
  }
</style>
