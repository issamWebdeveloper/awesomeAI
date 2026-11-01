<script lang="ts">
  import { resolve } from '$app/paths';
  import Activity from '#lib/components/Activity.svelte';
  import BarList from '#lib/components/BarList.svelte';
  import CompareToggle from '#lib/components/CompareToggle.svelte';
  import {
    capabilityLabel, compact, featureLabel, LICENSE_FAMILIES, pct, toolById, type Tool
  } from '#lib/data.js';

  let { data } = $props();
  const c = $derived(data.category);
  const members = $derived(c.tools.map((id) => toolById.get(id)!) as Tool[]);
  const top3 = $derived(c.tools.slice(0, 3));
</script>

<svelte:head>
  <title>{c.label} · Catégories · Awesome AI</title>
  <meta name="description" content={`${c.count} outils ${c.label.toLowerCase()} : ${c.description}`} />
</svelte:head>

<nav class="crumbs" aria-label="Fil d'Ariane"><a href={resolve('/categories')}>Catégories</a> › <span>{c.label}</span></nav>

<header class="head" style:--c={c.color}>
  <h1>{c.label}</h1>
  <p class="lead">{c.description}</p>
  <div class="actions">
    <a class="btn primary" href={resolve(`/comparer?ids=${top3.join(',')}`)}>Comparer les 3 plus populaires</a>
    <a class="btn" href={resolve(`/outils?cat=${c.slug}`)}>Voir dans le catalogue</a>
  </div>
</header>

<div class="kpis">
  <div class="kpi"><strong>{c.count}</strong><span>outils</span></div>
  <div class="kpi"><strong>{compact(c.stars)}</strong><span>étoiles cumulées</span></div>
  <div class="kpi"><strong>{pct(c.active_share)}</strong><span>actifs sur 90 jours</span></div>
  <div class="kpi"><strong>{c.languages[0]?.key ?? LICENSE_FAMILIES[c.licenses[0]?.key] ?? '—'}</strong><span>{c.languages[0] ? 'langage principal' : 'licence dominante'}</span></div>
</div>

<div class="two">
  <section class="panel">
    <h2>À quoi servent ces outils</h2>
    <p><strong>Objectif :</strong> {c.goal}</p>
    <p><strong>Problème résolu :</strong> {c.problem}</p>
  </section>

  <section class="panel">
    <h2>Points communs</h2>
    {#if c.common.length}
      <p class="muted small">Fonctionnalités présentes chez au moins la moitié des outils.</p>
      <ul class="common">
        {#each c.common as s (s.key)}
          <li><a href={resolve(`/outils?cat=${c.slug}&feat=${s.key}`)}>{featureLabel(s.key)}</a> <span class="muted">{s.count}/{c.count}</span></li>
        {/each}
      </ul>
    {:else}
      <p class="muted">Aucune fonctionnalité n'est partagée par la majorité : les outils de cette catégorie sont très différents les uns des autres.</p>
    {/if}
    {#if c.facets.length}
      <h3>Déploiement et interfaces les plus fréquents</h3>
      <p class="small">{c.facets.slice(0, 5).map((f) => `${capabilityLabel(f.key)} (${f.count})`).join(' · ')}</p>
    {/if}
  </section>
</div>

<section class="panel block">
  <h2>Fréquence des fonctionnalités</h2>
  <BarList
    rows={[...c.features].sort((a, b) => b.count - a.count).map((s) => ({
      key: s.key, label: featureLabel(s.key), value: s.count, href: resolve(`/outils?cat=${c.slug}&feat=${s.key}`)
    }))}
    max={c.count}
    format={(v) => `${v}/${c.count}`}
  />
</section>

{#if c.distinctive.length}
  <section class="panel block">
    <h2>Points distinctifs</h2>
    <p class="muted small">Fonctionnalités rares dans la catégorie (25 % des outils au plus) et qui les propose.</p>
    <dl class="dist">
      {#each c.distinctive as d (d.key)}
        <div>
          <dt>{featureLabel(d.key)}</dt>
          <dd>
            {#each d.tools as id, i (id)}{#if i}, {/if}<a href={resolve('/outils/[id]', { id })}>{toolById.get(id)?.name}</a>{/each}
          </dd>
        </div>
      {/each}
    </dl>
  </section>
{/if}

<section class="block">
  <h2>Tous les outils de la catégorie</h2>
  <div class="table-wrap panel flush">
    <table class="data">
      <thead>
        <tr><th>Outil</th><th>Fonctionnalités clés</th><th class="num">Étoiles</th><th>Licence</th><th>Activité</th><th><span class="sr-only">Comparer</span></th></tr>
      </thead>
      <tbody>
        {#each members as t (t.id)}
          <tr>
            <td><a href={resolve('/outils/[id]', { id: t.id })}><strong>{t.name}</strong></a><div class="sum">{t.summary}</div></td>
            <td class="small">{t.features.slice(0, 4).map(featureLabel).join(', ') || '—'}</td>
            <td class="num">{compact(t.stars)}</td>
            <td class="small">{t.license ?? LICENSE_FAMILIES[t.license_family]}</td>
            <td><Activity status={t.status} /></td>
            <td><CompareToggle id={t.id} name={t.name} /></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>

<style>
  .crumbs { font-size: 0.875rem; color: var(--muted); margin-bottom: 1rem; }
  .crumbs a { color: var(--fg-2); }
  .head { border-left: 5px solid var(--c); padding-left: 1rem; margin-bottom: 1.5rem; }
  .head .lead { margin-bottom: 1rem; }
  .actions { display: flex; gap: 0.5rem; flex-wrap: wrap; }
  .two { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
  .two p { margin: 0 0 0.6rem; }
  .block { margin-top: 1.5rem; }
  .small { font-size: 0.875rem; }
  h3 { font-size: 0.9rem; margin: 1rem 0 0.25rem; }
  .common { margin: 0; padding-left: 1.1rem; display: grid; gap: 0.2rem; }
  .dist { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(100%, 18rem), 1fr)); gap: 0.75rem 1.5rem; margin: 0; }
  .dist dt { font-weight: 600; }
  .dist dd { margin: 0.1rem 0 0; font-size: 0.9rem; overflow-wrap: anywhere; }
  .flush { padding: 0; }
  .sum { color: var(--muted); font-size: 0.8125rem; max-width: 30rem; }
  @media (max-width: 760px) { .two { grid-template-columns: 1fr; } }
</style>
