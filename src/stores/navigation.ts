import { writable } from 'svelte/store';

export type View =
	| 'dashboard'
	| 'process-detail'
	| 'port-map'
	| 'network'
	| 'shell'
	| 'settings';

export const currentView = writable<View>('dashboard');
export const selectedProcess = writable<string | null>(null);

export function navigateTo(view: View, processName?: string) {
  currentView.set(view);
  if (processName) {
    selectedProcess.set(processName);
  }
}

export function navigateToProcess(processName: string) {
  selectedProcess.set(processName);
  currentView.set('process-detail');
}

export function navigateBack() {
  currentView.set('dashboard');
  selectedProcess.set(null);
}
