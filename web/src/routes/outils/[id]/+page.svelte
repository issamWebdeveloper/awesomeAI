<script lang="ts">
  import { resolve } from '$app/paths';
  import Activity from '#lib/components/Activity.svelte';
  import CategoryBadge from '#lib/components/CategoryBadge.svelte';
  import Chip from '#lib/components/Chip.svelte';
  import CompareToggle from '#lib/components/CompareToggle.svelte';
  import ToolCard from '#lib/components/ToolCard.svelte';
  import {
    categoryBySlug, FACET_LABELS, FACETS, facetLabel, featureLabel, fmt, fmtDate, LICENSE_FAMILIES,
    relativeDays, similarTools, TAG_LABELS
  } from '#lib/data.js';

  let { data } = $props();
  const t = $derived(data.tool);
  const cat = $derived(categoryBySlug.get(t.category)!);
  const similar = $derived(similarTools(t));
  const missing = $derived(cat.features.map((f) => f.key).filter((k) => !t.features.includes(k)));
  const reviewed = $derived((field: string) => t.reviewed.includes(field));
</script>

<svelte:head>
  <title>{t.name} · {cat.label} · Awesome AI</title>
  <meta name="description" content={`${t.name} : ${t.summary}`} />
</svelte:head>

<nav class="crumbs" aria-label="Fil d'Ariane">
  <a href={resolve('/outils')}>Outils</a> › <a href={resolve('/categories/[slug]', { slug: cat.slug })}>{cat.label}</a> › <span>{t.name}</span>
</nav>

