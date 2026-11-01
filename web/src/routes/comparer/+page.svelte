<script lang="ts">
  import { onMount } from 'svelte';
  import { replaceState } from '$app/navigation';
  import { resolve } from '$app/paths';
  import CategoryBadge from '#lib/components/CategoryBadge.svelte';
  import { compare, MAX_COMPARE } from '#lib/compare.svelte.js';
  import { categories, compact, searchScore, toolById, tools, type Tool } from '#lib/data.js';
  import { buildCriteria, cellKey, GROUPS, PRESETS, type Criterion, type GroupId, type PresetId } from './criteria';

  let preset = $state<PresetId>('essentiel');
  // Écarts au préréglage : id -> visible ?
  let custom = $state<Record<string, boolean>>({});
  let onlyDiff = $state(false);
  let panelOpen = $state(false);
  let ready = $state(false);
  let addQuery = $state('');
  let copied = $state(false);

  const selected = $derived(compare.ids.map((id) => toolById.get(id)).filter(Boolean) as Tool[]);
  const criteria = $derived(buildCriteria(selected));

  onMount(() => {
    const p = new URLSearchParams(location.search);
    const ids = (p.get('ids') ?? '').split(',').filter((id) => toolById.has(id));
    if (ids.length) compare.set(ids);
    const pr = p.get('preset');
    if (pr && pr in PRESETS) preset = pr as PresetId;
    const c: Record<string, boolean> = {};
    for (const id of (p.get('on') ?? '').split(',').filter(Boolean)) c[id] = true;
    for (const id of (p.get('off') ?? '').split(',').filter(Boolean)) c[id] = false;
    custom = c;
    onlyDiff = p.get('diff') === '1';
    ready = true;
  });

  $effect(() => {
    if (!ready) return;
    const p = new URLSearchParams();
    if (compare.ids.length) p.set('ids', compare.ids.join(','));
    if (preset !== 'essentiel') p.set('preset', preset);
    const on = Object.entries(custom).filter(([, v]) => v).map(([k]) => k);
    const off = Object.entries(custom).filter(([, v]) => !v).map(([k]) => k);
    if (on.length) p.set('on', on.join(','));
    if (off.length) p.set('off', off.join(','));
    if (onlyDiff) p.set('diff', '1');
    const qs = p.toString().replaceAll('%2C', ',').replaceAll('%3A', ':');
    const url = location.pathname + (qs ? `?${qs}` : '');
    if (url !== location.pathname + location.search) replaceState(url, {});
  });

  const visible = (c: Criterion) => custom[c.id] ?? PRESETS[preset].includes(c);

  function setVisible(c: Criterion, v: boolean) {
    const def = PRESETS[preset].includes(c);
    const next = { ...custom };
    if (v === def) delete next[c.id];
    else next[c.id] = v;
    custom = next;
  }

  function setGroup(g: GroupId, v: boolean) {
    const next = { ...custom };
    for (const c of criteria.filter((c) => c.group === g)) {
      if (v === PRESETS[preset].includes(c)) delete next[c.id];
      else next[c.id] = v;
    }
    custom = next;
  }

  function choosePreset(p: PresetId) {
    preset = p;
    custom = {};
  }

  const differs = (c: Criterion) => new Set(selected.map((t) => cellKey(c.get(t)))).size > 1;

  const rowsByGroup = $derived(
    GROUPS.map((g) => ({
      ...g,
      rows: criteria.filter((c) => c.group === g.id && visible(c) && (!onlyDiff || selected.length < 2 || differs(c)))
    })).filter((g) => g.rows.length)
  );

  const hiddenCount = $derived(criteria.filter((c) => !visible(c)).length);

  function bestIndex(c: Criterion): number | null {
    if (!c.best || selected.length < 2) return null;
    let best: number | null = null;
    let bestVal: string | number | null = null;
    selected.forEach((t, i) => {
      const cell = c.get(t);
      const v = cell.kind === 'text' ? (cell.sort ?? null) : null;
      if (v == null) return;
      if (bestVal == null || v > bestVal) {
        bestVal = v;
        best = i;
      } else if (v === bestVal) best = null;
    });
    return best;
  }

  const suggestions = $derived(
    addQuery.trim()
      ? tools
          .filter((t) => !compare.has(t.id))
          .map((t) => ({ t, s: searchScore(t, addQuery) }))
          .filter((x) => x.s > 0)
          .sort((a, b) => b.s - a.s || (b.t.stars ?? 0) - (a.t.stars ?? 0))
          .slice(0, 6)
          .map((x) => x.t)
      : []
  );

  function add(id: string) {
    compare.toggle(id);
    addQuery = '';
  }

  async function share() {
    try {
      await navigator.clipboard.writeText(location.href);
      copied = true;
      setTimeout(() => (copied = false), 2000);
    } catch {
      /* presse-papiers indisponible : l'URL de la page reste partageable */
    }
  }

  const ideas = categories
    .filter((c) => c.tools.length >= 3)
    .sort((a, b) => b.count - a.count)
    .slice(0, 4);
