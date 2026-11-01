# Corrections et enrichissements manuels

Chaque fichier `.yaml` de ce dossier contient une ou plusieurs fiches, indexées par l'identifiant
de l'outil (le slug visible dans l'URL `/outils/<id>/`). Seuls les champs présents remplacent la
valeur collectée, et une valeur saisie ici n'est jamais écrasée par la collecte.

```yaml
ollama:
  goal: "Télécharger et faire tourner des LLM open source en local…"
  problem_solved: "Installer un modèle, choisir sa quantification…"
  features: [quantization, gguf, streaming]   # clés de data/taxonomy.yaml
  deployment: [docker, binary]
  interfaces: [cli, rest-api]
  hardware: [cpu, nvidia, amd, apple-silicon]
  openai_compatible: true
  # aussi possibles : summary, language, homepage, license
```

Après modification : `cargo run -p collector --release` depuis la racine, puis vérifier la fiche.
