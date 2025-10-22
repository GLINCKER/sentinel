<!--
  @file Glinr Button Component
  @glinr/ui-components

  Reusable button component for GLINR applications.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    loading?: boolean;
    fullWidth?: boolean;
    onclick?: (e?: MouseEvent) => void;
    type?: 'button' | 'submit' | 'reset';
    children?: Snippet;
  }

  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    loading = false,
    fullWidth = false,
    onclick,
    type = 'button',
    children,
    ...restProps
  }: Props = $props();
</script>

<button
  class="glinr-btn glinr-btn-{variant} glinr-btn-{size}"
  class:glinr-btn-full={fullWidth}
  class:glinr-btn-loading={loading}
  disabled={disabled || loading}
  {onclick}
  {type}
  {...restProps}
>
  {#if loading}
    <span class="glinr-btn-spinner" aria-hidden="true"></span>
  {/if}
  <span class="glinr-btn-content" class:glinr-btn-content-loading={loading}>
    {@render children?.()}
  </span>
</button>

<style>
  .glinr-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    font-family: inherit;
    font-weight: 500;
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--transition-fast);
    position: relative;
    white-space: nowrap;
  }

  .glinr-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .glinr-btn:focus-visible {
    outline: 2px solid var(--accent-primary);
    outline-offset: 2px;
  }

  /* Sizes */
  .glinr-btn-sm {
    padding: var(--space-xs) var(--space-md);
    font-size: var(--font-size-sm);
    min-height: 32px;
  }

  .glinr-btn-md {
    padding: var(--space-sm) var(--space-lg);
    font-size: var(--font-size-base);
    min-height: 40px;
  }

  .glinr-btn-lg {
    padding: var(--space-md) var(--space-xl);
    font-size: var(--font-size-lg);
    min-height: 48px;
  }

  /* Variants */
  .glinr-btn-primary {
    background: var(--accent-primary);
    color: white;
  }

  .glinr-btn-primary:hover:not(:disabled) {
    background: var(--accent-hover);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .glinr-btn-primary:active:not(:disabled) {
    background: var(--accent-active);
    transform: translateY(0);
  }

  .glinr-btn-secondary {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .glinr-btn-secondary:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
  }

  .glinr-btn-secondary:active:not(:disabled) {
    background: var(--bg-active);
  }

  .glinr-btn-danger {
    background: var(--error);
    color: white;
  }

  .glinr-btn-danger:hover:not(:disabled) {
    filter: brightness(1.1);
    transform: translateY(-1px);
    box-shadow: var(--shadow-md);
  }

  .glinr-btn-danger:active:not(:disabled) {
    filter: brightness(0.9);
    transform: translateY(0);
  }

  .glinr-btn-ghost {
    background: transparent;
    color: var(--text-secondary);
  }

  .glinr-btn-ghost:hover:not(:disabled) {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .glinr-btn-ghost:active:not(:disabled) {
    background: var(--bg-active);
  }

  /* Full Width */
  .glinr-btn-full {
    width: 100%;
  }

  /* Loading State */
  .glinr-btn-loading {
    pointer-events: none;
  }

  .glinr-btn-spinner {
    position: absolute;
    width: 16px;
    height: 16px;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  .glinr-btn-content {
    display: inline-flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .glinr-btn-content-loading {
    opacity: 0;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
