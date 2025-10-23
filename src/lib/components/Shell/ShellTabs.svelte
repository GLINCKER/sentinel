<script lang="ts">
  import { onMount } from 'svelte';
  import type { ShellInfo } from '$lib/types/shell';

  interface Props {
    shells: ShellInfo[];
    activeShellId: string | null;
    onSelectShell: (id: string) => void;
    onCloseShell: (id: string) => void;
    onNewShell: () => void;
    maxShells?: number;
  }

  let {
    shells,
    activeShellId,
    onSelectShell,
    onCloseShell,
    onNewShell,
    maxShells = 20
  }: Props = $props();

  function handleTabClick(shellId: string) {
    onSelectShell(shellId);
  }

  function handleCloseTab(e: MouseEvent, shellId: string) {
    e.stopPropagation();
    onCloseShell(shellId);
  }

  function handleNewTab() {
    if (shells.length < maxShells) {
      onNewShell();
    }
  }

  function getShellLabel(shell: ShellInfo, index: number): string {
    return shell.process_name || `Shell ${index + 1}`;
  }

  // Keyboard shortcuts
  onMount(() => {
    function handleKeyDown(e: KeyboardEvent) {
      const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
      const mod = isMac ? e.metaKey : e.ctrlKey;

      if (!mod) return;

      // Cmd/Ctrl + T: New shell
      if (e.key === 't' && shells.length < maxShells) {
        e.preventDefault();
        onNewShell();
      }

      // Cmd/Ctrl + W: Close active shell
      if (e.key === 'w' && activeShellId) {
        e.preventDefault();
        onCloseShell(activeShellId);
      }

      // Cmd/Ctrl + 1-9: Switch to shell by index
      if (e.key >= '1' && e.key <= '9') {
        const index = parseInt(e.key) - 1;
        if (index < shells.length) {
          e.preventDefault();
          onSelectShell(shells[index].id);
        }
      }
    }

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  });
</script>

<div class="shell-tabs">
  <div class="tabs-container">
    {#each shells as shell, index (shell.id)}
      <div
        class="tab"
        class:active={shell.id === activeShellId}
        role="tab"
        aria-selected={shell.id === activeShellId}
        tabindex="0"
        title={shell.cwd}
      >
        <button
          class="tab-main"
          onclick={() => handleTabClick(shell.id)}
          aria-label={getShellLabel(shell, index)}
        >
          <span class="tab-icon">
            {#if shell.shell_type === 'Bash'}
              $
            {:else if shell.shell_type === 'Zsh'}
              ❯
            {:else if shell.shell_type === 'Fish'}
              🐟
            {:else if shell.shell_type === 'PowerShell'}
              PS
            {:else}
              >_
            {/if}
          </span>
          <span class="tab-label">{getShellLabel(shell, index)}</span>
        </button>
        <button
          class="tab-close"
          onclick={(e) => handleCloseTab(e, shell.id)}
          aria-label="Close shell"
        >
          ×
        </button>
      </div>
    {/each}

    <button
      class="tab-new"
      onclick={handleNewTab}
      disabled={shells.length >= maxShells}
      title={shells.length >= maxShells
        ? `Maximum ${maxShells} shells reached`
        : 'New shell (⌘T)'}
    >
      +
    </button>
  </div>

  <div class="tabs-info">
    <span class="shell-count">{shells.length}/{maxShells}</span>
  </div>
</div>

<style>
  .shell-tabs {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    padding: 0 8px;
    min-height: 40px;
    gap: 8px;
  }

  .tabs-container {
    display: flex;
    align-items: center;
    gap: 4px;
    overflow-x: auto;
    flex: 1;
    scrollbar-width: thin;
  }

  .tabs-container::-webkit-scrollbar {
    height: 4px;
  }

  .tabs-container::-webkit-scrollbar-thumb {
    background: var(--border-color);
    border-radius: var(--radius-full);
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 4px;
    background: transparent;
    border-radius: var(--radius-md);
    min-width: 120px;
    max-width: 200px;
    transition: all 0.15s ease;
  }

  .tab:hover {
    background: var(--bg-hover);
  }

  .tab.active {
    background: var(--bg-primary);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .tab-main {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    padding: 6px 8px;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 13px;
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
  }

  .tab.active .tab-main {
    color: var(--text-primary);
  }

  .tab:hover .tab-main {
    color: var(--text-primary);
  }

  .tab-icon {
    font-size: 12px;
    opacity: 0.8;
    flex-shrink: 0;
  }

  .tab-label {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tab-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    padding: 0;
    margin-right: 4px;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    color: var(--text-tertiary);
    font-size: 18px;
    line-height: 1;
    cursor: pointer;
    opacity: 0;
    transition: all 0.15s ease;
    flex-shrink: 0;
  }

  .tab:hover .tab-close,
  .tab.active .tab-close {
    opacity: 1;
  }

  .tab-close:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .tab-new {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    padding: 0;
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: 18px;
    cursor: pointer;
    transition: all 0.15s ease;
    flex-shrink: 0;
  }

  .tab-new:hover:not(:disabled) {
    background: var(--bg-hover);
    color: var(--text-primary);
    border-color: var(--text-tertiary);
  }

  .tab-new:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .tabs-info {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-tertiary);
    flex-shrink: 0;
  }

  .shell-count {
    font-variant-numeric: tabular-nums;
  }
</style>
