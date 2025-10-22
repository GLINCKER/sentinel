import { writable } from 'svelte/store';

export interface Settings {
  autoStart: boolean;
  minimizeToTray: boolean;
  startMinimized: boolean;
  checkUpdates: boolean;
  notifyOnCrash: boolean;
  logRetentionDays: number;
  pollingInterval: number; // milliseconds
  enableAnimations: boolean;
}

export const theme = writable<'light' | 'dark' | 'system'>('system');

export const settings = writable<Settings>({
  autoStart: false,
  minimizeToTray: true,
  startMinimized: false,
  checkUpdates: true,
  notifyOnCrash: true,
  logRetentionDays: 7,
  pollingInterval: 2000, // 2 seconds default
  enableAnimations: true,
});

// Polling interval options
export const POLLING_INTERVALS = [
  { label: '1 second', value: 1000 },
  { label: '2 seconds', value: 2000 },
  { label: '5 seconds', value: 5000 },
  { label: '10 seconds', value: 10000 },
  { label: '30 seconds', value: 30000 },
  { label: '1 minute', value: 60000 },
] as const;

// Load settings from localStorage
export function loadSettings() {
  const saved = localStorage.getItem('sentinel-settings');
  if (saved) {
    try {
      settings.set(JSON.parse(saved));
    } catch (e) {
      console.error('Failed to load settings:', e);
    }
  }
}

// Save settings to localStorage
export function saveSettings(newSettings: Settings) {
  settings.set(newSettings);
  localStorage.setItem('sentinel-settings', JSON.stringify(newSettings));
}
