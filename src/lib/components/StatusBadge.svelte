<!--
  @file StatusBadge Component
  @glinr/sentinel

  Compact status badge showing running process count.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.
-->

<script lang="ts">
  import { Activity } from 'lucide-svelte';

  interface Props {
    count: number;
  }

  let { count }: Props = $props();
</script>

<div class="status-badge">
  <Activity size={14} class="icon" />
  <span class="text">{count} Running</span>
  <span class="dot" class:active={count > 0}></span>
</div>

<style>
  .status-badge {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
  }

  .status-badge :global(.icon) {
    flex-shrink: 0;
    color: var(--text-tertiary);
  }

  .text {
    flex: 1;
    color: var(--text-secondary);
    font-weight: 600;
    white-space: nowrap;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--text-tertiary);
    transition: background var(--transition-fast);
    flex-shrink: 0;
  }

  .dot.active {
    background: var(--success);
    box-shadow: 0 0 6px var(--success);
  }
</style>
