<!--
  @file AppearanceSection Component
  @glinr/sentinel

  Theme selection section for the Settings page.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { Sun, Moon, Monitor, Check } from 'lucide-svelte';
  import { theme } from '../../../stores/settings';

  const themeOptions = [
    {
      id: 'light' as const,
      title: 'Light',
      description: 'Clean and bright',
      icon: Sun,
      gradient: 'linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)'
    },
    {
      id: 'dark' as const,
      title: 'Dark',
      description: 'Easy on the eyes',
      icon: Moon,
      gradient: 'linear-gradient(135deg, #434343 0%, #000000 100%)'
    },
    {
      id: 'system' as const,
      title: 'System',
      description: 'Match your OS',
      icon: Monitor,
      gradient:
        'linear-gradient(90deg, #f5f7fa 0%, #f5f7fa 50%, #434343 50%, #000000 100%)'
    }
  ];

  function setTheme(newTheme: 'light' | 'dark' | 'system') {
    theme.set(newTheme);
  }
</script>

<section class="appearance-section">
  <h2 class="section-title">Appearance</h2>
  <p class="section-description">Choose your preferred color theme</p>

  <div class="theme-cards">
    {#each themeOptions as option (option.id)}
      {@const Icon = option.icon}
      <button
        class="theme-card"
        class:active={$theme === option.id}
        onclick={() => setTheme(option.id)}
        aria-label={`Select ${option.title} theme`}
      >
        <div class="theme-preview" style="background: {option.gradient};">
          {#if $theme === option.id}
            <div class="theme-check">
              <Check size={20} strokeWidth={3} />
            </div>
          {/if}
        </div>
        <div class="theme-info">
          <div class="theme-icon">
            <Icon size={20} />
          </div>
          <h3 class="theme-title">{option.title}</h3>
          <p class="theme-description">{option.description}</p>
        </div>
      </button>
    {/each}
  </div>
</section>

<style>
  .appearance-section {
    margin-bottom: var(--space-2xl);
    padding-bottom: var(--space-2xl);
    border-bottom: 1px solid var(--border-light);
  }

  .section-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--space-xs) 0;
  }

  .section-description {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin: 0 0 var(--space-lg) 0;
  }

  /* Theme Cards */
  .theme-cards {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-lg);
  }

  @media (max-width: 768px) {
    .theme-cards {
      grid-template-columns: 1fr;
    }
  }

  .theme-card {
    background: var(--bg-secondary);
    border: 2px solid var(--border-color);
    border-radius: var(--radius-lg);
    overflow: hidden;
    cursor: pointer;
    transition: all var(--transition-fast);
    padding: 0;
  }

  .theme-card:hover {
    border-color: var(--accent-primary);
    transform: translateY(-2px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
  }

  .theme-card.active {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 1px var(--accent-primary);
  }

  .theme-preview {
    height: 120px;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-lg) var(--radius-lg) 0 0;
  }

  .theme-check {
    background: white;
    border-radius: 50%;
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent-primary);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .theme-info {
    padding: var(--space-lg);
    text-align: center;
  }

  .theme-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    margin-bottom: var(--space-md);
  }

  .theme-title {
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 var(--space-xs) 0;
  }

  .theme-description {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin: 0;
  }
</style>
