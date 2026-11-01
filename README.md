# Awesome AI

Tableau de bord et comparateur des outils d'IA auto-hébergeables listés dans [awesome-selfhosted-ai](https://github.com/glincker/awesome-selfhosted-ai), publié sur GitHub Pages.

## Développement

```sh
cd web
npm install
npm run dev      # http://localhost:5173
npm run build    # site statique dans web/build
```

## Déploiement

Chaque push sur `main` construit le site et le publie sur GitHub Pages via `.github/workflows/deploy.yml`.
Une seule fois, dans **Settings → Pages**, choisir **Source : GitHub Actions**.

Adresse : https://issamwebdeveloper.github.io/awesomeAI/

Le cahier des charges est dans [docs/cahier-des-charges.md](docs/cahier-des-charges.md).
