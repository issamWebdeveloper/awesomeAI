<script lang="ts">
  let { label, href = undefined, muted = false }: { label: string; href?: string; muted?: boolean } = $props();
</script>

{#if href}
  <a class="chip" class:muted {href}>{label}</a>
{:else}
  <span class="chip" class:muted>{label}</span>
{/if}

<style>
  .chip {
    display: inline-block;
    padding: 0.1rem 0.55rem;
    border-radius: 999px;
    background: var(--chip);
    color: var(--fg);
    font-size: 0.8125rem;
    line-height: 1.6;
    text-decoration: none;
    white-space: nowrap;
  }
  .muted { background: transparent; border: 1px solid var(--border); color: var(--fg-2); }
  a.chip:hover { background: var(--accent-soft); }
</style>
