<!--
  @file NavButton Component
  @glinr/sentinel

  Reusable navigation button component.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.
-->

<script lang="ts">
  import type { ComponentType } from 'svelte';

  interface Props {
    icon: ComponentType;
    label?: string;
    badge?: number;
    active?: boolean;
    collapsed?: boolean;
    title?: string;
    onclick?: () => void;
    onkeydown?: (e: KeyboardEvent) => void;
  }

  let {
    icon: Icon,
    label = '',
    badge = undefined,
    active = false,
    collapsed = false,
    title = '',
    onclick,
    onkeydown
  }: Props = $props();
</script>

<button
  class="glinr-nav-button"
  class:active
  class:collapsed
  {title}
  aria-current={active ? 'page' : undefined}
  {onclick}
  {onkeydown}
>
  <Icon size={18} class="icon" />
  {#if !collapsed && label}
    <span class="label">{label}</span>
  {/if}
  {#if !collapsed && badge}
    <span class="badge" aria-label="{badge} items">
      {badge}
    </span>
  {/if}
</button>

<style>
  .glinr-nav-button {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    background: transparent;
    font-size: var(--font-size-base);
    font-weight: 500;
    border: none;
    cursor: pointer;
    transition: all var(--transition-fast);
    position: relative;
  }

  .glinr-nav-button :global(.icon) {
    flex-shrink: 0;
    color: var(--text-secondary);
    transition: color var(--transition-fast);
  }

  .glinr-nav-button.collapsed {
    justify-content: center;
    padding: var(--space-sm);
  }

  .glinr-nav-button:hover {
    background: var(--bg-hover);
  }

  .glinr-nav-button:hover :global(.icon) {
    color: var(--text-primary);
  }

  .glinr-nav-button.active {
    background: var(--button-active-bg);
  }

  .glinr-nav-button.active :global(.icon) {
    color: var(--button-active-text);
  }

  .glinr-nav-button.active::before {
    content: '';
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    width: 3px;
    height: 60%;
    background: var(--button-active-text);
    border-radius: 0 2px 2px 0;
  }

  .label {
    flex: 1;
    color: var(--text-secondary);
    transition: color var(--transition-fast);
  }

  .glinr-nav-button:hover .label {
    color: var(--text-primary);
  }

  .glinr-nav-button.active .label {
    color: var(--button-active-text);
  }

  .badge {
    padding: 2px 8px;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    font-weight: 600;
    line-height: 1.2;
    background: var(--error);
    color: white;
  }
</style>