</script>

<svelte:head>
  <title>Comparateur · Awesome AI</title>
  <meta name="description" content="Comparez jusqu'à 4 outils d'IA auto-hébergeables côte à côte, sur les critères de votre choix." />
</svelte:head>

<div class="top">
  <div>
    <h1>Comparateur</h1>
    <p class="lead">Jusqu'à {MAX_COMPARE} outils côte à côte. Choisissez les critères affichés et partagez la comparaison par lien.</p>
  </div>
  {#if selected.length < MAX_COMPARE}
    <div class="adder">
      <label class="sr-only" for="add">Ajouter un outil</label>
      <input id="add" type="search" bind:value={addQuery} placeholder="+ Ajouter un outil…" autocomplete="off" />
      {#if suggestions.length}
        <ul class="sugg" role="listbox">
          {#each suggestions as t (t.id)}
            <li><button onclick={() => add(t.id)}><strong>{t.name}</strong> <CategoryBadge slug={t.category} link={false} /></button></li>
          {/each}
        </ul>
      {/if}
    </div>
  {/if}
</div>

{#if selected.length === 0}
  <div class="panel empty">
    <p>Aucun outil sélectionné. Cochez « Comparer » sur les cartes du <a href={resolve('/outils')}>catalogue</a>, ou partez d'une idée :</p>
    <ul class="ideas">
      {#each ideas as c (c.slug)}
        <li><a class="btn" href={resolve(`/comparer?ids=${c.tools.slice(0, 3).join(',')}`)}>Top 3 {c.label}</a></li>
      {/each}
    </ul>
  </div>
{:else}
  <div class="controls">
    <div class="seg" role="group" aria-label="Préréglages de critères">
      {#each Object.entries(PRESETS) as [id, p] (id)}
        <button aria-pressed={preset === id && !Object.keys(custom).length} onclick={() => choosePreset(id as PresetId)}>{p.label}</button>
      {/each}
    </div>
    <button class="btn" onclick={() => (panelOpen = !panelOpen)} aria-expanded={panelOpen} aria-controls="criteria">
      Critères{#if hiddenCount}&nbsp;({criteria.length - hiddenCount}/{criteria.length}){/if}
    </button>
    <label class="switch">
      <input type="checkbox" role="switch" bind:checked={onlyDiff} />
      Seulement les différences
    </label>
    <span class="grow"></span>
    <button class="btn ghost" onclick={share}>{copied ? 'Lien copié ✓' : 'Copier le lien'}</button>
  </div>

  {#if panelOpen}
    <section id="criteria" class="panel criteria" aria-label="Choix des critères">
      {#each GROUPS as g (g.id)}
        {@const items = criteria.filter((c) => c.group === g.id)}
        {#if items.length}
          {@const on = items.filter(visible).length}
          <fieldset>
            <legend>
              <label>
                <input type="checkbox" checked={on === items.length} indeterminate={on > 0 && on < items.length}
                  onchange={(e) => setGroup(g.id, e.currentTarget.checked)} />
                {g.label} <span class="muted">{on}/{items.length}</span>
              </label>
            </legend>
            <div class="items">
              {#each items as c (c.id)}
                <label><input type="checkbox" checked={visible(c)} onchange={(e) => setVisible(c, e.currentTarget.checked)} /> {c.label}</label>
              {/each}
            </div>
          </fieldset>
        {/if}
      {/each}
    </section>
  {/if}

  {#if selected.length === 1}
    <p class="hint">Ajoutez au moins un autre outil pour comparer.</p>
  {/if}

  <div class="table-wrap cmp-wrap">
    <table class="cmp" style:--cols={selected.length}>
      <thead>
        <tr>
          <th class="crit" scope="col"><span class="sr-only">Critère</span></th>
          {#each selected as t (t.id)}
            <th scope="col">
              <div class="th">
                <a href={resolve('/outils/[id]', { id: t.id })}>{t.name}</a>
                <button class="x" onclick={() => compare.remove(t.id)} aria-label={`Retirer ${t.name}`}>×</button>
              </div>
              <div class="sub">★ {compact(t.stars)}</div>
            </th>
          {/each}
        </tr>
      </thead>
      {#each rowsByGroup as g (g.id)}
        <tbody>
          <tr class="group"><th colspan={selected.length + 1} scope="colgroup">{g.label}</th></tr>
          {#each g.rows as c (c.id)}
            {@const best = bestIndex(c)}
            <tr>
              <th class="crit" scope="row">{c.label}</th>
              {#each selected as t, i (t.id)}
                {@const cell = c.get(t)}
                <td class:best={best === i}>
                  {#if cell.kind === 'bool'}
                    {#if cell.value}<span class="yes">✓<span class="sr-only">Oui</span></span>{:else}<span class="no">✗<span class="sr-only">Non</span></span>{/if}
                  {:else}
                    {cell.value}{#if best === i}<span class="badge">meilleur</span>{/if}
                  {/if}
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      {/each}
    </table>
    {#if !rowsByGroup.length}
      <p class="panel empty">{onlyDiff ? 'Aucune différence sur les critères affichés.' : 'Aucun critère affiché : ouvrez « Critères » ou choisissez un préréglage.'}</p>
    {/if}
  </div>
  <p class="muted small">Fonctionnalités et compatibilités détectées automatiquement (description, README, topics) : un ✗ signifie « non détecté », pas forcément « absent ».</p>
{/if}

<style>
  .top { display: flex; justify-content: space-between; align-items: flex-start; gap: 1rem; flex-wrap: wrap; }
  .adder { position: relative; width: min(100%, 22rem); }
  .adder input { width: 100%; }
  .sugg { position: absolute; z-index: 10; top: 100%; left: 0; right: 0; list-style: none; margin: 0.25rem 0 0; padding: 0.25rem; background: var(--surface); border: 1px solid var(--border); border-radius: 8px; box-shadow: 0 8px 24px rgb(0 0 0 / 0.12); }
  .sugg button { display: flex; justify-content: space-between; gap: 0.5rem; width: 100%; padding: 0.5rem; border: 0; background: none; color: var(--fg); font: inherit; text-align: left; border-radius: 6px; cursor: pointer; }
  .sugg button:hover, .sugg button:focus-visible { background: var(--chip); }
  .empty { text-align: center; padding: 2rem 1rem; }
  .ideas { list-style: none; padding: 0; display: flex; flex-wrap: wrap; justify-content: center; gap: 0.5rem; }
  .controls { display: flex; flex-wrap: wrap; align-items: center; gap: 0.6rem; margin-bottom: 1rem; }
  .seg { display: inline-flex; border: 1px solid var(--border); border-radius: 8px; overflow: hidden; }
  .seg button { border: 0; background: var(--surface); color: var(--fg-2); padding: 0.45rem 0.8rem; font: inherit; font-size: 0.9rem; cursor: pointer; }
  .seg button[aria-pressed='true'] { background: var(--accent-soft); color: var(--accent); font-weight: 600; }
  .switch { display: inline-flex; align-items: center; gap: 0.45rem; font-size: 0.9375rem; cursor: pointer; }
  .switch input { accent-color: var(--accent); width: 1.05rem; height: 1.05rem; }
  .grow { flex: 1; }
  .criteria { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(100%, 15rem), 1fr)); gap: 1rem; margin-bottom: 1rem; }
  fieldset { border: 0; padding: 0; margin: 0; min-width: 0; }
  legend { font-weight: 600; margin-bottom: 0.35rem; }
  .items { display: grid; gap: 0.15rem; font-size: 0.875rem; }
  .criteria label { display: flex; align-items: center; gap: 0.4rem; cursor: pointer; }
  .criteria input { accent-color: var(--accent); }
  .hint { color: var(--fg-2); }
  .cmp-wrap { border: 1px solid var(--border); border-radius: 12px; background: var(--surface); max-height: 75vh; overflow: auto; }
  .cmp { border-collapse: separate; border-spacing: 0; width: 100%; font-size: 0.9rem; }
  .cmp th, .cmp td { padding: 0.55rem 0.75rem; border-bottom: 1px solid var(--border); text-align: left; vertical-align: top; }
  .cmp td, .cmp thead th:not(.crit) { min-width: 11rem; width: calc((100% - 13rem) / var(--cols)); }
  .cmp thead th { position: sticky; top: 0; background: var(--surface); z-index: 2; box-shadow: 0 1px 0 var(--border); }
  .cmp .crit { position: sticky; left: 0; background: var(--surface); z-index: 1; width: 13rem; min-width: 9rem; font-weight: 500; color: var(--fg-2); }
  .cmp thead .crit { z-index: 3; }
  .th { display: flex; justify-content: space-between; align-items: center; gap: 0.25rem; }
  .th a { font-weight: 700; color: var(--fg); }
  .sub { font-weight: 400; color: var(--muted); font-size: 0.8125rem; }
  .x { border: 0; background: none; color: var(--muted); font-size: 1.25rem; line-height: 1; cursor: pointer; padding: 0.1rem 0.35rem; border-radius: 6px; }
  .x:hover { background: var(--chip); color: var(--fg); }
  .group th { background: var(--chip); font-size: 0.8rem; text-transform: uppercase; letter-spacing: 0.04em; color: var(--fg-2); position: sticky; left: 0; }
  .yes { color: var(--good); font-weight: 700; font-size: 1.05rem; }
  .no { color: var(--muted); }
  td.best { background: color-mix(in srgb, var(--good) 12%, transparent); font-weight: 600; }
  .badge { margin-left: 0.4rem; font-size: 0.7rem; font-weight: 600; color: var(--good); border: 1px solid currentColor; border-radius: 999px; padding: 0 0.4rem; }
  .small { font-size: 0.8125rem; margin-top: 0.75rem; }
  @media (max-width: 640px) {
    .cmp .crit { width: 7.5rem; min-width: 7.5rem; }
    .cmp td, .cmp thead th:not(.crit) { min-width: calc((100vw - 2rem - 7.5rem) / 2); }
  }
</style>
