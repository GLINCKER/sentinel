<script lang="ts">
  interface Option {
    value: any;
    label: string;
  }

  interface Props {
    options: Option[];
    selected: any;
    onSelect: (value: any) => void;
    icon?: any; // Lucide icon component
    label?: string;
  }

  let { options, selected, onSelect, icon, label }: Props = $props();

  let showDropdown = $state(false);
  let buttonRef: HTMLButtonElement | null = $state(null);
  let dropdownPosition = $state<'bottom' | 'top'>('bottom');

  function toggleDropdown() {
    if (!showDropdown && buttonRef) {
      // Calculate available space
      const rect = buttonRef.getBoundingClientRect();
      const viewportHeight = window.innerHeight;
      const spaceBelow = viewportHeight - rect.bottom;
      const spaceAbove = rect.top;

      // Estimate dropdown height (roughly 40px per item + padding)
      const estimatedHeight = Math.min(options.length * 40 + 8, 300);

      // Position dropdown where there's more space
      dropdownPosition =
        spaceBelow >= estimatedHeight || spaceBelow >= spaceAbove
          ? 'bottom'
          : 'top';
    }
    showDropdown = !showDropdown;
  }

  function selectOption(value: any) {
    onSelect(value);
    showDropdown = false;
  }

  const currentLabel = $derived.by(() => {
    const option = options.find((o) => o.value === selected);
    return option?.label || options[0]?.label || '';
  });
</script>

<div class="dropdown-wrapper">
  <button
    bind:this={buttonRef}
    class="dropdown-btn"
    onclick={toggleDropdown}
    class:active={showDropdown}
  >
    {#if icon}
      <svelte:component this={icon} size={14} />
    {/if}
    <span class="dropdown-label">{label || currentLabel}</span>
    <svg
      class="chevron"
      class:open={showDropdown}
      width="12"
      height="12"
      viewBox="0 0 12 12"
      fill="none"
    >
      <path
        d="M3 4.5L6 7.5L9 4.5"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </svg>
  </button>

  {#if showDropdown}
    <div class="dropdown-menu" class:position-top={dropdownPosition === 'top'}>
      {#each options as option (option.value)}
        <button
          class="dropdown-item"
          class:selected={selected === option.value}
          onclick={() => selectOption(option.value)}
        >
          <span>{option.label}</span>
          {#if selected === option.value}
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
              <path
                d="M11.667 3.5L5.25 9.917L2.333 7"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>

<!-- Click outside to close -->
{#if showDropdown}
  <button class="backdrop" onclick={() => (showDropdown = false)}></button>
{/if}

<style>
  .dropdown-wrapper {
    position: relative;
    z-index: 50;
  }

  .dropdown-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 12px;
    background: var(--muted);
    border: 1px solid transparent;
    border-radius: var(--radius-lg);
    color: var(--foreground);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .dropdown-btn:hover {
    border-color: var(--border);
  }

  .dropdown-btn.active {
    background: var(--background);
    border-color: var(--foreground);
    box-shadow: 0 0 0 1px var(--foreground);
  }

  .dropdown-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--foreground);
    flex: 1;
  }

  .chevron {
    color: var(--muted-foreground);
    transition: transform 0.2s;
  }

  .chevron.open {
    transform: rotate(180deg);
  }

  .dropdown-menu {
    position: absolute;
    top: calc(100% + 4px);
    right: 0;
    left: 0;
    min-width: 160px;
    max-height: 300px;
    overflow-y: auto;
    background: var(--background);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 4px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
    animation: dropdown-in 0.15s ease-out;
    z-index: 100;
  }

  .dropdown-menu.position-top {
    top: auto;
    bottom: calc(100% + 4px);
    animation: dropdown-in-up 0.15s ease-out;
  }

  @keyframes dropdown-in {
    from {
      opacity: 0;
      transform: translateY(-8px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  @keyframes dropdown-in-up {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 8px 12px;
    background: transparent;
    border: none;
    border-radius: 0.375rem;
    color: var(--muted-foreground);
    font-size: 0.875rem;
    font-weight: 500;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
  }

  .dropdown-item:hover {
    background: var(--accent);
    color: var(--foreground);
  }

  .dropdown-item.selected {
    background: var(--accent);
    color: var(--foreground);
  }

  .dropdown-item svg {
    color: var(--primary);
    flex-shrink: 0;
  }

  .backdrop {
    position: fixed;
    inset: 0;
    background: transparent;
    border: none;
    cursor: default;
    z-index: 40;
  }
</style>
