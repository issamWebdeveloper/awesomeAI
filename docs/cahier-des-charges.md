# Awesome AI : cahier des charges fonctionnel

> Version 0.1 (brouillon à valider) · 1er novembre 2026

## 1. Contexte et objectif

La liste [awesome-selfhosted-ai](https://github.com/glincker/awesome-selfhosted-ai) recense environ 308 outils d'IA auto-hébergeables, répartis en 18 catégories (runtimes LLM, agents, RAG, MCP, speech, etc.). Une liste Markdown permet de découvrir les outils, mais pas de les **comprendre**, de les **filtrer** ni de les **comparer**.

**Awesome AI** est un site web statique, hébergé sur GitHub Pages, qui :

1. récupère automatiquement tous les outils de la liste source ;
2. les analyse et les enrichit (objectif, problème résolu, fonctionnalités, stack, licence, popularité, activité) ;
3. dégage les **points communs** et **points clés** par catégorie ;
4. les présente dans un **tableau de bord** clair et un **catalogue** filtrable ;
5. propose un **comparateur** côte à côte, dont on choisit les critères affichés.

## 2. Périmètre

**Inclus (V1)**

- Import des outils de `README.md` et des métriques de `LIVE.md` (étoiles, dernier push, licence, statut).
- Fiche détaillée par outil, catalogue, dashboard, comparateur (2 à 4 outils).
- Mise à jour automatique hebdomadaire des données.
- Interface en français, structure prête pour l'anglais.

**Hors périmètre (V1)**

- Comptes utilisateurs, avis ou notes saisis par les visiteurs.
- Back-office d'édition en ligne (les corrections passent par des fichiers du dépôt).
- Exécution ou démonstration des outils eux-mêmes.

## 3. Utilisateurs et cas d'usage

| Profil | Besoin | Exemple |
| --- | --- | --- |
| Développeur·se | Trouver l'outil adapté à un besoin technique | « Quel runtime LLM supporte le GPU AMD et une API compatible OpenAI ? » |
| Décideur·se / CTO | Comparer des solutions sur des critères objectifs | Licence, activité, maturité, facilité de déploiement |
| Curieux·se / veille | Comprendre le paysage de l'IA auto-hébergée | Quelles catégories, quels outils montent, quelles tendances |

Cas d'usage principaux :

- **CU1** Rechercher un outil par mot-clé, catégorie ou fonctionnalité.
- **CU2** Lire en 30 secondes ce que fait un outil et quel problème il résout.
- **CU3** Comparer 2 à 4 outils sur les critères de son choix et partager la comparaison par lien.
- **CU4** Voir les points communs d'une catégorie et ce qui distingue chaque outil.
- **CU5** Suivre les tendances (outils « Rising », activité, popularité).

## 4. Données

### 4.1 Sources

| Source | Contenu récupéré |
| --- | --- |
| `README.md` de la liste | Nom, URL, description courte, catégorie (y compris « Rising » et « Source-Available ») |
| `LIVE.md` de la liste | Étoiles, date du dernier push, licence, statut (`active` / `slow`) |
| API GitHub (dépôt de chaque outil) | Langage principal, topics, date de création, dernière release, issues ouvertes, URL du site, extrait du README |
| Enrichissement (voir 4.3) | Objectif, problème résolu, fonctionnalités, mode de déploiement, compatibilités |

### 4.2 Modèle d'une fiche outil

| Champ | Type | Origine | Exemple |
| --- | --- | --- | --- |
| `id` | slug | calculé | `crewai` |
| `name` | texte | README | CrewAI |
| `repo` | URL | README | github.com/crewAIInc/crewAI |
| `category` | énum (18) | README | Agents and Automation |
| `tags` | liste | README (Rising, Source-Available) | `["rising"]` |
| `summary` | texte | README | Framework for orchestrating role-based teams… |
| `goal` | texte (FR) | enrichissement | Orchestrer des équipes d'agents spécialisés |
| `problem_solved` | texte (FR) | enrichissement | Coordonner plusieurs LLM sur une tâche complexe |
| `features` | liste de clés | enrichissement | `multi-agent`, `memory`, `tool-calling` |
| `deployment` | liste | enrichissement | `pip`, `docker`, `binary`, `desktop-app` |
| `interfaces` | liste | enrichissement | `python-sdk`, `rest-api`, `web-ui`, `cli` |
| `openai_compatible` | booléen | enrichissement | true |
| `hardware` | liste | enrichissement | `cpu`, `nvidia`, `amd`, `apple-silicon` |
| `language` | texte | API GitHub | Python |
| `license` | SPDX | LIVE.md | MIT |
| `stars` | entier | LIVE.md / API | 60000 |
| `last_push` | date | LIVE.md | 2026-10-30 |
| `status` | énum | LIVE.md | active |
| `first_seen` | date | historique | date d'apparition dans la liste |
| `updated_at` | date | pipeline | date de la dernière collecte |

Les fonctionnalités (`features`) viennent d'un **vocabulaire contrôlé** par catégorie (fichier `taxonomy.yaml`), sinon la comparaison est impossible. Exemple pour « LLM Runtimes » : quantification, GPU multiples, API OpenAI, streaming, batching, formats GGUF/safetensors.

### 4.3 Analyse et enrichissement

1. **Extraction automatique** : parsing du README et de LIVE.md, appels à l'API GitHub.
2. **Classification** : des règles (mots-clés dans description, topics, README) attribuent les fonctionnalités du vocabulaire contrôlé. Optionnellement, un LLM propose `goal`, `problem_solved` et `features` à partir du README de l'outil.
3. **Revue humaine** : les propositions sont écrites dans `data/overrides/<id>.yaml`, relues et corrigées à la main. Une valeur saisie à la main n'est jamais écrasée par la collecte.
4. **Synthèse par catégorie** : calcul des fonctionnalités partagées par la majorité des outils (points communs) et de celles rares (points distinctifs), plus un court texte de synthèse relu.

## 5. Fonctionnalités

| ID | Fonctionnalité | Priorité |
| --- | --- | --- |
| F1 | Recherche plein texte instantanée (nom, description, fonctionnalités) | Must |
| F2 | Filtres combinables : catégorie, licence, langage, statut, déploiement, matériel, fonctionnalités | Must |
| F3 | Tri : popularité, activité récente, nom, date d'ajout | Must |
| F4 | Vues catalogue en cartes ou en tableau | Must |
| F5 | Fiche outil : objectif, problème résolu, fonctionnalités, métriques, liens, outils similaires | Must |
| F6 | Dashboard : indicateurs clés et graphiques (voir page 6.2) | Must |
| F7 | Comparateur 2 à 4 outils, ajout depuis toute carte ou fiche | Must |
| F8 | Comparateur : afficher/masquer chaque critère, groupes de critères, option « n'afficher que les différences » | Must |
| F9 | État des filtres et du comparateur encodé dans l'URL (partageable, favoris) | Must |
| F10 | Page catégorie avec points communs et points distinctifs | Should |
| F11 | Export CSV / JSON de la sélection courante | Should |
| F12 | Thème clair / sombre (suit le système, bascule manuelle) | Should |
| F13 | Historique : courbe d'étoiles et date d'entrée dans la liste | Could |
| F14 | Bascule FR / EN | Could |

## 6. Pages proposées

```
/                       Accueil
/dashboard              Tableau de bord
/outils                 Catalogue (recherche + filtres)
/outils/[id]            Fiche outil
/categories             Liste des catégories
/categories/[slug]      Synthèse d'une catégorie
/comparer?ids=a,b,c     Comparateur
/a-propos               Méthodologie, sources, licence, mise à jour
```

### 6.1 Accueil

Une phrase qui dit ce que fait le site, une grande barre de recherche, 4 chiffres clés (nombre d'outils, catégories, part active, dernière mise à jour), la grille des catégories avec leur nombre d'outils, et la rangée « Rising ».

### 6.2 Tableau de bord

- Rangée d'indicateurs : outils, catégories, étoiles cumulées, % actifs sur 90 jours.
- Outils par catégorie (barres horizontales triées).
- Répartition des licences (permissives, copyleft, source-available).
- Langages principaux.
- Fraîcheur : distribution des dates de dernier push.
- Top 10 par étoiles, global et par catégorie (sélecteur).
- Matrice catégories × fonctionnalités (heatmap) : montre d'un coup d'œil les points communs.
- Chaque graphique est cliquable et ouvre le catalogue filtré correspondant.

### 6.3 Catalogue

Barre de recherche en haut, filtres en panneau latéral (tiroir sur mobile), compteur de résultats, bascule cartes/tableau. Chaque carte montre : nom, catégorie, résumé en une ligne, 3 fonctionnalités clés, étoiles, licence, pastille d'activité, case « Comparer ».

### 6.4 Fiche outil

En-tête (nom, catégorie, liens dépôt/site, métriques), bloc « En bref » (objectif, problème résolu), fonctionnalités groupées, déploiement et compatibilités, outils similaires de la même catégorie, bouton « Ajouter au comparateur ».

### 6.5 Catégorie

Description de la catégorie, **points communs** (fonctionnalités présentes chez la majorité), **points distinctifs** (qui fait quoi de rare), tableau compact de tous les outils de la catégorie, raccourci « Comparer les 3 plus populaires ».

### 6.6 Comparateur

- Une colonne par outil (2 à 4), en-têtes collants, retrait d'un outil par une croix.
- Critères groupés : Généralités, Fonctionnalités, Déploiement, Compatibilité, Communauté et activité, Licence.
- Panneau « Critères » : une case par critère et par groupe pour l'afficher ou le masquer ; préréglages (« Essentiel », « Technique », « Tout »).
- Interrupteur « Seulement les différences ».
- Cellules ✓ / ✗ / valeur, avec mise en valeur de la meilleure valeur quand elle est mesurable (plus d'étoiles, push le plus récent).
- Barre de comparaison persistante en bas de l'écran sur toutes les pages, qui affiche la sélection en cours.
- Sur mobile : 2 colonnes visibles avec défilement horizontal, critère figé à gauche.

### 6.7 À propos

Sources, méthode d'analyse, fréquence de mise à jour, comment proposer une correction (lien vers le dépôt), attribution de la liste d'origine.

## 7. Principes de design

1. **Une page, une tâche** : chaque page répond à une question précise, l'action principale est visible sans défiler.
2. **Lisible avant d'être joli** : typographie système ou Inter, 16 px minimum, contraste AA, beaucoup d'espace blanc.
3. **Couleur avec parcimonie** : une couleur d'accent, une couleur par catégorie réutilisée partout (carte, graphique, badge), vert/orange pour l'activité.
4. **Tout est filtrable, tout est partageable** : chaque vue a une URL stable.
5. **Retour immédiat** : recherche et filtres sans rechargement (< 100 ms), état vide explicite (« Aucun outil ne correspond, réinitialiser les filtres »).
6. **Mobile d'abord** : grille d'une colonne à 360 px, filtres en tiroir, comparateur utilisable au pouce.
7. **Accessible** : navigation clavier complète, focus visible, graphiques doublés d'un tableau ou d'un texte alternatif.
8. **Composants réutilisés** : carte outil, badge, pastille d'activité, puce de fonctionnalité, en-tête de page, identiques sur toutes les pages.

## 8. Architecture technique

### 8.1 Contrainte de GitHub Pages

GitHub Pages sert **uniquement des fichiers statiques** (HTML, CSS, JS, JSON, WASM). Aucun processus serveur ne peut y tourner. **Une API Rust/axum ne peut donc pas être hébergée sur GitHub Pages.** Rust reste tout à fait utilisable, de trois façons :

| Option | Principe | Avantages | Limites |
| --- | --- | --- | --- |
| **A. Rust au build** (recommandée) | Un binaire Rust (`crates/collector`) tourne dans GitHub Actions : il collecte, analyse et écrit des fichiers JSON que Svelte lit. | 100 % GitHub Pages, gratuit, rapide, aucune infra à maintenir | Données rafraîchies à chaque build (hebdo ou à la demande), pas en temps réel |
| **B. Rust → WebAssembly** | Une partie de la logique Rust (recherche, scoring, comparaison) est compilée en WASM et appelée depuis Svelte dans le navigateur. | Rust côté client, reste sur GitHub Pages | Utile seulement si le JS ne suffit pas ; ajoute une étape de build |
| **C. API axum hébergée ailleurs** | Svelte reste sur GitHub Pages, l'API axum tourne sur Fly.io, Render, Koyeb ou un VPS. | Vraie API, données dynamiques, écriture possible (votes, avis) | Hébergement séparé, CORS, coût éventuel, maintenance |

**Recommandation** : option A pour la V1. Le volume (≈ 300 outils, < 1 Mo de JSON) se filtre très bien dans le navigateur. Le code Rust est structuré en bibliothèque (`crates/core`) + binaire, ce qui permet de l'exposer plus tard via axum (option C) ou WASM (option B) sans le réécrire.

### 8.2 Front-end

- **SvelteKit** avec `@sveltejs/adapter-static` : chaque page (y compris les 300 fiches) est pré-rendue en HTML, bon pour le SEO et la vitesse.
- `paths.base = '/awesomeAI'` pour l'URL de GitHub Pages.
- Recherche : MiniSearch (index construit au build).
- Graphiques : LayerChart ou Chart.js.
- Style : Tailwind CSS ou CSS natif avec variables (thème clair/sombre).

### 8.3 Données : options de « base de données »

| Option | Fonctionne sur GitHub Pages | Pour | Contre |
| --- | --- | --- | --- |
| **JSON versionné dans le dépôt** (recommandée) | Oui | Simple, historique Git gratuit, diff lisible, relecture par PR | Pas de requêtes SQL (inutile à ce volume) |
| SQLite via sql.js (WASM) | Oui | Requêtes SQL dans le navigateur | ≈ 1 Mo de WASM à charger, plus complexe |
| DuckDB-WASM + Parquet | Oui | Analytique puissante, gros volumes | Lourd (plusieurs Mo), surdimensionné ici |
| Supabase / Turso / PocketBase | Front oui, données hébergées ailleurs | Écriture, comptes, temps réel | Service externe, clés à protéger, dépendance |

**Recommandation** : JSON versionné. Fichiers YAML relus à la main pour les enrichissements (`data/overrides/`), fusionnés par le collecteur Rust en `static/data/tools.json`, `categories.json` et `stats.json`. Si le projet évolue vers des contributions des visiteurs, passer à l'option C avec SQLite côté serveur.

### 8.4 Organisation du dépôt

```
awesomeAI/
├─ crates/
│  ├─ core/          modèle, parsing, analyse (bibliothèque Rust)
│  └─ collector/     binaire CLI : collecte → JSON
├─ data/
│  ├─ taxonomy.yaml  vocabulaire de fonctionnalités par catégorie
│  ├─ overrides/     corrections et enrichissements manuels
│  └─ snapshots/     historique (étoiles, dates)
├─ web/              application SvelteKit
│  └─ static/data/   JSON générés
├─ docs/             ce cahier des charges
└─ .github/workflows/
   ├─ collect.yml    hebdomadaire : collecte + PR de mise à jour des données
   └─ deploy.yml     à chaque push sur main : build + déploiement Pages
```

### 8.5 Chaîne de mise à jour et de déploiement

1. **Collecte** (`collect.yml`, chaque lundi + déclenchement manuel) : `cargo run -p collector` lit la liste source et l'API GitHub (jeton `GITHUB_TOKEN`), écrit les JSON, ouvre une PR si les données ont changé.
2. **Relecture** : on vérifie les nouveaux outils et on complète leurs `overrides`.
3. **Déploiement** (`deploy.yml`, à chaque merge sur `main`) : build SvelteKit, puis `actions/upload-pages-artifact` et `actions/deploy-pages`.

Adresse finale : `https://issamwebdeveloper.github.io/awesomeAI/`.

## 9. Exigences non fonctionnelles

- **Performance** : Lighthouse ≥ 90 sur toutes les catégories ; première page < 200 Ko de JS compressé.
- **Accessibilité** : WCAG 2.1 AA.
- **SEO** : pages pré-rendues, titres et descriptions par fiche, `sitemap.xml`.
- **Fiabilité** : si la collecte échoue, le site garde les dernières données valides.
- **Licence et attribution** : citer la liste d'origine et vérifier sa licence avant de republier les descriptions.
- **Coût** : zéro euro en V1 (GitHub Pages + Actions).

## 10. Découpage en lots

| Lot | Contenu |
| --- | --- |
| L0 | Validation de ce cahier, mise en place du dépôt, workflow de déploiement Pages avec page d'accueil minimale |
| L1 | Collecteur Rust : parsing README + LIVE.md → `tools.json` |
| L2 | Catalogue, recherche, filtres, fiche outil |
| L3 | Comparateur avec sélection des critères |
| L4 | Dashboard et pages catégories |
| L5 | Enrichissement (taxonomie, API GitHub, LLM optionnel) et collecte hebdomadaire automatique |

## 11. Questions ouvertes

1. Option d'architecture : A (Rust au build, recommandée) ou dès maintenant C (API axum sur un autre hébergeur) ?
2. Enrichissement par LLM : acceptable (avec relecture humaine), ou uniquement des règles et de la saisie manuelle ?
3. Langue : français seul en V1, ou bilingue dès le départ ?
4. Nom de domaine personnalisé, ou l'adresse `github.io` suffit ?
