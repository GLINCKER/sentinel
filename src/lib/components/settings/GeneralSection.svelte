<!--
  @file GeneralSection Component
  @glinr/sentinel

  General settings section including auto-start, minimize, and notifications.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import Toggle from '../Toggle.svelte';
  import type { Settings } from '../../../stores/settings';

  interface Props {
    settings: Settings;
  }

  let { settings = $bindable() }: Props = $props();
</script>

<section class="general-section">
  <h2 class="section-title">General</h2>

  <div class="setting-item">
    <div class="setting-info">
      <label for="auto-start" class="setting-label"
        >Auto-start with system</label
      >
      <p class="setting-description">
        Launch Sentinel automatically when your computer starts
      </p>
    </div>
    <Toggle
      bind:checked={settings.autoStart}
      ariaLabel="Auto-start with system"
    />
  </div>

  <div class="setting-item">
    <div class="setting-info">
      <label for="minimize-tray" class="setting-label">Minimize to tray</label>
      <p class="setting-description">
        Keep Sentinel running in the system tray when minimized
      </p>
    </div>
    <Toggle
      bind:checked={settings.minimizeToTray}
      ariaLabel="Minimize to tray"
    />
  </div>

  <div class="setting-item">
    <div class="setting-info">
      <label for="start-minimized" class="setting-label">Start minimized</label>
      <p class="setting-description">
        Launch in the background without opening the window
      </p>
    </div>
    <Toggle
      bind:checked={settings.startMinimized}
      ariaLabel="Start minimized"
    />
  </div>
</section>

<section class="notifications-section">
  <h2 class="section-title">Notifications</h2>

  <div class="setting-item">
    <div class="setting-info">
      <label for="notify-crash" class="setting-label"
        >Notify on process crash</label
      >
      <p class="setting-description">
        Show desktop notification when a process crashes
      </p>
    </div>
    <Toggle
      bind:checked={settings.notifyOnCrash}
      ariaLabel="Notify on process crash"
    />
  </div>

  <div class="setting-item">
    <div class="setting-info">
      <label for="check-updates" class="setting-label">Check for updates</label>
      <p class="setting-description">
        Automatically check for Sentinel updates
      </p>
    </div>
    <Toggle
      bind:checked={settings.checkUpdates}
      ariaLabel="Check for updates"
    />
  </div>
</section>

<section class="advanced-section">
  <h2 class="section-title">Advanced</h2>

  <div class="setting-item">
    <div class="setting-info">
      <label for="log-retention" class="setting-label">Log retention</label>
      <p class="setting-description">Number of days to keep process logs</p>
    </div>
    <input
      id="log-retention"
      type="number"
      class="input"
      min="1"
      max="90"
      bind:value={settings.logRetentionDays}
    />
  </div>
</section>

<style>
  .general-section,
  .notifications-section,
  .advanced-section {
    margin-bottom: var(--space-2xl);
    padding-bottom: var(--space-2xl);
    border-bottom: 1px solid var(--border-light);
  }

  .section-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--space-lg) 0;
  }

  .setting-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-lg);
    padding: var(--space-lg);
    background: var(--bg-secondary);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-md);
  }

  .setting-item:last-child {
    margin-bottom: 0;
  }

  .setting-info {
    flex: 1;
  }

  .setting-label {
    display: block;
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: var(--space-xs);
  }

  .setting-description {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin: 0;
  }

  .input {
    width: 100px;
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    background: var(--bg-tertiary);
    color: var(--text-primary);
    font-size: var(--font-size-base);
  }

  .input:focus {
    outline: 2px solid var(--accent-primary);
    outline-offset: 2px;
    border-color: var(--accent-primary);
  }
</style>
