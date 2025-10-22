<!--
  @file BrandingSection Component
  @glinr/sentinel

  Branding section with logo, version, and links.

  Built by Glincker (A GLINR Product)
  Copyright (c) 2025 Glincker. All rights reserved.

  @see https://glincker.com/sentinel
-->

<script lang="ts">
  import { ExternalLink } from 'lucide-svelte';

  interface Props {
    version?: string;
  }

  let { version = '0.1.0-alpha' }: Props = $props();

  const links = [
    { label: 'Website', href: 'https://glincker.com/sentinel' },
    { label: 'GitHub', href: 'https://github.com/glincker/sentinel' },
    { label: 'Documentation', href: 'https://docs.glincker.com/sentinel' }
  ];
</script>

<section class="branding-section">
  <div class="branding-content">
    <div class="logo-container">
      <img src="/assets/sentinel-logo.svg" alt="Sentinel Logo" class="logo" />
    </div>

    <h3 class="app-name">Sentinel</h3>
    <p class="version">Version {version}</p>
    <p class="tagline">Your Development Guardian</p>

    <div class="brand-text">
      <p>Built by <strong>Glincker</strong></p>
      <p class="glinr-text">(A GLINR Product)</p>
    </div>

    <div class="links">
      {#each links as link, index (link.href)}
        <a
          href={link.href}
          target="_blank"
          rel="noopener noreferrer"
          class="link"
        >
          {link.label}
          <ExternalLink size={14} />
        </a>
        {#if index < links.length - 1}
          <span class="separator">•</span>
        {/if}
      {/each}
    </div>
  </div>
</section>

<style>
  .branding-section {
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.05) 0%,
      rgba(255, 255, 255, 0.02) 100%
    );
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid var(--border-light);
    border-radius: var(--radius-lg);
    padding: var(--space-2xl);
    margin-bottom: var(--space-2xl);
  }

  .branding-content {
    text-align: center;
  }

  .logo-container {
    display: flex;
    justify-content: center;
    margin-bottom: var(--space-lg);
  }

  .logo {
    width: 80px;
    height: 80px;
    filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.1));
    transition: transform var(--transition-fast);
  }

  .logo:hover {
    transform: scale(1.05);
  }

  .app-name {
    font-size: var(--font-size-2xl);
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 var(--space-xs) 0;
    letter-spacing: -0.02em;
  }

  .version {
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    margin: 0 0 var(--space-md) 0;
    padding: var(--space-xs) var(--space-sm);
    background: var(--bg-tertiary);
    border-radius: var(--radius-sm);
    display: inline-block;
  }

  .tagline {
    font-size: var(--font-size-base);
    color: var(--text-secondary);
    margin: 0 0 var(--space-lg) 0;
    font-weight: 500;
  }

  .brand-text {
    margin-bottom: var(--space-lg);
  }

  .brand-text p {
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    margin: 0;
    line-height: 1.6;
  }

  .brand-text strong {
    color: var(--accent-primary);
    font-weight: 700;
  }

  .glinr-text {
    font-size: var(--font-size-xs);
    opacity: 0.8;
  }

  .links {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-md);
    font-size: var(--font-size-sm);
    flex-wrap: wrap;
  }

  .link {
    color: var(--accent-primary);
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    gap: var(--space-xs);
    font-weight: 500;
    transition: all var(--transition-fast);
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
  }

  .link:hover {
    background: var(--bg-tertiary);
    transform: translateY(-1px);
  }

  .link:active {
    transform: translateY(0);
  }

  .separator {
    color: var(--text-tertiary);
    opacity: 0.5;
  }

  @media (max-width: 640px) {
    .links {
      flex-direction: column;
      gap: var(--space-sm);
    }

    .separator {
      display: none;
    }
  }
</style>
