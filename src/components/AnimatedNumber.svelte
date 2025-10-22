<!--
  @file Animated Number Component
  @glinr/ui-components

  Smooth animated number transitions with spring physics

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.
-->

<script lang="ts">
  import { spring } from 'svelte/motion';

  interface Props {
    value: number;
    precision?: number;
    class?: string;
  }

  let { value, precision = 1, class: className = '' }: Props = $props();

  // Spring animation for smooth transitions
  const displayed = spring(value, {
    stiffness: 0.1,
    damping: 0.25
  });

  // Update spring when value changes
  $effect(() => {
    displayed.set(value);
  });
</script>

<span class={className}>
  {$displayed.toFixed(precision)}
</span>
