<script lang="ts">
  import { resolve } from '$app/paths';
  import BarList from '#lib/components/BarList.svelte';
  import CategoryBadge from '#lib/components/CategoryBadge.svelte';
  import {
    capabilityLabel, categories, categoryBySlug, compact, fmt, fmtDate, FRESHNESS_LABELS, LICENSE_FAMILIES,
    pct, stats, tools
  } from '#lib/data.js';

  const byCount = [...categories].sort((a, b) => b.count - a.count);
  let topCat = $state('');
  const top10 = $derived(
    tools
      .filter((t) => !topCat || t.category === topCat)
      .sort((a, b) => (b.stars ?? 0) - (a.stars ?? 0))
      .slice(0, 10)
  );

  const heat = new Map(stats.heatmap.map((h) => [`${h.category}|${h.key}`, h]));
  // Lien vers le catalogue filtré pour une capacité transverse.
  function capHref(cat: string, key: string) {
    if (key === 'openai_compatible') return resolve(`/outils?cat=${cat}&openai=1`);
    const [facet, k] = key.split(':');
    const param = facet === 'deployment' ? 'dep' : facet === 'interfaces' ? 'iface' : 'hw';
    return resolve(`/outils?cat=${cat}&${param}=${k}`);
  }
  let showHeatTable = $state(false);
</script>

<svelte:head>
  <title>Tableau de bord · Awesome AI</title>
  <meta name="description" content="Indicateurs clés et graphiques sur les outils d'IA auto-hébergeables : catégories, licences, langages, activité, popularité." />
</svelte:head>

<h1>Tableau de bord</h1>
<p class="lead">Le paysage de l'IA auto-hébergée en un coup d'œil. Chaque barre et chaque case ouvre le catalogue filtré correspondant.</p>

<div class="kpis">
  <div class="kpi"><strong>{fmt(stats.tools)}</strong><span>outils</span></div>
  <div class="kpi"><strong>{stats.categories}</strong><span>catégories</span></div>
  <div class="kpi"><strong>{compact(stats.stars)}</strong><span>étoiles cumulées</span></div>
  <div class="kpi"><strong>{pct(stats.active_share)}</strong><span>actifs sur 90 jours ({stats.active})</span></div>
  <div class="kpi"><strong>{stats.rising}</strong><span>projets « Rising »</span></div>
</div>

