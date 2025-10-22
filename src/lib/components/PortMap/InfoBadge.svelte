<script lang="ts">
  import { Info } from 'lucide-svelte';

  interface Props {
    onClick: () => void;
  }

  let { onClick }: Props = $props();
</script>

<button class="info-badge-btn" onclick={onClick} aria-label="Learn more">
  <Info size={12} />
</button>

<style>
  .info-badge-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    padding: 0;
    background: transparent;
    border: none;
    color: inherit;
    opacity: 0.5;
    cursor: pointer;
    transition: all 0.2s;
    margin-left: 0.25rem;
  }

  .info-badge-btn:hover {
    opacity: 1;
    transform: scale(1.15);
  }
</style>
