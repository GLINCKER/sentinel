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

  interface NavItem {
    id: View;
    label: string;
    icon: string;
    badge?: number;
  }

  let navItems = $derived<NavItem[]>([
    {
      id: 'dashboard',
      label: 'Dashboard',
      icon: '📊',
      badge: $crashedProcesses.length > 0 ? $crashedProcesses.length : undefined
    },
    {
      id: 'settings',
      label: 'Settings',
      icon: '⚙️'
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
</script>

<aside class="glinr-sidebar" role="navigation" aria-label="Main navigation">
  <!-- Brand Header -->
  <div class="glinr-brand">
    <div class="glinr-logo">
      <svg width="32" height="32" viewBox="0 0 32 32" aria-hidden="true">
        <rect width="32" height="32" rx="6" fill="var(--accent-primary)" />
        <text
          x="16"
          y="22"
          text-anchor="middle"
          fill="white"
          font-size="18"
          font-weight="bold">S</text
        >
      </svg>
    </div>
    <div class="glinr-brand-text">
      <h1 class="glinr-product-name">Sentinel</h1>
      <p class="glinr-tagline">A GLINR Product</p>
    </div>
  </div>

  <!-- Status Badge -->
  <div class="sentinel-status-badge">
    <span class="status-dot" class:active={$runningCount > 0}></span>
    <span class="status-text">
      {$runningCount} Process{$runningCount !== 1 ? 'es' : ''} Running
    </span>
  </div>

  <!-- Navigation Items -->
  <nav class="glinr-nav">
    {#each navItems as item (item.id)}
      <button
        class="glinr-nav-item"
        class:active={$currentView === item.id}
        onclick={() => handleNavigation(item.id)}
        onkeydown={(e) => handleKeyDown(e, item.id)}
        aria-current={$currentView === item.id ? 'page' : undefined}
      >
        <span class="glinr-nav-icon" aria-hidden="true">{item.icon}</span>
        <span class="glinr-nav-label">{item.label}</span>
        {#if item.badge}
          <span
            class="glinr-badge glinr-badge-error"
            aria-label="{item.badge} items"
          >
            {item.badge}
          </span>
        {/if}
      </button>
    {/each}
  </nav>

  <!-- Footer -->
  <div class="glinr-sidebar-footer">
    <a
      href="https://glincker.com/sentinel"
      target="_blank"
      rel="noopener noreferrer"
      class="glinr-footer-link"
    >
      <span class="glinr-logo-small">GLINR</span>
      <span class="glinr-version">v0.1.0-alpha</span>
    </a>
  </div>
</aside>

<style>
  .glinr-sidebar {
    width: var(--sidebar-width);
    height: 100%;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    padding: var(--space-lg);
    gap: var(--space-lg);
    overflow-y: auto;
    flex-shrink: 0;
  }

  /* Brand Header */
  .glinr-brand {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding-bottom: var(--space-lg);
    border-bottom: 1px solid var(--border-light);
  }

  .glinr-logo {
    flex-shrink: 0;
  }

  .glinr-brand-text {
    flex: 1;
    min-width: 0;
  }

  .glinr-product-name {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
    line-height: 1.2;
  }

  .glinr-tagline {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    margin: 0;
    line-height: 1.2;
  }

  /* Status Badge */
  .sentinel-status-badge {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--text-tertiary);
    transition: background var(--transition-fast);
  }

  .status-dot.active {
    background: var(--success);
    box-shadow: 0 0 8px var(--success);
  }

  .status-text {
    color: var(--text-secondary);
    font-weight: 500;
  }

  /* Navigation */
  .glinr-nav {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .glinr-nav-item {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-md);
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--text-secondary);
    font-size: var(--font-size-base);
    font-weight: 500;
    border: none;
    cursor: pointer;
    transition: all var(--transition-fast);
    position: relative;
  }

  .glinr-nav-item:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .glinr-nav-item.active {
    background: var(--accent-primary);
    color: white;
  }

  .glinr-nav-item.active::before {
    content: '';
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    width: 3px;
    height: 60%;
    background: white;
    border-radius: 0 2px 2px 0;
  }

  .glinr-nav-icon {
    font-size: var(--font-size-xl);
    line-height: 1;
  }

  .glinr-nav-label {
    flex: 1;
  }

  .glinr-badge {
    padding: 2px 8px;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    font-weight: 600;
    line-height: 1.2;
  }

  .glinr-badge-error {
    background: var(--error);
    color: white;
  }

  /* Footer */
  .glinr-sidebar-footer {
    padding-top: var(--space-lg);
    border-top: 1px solid var(--border-light);
  }

  .glinr-footer-link {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm);
    color: var(--text-tertiary);
    text-decoration: none;
    font-size: var(--font-size-xs);
    border-radius: var(--radius-sm);
    transition: all var(--transition-fast);
  }

  .glinr-footer-link:hover {
    background: var(--bg-hover);
    color: var(--text-secondary);
  }

  .glinr-logo-small {
    font-weight: 700;
    letter-spacing: 0.5px;
  }

  .glinr-version {
    opacity: 0.7;
  }
</style>
