<script lang="ts">
  // Barres horizontales triées, chaque ligne est un lien (catalogue filtré).
  interface Row { key: string; label: string; value: number; href?: string; color?: string; note?: string }
  let { rows, format = (v: number) => String(v), max = undefined }: {
    rows: Row[];
    format?: (v: number) => string;
    max?: number;
  } = $props();
  const top = $derived(max ?? Math.max(1, ...rows.map((r) => r.value)));
</script>

<ul class="bars">
  {#each rows as r (r.key)}
    <li>
      <svelte:element this={r.href ? 'a' : 'div'} href={r.href} class="row" title={`${r.label} : ${format(r.value)}${r.note ? ` (${r.note})` : ''}`}>
        <span class="label">
          {#if r.color}<span class="dot" style:background={r.color} aria-hidden="true"></span>{/if}
          {r.label}
        </span>
        <span class="track"><span class="fill" style:width={`${(r.value / top) * 100}%`}></span></span>
        <span class="value">{format(r.value)}</span>
      </svelte:element>
    </li>
  {/each}
</ul>

<style>
  .bars { list-style: none; margin: 0; padding: 0; display: grid; gap: 2px; }
  .row {
    display: grid;
    grid-template-columns: minmax(7rem, 14rem) 1fr 3.5rem;
    align-items: center;
    gap: 0.75rem;
    padding: 0.3rem 0.4rem;
    border-radius: 6px;
    color: var(--fg);
    text-decoration: none;
    font-size: 0.875rem;
  }
  a.row:hover { background: var(--chip); }
  a.row:hover .fill { background: var(--accent-strong); }
  .label { display: flex; align-items: center; gap: 0.45rem; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .dot { width: 0.6rem; height: 0.6rem; border-radius: 50%; flex: none; }
  .track { height: 0.7rem; background: var(--chip); border-radius: 4px; overflow: hidden; }
  .fill { display: block; height: 100%; background: var(--accent); border-radius: 0 4px 4px 0; min-width: 2px; }
  .value { text-align: right; font-variant-numeric: tabular-nums; color: var(--fg-2); }
  @media (max-width: 480px) {
    .row { grid-template-columns: 1fr 3rem; }
    .track { grid-column: 1 / -1; grid-row: 2; }
  }
</style>
