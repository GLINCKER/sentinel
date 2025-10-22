<!--
  @file IconButton Component
  @glinr/sentinel

  Reusable icon button component with proper theme support.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import type { ComponentType } from 'svelte';

  interface Props {
    icon: ComponentType;
    iconSize?: number;
    active?: boolean;
    title?: string;
    ariaLabel?: string;
    variant?: 'default' | 'toggle';
    onclick?: () => void;
  }

  let {
    icon: Icon,
    iconSize = 16,
    active = false,
    title = '',
    ariaLabel = '',
    variant = 'default',
    onclick
  }: Props = $props();
</script>

<button
  class="glinr-icon-button"
  class:active
  class:toggle={variant === 'toggle'}
  {title}
  aria-label={ariaLabel}
  {onclick}
>
  <Icon size={iconSize} />
</button>

<style>
  .glinr-icon-button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-xs);
    border: none;
    background: transparent;
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .glinr-icon-button :global(svg) {
    color: var(--text-secondary);
    transition: color var(--transition-fast);
  }

  .glinr-icon-button:hover :global(svg) {
    color: var(--text-primary);
  }

  .glinr-icon-button:hover {
    background: var(--bg-hover);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
  }

  /* Toggle variant - for theme switcher buttons */
  .glinr-icon-button.toggle {
    flex: 1;
  }

  .glinr-icon-button.toggle.active {
    background: var(--button-active-bg);
    border: 1px solid var(--border-color);
    box-shadow:
      0 2px 8px rgba(0, 0, 0, 0.2),
      0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .glinr-icon-button.toggle.active :global(svg) {
    color: var(--button-active-text);
  }
</style>
