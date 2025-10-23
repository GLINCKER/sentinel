<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Terminal } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import { SearchAddon } from '@xterm/addon-search';
  import { WebLinksAddon } from '@xterm/addon-web-links';
  import '@xterm/xterm/css/xterm.css';

  interface Props {
    shellId?: string;
    onInit?: (terminal: Terminal) => void;
    theme?: 'dark' | 'light';
  }

  let { onInit, theme = 'dark' }: Props = $props();

  let terminalRef: HTMLDivElement;
  let terminal: Terminal | null = null;
  let fitAddon: FitAddon | null = null;
  let searchAddon: SearchAddon | null = null;
  let resizeTimeout: ReturnType<typeof setTimeout>;

  // Terminal theme configurations
  const themes = {
    dark: {
      background: '#1e1e1e',
      foreground: '#d4d4d4',
      cursor: '#d4d4d4',
      cursorAccent: '#1e1e1e',
      selection: 'rgba(255, 255, 255, 0.3)',
      black: '#000000',
      red: '#cd3131',
      green: '#0dbc79',
      yellow: '#e5e510',
      blue: '#2472c8',
      magenta: '#bc3fbc',
      cyan: '#11a8cd',
      white: '#e5e5e5',
      brightBlack: '#666666',
      brightRed: '#f14c4c',
      brightGreen: '#23d18b',
      brightYellow: '#f5f543',
      brightBlue: '#3b8eea',
      brightMagenta: '#d670d6',
      brightCyan: '#29b8db',
      brightWhite: '#ffffff'
    },
    light: {
      background: '#ffffff',
      foreground: '#383a42',
      cursor: '#383a42',
      cursorAccent: '#ffffff',
      selection: 'rgba(0, 0, 0, 0.3)',
      black: '#383a42',
      red: '#e45649',
      green: '#50a14f',
      yellow: '#c18401',
      blue: '#0184bc',
      magenta: '#a626a4',
      cyan: '#0997b3',
      white: '#fafafa',
      brightBlack: '#4f525d',
      brightRed: '#e06c75',
      brightGreen: '#98c379',
      brightYellow: '#e5c07b',
      brightBlue: '#61afef',
      brightMagenta: '#c678dd',
      brightCyan: '#56b6c2',
      brightWhite: '#ffffff'
    }
  };

  onMount(() => {
    // Initialize terminal with proper PTY configuration
    terminal = new Terminal({
      cursorBlink: true,
      fontSize: 14,
      fontFamily:
        "'SF Mono', 'JetBrains Mono', 'Fira Code', 'Consolas', monospace",
      theme: themes[theme],
      allowProposedApi: true,
      scrollback: 10000,
      // PTY-specific settings
      convertEol: false, // Let PTY handle EOL conversion
      disableStdin: false,
      cursorStyle: 'block',
      // Improve rendering performance
      drawBoldTextInBrightColors: true,
      fastScrollModifier: 'shift',
      // Better scrolling behavior
      scrollOnUserInput: true,
      // Allow proper terminal resizing
      windowOptions: {
        setWinLines: true
      }
    });

    // Initialize addons
    fitAddon = new FitAddon();
    searchAddon = new SearchAddon();
    const webLinksAddon = new WebLinksAddon();

    terminal.loadAddon(fitAddon);
    terminal.loadAddon(searchAddon);
    terminal.loadAddon(webLinksAddon);

    // Open terminal in DOM
    terminal.open(terminalRef);

    // Initial fit
    fitAddon.fit();

    // Handle window resize with debouncing (best practice for PTY)
    const resizeObserver = new ResizeObserver(() => {
      clearTimeout(resizeTimeout);
      resizeTimeout = setTimeout(() => {
        if (fitAddon && terminal) {
          try {
            fitAddon.fit();
          } catch (e) {
            console.warn('Failed to fit terminal:', e);
          }
        }
      }, 100); // Debounce to prevent resize spam to PTY
    });
    resizeObserver.observe(terminalRef);

    // Focus terminal
    terminal.focus();

    // Notify parent that terminal is ready
    onInit?.(terminal);

    return () => {
      resizeObserver.disconnect();
    };
  });

  onDestroy(() => {
    terminal?.dispose();
  });

  // Public API
  export function write(data: string) {
    terminal?.write(data);
  }

  export function clear() {
    terminal?.clear();
  }

  export function focus() {
    terminal?.focus();
  }

  export function search(query: string, options?: { incremental?: boolean }) {
    searchAddon?.findNext(query, options);
  }

  export function selectAll() {
    terminal?.selectAll();
  }

  export function getSelection(): string {
    return terminal?.getSelection() || '';
  }

  export function fit() {
    fitAddon?.fit();
  }

  // Update theme reactively
  $effect(() => {
    if (terminal) {
      terminal.options.theme = themes[theme];
    }
  });
</script>

<div class="terminal-container">
  <div bind:this={terminalRef} class="terminal"></div>
</div>

<style>
  .terminal-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg-primary);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .terminal {
    flex: 1;
    padding: 8px;
  }

  /* Override xterm.js default styles */
  .terminal :global(.xterm) {
    height: 100%;
    padding: 0;
  }

  .terminal :global(.xterm-viewport) {
    overflow-y: auto;
  }

  .terminal :global(.xterm-screen) {
    height: 100%;
  }

  /* Custom scrollbar */
  .terminal :global(.xterm-viewport::-webkit-scrollbar) {
    width: 10px;
  }

  .terminal :global(.xterm-viewport::-webkit-scrollbar-track) {
    background: transparent;
  }

  .terminal :global(.xterm-viewport::-webkit-scrollbar-thumb) {
    background: var(--border-color);
    border-radius: var(--radius-full);
  }

  .terminal :global(.xterm-viewport::-webkit-scrollbar-thumb:hover) {
    background: var(--text-tertiary);
  }
</style>
