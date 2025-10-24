<script lang="ts">
  import { onMount } from 'svelte';
  import { currentView } from './stores/navigation';
  import { theme } from './stores/settings';
  import Dashboard from './views/Dashboard.svelte';
  import ProcessDetail from './views/ProcessDetail.svelte';
  import PortMapView from './views/PortMapView.svelte';
  import Settings from './views/Settings.svelte';
  import Sidebar from './components/Sidebar.svelte';
  import KeyboardShortcuts from './components/KeyboardShortcuts.svelte';
  import ShellView from './lib/components/Shell/ShellView.svelte';
  import NetworkMonitor from './routes/network/+page.svelte';
  import Docker from './routes/docker/+page.svelte';

  let mounted = $state(false);

  // Check if system prefers dark mode
  function getSystemPrefersDark(): boolean {
    return window.matchMedia('(prefers-color-scheme: dark)').matches;
  }

  // Determine if dark mode should be active
  function shouldUseDarkMode(themeValue: string): boolean {
    if (themeValue === 'dark') return true;
    if (themeValue === 'light') return false;
    if (themeValue === 'system') return getSystemPrefersDark();
    return false;
  }

  onMount(() => {
    // Apply saved theme
    const savedTheme = localStorage.getItem('sentinel-theme') || 'system';
    theme.set(savedTheme as 'light' | 'dark' | 'system');
    document.documentElement.classList.toggle(
      'dark',
      shouldUseDarkMode(savedTheme)
    );

    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = () => {
      if ($theme === 'system') {
        document.documentElement.classList.toggle(
          'dark',
          getSystemPrefersDark()
        );
      }
    };
    mediaQuery.addEventListener('change', handleChange);

    mounted = true;

    return () => {
      mediaQuery.removeEventListener('change', handleChange);
    };
  });

  // Reactive theme class updates
  $effect(() => {
    if (mounted) {
      document.documentElement.classList.toggle(
        'dark',
        shouldUseDarkMode($theme)
      );
      localStorage.setItem('sentinel-theme', $theme);
    }
  });
</script>

<div class="app" class:dark={shouldUseDarkMode($theme)}>
  <div class="app-container">
    <Sidebar />

    <main class="main-content">
      {#if $currentView === 'dashboard'}
        <Dashboard />
      {:else if $currentView === 'process-detail'}
        <ProcessDetail />
      {:else if $currentView === 'port-map'}
        <PortMapView />
      {:else if $currentView === 'network'}
        <NetworkMonitor />
      {:else if $currentView === 'docker'}
        <Docker />
      {:else if $currentView === 'shell'}
        <ShellView />
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
    font-family:
      -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu,
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
