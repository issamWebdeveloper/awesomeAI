<script lang="ts">
  import '../app.css';
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import CompareBar from '#lib/components/CompareBar.svelte';
  import { compare } from '#lib/compare.svelte.js';
  import { fmtDate, stats } from '#lib/data.js';

  let { children } = $props();

  const links = [
    { href: resolve('/dashboard'), id: '/dashboard', label: 'Tableau de bord' },
    { href: resolve('/outils'), id: '/outils', label: 'Outils' },
    { href: resolve('/categories'), id: '/categories', label: 'Catégories' },
    { href: resolve('/comparer'), id: '/comparer', label: 'Comparer' },
    { href: resolve('/a-propos'), id: '/a-propos', label: 'À propos' }
  ];

  let theme = $state<'light' | 'dark' | null>(null);
  $effect(() => {
    const t = document.documentElement.dataset.theme;
    theme = t === 'light' || t === 'dark' ? t : null;
  });

  function toggleTheme() {
    const current = theme ?? (matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
    theme = current === 'dark' ? 'light' : 'dark';
    document.documentElement.dataset.theme = theme;
    try {
      localStorage.setItem('awesome-ai:theme', theme);
    } catch {
      /* préférence non mémorisée */
    }
  }

  const active = (id: string) => page.route.id?.startsWith(id);
</script>

<a class="skip" href="#contenu">Aller au contenu</a>
<header class="site">
  <div class="inner">
    <a class="brand" href={resolve('/')}>Awesome&nbsp;AI</a>
    <nav aria-label="Navigation principale">
      {#each links as l (l.id)}
        <a href={l.href} aria-current={active(l.id) ? 'page' : undefined}>
          {l.label}{#if l.id === '/comparer' && compare.ids.length}<span class="pill">{compare.ids.length}</span>{/if}
        </a>
      {/each}
    </nav>
    <button class="theme" onclick={toggleTheme} aria-label="Basculer le thème clair ou sombre" title="Thème clair / sombre">
      <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true"><path fill="currentColor" d="M12 3a9 9 0 1 0 9 9c0-.46-.04-.92-.1-1.36a5.4 5.4 0 0 1-4.4 2.26 5.4 5.4 0 0 1-3.14-9.8C12.92 3.04 12.46 3 12 3Z"/></svg>
    </button>
  </div>
</header>

<main id="contenu" class:pad={compare.ids.length > 0}>{@render children()}</main>

<footer class="site">
  <div class="inner">
    <p>
      Données issues de
      <a href="https://github.com/glincker/awesome-selfhosted-ai">awesome-selfhosted-ai</a> (CC0),
      collectées le {fmtDate(stats.updated_at)}. Code source sur
      <a href="https://github.com/issamWebdeveloper/awesomeAI">GitHub</a>.
    </p>
  </div>
</footer>

<CompareBar />

<style>
  .skip { position: absolute; left: -999px; top: 0; background: var(--accent); color: var(--on-accent); padding: 0.5rem 1rem; z-index: 30; }
  .skip:focus { left: 1rem; }
  .inner { max-width: 76rem; margin: 0 auto; padding: 0 1rem; }
  header.site { border-bottom: 1px solid var(--border); background: var(--surface); position: sticky; top: 0; z-index: 15; }
  header .inner { display: flex; align-items: center; gap: 1.5rem; min-height: 3.5rem; }
  .brand { font-weight: 800; font-size: 1.125rem; color: var(--fg); text-decoration: none; letter-spacing: -0.01em; white-space: nowrap; }
  nav { display: flex; gap: 0.25rem; overflow-x: auto; flex: 1; scrollbar-width: none; }
  nav a { color: var(--fg-2); text-decoration: none; padding: 0.4rem 0.65rem; border-radius: 8px; white-space: nowrap; font-size: 0.9375rem; display: inline-flex; align-items: center; gap: 0.35rem; }
  nav a:hover { background: var(--chip); color: var(--fg); }
  nav a[aria-current='page'] { color: var(--accent); background: var(--accent-soft); font-weight: 600; }
  .pill { background: var(--accent); color: var(--on-accent); border-radius: 999px; font-size: 0.7rem; padding: 0 0.4rem; line-height: 1.4; }
  .theme { border: 1px solid var(--border); background: transparent; color: var(--fg-2); border-radius: 8px; width: 2.25rem; height: 2.25rem; display: grid; place-items: center; cursor: pointer; flex: none; }
  .theme:hover { color: var(--fg); border-color: var(--accent); }
  main { max-width: 76rem; margin: 0 auto; padding: 2rem 1rem 3rem; min-height: 60vh; }
  main.pad { padding-bottom: 6rem; }
  footer.site { border-top: 1px solid var(--border); color: var(--muted); font-size: 0.875rem; }
  @media (max-width: 640px) {
    header .inner { flex-wrap: wrap; gap: 0.25rem 1rem; padding-top: 0.5rem; }
    nav { order: 3; flex-basis: 100%; margin: 0 -0.5rem; padding-bottom: 0.4rem; }
    .theme { margin-left: auto; }
  }
</style>
