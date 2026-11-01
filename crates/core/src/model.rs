//! Modèle de données partagé entre le collecteur et (plus tard) une API ou un module WASM.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Une entrée de `data/projects.json` dans la liste source.
#[derive(Debug, Clone, Deserialize)]
pub struct SourceProject {
    pub name: String,
    /// `owner/nom` du dépôt GitHub.
    pub repo: String,
    pub category: String,
    pub description: String,
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub license_note: Option<String>,
    #[serde(default)]
    pub rising: Option<bool>,
}

/// Une ligne du tableau `LIVE.md`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LiveRow {
    pub repo: String,
    pub stars: Option<u64>,
    pub last_push: Option<String>,
    pub license: Option<String>,
    pub status: Option<String>,
}

/// Informations récupérées sur l'API GitHub (facultatif, nécessite un jeton).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GithubRepo {
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub pushed_at: Option<String>,
    #[serde(default)]
    pub stargazers_count: Option<u64>,
    #[serde(default)]
    pub open_issues_count: Option<u64>,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub license: Option<GithubLicense>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GithubLicense {
    #[serde(default)]
    pub spdx_id: Option<String>,
}

/// La fiche outil publiée dans `tools.json` (voir cahier des charges, §4.2).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub repo: String,
    pub repo_slug: String,
    pub category: String,
    pub tags: Vec<String>,
    pub summary: String,
    pub goal: Option<String>,
    pub problem_solved: Option<String>,
    pub features: Vec<String>,
    pub deployment: Vec<String>,
    pub interfaces: Vec<String>,
    pub hardware: Vec<String>,
    pub openai_compatible: bool,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub license_note: Option<String>,
    pub license_family: String,
    pub stars: Option<u64>,
    pub last_push: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub open_issues: Option<u64>,
    pub first_seen: String,
    pub updated_at: String,
    /// Champs saisis à la main (jamais écrasés par la collecte).
    pub reviewed: Vec<String>,
}

/// Valeurs saisies à la main dans `data/overrides/*.yaml`. Chaque fichier est une table
/// `id -> Override` ; seuls les champs présents remplacent la valeur collectée.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Override {
    pub summary: Option<String>,
    pub goal: Option<String>,
    pub problem_solved: Option<String>,
    pub features: Option<Vec<String>>,
    pub deployment: Option<Vec<String>>,
    pub interfaces: Option<Vec<String>>,
    pub hardware: Option<Vec<String>>,
    pub openai_compatible: Option<bool>,
    pub language: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
}

pub type Overrides = BTreeMap<String, Override>;

impl Tool {
    pub fn apply_override(&mut self, o: &Override) {
        macro_rules! set {
            ($field:ident) => {
                if let Some(v) = &o.$field {
                    self.$field = v.clone().into();
                    self.reviewed.push(stringify!($field).to_string());
                }
            };
        }
        set!(summary);
        set!(goal);
        set!(problem_solved);
        set!(features);
        set!(deployment);
        set!(interfaces);
        set!(hardware);
        set!(openai_compatible);
        set!(language);
        set!(homepage);
        set!(license);
    }
}
