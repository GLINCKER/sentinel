import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ProcessInfo, SystemStats } from '../types';

export const processes = writable<ProcessInfo[]>([]);
export const systemStats = writable<SystemStats | null>(null);
export const isLoading = writable(false);
export const error = writable<string | null>(null);

// Derived store for running processes count
export const runningCount = derived(processes, ($processes) =>
  $processes.filter((p) => p.state === 'running').length
);

// Derived store for crashed processes
export const crashedProcesses = derived(processes, ($processes) =>
  $processes.filter((p) => p.state && typeof p.state === 'object' && 'crashed' in p.state)
);

/**
 * Fetch all processes from the backend
 */
export async function fetchProcesses() {
  try {
    isLoading.set(true);
    error.set(null);

    const result = await invoke<ProcessInfo[]>('list_processes');
    processes.set(result);
  } catch (e) {
    error.set(e instanceof Error ? e.message : 'Failed to fetch processes');
    console.error('Failed to fetch processes:', e);
  } finally {
    isLoading.set(false);
  }
}

/**
 * Fetch system statistics from the backend
 */
export async function fetchSystemStats() {
  try {
    const stats = await invoke<SystemStats>('get_system_stats');
    systemStats.set(stats);
  } catch (e) {
    console.error('Failed to fetch system stats:', e);
  }
}

/**
 * Start a process
 */
export async function startProcess(name: string): Promise<void> {
  try {
    await invoke('start_process_by_name', { name });
    await fetchProcesses();
  } catch (e) {
    throw new Error(e instanceof Error ? e.message : 'Failed to start process');
  }
}

/**
 * Stop a process
 */
export async function stopProcess(name: string): Promise<void> {
  try {
    await invoke('stop_process', { name });
    await fetchProcesses();
  } catch (e) {
    throw new Error(e instanceof Error ? e.message : 'Failed to stop process');
  }
}

/**
 * Restart a process
 */
export async function restartProcess(name: string): Promise<void> {
  try {
    await invoke('restart_process', { name });
    await fetchProcesses();
  } catch (e) {
    throw new Error(e instanceof Error ? e.message : 'Failed to restart process');
  }
}

/**
 * Get logs for a process
 */
export async function getProcessLogs(name: string, lines: number = 1000): Promise<string[]> {
  try {
    return await invoke<string[]>('get_logs', { name, lines });
  } catch (e) {
    throw new Error(e instanceof Error ? e.message : 'Failed to get logs');
  }
}

/**
 * Stop all processes
 */
export async function stopAllProcesses(): Promise<void> {
  try {
    await invoke('stop_all_processes');
    await fetchProcesses();
  } catch (e) {
    throw new Error(e instanceof Error ? e.message : 'Failed to stop all processes');
  }
}

/**
 * Start polling for updates
 */
export function startPolling(intervalMs: number = 2000) {
  // Initial fetch
  fetchProcesses();
  fetchSystemStats();

  // Poll for updates
  const interval = setInterval(() => {
    fetchProcesses();
    fetchSystemStats();
  }, intervalMs);

  // Return cleanup function
  return () => clearInterval(interval);
}
