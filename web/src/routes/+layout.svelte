<script>
  import { resolve } from '$app/paths';
  let { children } = $props();
</script>

<header>
  <a class="brand" href={resolve('/')}>Awesome AI</a>
  <nav>
    <span>Dashboard</span>
    <span>Outils</span>
    <span>Comparer</span>
  </nav>
</header>

<main>{@render children()}</main>

<footer>
  Données issues de
  <a href="https://github.com/glincker/awesome-selfhosted-ai">awesome-selfhosted-ai</a>.
</footer>

<style>
  :global(:root) {
    --bg: #fafaf9;
    --fg: #1c1917;
    --muted: #57534e;
    --accent: #4f46e5;
    --border: #e7e5e4;
    color-scheme: light dark;
  }
  @media (prefers-color-scheme: dark) {
    :global(:root) {
      --bg: #0c0a09;
      --fg: #f5f5f4;
      --muted: #a8a29e;
      --accent: #818cf8;
      --border: #292524;
    }
  }
  :global(body) {
    margin: 0;
    background: var(--bg);
    color: var(--fg);
    font: 16px/1.6 system-ui, sans-serif;
  }
  :global(a) { color: var(--accent); }
  header, main, footer { max-width: 72rem; margin: 0 auto; padding: 1rem; }
  header { display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--border); }
  .brand { font-weight: 700; text-decoration: none; color: var(--fg); }
  nav { display: flex; gap: 1.25rem; color: var(--muted); }
  footer { color: var(--muted); font-size: 0.875rem; border-top: 1px solid var(--border); }
</style>
