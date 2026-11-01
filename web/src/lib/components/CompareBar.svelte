<script lang="ts">
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import { compare, MAX_COMPARE } from '#lib/compare.svelte.js';
  import { toolById } from '#lib/data.js';
  const onComparePage = $derived(page.route.id === '/comparer');
</script>

{#if compare.ids.length && !onComparePage}
  <aside class="bar" aria-label="Sélection du comparateur">
    <div class="inner">
      <span class="count">{compare.ids.length}/{MAX_COMPARE}</span>
      <ul>
        {#each compare.ids as id (id)}
          <li>
            {toolById.get(id)?.name}
            <button class="x" onclick={() => compare.remove(id)} aria-label={`Retirer ${toolById.get(id)?.name}`}>×</button>
          </li>
        {/each}
      </ul>
      <button class="btn ghost" onclick={() => compare.clear()}>Vider</button>
      <a class="btn primary" class:disabled={compare.ids.length < 2}
        href={resolve(`/comparer?ids=${compare.ids.join(',')}`)}
        aria-disabled={compare.ids.length < 2}>
        {compare.ids.length < 2 ? 'Choisissez 2 outils' : 'Comparer'}
      </a>
    </div>
  </aside>
{/if}

<style>
  .bar {
    position: fixed;
    inset: auto 0 0 0;
    z-index: 20;
    background: var(--surface);
    border-top: 1px solid var(--border);
    box-shadow: 0 -4px 16px rgb(0 0 0 / 0.08);
  }
  .inner { max-width: 76rem; margin: 0 auto; padding: 0.6rem 1rem; display: flex; align-items: center; gap: 0.75rem; }
  .count { font-weight: 600; font-variant-numeric: tabular-nums; }
  ul { list-style: none; margin: 0; padding: 0; display: flex; gap: 0.4rem; flex: 1; overflow-x: auto; }
  li { display: flex; align-items: center; gap: 0.2rem; background: var(--chip); border-radius: 999px; padding: 0.15rem 0.3rem 0.15rem 0.7rem; font-size: 0.875rem; white-space: nowrap; }
  .x { border: 0; background: none; color: var(--fg-2); font-size: 1.1rem; line-height: 1; cursor: pointer; padding: 0 0.3rem; border-radius: 50%; }
  .x:hover { color: var(--fg); background: var(--border); }
  .disabled { opacity: 0.55; pointer-events: none; }
  @media (max-width: 600px) { .ghost { display: none; } }
</style>
