<script lang="ts">
  import { fmtDate, stats } from '#lib/data.js';
  const repo = 'https://github.com/issamWebdeveloper/awesomeAI';
</script>

<svelte:head>
  <title>À propos · Awesome AI</title>
  <meta name="description" content="Sources, méthode d'analyse et mise à jour des données d'Awesome AI." />
</svelte:head>

<article class="prose">
  <h1>À propos</h1>
  <p class="lead">Awesome AI rend la liste <a href="https://github.com/glincker/awesome-selfhosted-ai">awesome-selfhosted-ai</a> explorable : on peut la filtrer, comprendre chaque outil et comparer les solutions entre elles.</p>

  <h2>Sources</h2>
  <ul>
    <li><strong>Liste des outils</strong> : <code>data/projects.json</code> de awesome-selfhosted-ai (nom, dépôt, catégorie, description, étiquettes Rising et Source-Available). La liste est publiée sous licence CC0.</li>
    <li><strong>Santé des projets</strong> : <code>LIVE.md</code> de la même liste (étoiles, dernier push, licence, statut).</li>
    <li><strong>Dépôt de chaque outil</strong> : le README (pour détecter fonctionnalités et modes de déploiement) et, lors du déploiement, l'API GitHub (langage, topics, site, date de création, issues ouvertes).</li>
  </ul>

  <h2>Méthode d'analyse</h2>
  <ol>
    <li><strong>Collecte</strong> : un programme Rust lit les sources et télécharge le README de chaque outil.</li>
    <li><strong>Classification</strong> : chaque catégorie a un vocabulaire contrôlé de fonctionnalités (fichier <code>data/taxonomy.yaml</code>). Une fonctionnalité est attribuée si l'un de ses mots-clés figure dans la description, les topics ou le README. Même principe pour le déploiement (Docker, pip, binaire…), les interfaces (web, CLI, API, SDK, MCP) et le matériel (CPU, NVIDIA, AMD, Apple Silicon…).</li>
    <li><strong>Revue humaine</strong> : les corrections et les textes « objectif » et « problème résolu » propres à un outil se saisissent dans <code>data/overrides/</code>. Une valeur relue n'est jamais écrasée par la collecte.</li>
    <li><strong>Synthèse</strong> : pour chaque catégorie, les fonctionnalités présentes chez au moins la moitié des outils forment les <em>points communs</em> ; celles présentes chez 25 % des outils au plus forment les <em>points distinctifs</em>.</li>
  </ol>
  <p>La détection automatique est une aide, pas une vérité : un ✗ signifie « non détecté ». Vérifiez sur le dépôt avant de décider.</p>

  <h2>Activité</h2>
  <p><strong>Actif</strong> : dernier push il y a 90 jours au plus. <strong>Ralenti</strong> : entre 90 et 365 jours. <strong>Inactif</strong> : plus d'un an.</p>

  <h2>Mise à jour</h2>
  <p>Les données sont recollectées à chaque déploiement et automatiquement chaque lundi. Si la collecte échoue, le site garde les dernières données valides. Dernière collecte : <strong>{fmtDate(stats.updated_at)}</strong>{stats.source_refreshed ? ` ; liste source rafraîchie le ${fmtDate(stats.source_refreshed)}` : ''}.</p>

  <h2>Proposer une correction</h2>
  <p>Une fonctionnalité manque ou est fausse ? Ouvrez une issue ou une pull request sur <a href={repo}>le dépôt du site</a> en ajoutant l'outil dans <code>data/overrides/</code>. Pour ajouter un outil à la liste, contribuez à <a href="https://github.com/glincker/awesome-selfhosted-ai">la liste d'origine</a>.</p>

  <h2>Licence et attribution</h2>
  <p>Descriptions et classement d'origine : awesome-selfhosted-ai, par glincker et ses contributeurs (CC0 1.0). Chaque outil reste sous sa propre licence, indiquée sur sa fiche.</p>
</article>

<style>
  .prose { max-width: 46rem; }
  h2 { margin-top: 2rem; }
  li { margin-bottom: 0.4rem; }
  code { background: var(--chip); padding: 0.05rem 0.35rem; border-radius: 4px; font-size: 0.9em; }
</style>
