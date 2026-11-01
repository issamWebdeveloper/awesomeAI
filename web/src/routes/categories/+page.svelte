<script lang="ts">
  import { resolve } from '$app/paths';
  import { categories, compact, featureLabel, pct, toolById } from '#lib/data.js';
  const sorted = [...categories].sort((a, b) => b.count - a.count);
</script>

<svelte:head>
  <title>Catégories · Awesome AI</title>
  <meta name="description" content="Les 16 catégories d'outils d'IA auto-hébergeables, avec leurs points communs et leurs outils phares." />
</svelte:head>

<h1>Catégories</h1>
<p class="lead">Chaque catégorie regroupe des outils qui répondent au même besoin. Ouvrez-en une pour voir ce que ses outils ont en commun et ce qui les distingue.</p>

<div class="grid">
  {#each sorted as c (c.slug)}
    <a class="cat panel" href={resolve('/categories/[slug]', { slug: c.slug })} style:--c={c.color}>
      <div class="top">
        <h2>{c.label}</h2>
        <span class="n">{c.count} outils</span>
      </div>
      <p>{c.description}</p>
      {#if c.common.length}
        <p class="common"><strong>Points communs :</strong> {c.common.slice(0, 3).map((s) => featureLabel(s.key).toLowerCase()).join(', ')}</p>
      {/if}
      <p class="foot muted">
        ★ {compact(c.stars)} · {pct(c.active_share)} actifs · phares : {c.tools.slice(0, 3).map((id) => toolById.get(id)?.name).join(', ')}
      </p>
    </a>
  {/each}
</div>

<style>
  .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(100%, 22rem), 1fr)); gap: 1rem; }
  .cat { display: flex; flex-direction: column; gap: 0.4rem; color: var(--fg); text-decoration: none; border-top: 4px solid var(--c); }
  .cat:hover { border-color: var(--c); box-shadow: 0 2px 12px rgb(0 0 0 / 0.06); }
  .top { display: flex; justify-content: space-between; align-items: baseline; gap: 0.5rem; }
  h2 { margin: 0; font-size: 1.125rem; }
  .n { color: var(--fg-2); font-size: 0.875rem; white-space: nowrap; }
  p { margin: 0; color: var(--fg-2); font-size: 0.9375rem; }
  .common { font-size: 0.875rem; }
  .foot { margin-top: auto; padding-top: 0.4rem; font-size: 0.8125rem; color: var(--muted); }
</style>
