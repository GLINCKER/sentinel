<!--
  @file Keyboard Shortcuts Handler
  @glinr/sentinel

  Global keyboard shortcuts for Sentinel application.
  Follows WCAG 2.1.4 - Uses modifier keys to avoid conflicts with assistive technology.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { navigateTo } from '../stores/navigation';
  import { theme } from '../stores/settings';
  import { stopAllProcesses, fetchProcesses } from '../stores/processes';

  let showHelp = $state(false);

  /**
   * Keyboard shortcuts configuration
   * Using Cmd/Ctrl + key to comply with WCAG 2.1.4
   */
  const shortcuts = [
    {
      key: 'd',
      modifiers: ['ctrlKey'],
      action: () => navigateTo('dashboard'),
      description: 'Go to Dashboard'
    },
    {
      key: 's',
      modifiers: ['ctrlKey', 'shiftKey'],
      action: () => navigateTo('settings'),
      description: 'Go to Settings'
    },
    {
      key: 't',
      modifiers: ['ctrlKey'],
      action: () => toggleTheme(),
      description: 'Toggle Theme'
    },
    {
      key: 'r',
      modifiers: ['ctrlKey'],
      action: () => handleRefresh(),
      description: 'Refresh Process List'
    },
    {
      key: 'q',
      modifiers: ['ctrlKey'],
      action: () => handleStopAll(),
      description: 'Stop All Processes'
    },
    {
      key: '/',
      modifiers: ['ctrlKey'],
      action: () => (showHelp = !showHelp),
      description: 'Show/Hide Keyboard Shortcuts'
    },
    {
      key: 'Escape',
      modifiers: [],
      action: () => (showHelp = false),
      description: 'Close Shortcuts Panel'
    }
  ];

  function toggleTheme() {
    theme.update((t) => (t === 'dark' ? 'light' : 'dark'));
  }

  async function handleRefresh() {
    await fetchProcesses();
  }

  async function handleStopAll() {
    try {
      await stopAllProcesses();
    } catch (e) {
      console.error('Failed to stop all processes:', e);
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    // Find matching shortcut
    const shortcut = shortcuts.find((s) => {
      const keyMatches = s.key.toLowerCase() === event.key.toLowerCase();
      const modifiersMatch = s.modifiers.every(
        (mod) => event[mod as keyof KeyboardEvent]
      );
      const noExtraModifiers = [
        'ctrlKey',
        'shiftKey',
        'altKey',
        'metaKey'
      ].every(
        (mod) => s.modifiers.includes(mod) || !event[mod as keyof KeyboardEvent]
      );

      return keyMatches && modifiersMatch && noExtraModifiers;
    });

    if (shortcut) {
      event.preventDefault();
      shortcut.action();
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeyDown);
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleKeyDown);
  });

  function getModifierKey(): string {
    return navigator.platform.includes('Mac') ? '⌘' : 'Ctrl';
  }
</script>

<!-- Keyboard Shortcuts Help Panel -->
{#if showHelp}
  <div
    class="glinr-shortcuts-overlay"
    onclick={() => (showHelp = false)}
    role="dialog"
    aria-label="Keyboard shortcuts"
    aria-modal="true"
  >
    <div
      class="glinr-shortcuts-panel"
      onclick={(e) => e.stopPropagation()}
      role="document"
    >
      <header class="glinr-shortcuts-header">
        <h2 class="glinr-shortcuts-title">Keyboard Shortcuts</h2>
        <button
          class="glinr-shortcuts-close"
          onclick={() => (showHelp = false)}
          aria-label="Close shortcuts panel"
        >
          ✕
        </button>
      </header>

      <div class="glinr-shortcuts-content">
        {#each shortcuts.filter((s) => s.key !== 'Escape') as shortcut (shortcut.key + shortcut.modifiers.join('-'))}
          <div class="glinr-shortcut-item">
            <div class="glinr-shortcut-keys">
              {#if shortcut.modifiers.includes('ctrlKey')}
                <kbd class="glinr-kbd">{getModifierKey()}</kbd>
                <span class="glinr-shortcut-plus">+</span>
              {/if}
              {#if shortcut.modifiers.includes('shiftKey')}
                <kbd class="glinr-kbd">Shift</kbd>
                <span class="glinr-shortcut-plus">+</span>
              {/if}
              <kbd class="glinr-kbd">{shortcut.key.toUpperCase()}</kbd>
            </div>
            <span class="glinr-shortcut-description"
              >{shortcut.description}</span
            >
          </div>
        {/each}
      </div>

      <footer class="glinr-shortcuts-footer">
        <p>Built by <strong>Glincker</strong> (A GLINR Product)</p>
      </footer>
    </div>
  </div>
{/if}

<style>
  .glinr-shortcuts-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn var(--transition-fast);
  }

  .glinr-shortcuts-panel {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-lg);
    max-width: 500px;
    width: 90%;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    animation: slideInUp var(--transition-base);
  }

  @keyframes slideInUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .glinr-shortcuts-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-xl);
    border-bottom: 1px solid var(--border-light);
  }

  .glinr-shortcuts-title {
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
  }

  .glinr-shortcuts-close {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-md);
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: var(--font-size-xl);
    cursor: pointer;
    transition: all var(--transition-fast);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .glinr-shortcuts-close:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .glinr-shortcuts-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-xl);
  }

  .glinr-shortcut-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-lg);
    padding: var(--space-md) 0;
  }

  .glinr-shortcut-item:not(:last-child) {
    border-bottom: 1px solid var(--border-light);
  }

  .glinr-shortcut-keys {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .glinr-kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 32px;
    height: 32px;
    padding: 0 var(--space-sm);
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', monospace;
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  }

  .glinr-shortcut-plus {
    color: var(--text-tertiary);
    font-weight: 600;
  }

  .glinr-shortcut-description {
    flex: 1;
    color: var(--text-secondary);
    font-size: var(--font-size-base);
  }

  .glinr-shortcuts-footer {
    padding: var(--space-lg) var(--space-xl);
    border-top: 1px solid var(--border-light);
    text-align: center;
  }

  .glinr-shortcuts-footer p {
    margin: 0;
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
  }

  .glinr-shortcuts-footer strong {
    color: var(--accent-primary);
    font-weight: 700;
  }
</style>
