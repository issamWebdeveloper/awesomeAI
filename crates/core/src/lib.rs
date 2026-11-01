//! Bibliothèque partagée d'Awesome AI : modèle, parsing de la liste source, classification
//! par vocabulaire contrôlé et synthèse. Le binaire `collector` s'en sert au build ; elle
//! pourra plus tard être exposée par une API (axum) ou compilée en WebAssembly.

pub mod analyze;
pub mod model;
pub mod source;
pub mod taxonomy;

use model::{GithubRepo, LiveRow, Overrides, SourceProject, Tool};
use std::collections::{BTreeMap, HashMap};
use taxonomy::{Corpus, Taxonomy};

/// Identifiant stable dérivé du nom : minuscules, ASCII, tirets.
pub fn slugify(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        let c = match c {
            'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            c => c.to_ascii_lowercase(),
        };
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_string()
}

/// Données complémentaires récupérées pour un outil.
#[derive(Default)]
pub struct Extra {
    pub readme: String,
    pub github: Option<GithubRepo>,
}

pub struct BuildInput<'a> {
    pub taxonomy: &'a Taxonomy,
    pub projects: &'a [SourceProject],
    pub live: &'a HashMap<String, LiveRow>,
    /// Clé : `owner/nom` en minuscules.
    pub extras: &'a HashMap<String, Extra>,
    pub overrides: &'a Overrides,
    /// Clé : identifiant ; valeur : date d'apparition.
    pub first_seen: &'a BTreeMap<String, String>,
    pub today: &'a str,
}

pub fn build_tools(input: &BuildInput) -> Result<Vec<Tool>, String> {
    let tax = input.taxonomy;
    let mut ids: HashMap<String, usize> = HashMap::new();
    let mut tools = Vec::with_capacity(input.projects.len());
    for p in input.projects {
        let cat = tax.category_by_name(&p.category).ok_or_else(|| {
            format!(
                "catégorie inconnue dans la source : « {} » ({})",
                p.category, p.name
            )
        })?;
        let key = p.repo.to_lowercase();
        let mut id = slugify(&p.name);
        let seen = ids.entry(id.clone()).or_default();
        *seen += 1;
        if *seen > 1 {
            id = format!("{id}-{}", seen);
        }
        let live = input.live.get(&key);
        let extra = input.extras.get(&key);
        let gh = extra.and_then(|e| e.github.as_ref());
        let source_available = p.tier.as_deref() == Some("source-available");

        let mut tags = Vec::new();
        if p.rising == Some(true) {
            tags.push("rising".to_string());
        }
        if source_available {
            tags.push("source-available".to_string());
        }
        if gh.is_some_and(|g| g.archived) {
            tags.push("archived".to_string());
        }

        let topics = gh.map(|g| g.topics.clone()).unwrap_or_default();
        let strong = format!("{} {} {}", p.name, p.description, topics.join(" "));
        let corpus = Corpus::new(&strong, extra.map(|e| e.readme.as_str()).unwrap_or(""));
        let cls = taxonomy::classify(tax, cat, &corpus);

        let license = gh
            .and_then(|g| g.license.as_ref())
            .and_then(|l| l.spdx_id.clone())
            .filter(|s| s != "NOASSERTION")
            .or_else(|| {
                live.and_then(|l| l.license.clone())
                    .filter(|_| !source_available)
            });
        let last_push = gh
            .and_then(|g| g.pushed_at.as_deref())
            .map(|d| d[..10.min(d.len())].to_string())
            .or_else(|| live.and_then(|l| l.last_push.clone()));
        let status = live
            .and_then(|l| l.status.clone())
            .unwrap_or_else(|| analyze::status_from_push(last_push.as_deref(), input.today));

        let mut tool = Tool {
            id: id.clone(),
            name: p.name.clone(),
            repo: format!("https://github.com/{}", p.repo),
            repo_slug: p.repo.clone(),
            category: cat.slug.clone(),
            tags,
            summary: p.description.clone(),
            goal: None,
            problem_solved: None,
            features: cls.features,
            deployment: cls.deployment,
            interfaces: cls.interfaces,
            hardware: cls.hardware,
            openai_compatible: cls.openai_compatible,
            language: gh.and_then(|g| g.language.clone()),
            topics,
            homepage: gh
                .and_then(|g| g.homepage.clone())
                .filter(|h| !h.trim().is_empty()),
            license_family: analyze::license_family(license.as_deref(), source_available),
            license,
            license_note: p.license_note.clone(),
            stars: gh
                .and_then(|g| g.stargazers_count)
                .or_else(|| live.and_then(|l| l.stars)),
            last_push,
            status,
            created_at: gh
                .and_then(|g| g.created_at.as_deref())
                .map(|d| d[..10.min(d.len())].to_string()),
            open_issues: gh.and_then(|g| g.open_issues_count),
            first_seen: input
                .first_seen
                .get(&id)
                .cloned()
                .unwrap_or_else(|| input.today.to_string()),
            updated_at: input.today.to_string(),
            reviewed: Vec::new(),
        };
        if let Some(o) = input.overrides.get(&id) {
            tool.apply_override(o);
        }
        tools.push(tool);
    }
    for id in input.overrides.keys() {
        if !tools.iter().any(|t| &t.id == id) {
            eprintln!("avertissement : correction pour un outil absent de la liste : {id}");
        }
    }
    tools.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(tools)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs() {
        assert_eq!(
            slugify("HuggingChat (self-hosted)"),
            "huggingchat-self-hosted"
        );
        assert_eq!(slugify("Trigger.dev"), "trigger-dev");
        assert_eq!(slugify("llama.cpp"), "llama-cpp");
    }
}
