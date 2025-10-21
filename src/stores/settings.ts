import { writable } from 'svelte/store';

export interface Settings {
  autoStart: boolean;
  minimizeToTray: boolean;
  startMinimized: boolean;
  checkUpdates: boolean;
  notifyOnCrash: boolean;
  logRetentionDays: number;
}

export const theme = writable<'light' | 'dark'>('dark');

export const settings = writable<Settings>({
  autoStart: false,
  minimizeToTray: true,
  startMinimized: false,
  checkUpdates: true,
  notifyOnCrash: true,
  logRetentionDays: 7,
});

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
