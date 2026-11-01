<script lang="ts">
  import { onMount } from 'svelte';
  import { replaceState } from '$app/navigation';
  import { resolve } from '$app/paths';
  import Activity from '#lib/components/Activity.svelte';
  import CategoryBadge from '#lib/components/CategoryBadge.svelte';
  import CompareToggle from '#lib/components/CompareToggle.svelte';
  import ToolCard from '#lib/components/ToolCard.svelte';
  import {
    categories, categoryBySlug, compact, facetLabel, featureLabel, fmtDate, LICENSE_FAMILIES,
    searchScore, STATUS_LABELS, TAG_LABELS, tools, type Tool
  } from '#lib/data.js';

  // Filtres multi-valeurs encodés dans l'URL (?cat=a,b&dep=docker…).
  const KEYS = ['cat', 'lic', 'lang', 'status', 'dep', 'iface', 'hw', 'feat', 'tag'] as const;
  type Key = (typeof KEYS)[number];

  const getters: Record<Key, (t: Tool) => string[]> = {
    cat: (t) => [t.category],
    lic: (t) => [t.license_family],
    lang: (t) => (t.language ? [t.language] : []),
    status: (t) => [t.status],
    dep: (t) => t.deployment,
    iface: (t) => t.interfaces,
    hw: (t) => t.hardware,
    feat: (t) => t.features,
    tag: (t) => t.tags
  };

  const groupTitles: Record<Key, string> = {
    cat: 'Catégorie', lic: 'Licence', lang: 'Langage', status: 'Activité', dep: 'Déploiement',
    iface: 'Interfaces', hw: 'Matériel', feat: 'Fonctionnalités', tag: 'Étiquettes'
  };

  function labelFor(k: Key, v: string): string {
    switch (k) {
      case 'cat': return categoryBySlug.get(v)?.label ?? v;
      case 'lic': return LICENSE_FAMILIES[v] ?? v;
      case 'status': return STATUS_LABELS[v] ?? v;
      case 'dep': return facetLabel('deployment', v);
      case 'iface': return facetLabel('interfaces', v);
      case 'hw': return facetLabel('hardware', v);
      case 'feat': return featureLabel(v);
      case 'tag': return TAG_LABELS[v] ?? v;
      default: return v;
    }
  }

  const SORTS = {
    stars: { label: 'Popularité', fn: (a: Tool, b: Tool) => (b.stars ?? 0) - (a.stars ?? 0) },
    recent: { label: 'Activité récente', fn: (a: Tool, b: Tool) => (b.last_push ?? '').localeCompare(a.last_push ?? '') },
    name: { label: 'Nom', fn: (a: Tool, b: Tool) => a.name.localeCompare(b.name, 'fr', { sensitivity: 'base' }) },
    added: { label: "Date d'ajout", fn: (a: Tool, b: Tool) => b.first_seen.localeCompare(a.first_seen) }
  };
  type Sort = keyof typeof SORTS;

  let q = $state('');
  let filters = $state<Record<Key, string[]>>(Object.fromEntries(KEYS.map((k) => [k, []])) as unknown as Record<Key, string[]>);
  let openai = $state(false);
  let sort = $state<Sort>('stars');
  let view = $state<'cards' | 'table'>('cards');
  let drawer = $state(false);
  let ready = $state(false);
  let showAllFeatures = $state(false);

  onMount(() => {
    const p = new URLSearchParams(location.search);
    q = p.get('q') ?? '';
    for (const k of KEYS) filters[k] = (p.get(k) ?? '').split(',').filter(Boolean);
    openai = p.get('openai') === '1';
    const s = p.get('sort');
    if (s && s in SORTS) sort = s as Sort;
    if (p.get('view') === 'table') view = 'table';
    ready = true;
  });

  // État → URL (partageable, favoris).
  $effect(() => {
    if (!ready) return;
    const p = new URLSearchParams();
    if (q.trim()) p.set('q', q.trim());
    for (const k of KEYS) if (filters[k].length) p.set(k, filters[k].join(','));
    if (openai) p.set('openai', '1');
    if (sort !== 'stars') p.set('sort', sort);
    if (view !== 'cards') p.set('view', view);
    const qs = p.toString().replaceAll('%2C', ',');
    const url = location.pathname + (qs ? `?${qs}` : '');
    if (url !== location.pathname + location.search) replaceState(url, {});
  });

  function matches(t: Tool, except?: Key): boolean {
    if (openai && !t.openai_compatible) return false;
    for (const k of KEYS) {
      if (k === except || !filters[k].length) continue;
      const vals = getters[k](t);
      // Fonctionnalités, déploiement, interfaces, matériel, étiquettes : toutes requises (ET).
      // Catégorie, licence, langage, activité : au moins une (OU).
      const all = k === 'feat' || k === 'dep' || k === 'iface' || k === 'hw' || k === 'tag';
      const ok = all ? filters[k].every((v) => vals.includes(v)) : filters[k].some((v) => vals.includes(v));
      if (!ok) return false;
    }
    return true;
  }

  const searched = $derived(
    tools.map((t) => ({ t, s: searchScore(t, q) })).filter((x) => x.s > 0)
  );

  const results = $derived.by(() => {
    const list = searched.filter((x) => matches(x.t));
    const byRelevance = q.trim() && sort === 'stars';
    return list
      .sort((a, b) => (byRelevance ? b.s - a.s : 0) || SORTS[sort].fn(a.t, b.t))
      .map((x) => x.t);
  });

  // Options de chaque groupe, avec le nombre d'outils si on cochait cette valeur.
  function options(k: Key): { v: string; n: number }[] {
    const counts = new Map<string, number>();
    for (const { t } of searched) {
      if (!matches(t, k)) continue;
      for (const v of getters[k](t)) counts.set(v, (counts.get(v) ?? 0) + 1);
    }
    for (const v of filters[k]) if (!counts.has(v)) counts.set(v, 0);
    let entries = [...counts].map(([v, n]) => ({ v, n }));
    if (k === 'cat') entries.sort((a, b) => labelFor(k, a.v).localeCompare(labelFor(k, b.v), 'fr'));
    else entries.sort((a, b) => b.n - a.n || labelFor(k, a.v).localeCompare(labelFor(k, b.v), 'fr'));
    return entries;
  }

  const activeCount = $derived(KEYS.reduce((n, k) => n + filters[k].length, 0) + (openai ? 1 : 0));

  function toggle(k: Key, v: string) {
    filters[k] = filters[k].includes(v) ? filters[k].filter((x) => x !== v) : [...filters[k], v];
  }

  function reset() {
    q = '';
    for (const k of KEYS) filters[k] = [];
    openai = false;
  }

  function download(name: string, type: string, content: string) {
    const a = document.createElement('a');
    a.href = URL.createObjectURL(new Blob([content], { type }));
    a.download = name;
    a.click();
    URL.revokeObjectURL(a.href);
  }

  function exportJson() {
    download('awesome-ai-outils.json', 'application/json', JSON.stringify(results, null, 2));
  }

  function exportCsv() {
    const cols: (keyof Tool)[] = ['name', 'category', 'summary', 'repo', 'stars', 'license', 'license_family', 'language', 'status', 'last_push', 'features', 'deployment', 'interfaces', 'hardware', 'openai_compatible'];
    const esc = (v: unknown) => {
      const s = Array.isArray(v) ? v.join('|') : v == null ? '' : String(v);
      return /[",\n;]/.test(s) ? `"${s.replaceAll('"', '""')}"` : s;
    };
    const rows = results.map((t) => cols.map((c) => esc(c === 'category' ? categoryBySlug.get(t.category)?.label : t[c])).join(','));
    download('awesome-ai-outils.csv', 'text/csv;charset=utf-8', '﻿' + [cols.join(','), ...rows].join('\n'));
  }

  const featureGroupLimit = 14;
</script>

<svelte:head>
  <title>Catalogue des outils · Awesome AI</title>
  <meta name="description" content="Recherchez et filtrez les outils d'IA auto-hébergeables par catégorie, licence, langage, déploiement, matériel et fonctionnalités." />
</svelte:head>

<h1>Catalogue des outils</h1>

<div class="searchbar">
  <label class="sr-only" for="q">Rechercher</label>
  <input id="q" type="search" bind:value={q} placeholder="Nom, description, fonctionnalité…" autocomplete="off" />
  <button class="btn filters-btn" onclick={() => (drawer = true)} aria-expanded={drawer} aria-controls="filters">
    Filtres{#if activeCount}<span class="pill">{activeCount}</span>{/if}
  </button>
</div>

<div class="layout">
  <aside id="filters" class:open={drawer} aria-label="Filtres">
    <div class="drawer-head">
      <strong>Filtres</strong>
      <button class="btn ghost" onclick={() => (drawer = false)}>Fermer</button>
    </div>
    {#if activeCount}
      <button class="reset" onclick={reset}>Réinitialiser les filtres ({activeCount})</button>
    {/if}
    <label class="check toggle">
      <input type="checkbox" bind:checked={openai} /> API compatible OpenAI
    </label>
    {#each KEYS as k (k)}
      {@const opts = options(k)}
      {#if opts.length}
        <details open={k === 'cat' || k === 'lic' || k === 'status' || filters[k].length > 0}>
          <summary>{groupTitles[k]}{#if filters[k].length}<span class="pill">{filters[k].length}</span>{/if}</summary>
          <div class="opts">
            {#each k === 'feat' && !showAllFeatures ? opts.slice(0, featureGroupLimit) : opts as o (o.v)}
              <label class="check" class:zero={o.n === 0 && !filters[k].includes(o.v)}>
                <input type="checkbox" checked={filters[k].includes(o.v)} onchange={() => toggle(k, o.v)} />
                <span class="lbl">{labelFor(k, o.v)}</span>
                <span class="n">{o.n}</span>
              </label>
            {/each}
            {#if k === 'feat' && opts.length > featureGroupLimit}
              <button class="more" onclick={() => (showAllFeatures = !showAllFeatures)}>
                {showAllFeatures ? 'Moins' : `Voir les ${opts.length}`}
              </button>
            {/if}
          </div>
        </details>
      {/if}
    {/each}
    <p class="hint muted">Dans une même rubrique, catégorie, licence, langage et activité s'additionnent (OU) ; les autres doivent toutes être présentes (ET).</p>
  </aside>
  {#if drawer}<button class="backdrop" aria-label="Fermer les filtres" onclick={() => (drawer = false)}></button>{/if}

  <section class="results" aria-live="polite">
    <div class="toolbar">
      <p class="count"><strong>{results.length}</strong> outil{results.length > 1 ? 's' : ''} sur {tools.length}</p>
      <div class="actions">
        <label>
          <span class="sr-only">Trier par</span>
          <select bind:value={sort}>
            {#each Object.entries(SORTS) as [k, s] (k)}<option value={k}>Tri : {s.label}</option>{/each}
          </select>
        </label>
        <div class="seg" role="group" aria-label="Affichage">
          <button aria-pressed={view === 'cards'} onclick={() => (view = 'cards')}>Cartes</button>
          <button aria-pressed={view === 'table'} onclick={() => (view = 'table')}>Tableau</button>
        </div>
        <button class="btn ghost small" onclick={exportCsv} title="Exporter la sélection en CSV">CSV</button>
        <button class="btn ghost small" onclick={exportJson} title="Exporter la sélection en JSON">JSON</button>
      </div>
    </div>

    {#if activeCount}
      <div class="active-filters">
        {#if openai}<button onclick={() => (openai = false)}>API compatible OpenAI ×</button>{/if}
        {#each KEYS as k (k)}
          {#each filters[k] as v (v)}
            <button onclick={() => toggle(k, v)} aria-label={`Retirer le filtre ${labelFor(k, v)}`}>{labelFor(k, v)} ×</button>
          {/each}
        {/each}
      </div>
    {/if}

    {#if !results.length}
      <div class="empty panel">
        <p>Aucun outil ne correspond à ces critères.</p>
        <button class="btn primary" onclick={reset}>Réinitialiser les filtres</button>
      </div>
    {:else if view === 'cards'}
      <div class="grid-cards">
        {#each results as t (t.id)}<ToolCard tool={t} />{/each}
      </div>
    {:else}
      <div class="table-wrap panel flush">
        <table class="data">
          <thead>
            <tr>
              <th>Outil</th><th>Catégorie</th><th class="num">Étoiles</th><th>Licence</th><th>Langage</th>
              <th>Activité</th><th>Dernier push</th><th><span class="sr-only">Comparer</span></th>
            </tr>
          </thead>
          <tbody>
            {#each results as t (t.id)}
              <tr>
                <td><a href={resolve('/outils/[id]', { id: t.id })}><strong>{t.name}</strong></a><div class="sum">{t.summary}</div></td>
                <td><CategoryBadge slug={t.category} /></td>
                <td class="num">{compact(t.stars)}</td>
                <td>{t.license ?? LICENSE_FAMILIES[t.license_family]}</td>
                <td>{t.language ?? '—'}</td>
                <td><Activity status={t.status} /></td>
                <td class="nowrap">{fmtDate(t.last_push)}</td>
                <td><CompareToggle id={t.id} name={t.name} /></td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </section>
</div>

<style>
  .searchbar { display: flex; gap: 0.5rem; margin: 0.5rem 0 1.25rem; position: sticky; top: 3.6rem; z-index: 5; background: var(--bg); padding: 0.4rem 0; }
  .searchbar input { flex: 1; font-size: 1.0625rem; padding: 0.75rem 1rem; min-width: 0; }
  .filters-btn { display: none; }
  .pill { background: var(--accent); color: var(--on-accent); border-radius: 999px; font-size: 0.75rem; padding: 0 0.45rem; margin-left: 0.35rem; }
  .layout { display: grid; grid-template-columns: 16rem 1fr; gap: 1.5rem; align-items: start; }
  aside { position: sticky; top: 8.5rem; max-height: calc(100vh - 9.5rem); overflow-y: auto; padding-right: 0.25rem; font-size: 0.9rem; }
  .drawer-head { display: none; }
  details { border-bottom: 1px solid var(--border); padding: 0.5rem 0; }
  summary { cursor: pointer; font-weight: 600; padding: 0.25rem 0; display: flex; align-items: center; }
  .opts { display: grid; gap: 0.1rem; padding: 0.35rem 0 0.25rem; }
  .check { display: flex; align-items: center; gap: 0.5rem; padding: 0.2rem 0.25rem; border-radius: 6px; cursor: pointer; }
  .check:hover { background: var(--chip); }
  .check input { accent-color: var(--accent); width: 1rem; height: 1rem; flex: none; }
  .check .n { flex: none; }
  .check .lbl { flex: 1 1 0; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .check .n { color: var(--muted); font-variant-numeric: tabular-nums; font-size: 0.8rem; }
  .check.zero { opacity: 0.45; }
  .toggle { font-weight: 600; padding: 0.5rem 0.25rem; border-bottom: 1px solid var(--border); border-radius: 0; }
  .reset, .more { background: none; border: 0; color: var(--accent); cursor: pointer; padding: 0.35rem 0.25rem; font: inherit; text-align: left; }
  .reset { font-weight: 600; }
  .hint { font-size: 0.8rem; margin-top: 0.75rem; }
  .results { min-width: 0; }
  .toolbar { display: flex; justify-content: space-between; align-items: center; gap: 0.75rem; flex-wrap: wrap; margin-bottom: 0.75rem; }
  .count { margin: 0; color: var(--fg-2); }
  .actions { display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap; }
  .actions select { padding: 0.4rem 0.6rem; font-size: 0.9rem; }
  .seg { display: inline-flex; border: 1px solid var(--border); border-radius: 8px; overflow: hidden; }
  .seg button { border: 0; background: var(--surface); color: var(--fg-2); padding: 0.4rem 0.75rem; font: inherit; font-size: 0.9rem; cursor: pointer; }
  .seg button[aria-pressed='true'] { background: var(--accent-soft); color: var(--accent); font-weight: 600; }
  .small { padding: 0.35rem 0.6rem; font-size: 0.85rem; }
  .active-filters { display: flex; flex-wrap: wrap; gap: 0.4rem; margin-bottom: 1rem; }
  .active-filters button { border: 1px solid var(--accent); background: var(--accent-soft); color: var(--fg); border-radius: 999px; padding: 0.15rem 0.65rem; font: inherit; font-size: 0.8125rem; cursor: pointer; }
  .empty { text-align: center; padding: 2.5rem 1rem; }
  .flush { padding: 0; }
  .sum { color: var(--muted); font-size: 0.8125rem; max-width: 28rem; }
  .nowrap { white-space: nowrap; }
  .backdrop { display: none; }
  @media (max-width: 860px) {
    .layout { grid-template-columns: 1fr; }
    .filters-btn { display: inline-flex; }
    aside {
      position: fixed; inset: 0 auto 0 0; width: min(22rem, 88vw); max-height: none; z-index: 40;
      background: var(--surface); padding: 1rem; transform: translateX(-105%); transition: transform 0.2s ease;
      box-shadow: 4px 0 24px rgb(0 0 0 / 0.2);
    }
    aside.open { transform: none; }
    .drawer-head { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; }
    .backdrop { display: block; position: fixed; inset: 0; background: rgb(0 0 0 / 0.4); z-index: 35; border: 0; }
  }
  @media (prefers-reduced-motion: reduce) { aside { transition: none; } }
</style>
