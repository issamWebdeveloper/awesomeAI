<script lang="ts">
  import { compare } from '#lib/compare.svelte.js';
  let { id, name, button = false }: { id: string; name: string; button?: boolean } = $props();
  const checked = $derived(compare.has(id));
  const disabled = $derived(!checked && compare.full);
</script>

{#if button}
  <button class="btn" class:primary={!checked} onclick={() => compare.toggle(id)} {disabled}
    title={disabled ? 'Le comparateur contient déjà 4 outils' : undefined}>
    {checked ? '✓ Dans le comparateur' : '+ Ajouter au comparateur'}
  </button>
{:else}
  <label class="cmp" title={disabled ? 'Le comparateur contient déjà 4 outils' : undefined}>
    <input type="checkbox" {checked} {disabled} onchange={() => compare.toggle(id)} aria-label={`Comparer ${name}`} />
    Comparer
  </label>
{/if}

<style>
  .cmp { display: inline-flex; align-items: center; gap: 0.35rem; font-size: 0.8125rem; color: var(--fg-2); cursor: pointer; }
  .cmp input { width: 1rem; height: 1rem; accent-color: var(--accent); }
</style>
