<script lang="ts">
  import { resolve } from '$app/paths';
  import { categoryBySlug } from '#lib/data.js';
  let { slug, link = true }: { slug: string; link?: boolean } = $props();
  const cat = $derived(categoryBySlug.get(slug));
</script>

{#if cat}
  {#if link}
    <a class="badge" href={resolve('/categories/[slug]', { slug })} style:--c={cat.color}>{cat.label}</a>
  {:else}
    <span class="badge" style:--c={cat.color}>{cat.label}</span>
  {/if}
{/if}

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    gap: 0.4em;
    font-size: 0.8125rem;
    color: var(--fg-2);
    text-decoration: none;
    white-space: nowrap;
  }
  .badge::before {
    content: '';
    width: 0.6em;
    height: 0.6em;
    border-radius: 50%;
    background: var(--c);
    flex: none;
  }
  a.badge:hover { color: var(--fg); text-decoration: underline; }
</style>
