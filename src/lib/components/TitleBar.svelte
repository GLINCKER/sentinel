<script lang="ts">
  import { onMount } from 'svelte';

  let titleBarRef: HTMLDivElement;

  onMount(() => {
    // Enable window dragging on the title bar
    if (titleBarRef) {
      titleBarRef.setAttribute('data-tauri-drag-region', '');
    }
  });
</script>

<div bind:this={titleBarRef} class="title-bar" data-tauri-drag-region>
  <slot />
</div>

<style>
  .title-bar {
    -webkit-app-region: drag;
    app-region: drag;
    user-select: none;
    -webkit-user-select: none;
  }

  /* Buttons and interactive elements inside should not drag */
  .title-bar :global(button),
  .title-bar :global(input),
  .title-bar :global(select),
  .title-bar :global(a) {
    -webkit-app-region: no-drag;
    app-region: no-drag;
  }
</style>
