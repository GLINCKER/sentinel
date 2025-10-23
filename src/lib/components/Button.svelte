<script lang="ts">
  import type { ComponentType } from 'svelte';

  interface Props {
    variant?: 'default' | 'success' | 'error' | 'primary';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    icon?: ComponentType;
    iconSize?: number;
    onclick?: (e: MouseEvent) => void;
    children?: import('svelte').Snippet;
    class?: string;
    type?: 'button' | 'submit' | 'reset';
    ariaLabel?: string;
  }

  let {
    variant = 'default',
    size = 'md',
    disabled = false,
    icon,
    iconSize = 16,
    onclick,
    children,
    class: className = '',
    type = 'button',
    ariaLabel
  }: Props = $props();
</script>

<button
  class="glinr-btn glinr-btn-{variant} glinr-btn-{size} {className}"
  {disabled}
  {type}
  {onclick}
  aria-label={ariaLabel}
>
  {#if icon}
    <svelte:component this={icon} size={iconSize} />
  {/if}
  {#if children}
    {@render children()}
  {/if}
</button>

<style>
  .glinr-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    font-weight: 600;
    font-family: inherit;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    cursor: pointer;
    transition: all var(--transition-fast);
    white-space: nowrap;
  }

  /* Sizes */
  .glinr-btn-sm {
    padding: 4px 10px;
    font-size: var(--font-size-xs);
  }

  .glinr-btn-md {
    padding: 6px 12px;
    font-size: var(--font-size-xs);
  }

  .glinr-btn-lg {
    padding: 8px 16px;
    font-size: var(--font-size-sm);
  }

  /* Hover & Active */
  .glinr-btn:hover:not(:disabled) {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
    transform: translateY(-1px);
  }

  .glinr-btn:active:not(:disabled) {
    transform: translateY(0);
  }

  /* Disabled */
  .glinr-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Variants */
  .glinr-btn-success:hover:not(:disabled) {
    background: var(--success-bg);
    border-color: var(--success);
    color: var(--success);
  }

  .glinr-btn-error:hover:not(:disabled) {
    background: var(--error-bg);
    border-color: var(--error);
    color: var(--error);
  }

  .glinr-btn-primary {
    background: var(--accent-primary);
    border-color: var(--accent-primary);
    color: white;
  }

  .glinr-btn-primary:hover:not(:disabled) {
    background: var(--accent-secondary);
    border-color: var(--accent-secondary);
    color: white;
    transform: translateY(-1px);
  }
</style>
