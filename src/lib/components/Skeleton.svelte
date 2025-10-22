<!--
  @file Skeleton Component
  @glinr/sentinel

  Reusable skeleton loader for loading states.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.
-->

<script lang="ts">
  interface Props {
    width?: string;
    height?: string;
    variant?: 'text' | 'rectangular' | 'circular';
    count?: number;
  }

  let {
    width = '100%',
    height = '1rem',
    variant = 'rectangular',
    count = 1
  }: Props = $props();
</script>

{#each Array(count) as _, i (i)}
  <div
    class="skeleton {variant}"
    style="width: {width}; height: {height};"
  ></div>
{/each}

<style>
  .skeleton {
    background: linear-gradient(
      90deg,
      var(--bg-tertiary) 0%,
      var(--bg-hover) 50%,
      var(--bg-tertiary) 100%
    );
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
    border-radius: var(--radius-md);
  }

  .skeleton.text {
    height: 1rem;
    margin-bottom: var(--space-xs);
    border-radius: var(--radius-sm);
  }

  .skeleton.circular {
    border-radius: 50%;
  }

  @keyframes shimmer {
    0% {
      background-position: 200% 0;
    }
    100% {
      background-position: -200% 0;
    }
  }
</style>
