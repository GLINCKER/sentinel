<!--
  @file Settings View
  @glinr/sentinel

  Application settings including theme toggle and preferences.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { onMount } from 'svelte';
  import {
    theme,
    settings,
    saveSettings,
    loadSettings
  } from '../stores/settings';
  import GlinrButton from '../components/GlinrButton.svelte';

  let localSettings = $state({ ...$settings });
  let hasChanges = $derived(
    JSON.stringify(localSettings) !== JSON.stringify($settings)
  );

  onMount(() => {
    loadSettings();
  });

  function handleThemeToggle() {
    theme.update((t) => (t === 'dark' ? 'light' : 'dark'));
  }

  function handleSave() {
    saveSettings(localSettings);
  }

  function handleReset() {
    localSettings = { ...$settings };
  }
</script>

<div class="glinr-settings">
  <header class="glinr-settings-header">
    <div>
      <h1 class="glinr-settings-title">Settings</h1>
      <p class="glinr-settings-subtitle">Configure your Sentinel preferences</p>
    </div>
  </header>

  <div class="glinr-settings-content">
    <!-- Appearance Section -->
    <section class="glinr-settings-section">
      <h2 class="glinr-section-title">Appearance</h2>

      <div class="glinr-setting-item">
        <div class="glinr-setting-info">
          <label for="theme-toggle" class="glinr-setting-label">Theme</label>
          <p class="glinr-setting-description">
            Choose between light and dark mode
          </p>
        </div>
        <button
          id="theme-toggle"
          class="glinr-theme-toggle"
          onclick={handleThemeToggle}
          aria-label="Toggle theme"
        >
          <span class="glinr-theme-option" class:active={$theme === 'light'}
            >☀️ Light</span
          >
          <span class="glinr-theme-option" class:active={$theme === 'dark'}
            >🌙 Dark</span
          >
        </button>
      </div>
    </section>

    <!-- General Settings -->
    <section class="glinr-settings-section">
      <h2 class="glinr-section-title">General</h2>

      <div class="glinr-setting-item">
        <div class="glinr-setting-info">
          <label for="auto-start" class="glinr-setting-label"
            >Auto-start with system</label
          >
          <p class="glinr-setting-description">
            Launch Sentinel automatically when your computer starts
          </p>
        </div>
        <input
          id="auto-start"
          type="checkbox"
          class="glinr-toggle"
          bind:checked={localSettings.autoStart}
        />
      </div>

      <div class="glinr-setting-item">
        <div class="glinr-setting-info">
          <label for="minimize-tray" class="glinr-setting-label"
            >Minimize to tray</label
          >
          <p class="glinr-setting-description">
            Keep Sentinel running in the system tray when minimized
          </p>
        </div>
        <input
          id="minimize-tray"
          type="checkbox"
          class="glinr-toggle"
          bind:checked={localSettings.minimizeToTray}
        />
      </div>

      <div class="glinr-setting-item">
        <div class="glinr-setting-info">
          <label for="start-minimized" class="glinr-setting-label"
            >Start minimized</label
          >
          <p class="glinr-setting-description">
            Launch in the background without opening the window
          </p>
        </div>
        <input
          id="start-minimized"
          type="checkbox"
          class="glinr-toggle"
          bind:checked={localSettings.startMinimized}
        />
      </div>
    </section>

    <!-- Notifications -->
    <section class="glinr-settings-section">
      <h2 class="glinr-section-title">Notifications</h2>

      <div class="glinr-setting-item">
        <div class="glinr-setting-info">
          <label for="notify-crash" class="glinr-setting-label"
            >Notify on process crash</label
          >
          <p class="glinr-setting-description">
            Show desktop notification when a process crashes
          </p>
        </div>
        <input
          id="notify-crash"
          type="checkbox"
          class="glinr-toggle"
          bind:checked={localSettings.notifyOnCrash}
        />
      </div>

      <div class="glinr-setting-item">
        <div class="glinr-setting-info">
          <label for="check-updates" class="glinr-setting-label"
            >Check for updates</label
          >
          <p class="glinr-setting-description">
            Automatically check for Sentinel updates
          </p>
        </div>
        <input
          id="check-updates"
          type="checkbox"
          class="glinr-toggle"
          bind:checked={localSettings.checkUpdates}
        />
      </div>
    </section>

    <!-- Advanced -->
    <section class="glinr-settings-section">
      <h2 class="glinr-section-title">Advanced</h2>

      <div class="glinr-setting-item">
        <div class="glinr-setting-info">
          <label for="log-retention" class="glinr-setting-label"
            >Log retention</label
          >
          <p class="glinr-setting-description">
            Number of days to keep process logs
          </p>
        </div>
        <input
          id="log-retention"
          type="number"
          class="glinr-input"
          min="1"
          max="90"
          bind:value={localSettings.logRetentionDays}
        />
      </div>
    </section>

    <!-- About -->
    <section class="glinr-settings-section glinr-about-section">
      <h2 class="glinr-section-title">About</h2>
      <div class="glinr-about-content">
        <h3 class="glinr-about-name">Sentinel</h3>
        <p class="glinr-about-version">Version 0.1.0-alpha</p>
        <p class="glinr-about-description">Your Development Guardian</p>
        <p class="glinr-about-brand">
          Built by <strong>Glincker</strong> (A GLINR Product)
        </p>
        <div class="glinr-about-links">
          <a
            href="https://glincker.com/sentinel"
            target="_blank"
            rel="noopener noreferrer">Website</a
          >
          <span>•</span>
          <a
            href="https://github.com/glincker/sentinel"
            target="_blank"
            rel="noopener noreferrer">GitHub</a
          >
          <span>•</span>
          <a
            href="https://docs.glincker.com/sentinel"
            target="_blank"
            rel="noopener noreferrer">Documentation</a
          >
        </div>
      </div>
    </section>
  </div>

  <!-- Action Buttons -->
  {#if hasChanges}
    <footer class="glinr-settings-footer">
      <GlinrButton variant="secondary" onclick={handleReset}>
        Reset Changes
      </GlinrButton>
      <GlinrButton variant="primary" onclick={handleSave}>
        Save Settings
      </GlinrButton>
    </footer>
  {/if}
</div>

<style>
  .glinr-settings {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .glinr-settings-header {
    padding: var(--space-xl) var(--space-2xl);
    border-bottom: 1px solid var(--border-light);
    flex-shrink: 0;
  }

  .glinr-settings-title {
    font-size: var(--font-size-3xl);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 var(--space-xs) 0;
  }

  .glinr-settings-subtitle {
    font-size: var(--font-size-base);
    color: var(--text-secondary);
    margin: 0;
  }

  .glinr-settings-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-2xl);
  }

  .glinr-settings-section {
    margin-bottom: var(--space-2xl);
    padding-bottom: var(--space-2xl);
    border-bottom: 1px solid var(--border-light);
  }

  .glinr-settings-section:last-child {
    border-bottom: none;
  }

  .glinr-section-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--space-lg) 0;
  }

  .glinr-setting-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-lg);
    padding: var(--space-lg);
    background: var(--bg-secondary);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-md);
  }

  .glinr-setting-info {
    flex: 1;
  }

  .glinr-setting-label {
    display: block;
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: var(--space-xs);
  }

  .glinr-setting-description {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin: 0;
  }

  /* Theme Toggle */
  .glinr-theme-toggle {
    display: flex;
    gap: var(--space-xs);
    background: var(--bg-tertiary);
    padding: var(--space-xs);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
  }

  .glinr-theme-option {
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--text-secondary);
    transition: all var(--transition-fast);
    cursor: pointer;
  }

  .glinr-theme-option.active {
    background: var(--accent-primary);
    color: white;
  }

  /* Toggle Switch */
  .glinr-toggle {
    width: 48px;
    height: 28px;
    appearance: none;
    background: var(--bg-tertiary);
    border-radius: 14px;
    position: relative;
    cursor: pointer;
    transition: background var(--transition-fast);
    border: 1px solid var(--border-color);
  }

  .glinr-toggle::before {
    content: '';
    position: absolute;
    width: 22px;
    height: 22px;
    background: white;
    border-radius: 50%;
    top: 2px;
    left: 2px;
    transition: transform var(--transition-fast);
  }

  .glinr-toggle:checked {
    background: var(--accent-primary);
    border-color: var(--accent-primary);
  }

  .glinr-toggle:checked::before {
    transform: translateX(20px);
  }

  /* Input */
  .glinr-input {
    width: 100px;
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    background: var(--bg-tertiary);
    color: var(--text-primary);
    font-size: var(--font-size-base);
  }

  /* About Section */
  .glinr-about-section {
    background: var(--bg-secondary);
    border-radius: var(--radius-lg);
    padding: var(--space-xl);
  }

  .glinr-about-content {
    text-align: center;
  }

  .glinr-about-name {
    font-size: var(--font-size-2xl);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 var(--space-xs) 0;
  }

  .glinr-about-version {
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    margin: 0 0 var(--space-md) 0;
  }

  .glinr-about-description {
    font-size: var(--font-size-base);
    color: var(--text-secondary);
    margin: 0 0 var(--space-sm) 0;
  }

  .glinr-about-brand {
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    margin: 0 0 var(--space-lg) 0;
  }

  .glinr-about-brand strong {
    color: var(--accent-primary);
    font-weight: 700;
  }

  .glinr-about-links {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-md);
    font-size: var(--font-size-sm);
  }

  .glinr-about-links a {
    color: var(--accent-primary);
    text-decoration: none;
  }

  .glinr-about-links a:hover {
    text-decoration: underline;
  }

  .glinr-about-links span {
    color: var(--text-tertiary);
  }

  /* Footer */
  .glinr-settings-footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-md);
    padding: var(--space-lg) var(--space-2xl);
    border-top: 1px solid var(--border-light);
    background: var(--bg-secondary);
    flex-shrink: 0;
  }
</style>