<header class="head">
  <div class="title">
    <img src={`https://github.com/${t.repo_slug.split('/')[0]}.png?size=96`} alt="" width="48" height="48" loading="lazy" />
    <div>
      <h1>{t.name}</h1>
      <div class="meta">
        <CategoryBadge slug={t.category} />
        {#each t.tags as tag (tag)}<Chip label={TAG_LABELS[tag] ?? tag} muted />{/each}
      </div>
    </div>
  </div>
  <div class="cta">
    <CompareToggle id={t.id} name={t.name} button />
    <a class="btn" href={t.repo} rel="external noopener">Dépôt GitHub ↗</a>
    {#if t.homepage}<a class="btn" href={t.homepage} rel="external noopener">Site ↗</a>{/if}
  </div>
</header>

<p class="summary">{t.summary}</p>

<dl class="metrics">
  <div><dt>Étoiles</dt><dd>★ {fmt(t.stars)}</dd></div>
  <div><dt>Activité</dt><dd><Activity status={t.status} /></dd></div>
  <div><dt>Dernier push</dt><dd>{relativeDays(t.last_push)}</dd></div>
  <div><dt>Licence</dt><dd title={t.license_note ?? ''}>{t.license ?? LICENSE_FAMILIES[t.license_family]}</dd></div>
  <div><dt>Langage</dt><dd>{t.language ?? '—'}</dd></div>
  {#if t.open_issues != null}<div><dt>Issues ouvertes</dt><dd>{fmt(t.open_issues)}</dd></div>{/if}
</dl>

<div class="cols">
  <div class="main">
    <section class="panel brief">
      <h2>En bref</h2>
      <h3>Objectif</h3>
      <p>{t.goal ?? cat.goal}</p>
      <h3>Problème résolu</h3>
      <p>{t.problem_solved ?? cat.problem}</p>
      {#if !t.goal || !t.problem_solved}
        <p class="note muted">Texte générique de la catégorie, en attendant une fiche relue pour cet outil.</p>
      {/if}
    </section>

    <section class="panel">
      <h2>Fonctionnalités</h2>
      {#if t.features.length}
        <ul class="feat">
          {#each t.features as f (f)}<li><span class="yes" aria-hidden="true">✓</span>{featureLabel(f)}</li>{/each}
        </ul>
      {:else}
        <p class="muted">Aucune fonctionnalité du vocabulaire de la catégorie n'a été détectée.</p>
      {/if}
      {#if missing.length}
        <details>
          <summary>Non détectées ({missing.length})</summary>
          <ul class="feat off">
            {#each missing as f (f)}<li><span aria-hidden="true">–</span>{featureLabel(f)}</li>{/each}
          </ul>
        </details>
      {/if}
      <p class="note muted">
        {reviewed('features') ? 'Liste relue à la main.' : 'Détection automatique à partir de la description et du README ; elle peut contenir des erreurs.'}
      </p>
    </section>

    {#if t.license_note}
      <section class="panel warn">
        <h2>Licence : à vérifier</h2>
        <p>{t.license_note}</p>
        <p class="muted">Projet « source disponible » : sa licence ajoute des conditions à une licence open source standard. Vérifiez-les avant un usage commercial.</p>
      </section>
    {/if}
  </div>

  <aside class="side">
    <section class="panel">
      <h2>Déploiement et compatibilité</h2>
      {#each FACETS as f (f)}
        <h3>{FACET_LABELS[f]}</h3>
        {#if t[f].length}
          <div class="chips">{#each t[f] as k (k)}<Chip label={facetLabel(f, k)} href={resolve(`/outils?${f === 'deployment' ? 'dep' : f === 'interfaces' ? 'iface' : 'hw'}=${k}`)} />{/each}</div>
        {:else}
          <p class="muted small">Non précisé</p>
        {/if}
      {/each}
      <h3>API compatible OpenAI</h3>
      <p class="small">{t.openai_compatible ? '✓ Oui' : 'Non détectée'}</p>
    </section>
    {#if t.topics.length}
      <section class="panel">
        <h2>Topics GitHub</h2>
        <div class="chips">{#each t.topics as k (k)}<Chip label={k} muted />{/each}</div>
      </section>
    {/if}
    <p class="muted small">Ajouté à la liste le {fmtDate(t.first_seen)} · données du {fmtDate(t.updated_at)}</p>
  </aside>
</div>

{#if similar.length}
  <section class="similar">
    <h2>Outils similaires</h2>
    <div class="grid-cards">
      {#each similar as s (s.id)}<ToolCard tool={s} showCategory={false} />{/each}
    </div>
    <p><a href={resolve(`/comparer?ids=${[t.id, ...similar.slice(0, 2).map((s) => s.id)].join(',')}`)}>Comparer {t.name} avec {similar.slice(0, 2).map((s) => s.name).join(' et ')} →</a></p>
  </section>
{/if}

<style>
  .crumbs { font-size: 0.875rem; color: var(--muted); margin-bottom: 1rem; }
  .crumbs a { color: var(--fg-2); }
  .head { display: flex; justify-content: space-between; align-items: flex-start; gap: 1rem; flex-wrap: wrap; }
  .title { display: flex; gap: 0.9rem; align-items: center; }
  .title img { border-radius: 10px; background: var(--chip); }
  .title h1 { margin: 0 0 0.2rem; }
  .meta { display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap; }
  .cta { display: flex; gap: 0.5rem; flex-wrap: wrap; }
  .summary { font-size: 1.125rem; color: var(--fg-2); max-width: 48rem; margin: 1rem 0 1.25rem; }
  .metrics { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 9rem), 1fr)); gap: 0.75rem; margin: 0 0 1.5rem; }
  .metrics div { background: var(--surface); border: 1px solid var(--border); border-radius: 10px; padding: 0.7rem 0.9rem; }
  .metrics dt { font-size: 0.8rem; color: var(--muted); }
  .metrics dd { margin: 0.15rem 0 0; font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .cols { display: grid; grid-template-columns: 1fr 20rem; gap: 1rem; align-items: start; }
  .main, .side { display: grid; gap: 1rem; min-width: 0; }
  h3 { font-size: 0.875rem; color: var(--muted); font-weight: 600; margin: 0.9rem 0 0.35rem; text-transform: uppercase; letter-spacing: 0.03em; }
  .brief h3:first-of-type, .side h2 + h3 { margin-top: 0; }
  .brief p { margin: 0; }
  .note { font-size: 0.8125rem; margin-top: 0.9rem !important; }
  .feat { list-style: none; padding: 0; margin: 0; display: grid; grid-template-columns: repeat(auto-fill, minmax(13rem, 1fr)); gap: 0.35rem 1rem; }
  .feat li { display: flex; gap: 0.5rem; }
  .yes { color: var(--good); font-weight: 700; }
  .off { color: var(--muted); margin-top: 0.5rem; }
  details summary { cursor: pointer; color: var(--fg-2); margin-top: 0.75rem; font-size: 0.875rem; }
  .chips { display: flex; flex-wrap: wrap; gap: 0.35rem; }
  .small { font-size: 0.875rem; margin: 0; }
  .warn { border-left: 4px solid var(--warn); }
  .warn p { margin: 0 0 0.5rem; }
  .similar { margin-top: 2.5rem; }
  @media (max-width: 860px) { .cols { grid-template-columns: 1fr; } }
</style>
