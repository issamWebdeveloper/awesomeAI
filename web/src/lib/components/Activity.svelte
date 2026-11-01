<script lang="ts">
  import { STATUS_LABELS, relativeDays } from '#lib/data.js';
  let { status, lastPush = null, long = false }: { status: string; lastPush?: string | null; long?: boolean } = $props();
</script>

<span class="act {status}" title={lastPush ? `Dernier push ${relativeDays(lastPush)}` : undefined}>
  <span class="dot" aria-hidden="true"></span>
  {STATUS_LABELS[status] ?? status}{#if long && lastPush}<span class="when"> · push {relativeDays(lastPush)}</span>{/if}
</span>

<style>
  .act { display: inline-flex; align-items: center; gap: 0.35em; font-size: 0.8125rem; color: var(--fg-2); white-space: nowrap; }
  .dot { width: 0.55em; height: 0.55em; border-radius: 50%; background: var(--muted); }
  .active .dot { background: var(--good); }
  .slow .dot { background: var(--warn); }
  .stale .dot { background: var(--bad); }
  .when { color: var(--muted); }
</style>
