<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import ToolCard from '#lib/components/ToolCard.svelte';
  import { categories, compact, fmtDate, pct, stats, tools } from '#lib/data.js';

  let q = $state('');
  const rising = tools.filter((t) => t.tags.includes('rising')).sort((a, b) => (b.stars ?? 0) - (a.stars ?? 0)).slice(0, 6);
  const sortedCats = [...categories].sort((a, b) => b.count - a.count);

  function search(e: SubmitEvent) {
    e.preventDefault();
    goto(resolve(`/outils?q=${encodeURIComponent(q.trim())}`));
  }
</script>

<svelte:head>
  <title>Awesome AI · outils d'IA auto-hébergeables analysés et comparés</title>
  <meta name="description" content={`${stats.tools} outils d'IA open source à héberger soi-même, classés, analysés et comparables côte à côte.`} />
</svelte:head>

<section class="hero">
  <h1>Trouvez, comprenez et comparez les outils d'IA à héberger vous-même</h1>
  <p class="lead">
    {stats.tools} projets open source de la liste awesome-selfhosted-ai, analysés par catégorie, fonctionnalité,
    licence et activité. Filtrez le catalogue, lisez l'essentiel en 30 secondes et comparez jusqu'à 4 outils.
  </p>
  <form role="search" onsubmit={search}>
    <label class="sr-only" for="home-q">Rechercher un outil</label>
    <input id="home-q" type="search" bind:value={q} placeholder="Ex. : runtime GPU AMD compatible OpenAI, RAG, transcription…" />
    <button class="btn primary" type="submit">Rechercher</button>
  </form>
</section>

<div class="kpis">
  <div class="kpi"><strong>{stats.tools}</strong><span>outils référencés</span></div>
  <div class="kpi"><strong>{stats.categories}</strong><span>catégories</span></div>
  <div class="kpi"><strong>{pct(stats.active_share)}</strong><span>actifs sur 90 jours</span></div>
  <div class="kpi"><strong>{compact(stats.stars)}</strong><span>étoiles GitHub cumulées</span></div>
</div>

<section>
  <div class="head">
    <h2>Catégories</h2>
    <a href={resolve('/categories')}>Toutes les synthèses →</a>
  </div>
  <ul class="cats">
    {#each sortedCats as c (c.slug)}
      <li>
        <a href={resolve('/categories/[slug]', { slug: c.slug })} style:--c={c.color}>
          <span class="name">{c.label}</span>
          <span class="n">{c.count}</span>
        </a>
      </li>
    {/each}
  </ul>
</section>

<section>
  <div class="head">
    <h2>Rising : les nouveaux à suivre</h2>
    <a href={resolve('/outils?tag=rising')}>Voir les {stats.rising} →</a>
  </div>
  <p class="muted small">Projets récents de moins de 500 étoiles repérés par la liste source.</p>
  <div class="grid-cards">
    {#each rising as t (t.id)}<ToolCard tool={t} />{/each}
  </div>
</section>

<p class="muted small updated">Dernière collecte : {fmtDate(stats.updated_at)}.</p>

<style>
  .hero { padding: 1.5rem 0 1rem; max-width: 52rem; }
  .hero h1 { font-size: clamp(1.8rem, 5vw, 2.75rem); letter-spacing: -0.02em; }
  form { display: flex; gap: 0.5rem; }
  form input { flex: 1; font-size: 1.0625rem; padding: 0.8rem 1rem; min-width: 0; }
  section { margin: 0 0 2.5rem; }
  .head { display: flex; justify-content: space-between; align-items: baseline; gap: 1rem; flex-wrap: wrap; }
  .small { font-size: 0.875rem; margin-top: -0.25rem; }
  .cats { list-style: none; padding: 0; margin: 0; display: grid; grid-template-columns: repeat(auto-fill, minmax(min(100%, 15rem), 1fr)); gap: 0.6rem; }
  .cats a {
    display: flex; justify-content: space-between; align-items: center; gap: 0.5rem;
    padding: 0.75rem 1rem; background: var(--surface); border: 1px solid var(--border);
    border-left: 4px solid var(--c); border-radius: 10px; color: var(--fg); text-decoration: none;
  }
  .cats a:hover { border-color: var(--c); }
  .n { color: var(--fg-2); font-variant-numeric: tabular-nums; font-size: 0.9rem; }
  .updated { margin-top: 2rem; }
  @media (max-width: 480px) { form { flex-direction: column; } }
</style>