<div class="grid">
  <section class="panel wide">
    <h2>Outils par catégorie</h2>
    <BarList rows={byCount.map((c) => ({ key: c.slug, label: c.label, value: c.count, color: c.color, href: resolve(`/outils?cat=${c.slug}`) }))} />
  </section>

  <section class="panel">
    <h2>Familles de licences</h2>
    <BarList
      rows={stats.license_families.map((s) => ({ key: s.key, label: LICENSE_FAMILIES[s.key] ?? s.key, value: s.count, href: resolve(`/outils?lic=${s.key}`), note: pct(s.share) }))}
      format={(v) => `${v}`}
    />
    <h3>Licences les plus courantes</h3>
    <p class="small">{stats.licenses.slice(0, 6).map((l) => `${l.key} (${l.count})`).join(' · ')}</p>
  </section>

  <section class="panel">
    <h2>Fraîcheur du code</h2>
    <p class="muted small">Ancienneté du dernier push sur le dépôt.</p>
    <BarList rows={stats.freshness.map((s) => ({ key: s.key, label: FRESHNESS_LABELS[s.key] ?? s.key, value: s.count, note: pct(s.share) }))} />
  </section>

  <section class="panel">
    <h2>Langages principaux</h2>
    {#if stats.languages.length}
      <BarList rows={stats.languages.map((s) => ({ key: s.key, label: s.key, value: s.count, href: resolve(`/outils?lang=${encodeURIComponent(s.key)}`) }))} />
    {:else}
      <p class="muted">Le langage de chaque dépôt vient de l'API GitHub ; il apparaît après une collecte effectuée avec un jeton (déploiement automatique).</p>
    {/if}
  </section>

  <section class="panel">
    <div class="head">
      <h2>Top 10 par étoiles</h2>
      <label>
        <span class="sr-only">Catégorie</span>
        <select bind:value={topCat}>
          <option value="">Toutes catégories</option>
          {#each byCount as c (c.slug)}<option value={c.slug}>{c.label}</option>{/each}
        </select>
      </label>
    </div>
    <ol class="top">
      {#each top10 as t (t.id)}
        <li>
          <a href={resolve('/outils/[id]', { id: t.id })}>{t.name}</a>
          {#if !topCat}<CategoryBadge slug={t.category} link={false} />{/if}
          <span class="stars">★ {compact(t.stars)}</span>
        </li>
      {/each}
    </ol>
  </section>

  <section class="panel wide">
    <div class="head">
      <h2>Capacités par catégorie</h2>
      <button class="btn ghost small" onclick={() => (showHeatTable = !showHeatTable)} aria-pressed={showHeatTable}>
        {showHeatTable ? 'Vue matrice' : 'Vue nombres'}
      </button>
    </div>
    <p class="muted small">Part des outils de chaque catégorie qui proposent la capacité (détection automatique). Plus la case est foncée, plus c'est un point commun.</p>
    <div class="table-wrap">
      <table class="heat">
        <thead>
          <tr>
            <th scope="col">Catégorie</th>
            {#each stats.heatmap_keys as k (k)}<th scope="col"><span>{capabilityLabel(k)}</span></th>{/each}
          </tr>
        </thead>
        <tbody>
          {#each byCount as c (c.slug)}
            <tr>
              <th scope="row"><a href={resolve('/categories/[slug]', { slug: c.slug })}>{c.label}</a></th>
              {#each stats.heatmap_keys as k (k)}
                {@const h = heat.get(`${c.slug}|${k}`)}
                <td style:--a={h?.share ?? 0} class:dark={(h?.share ?? 0) > 0.55}>
                  <a href={capHref(c.slug, k)} title={`${categoryBySlug.get(c.slug)?.label} · ${capabilityLabel(k)} : ${h?.count ?? 0}/${c.count} (${pct(h?.share ?? 0)})`}>
                    {showHeatTable ? `${h?.count ?? 0}/${c.count}` : pct(h?.share ?? 0).replace(' ', '')}
                  </a>
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>
</div>

<p class="muted small">Données collectées le {fmtDate(stats.updated_at)}{stats.source_refreshed ? `, liste source rafraîchie le ${fmtDate(stats.source_refreshed)}` : ''}.</p>

<style>
  .grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 1rem; margin-bottom: 1.5rem; }
  .wide { grid-column: 1 / -1; }
  .head { display: flex; justify-content: space-between; align-items: center; gap: 0.75rem; flex-wrap: wrap; margin-bottom: 0.5rem; }
  .head h2 { margin: 0; }
  .small { font-size: 0.875rem; }
  h3 { font-size: 0.9rem; margin: 1rem 0 0.25rem; }
  .top { margin: 0; padding-left: 1.5rem; display: grid; gap: 0.35rem; }
  .top li { padding-left: 0.25rem; }
  .top li > * + * { margin-left: 0.5rem; }
  .stars { color: var(--fg-2); font-size: 0.875rem; font-variant-numeric: tabular-nums; float: right; }
  .heat { border-collapse: separate; border-spacing: 2px; font-size: 0.8125rem; width: 100%; }
  .heat thead th { height: 8.5rem; vertical-align: bottom; padding: 0 0 0.4rem; font-weight: 600; color: var(--fg-2); }
  .heat thead th span { display: inline-block; writing-mode: vertical-rl; transform: rotate(180deg); white-space: nowrap; }
  .heat thead th:first-child { text-align: left; }
  .heat tbody th { text-align: left; font-weight: 500; padding-right: 0.75rem; white-space: nowrap; }
  .heat tbody th a { color: var(--fg); text-decoration: none; }
  .heat tbody th a:hover { text-decoration: underline; }
  .heat td { padding: 0; min-width: 2.75rem; background: rgb(var(--heat) / calc(0.06 + var(--a) * 0.9)); border-radius: 4px; }
  .heat td a { display: block; padding: 0.45rem 0.2rem; text-align: center; color: var(--fg); text-decoration: none; font-variant-numeric: tabular-nums; }
  .heat td.dark a { color: #fff; }
  :global([data-theme='dark']) .heat td.dark a { color: #0f0e0d; }
  @media (prefers-color-scheme: dark) { :global(:root:not([data-theme='light'])) .heat td.dark a { color: #0f0e0d; } }
  .heat td a:hover { outline: 2px solid var(--fg); border-radius: 4px; }
  @media (max-width: 860px) { .grid { grid-template-columns: 1fr; } }
</style>
