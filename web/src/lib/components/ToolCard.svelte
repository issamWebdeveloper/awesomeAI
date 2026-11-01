<script lang="ts">
  import { resolve } from '$app/paths';
  import { compact, featureLabel, type Tool } from '#lib/data.js';
  import Activity from './Activity.svelte';
  import CategoryBadge from './CategoryBadge.svelte';
  import Chip from './Chip.svelte';
  import CompareToggle from './CompareToggle.svelte';
  let { tool, showCategory = true }: { tool: Tool; showCategory?: boolean } = $props();
</script>

<article class="card">
  <header>
    <h3><a href={resolve('/outils/[id]', { id: tool.id })}>{tool.name}</a></h3>
    {#if tool.tags.includes('rising')}<span class="rising">Rising</span>{/if}
  </header>
  {#if showCategory}<CategoryBadge slug={tool.category} />{/if}
  <p class="summary">{tool.summary}</p>
  {#if tool.features.length}
    <div class="chips">
      {#each tool.features.slice(0, 3) as f (f)}<Chip label={featureLabel(f)} />{/each}
    </div>
  {/if}
  <footer>
    <span title="Étoiles GitHub">★ {compact(tool.stars)}</span>
    <span class="lic" title={tool.license_note ?? tool.license ?? ''}>{tool.license ?? (tool.license_family === 'source-available' ? 'Source disp.' : '—')}</span>
    <Activity status={tool.status} lastPush={tool.last_push} />
    <span class="grow"></span>
    <CompareToggle id={tool.id} name={tool.name} />
  </footer>
</article>

<style>
  .card {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 12px;
    min-width: 0;
  }
  header { display: flex; align-items: baseline; gap: 0.5rem; }
  h3 { margin: 0; font-size: 1.0625rem; line-height: 1.3; }
  h3 a { color: var(--fg); text-decoration: none; }
  h3 a:hover { color: var(--accent); text-decoration: underline; }
  .rising { font-size: 0.75rem; color: var(--accent); font-weight: 600; }
  .summary {
    margin: 0;
    color: var(--fg-2);
    font-size: 0.9375rem;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .chips { display: flex; flex-wrap: wrap; gap: 0.35rem; }
  footer {
    margin-top: auto;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.4rem 0.9rem;
    font-size: 0.8125rem;
    color: var(--fg-2);
    padding-top: 0.25rem;
  }
  .lic { max-width: 9rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .grow { flex: 1; }
</style>
