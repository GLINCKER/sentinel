<!--
  @file Sidebar Navigation Component
  @glinr/sentinel

  Main navigation sidebar for Sentinel application.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { currentView, navigateTo, type View } from '../stores/navigation';
  import { runningCount, crashedProcesses } from '../stores/processes';
  import { theme } from '../stores/settings';
  import { portStore } from '$lib/stores/port.svelte';
  import {
    LayoutDashboard,
    Settings,
    PanelLeftClose,
    PanelLeftOpen,
    Sun,
    Moon,
    Monitor,
    Network as NetworkIcon,
    Terminal,
    Activity,
    Container
  } from 'lucide-svelte';
  import { IconButton, NavButton, StatusBadge } from '../lib/components';
  import type { ComponentType } from 'svelte';

  interface NavItem {
    id: View;
    label: string;
    icon: ComponentType;
    badge?: number;
  }

  let isCollapsed = $state(false);

  let mainNavItems = $derived<NavItem[]>([
    {
      id: 'dashboard',
      label: 'Dashboard',
      icon: LayoutDashboard,
      badge: $crashedProcesses.length > 0 ? $crashedProcesses.length : undefined
    },
    {
      id: 'port-map',
      label: 'Port Map',
      icon: NetworkIcon,
      badge: portStore.stats.total > 0 ? portStore.stats.total : undefined
    },
    {
      id: 'network',
      label: 'Network',
      icon: Activity
    },
    {
      id: 'docker',
      label: 'Docker',
      icon: Container
    },
    {
      id: 'shell',
      label: 'Terminal',
      icon: Terminal
    }
  ]);

  function handleNavigation(view: View) {
    navigateTo(view);
  }

  function handleKeyDown(event: KeyboardEvent, view: View) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleNavigation(view);
    }
  }

  function toggleSidebar() {
    isCollapsed = !isCollapsed;
  }

  function toggleTheme() {
    theme.update((t) => {
      if (t === 'system') return 'light';
      if (t === 'light') return 'dark';
      return 'system';
    });
  }
</script>

<aside
  class="sidebar"
  class:collapsed={isCollapsed}
  role="navigation"
  aria-label="Main navigation"
>
  <!-- Brand Header with Toggle -->
  <div class="brand">
    <div class="logo">
      <img src="/assets/sentinel-logo.svg" alt="Sentinel Logo" />
    </div>
    {#if !isCollapsed}
      <div class="brand-text">
        <h1 class="product-name">Sentinel</h1>
        <p class="tagline">A GLINR Product</p>
      </div>
    {/if}
    <button
      class="sidebar-toggle"
      onclick={toggleSidebar}
      title={isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
      aria-label={isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
    >
      {#if isCollapsed}
        <PanelLeftOpen size={16} />
      {:else}
        <PanelLeftClose size={16} />
      {/if}
    </button>
  </div>

  <!-- Status Badge -->
  {#if !isCollapsed && $runningCount > 0}
    <StatusBadge count={$runningCount} />
  {/if}

  <!-- Navigation Items -->
  <nav class="nav">
    {#each mainNavItems as item (item.id)}
      <NavButton
        icon={item.icon}
        label={item.label}
        badge={item.badge}
        active={$currentView === item.id}
        collapsed={isCollapsed}
        title={item.label}
        onclick={() => handleNavigation(item.id)}
        onkeydown={(e) => handleKeyDown(e, item.id)}
      />
    {/each}
  </nav>

  <!-- Bottom Actions -->
  <div class="sidebar-bottom">
    {#if !isCollapsed}
      <div class="divider"></div>
    {/if}

    <!-- Theme Switcher -->
    {#if isCollapsed}
      <button
        class="bottom-action collapsed"
        onclick={toggleTheme}
        title={$theme === 'system'
          ? 'System theme'
          : $theme === 'dark'
            ? 'Dark mode'
            : 'Light mode'}
        aria-label="Toggle theme"
      >
        {#if $theme === 'system'}
          <Monitor size={18} />
        {:else if $theme === 'dark'}
          <Moon size={18} />
        {:else}
          <Sun size={18} />
        {/if}
      </button>
    {:else}
      <div class="theme-toggle-group">
        <IconButton
          icon={Sun}
          active={$theme === 'light'}
          onclick={() => theme.set('light')}
          title="Light mode"
          ariaLabel="Light mode"
          variant="toggle"
        />
        <IconButton
          icon={Moon}
          active={$theme === 'dark'}
          onclick={() => theme.set('dark')}
          title="Dark mode"
          ariaLabel="Dark mode"
          variant="toggle"
        />
        <IconButton
          icon={Monitor}
          active={$theme === 'system'}
          onclick={() => theme.set('system')}
          title="System theme"
          ariaLabel="System theme"
          variant="toggle"
        />
      </div>
    {/if}

    <!-- Settings Button -->
    <NavButton
      icon={Settings}
      label="Settings"
      active={$currentView === 'settings'}
      collapsed={isCollapsed}
      title="Settings"
      onclick={() => handleNavigation('settings')}
      onkeydown={(e) => handleKeyDown(e, 'settings')}
    />

    <!-- Footer Version -->
    {#if !isCollapsed}
      <div class="footer">
        <span class="version">Sentinel v0.1.0-alpha</span>
      </div>
    {/if}
  </div>
</aside>

<style>
  .sidebar {
    position: relative;
    width: var(--sidebar-width);
    height: 100%;
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    padding: var(--space-lg);
    padding-top: 52px;
    gap: var(--space-lg);
    overflow-y: auto;
    overflow-x: hidden;
    flex-shrink: 0;
    transition: width var(--transition-base);
  }

  .sidebar.collapsed {
    width: 72px;
  }

  /* Brand Header */
  .brand {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding-bottom: var(--space-md);
    padding-right: 40px;
    border-bottom: 1px solid var(--border-light);
    position: relative;
  }

  .sidebar.collapsed .brand {
    justify-content: center;
    flex-direction: column;
    gap: var(--space-sm);
    padding-right: 0;
  }

  .sidebar-toggle {
    position: absolute;
    right: var(--space-sm);
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 0;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all var(--transition-fast);
    flex-shrink: 0;
  }

  .sidebar-toggle:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .sidebar.collapsed .sidebar-toggle {
    position: relative;
    right: auto;
    top: auto;
    transform: none;
  }

  .logo {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .logo img {
    width: 36px;
    height: 36px;
    object-fit: contain;
  }

  .brand-text {
    flex: 1;
    min-width: 0;
  }

  .product-name {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
    line-height: 1.2;
  }

  .tagline {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    margin: 0;
    line-height: 1.2;
    white-space: nowrap;
  }

  /* Navigation */
  .nav {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  /* Bottom Actions */
  .sidebar-bottom {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    margin-top: auto;
  }

  .divider {
    height: 1px;
    background: var(--border-light);
    margin: var(--space-md) 0;
  }

  .bottom-action {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-sm);
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--text-secondary);
    border: none;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .bottom-action:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .theme-toggle-group {
    display: flex;
    gap: var(--space-xs);
    padding: var(--space-xs);
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
  }

  /* Footer */
  .footer {
    margin-top: var(--space-md);
    padding-top: var(--space-md);
    border-top: 1px solid var(--border-light);
    text-align: center;
  }

  .version {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    font-weight: 500;
  }
</style>
