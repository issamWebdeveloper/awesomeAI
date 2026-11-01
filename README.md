# Awesome AI

Tableau de bord, catalogue et comparateur des outils d'IA auto-hébergeables listés dans [awesome-selfhosted-ai](https://github.com/glincker/awesome-selfhosted-ai), publié sur GitHub Pages.

Adresse : https://issamwebdeveloper.github.io/awesomeAI/

## Pages

| Page | Contenu |
| --- | --- |
| `/` | Recherche, chiffres clés, catégories, projets Rising |
| `/dashboard` | Indicateurs, outils par catégorie, licences, fraîcheur, langages, top 10, matrice catégories × capacités |
| `/outils` | Catalogue : recherche, filtres combinables, tri, cartes ou tableau, export CSV/JSON, état dans l'URL |
| `/outils/<id>` | Fiche outil : en bref, fonctionnalités, déploiement, compatibilités, outils similaires |
| `/categories`, `/categories/<slug>` | Synthèse par catégorie : points communs, points distinctifs, tableau des outils |
| `/comparer?ids=a,b,c` | Comparateur 2 à 4 outils, critères affichables/masquables, préréglages, « seulement les différences » |
| `/a-propos` | Sources, méthode, mise à jour |

## Organisation

```
crates/core/        modèle, parsing de la liste source, classification, synthèse (bibliothèque Rust)
crates/collector/   binaire : collecte → web/src/lib/data/*.json (+ copie dans web/static/data/)
data/taxonomy.yaml  catégories (libellés FR) et vocabulaire contrôlé des fonctionnalités
data/overrides/     fiches relues à la main (objectif, problème résolu, corrections)
data/snapshots/     date d'entrée de chaque outil dans la liste
web/                application SvelteKit statique
```

## Développement

```sh
# Données (facultatif : les JSON générés sont versionnés)
cargo run -p collector --release          # GITHUB_TOKEN=… pour enrichir via l'API GitHub
cargo test --workspace

# Site
cd web
npm install
npm run dev      # http://localhost:5173
npm run check
npm run build    # site statique dans web/build
```

## Déploiement

`.github/workflows/deploy.yml` lance les tests Rust, recollecte les données (avec le `GITHUB_TOKEN` du workflow), construit le site et le publie sur GitHub Pages à chaque push sur `main`, chaque lundi et à la demande. Si la collecte échoue, le site est construit avec les dernières données versionnées.
Une seule fois, dans **Settings → Pages**, choisir **Source : GitHub Actions**.

Le cahier des charges est dans [docs/cahier-des-charges.md](docs/cahier-des-charges.md).
