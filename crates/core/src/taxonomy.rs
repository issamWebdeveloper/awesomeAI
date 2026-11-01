//! Vocabulaire contrôlé (`data/taxonomy.yaml`) et classification par règles.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Taxonomy {
    pub categories: Vec<CategoryDef>,
    /// Fonctionnalités, référencées par clé depuis chaque catégorie.
    pub features: BTreeMap<String, Term>,
    /// Facettes transverses : `deployment`, `interfaces`, `hardware`.
    pub facets: BTreeMap<String, BTreeMap<String, Term>>,
    pub openai_compatible: Term,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryDef {
    pub slug: String,
    /// Nom dans la liste source (anglais).
    pub name: String,
    /// Nom affiché (français).
    pub label: String,
    pub color: String,
    pub description: String,
    /// Objectif générique des outils de la catégorie, utilisé si la fiche n'a pas d'objectif relu.
    pub goal: String,
    /// Problème générique résolu par la catégorie.
    pub problem: String,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Term {
    pub label: String,
    #[serde(default, skip_serializing)]
    pub keywords: Vec<String>,
    /// Si faux, seuls le nom, la description et les topics sont examinés (pas le README).
    #[serde(default = "yes", skip_serializing)]
    pub readme: bool,
}

fn yes() -> bool {
    true
}

impl Taxonomy {
    pub fn from_yaml(s: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(s)
    }

    pub fn category_by_name(&self, name: &str) -> Option<&CategoryDef> {
        self.categories
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(name))
    }

    /// Vérifie que chaque fonctionnalité citée par une catégorie est définie.
    pub fn validate(&self) -> Result<(), String> {
        for c in &self.categories {
            for f in &c.features {
                if !self.features.contains_key(f) {
                    return Err(format!(
                        "catégorie {} : fonctionnalité inconnue « {f} »",
                        c.slug
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Texte normalisé dans lequel on cherche des mots-clés.
pub struct Corpus {
    /// Nom, description, topics.
    pub strong: String,
    /// Début du README (pour les fonctionnalités).
    pub readme_head: String,
    /// README complet (pour déploiement, interfaces, matériel).
    pub readme: String,
}

impl Corpus {
    pub fn new(strong: &str, readme: &str) -> Self {
        let readme = normalize(readme);
        let head: String = readme.chars().take(8000).collect();
        Corpus {
            strong: normalize(strong),
            readme_head: head,
            readme,
        }
    }
}

pub fn normalize(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push(' ');
    for c in s.chars() {
        let c = c.to_ascii_lowercase();
        out.push(if c == '_' || c == '\n' || c == '\t' {
            ' '
        } else {
            c
        });
    }
    out.push(' ');
    out
}

/// Cherche `kw` comme mot entier (les caractères voisins ne sont pas alphanumériques).
pub fn contains_word(hay: &str, kw: &str) -> bool {
    let kw = kw.to_ascii_lowercase();
    let bytes = hay.as_bytes();
    let mut start = 0;
    while let Some(pos) = hay[start..].find(&kw) {
        let i = start + pos;
        let j = i + kw.len();
        let before = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
        let after = j >= bytes.len() || !bytes[j].is_ascii_alphanumeric();
        if before && after {
            return true;
        }
        start = i + 1;
        while !hay.is_char_boundary(start) {
            start += 1;
        }
    }
    false
}

fn term_matches(term: &Term, corpus: &Corpus, readme_text: &str) -> bool {
    term.keywords
        .iter()
        .any(|k| contains_word(&corpus.strong, k) || (term.readme && contains_word(readme_text, k)))
}

pub struct Classification {
    pub features: Vec<String>,
    pub deployment: Vec<String>,
    pub interfaces: Vec<String>,
    pub hardware: Vec<String>,
    pub openai_compatible: bool,
}

pub fn classify(tax: &Taxonomy, category: &CategoryDef, corpus: &Corpus) -> Classification {
    let features = category
        .features
        .iter()
        .filter(|k| term_matches(&tax.features[*k], corpus, &corpus.readme_head))
        .cloned()
        .collect();
    let facet = |name: &str| -> Vec<String> {
        tax.facets
            .get(name)
            .map(|terms| {
                terms
                    .iter()
                    .filter(|(_, t)| term_matches(t, corpus, &corpus.readme))
                    .map(|(k, _)| k.clone())
                    .collect()
            })
            .unwrap_or_default()
    };
    Classification {
        features,
        deployment: facet("deployment"),
        interfaces: facet("interfaces"),
        hardware: facet("hardware"),
        openai_compatible: term_matches(&tax.openai_compatible, corpus, &corpus.readme),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_boundaries() {
        let h = normalize("Runs on CUDA, ROCm and Apple-Silicon. multi-agent teams");
        assert!(contains_word(&h, "cuda"));
        assert!(contains_word(&h, "apple-silicon"));
        assert!(contains_word(&h, "multi-agent"));
        assert!(contains_word(&h, "agent"));
        assert!(!contains_word(&h, "teams of"));
        assert!(!contains_word(&h, "roc"));
    }
}
