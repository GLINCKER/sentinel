import { writable, derived } from 'svelte/store';
import type { ShellInfo, ShellId } from '$lib/types/shell';

interface ShellState {
  shells: ShellInfo[];
  activeShellId: ShellId | null;
}

function createShellStore() {
  const { subscribe, set, update } = writable<ShellState>({
    shells: [],
    activeShellId: null,
  });

  return {
    subscribe,

    setShells(shells: ShellInfo[]) {
      update(state => ({ ...state, shells }));
    },

    addShell(shell: ShellInfo) {
      update(state => ({
        shells: [...state.shells, shell],
        activeShellId: shell.id,
      }));
    },

    removeShell(shellId: ShellId) {
      update(state => {
        const shells = state.shells.filter(s => s.id !== shellId);
        let activeShellId = state.activeShellId;

        // If we removed the active shell, select another one
        if (activeShellId === shellId) {
          activeShellId = shells.length > 0 ? shells[shells.length - 1].id : null;
        }

        return { shells, activeShellId };
      });
    },

    setActiveShell(shellId: ShellId) {
      update(state => ({ ...state, activeShellId: shellId }));
    },

    updateShell(shellId: ShellId, updates: Partial<ShellInfo>) {
      update(state => ({
        ...state,
        shells: state.shells.map(shell =>
          shell.id === shellId ? { ...shell, ...updates } : shell
        ),
      }));
    },

    clear() {
      set({ shells: [], activeShellId: null });
    },
  };
}

export const shellStore = createShellStore();

export const activeShell = derived(
  shellStore,
  $shellStore => $shellStore.shells.find(s => s.id === $shellStore.activeShellId) || null
);
