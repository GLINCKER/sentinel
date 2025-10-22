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
  import { settings, saveSettings, loadSettings } from '../stores/settings';
  import GlinrButton from '../components/GlinrButton.svelte';
  import AppearanceSection from '../lib/components/settings/AppearanceSection.svelte';
  import GeneralSection from '../lib/components/settings/GeneralSection.svelte';
  import BrandingSection from '../lib/components/settings/BrandingSection.svelte';

  let localSettings = $state({ ...$settings });
  let hasChanges = $derived(
    JSON.stringify(localSettings) !== JSON.stringify($settings)
  );

  onMount(() => {
    loadSettings();
  });

  function handleSave() {
    saveSettings(localSettings);
  }

  function handleReset() {
    localSettings = { ...$settings };
  }

  const appVersion = '0.1.0-alpha';
</script>

<div class="glinr-settings">
  <header class="glinr-settings-header">
    <div>
      <h1 class="glinr-settings-title">Settings</h1>
      <p class="glinr-settings-subtitle">Configure your Sentinel preferences</p>
    </div>
  </header>

  <div class="glinr-settings-content">
    <AppearanceSection />
    <GeneralSection bind:settings={localSettings} />
    <BrandingSection version={appVersion} />
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
