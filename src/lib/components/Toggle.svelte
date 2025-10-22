<!--
  @file Toggle Component
  @glinr/sentinel

  Reusable toggle switch component with theme support.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.
-->

<script lang="ts">
  interface Props {
    checked?: boolean;
    disabled?: boolean;
    ariaLabel?: string;
    onchange?: (checked: boolean) => void;
  }

  let {
    checked = $bindable(false),
    disabled = false,
    ariaLabel = '',
    onchange
  }: Props = $props();

  function handleChange(event: Event) {
    const target = event.target as HTMLInputElement;
    checked = target.checked;
    onchange?.(checked);
  }
</script>

<input
  type="checkbox"
  class="toggle"
  bind:checked
  {disabled}
  aria-label={ariaLabel}
  onchange={handleChange}
/>

<style>
  .toggle {
    width: 48px;
    height: 28px;
    appearance: none;
    background: var(--bg-tertiary);
    border-radius: 14px;
    position: relative;
    cursor: pointer;
    transition: background var(--transition-fast);
    border: 1px solid var(--border-color);
    flex-shrink: 0;
  }

  .toggle:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .toggle::before {
    content: '';
    position: absolute;
    width: 22px;
    height: 22px;
    background: white;
    border-radius: 50%;
    top: 2px;
    left: 2px;
    transition: transform var(--transition-fast);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .toggle:checked {
    background: var(--button-active-bg);
    border-color: var(--button-active-bg);
  }

  .toggle:checked::before {
    transform: translateX(20px);
    background: var(--button-active-text);
  }

  .toggle:focus-visible {
    outline: 2px solid var(--accent-primary);
    outline-offset: 2px;
  }
</style>
