<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { spawn, type Child } from 'tauri-pty';
  import ShellTabs from './ShellTabs.svelte';
  import GlinrTerminal from './GlinrTerminal.svelte';
  import type { ShellInfo } from '$lib/types/shell';

  interface ShellSession {
    id: string;
    info: ShellInfo;
    pty: Child;
    terminal: any;
  }

  let shells: ShellSession[] = $state([]);
  let activeShellId: string | null = $state(null);

  onMount(async () => {
    // Create first shell
    await createNewShell();
  });

  async function createNewShell() {
    const shellId = crypto.randomUUID();

    // Detect shell command based on platform
    const isWindows = navigator.platform.toLowerCase().includes('win');
    const isMac = navigator.platform.toLowerCase().includes('mac');

    // Default shells by platform
    let shellCommand: string;
    let shellType: 'Bash' | 'Zsh' | 'PowerShell' | 'Cmd' | 'Fish' | 'Sh';

    if (isWindows) {
      shellCommand = 'powershell.exe';
      shellType = 'PowerShell';
    } else if (isMac) {
      // macOS defaults to zsh since Catalina
      shellCommand = 'zsh';
      shellType = 'Zsh';
    } else {
      // Linux defaults to bash
      shellCommand = 'bash';
      shellType = 'Bash';
    }

    const shell: ShellSession = {
      id: shellId,
      info: {
        id: shellId,
        pid: null,
        process_name: shellCommand,
        cwd: '',
        shell_type: shellType,
        created_at: new Date().toISOString(),
        status: 'Active'
      },
      pty: null as any,
      terminal: null
    };

    shells.push(shell);
    shells = shells;
    activeShellId = shellId;
  }

  async function initShellPty(shellId: string, terminal: any) {
    const shell = shells.find((s) => s.id === shellId);
    if (!shell) return;

    // Use the shell command from the shell info
    const shellCommand = shell.info.process_name || 'bash';

    try {
      // Get current terminal size
      const cols = terminal.cols || 80;
      const rows = terminal.rows || 24;

      // Spawn PTY with proper environment
      const pty = await spawn(shellCommand, [], {
        cols,
        rows,
        // Set TERM for proper terminal emulation
        env: {
          TERM: 'xterm-256color',
          COLORTERM: 'truecolor'
        }
      });

      shell.pty = pty;
      shell.terminal = terminal;

      // Connect PTY output to terminal (write data as-is)
      pty.onData((data) => {
        terminal.write(data);
      });

      // Connect terminal input to PTY (write data as-is)
      terminal.onData((data: string) => {
        if (shell.pty) {
          pty.write(data);
        }
      });

      // Handle terminal resize - debounced to prevent spam
      let resizeDebounce: ReturnType<typeof setTimeout>;
      terminal.onResize((size: { cols: number; rows: number }) => {
        clearTimeout(resizeDebounce);
        resizeDebounce = setTimeout(() => {
          if (shell.pty && size.cols > 0 && size.rows > 0) {
            try {
              pty.resize(size.cols, size.rows);
            } catch (e) {
              console.warn('Failed to resize PTY:', e);
            }
          }
        }, 50); // 50ms debounce
      });
    } catch (error) {
      console.error('Failed to spawn shell:', error);
      terminal.writeln('\r\n\x1b[31mError: Failed to spawn shell\x1b[0m');
      terminal.writeln(`\x1b[33m${error}\x1b[0m`);
    }
  }

  function closeShell(shellId: string) {
    const index = shells.findIndex((s) => s.id === shellId);
    if (index === -1) return;

    const shell = shells[index];

    // Close PTY
    if (shell.pty) {
      // Note: tauri-pty Child objects auto-close on drop
    }

    // Remove from list
    shells.splice(index, 1);
    shells = shells;

    // Select another shell if we closed the active one
    if (activeShellId === shellId) {
      activeShellId = shells.length > 0 ? shells[shells.length - 1].id : null;
    }
  }

  function selectShell(shellId: string) {
    activeShellId = shellId;
  }
</script>

<div class="shell-view">
  <ShellTabs
    shells={shells.map((s) => s.info)}
    {activeShellId}
    onSelectShell={selectShell}
    onCloseShell={closeShell}
    onNewShell={createNewShell}
  />

  <div class="shell-content">
    {#each shells as shell (shell.id)}
      <div class="terminal-wrapper" class:active={shell.id === activeShellId}>
        <GlinrTerminal
          shellId={shell.id}
          onInit={(terminal) => initShellPty(shell.id, terminal)}
        />
      </div>
    {/each}
  </div>
</div>

<style>
  .shell-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background: var(--bg-primary);
  }

  .shell-content {
    flex: 1;
    position: relative;
    overflow: hidden;
  }

  .terminal-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.15s ease;
  }

  .terminal-wrapper.active {
    opacity: 1;
    pointer-events: auto;
  }
</style>
