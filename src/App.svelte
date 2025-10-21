<script lang="ts">
  import { onMount } from 'svelte';
  import { currentView } from './stores/navigation';
  import { theme } from './stores/settings';
  import Dashboard from './views/Dashboard.svelte';
  import ProcessDetail from './views/ProcessDetail.svelte';
  import Settings from './views/Settings.svelte';
  import Sidebar from './components/Sidebar.svelte';
  import KeyboardShortcuts from './components/KeyboardShortcuts.svelte';

  let mounted = $state(false);

  onMount(() => {
    // Apply saved theme
    const savedTheme = localStorage.getItem('sentinel-theme') || 'dark';
    theme.set(savedTheme);
    document.documentElement.classList.toggle('dark', savedTheme === 'dark');

    mounted = true;
  });

  // Reactive theme class updates
  $effect(() => {
    if (mounted) {
      document.documentElement.classList.toggle('dark', $theme === 'dark');
      localStorage.setItem('sentinel-theme', $theme);
    }
  });
</script>

<div class="app" class:dark={$theme === 'dark'}>
  <div class="app-container">
    <Sidebar />

    <main class="main-content" role="main">
      {#if $currentView === 'dashboard'}
        <Dashboard />
      {:else if $currentView === 'process-detail'}
        <ProcessDetail />
      {:else if $currentView === 'settings'}
        <Settings />
      {/if}
    </main>
  </div>

  <KeyboardShortcuts />
</div>

<style>
  .app {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu,
                 Cantarell, 'Helvetica Neue', sans-serif;
  }

  .app-container {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .main-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
</style>
