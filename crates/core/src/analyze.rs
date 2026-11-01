//! Synthèse : familles de licences, statut, points communs par catégorie, indicateurs globaux.

use crate::model::Tool;
use crate::taxonomy::Taxonomy;
use serde::Serialize;
use std::collections::BTreeMap;

const PERMISSIVE: &[&str] = &[
    "MIT",
    "APACHE-2.0",
    "BSD-2-CLAUSE",
    "BSD-3-CLAUSE",
    "BSD-3",
    "ISC",
    "UNLICENSE",
    "0BSD",
    "ZLIB",
    "CC0-1.0",
    "BSL-1.0",
    "PSF-2.0",
    "MIT-0",
    "UPL-1.0",
    "ARTISTIC-2.0",
];
const COPYLEFT: &[&str] = &[
    "GPL-2.0",
    "GPL-3.0",
    "AGPL-3.0",
    "LGPL-2.1",
    "LGPL-3.0",
    "MPL-2.0",
    "EUPL-1.2",
    "GPL-2.0-ONLY",
    "GPL-3.0-ONLY",
    "AGPL-3.0-ONLY",
    "GPL-3.0-OR-LATER",
    "AGPL-3.0-OR-LATER",
    "LGPL-3.0-ONLY",
    "EPL-2.0",
    "CDDL-1.0",
];

/// `permissive`, `copyleft`, `source-available` ou `unknown`.
pub fn license_family(license: Option<&str>, source_available: bool) -> String {
    if source_available {
        return "source-available".into();
    }
    let Some(l) = license else {
        return "unknown".into();
    };
    let u = l.trim().to_ascii_uppercase();
    if PERMISSIVE.contains(&u.as_str()) {
        "permissive".into()
    } else if COPYLEFT.contains(&u.as_str()) || u.contains("GPL") {
        "copyleft".into()
    } else {
        "unknown".into()
    }
}

/// Nombre de jours depuis 1970-01-01 pour une date `AAAA-MM-JJ` (algorithme de H. Hinnant).
pub fn days_from_civil(date: &str) -> Option<i64> {
    let y: i64 = date.get(0..4)?.parse().ok()?;
    let m: i64 = date.get(5..7)?.parse().ok()?;
    let d: i64 = date.get(8..10)?.parse().ok()?;
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146_097 + doe - 719_468)
}

pub fn civil_from_days(z: i64) -> String {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

pub fn days_between(from: &str, to: &str) -> Option<i64> {
    Some(days_from_civil(to)? - days_from_civil(from)?)
}

/// `active` (push depuis 90 jours), `slow` (365 jours) ou `stale`.
pub fn status_from_push(last_push: Option<&str>, today: &str) -> String {
    match last_push.and_then(|d| days_between(d, today)) {
        Some(n) if n <= 90 => "active".into(),
        Some(n) if n <= 365 => "slow".into(),
        Some(_) => "stale".into(),
        None => "unknown".into(),
    }
}

#[derive(Debug, Serialize)]
pub struct Share {
    pub key: String,
    pub count: usize,
    pub share: f64,
}

#[derive(Debug, Serialize)]
pub struct Distinctive {
    pub key: String,
    pub tools: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CategorySummary {
    pub slug: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub description: String,
    pub goal: String,
    pub problem: String,
    pub count: usize,
    pub stars: u64,
    pub active_share: f64,
    /// Identifiants triés par étoiles décroissantes.
    pub tools: Vec<String>,
    /// Toutes les fonctionnalités du vocabulaire de la catégorie, avec leur fréquence.
    pub features: Vec<Share>,
    /// Fonctionnalités présentes chez au moins la moitié des outils.
    pub common: Vec<Share>,
    /// Fonctionnalités rares (au plus 25 % des outils, au moins un).
    pub distinctive: Vec<Distinctive>,
    pub languages: Vec<Share>,
    pub licenses: Vec<Share>,
    /// Facettes transverses les plus fréquentes (déploiement, interfaces).
    pub facets: Vec<Share>,
}

fn ratio(n: usize, d: usize) -> f64 {
    if d == 0 {
        0.0
    } else {
        ((n as f64 / d as f64) * 1000.0).round() / 1000.0
    }
}

fn shares<'a>(items: impl Iterator<Item = &'a str>, total: usize, limit: usize) -> Vec<Share> {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for k in items {
        *counts.entry(k).or_default() += 1;
    }
    let mut v: Vec<Share> = counts
        .into_iter()
        .map(|(k, c)| Share {
            key: k.to_string(),
            count: c,
            share: ratio(c, total),
        })
        .collect();
    v.sort_by(|a, b| b.count.cmp(&a.count).then(a.key.cmp(&b.key)));
    v.truncate(limit);
    v
}

pub fn summarize_categories(tax: &Taxonomy, tools: &[Tool]) -> Vec<CategorySummary> {
    tax.categories
        .iter()
        .map(|c| {
            let mut members: Vec<&Tool> = tools.iter().filter(|t| t.category == c.slug).collect();
            members.sort_by(|a, b| b.stars.cmp(&a.stars).then(a.name.cmp(&b.name)));
            let n = members.len();
            let features: Vec<Share> = c
                .features
                .iter()
                .map(|f| {
                    let count = members.iter().filter(|t| t.features.contains(f)).count();
                    Share {
                        key: f.clone(),
                        count,
                        share: ratio(count, n),
                    }
                })
                .collect();
            let mut common: Vec<Share> = features
                .iter()
                .filter(|s| s.share >= 0.5)
                .map(|s| Share {
                    key: s.key.clone(),
                    count: s.count,
                    share: s.share,
                })
                .collect();
            common.sort_by(|a, b| b.count.cmp(&a.count));
            let distinctive = features
                .iter()
                .filter(|s| s.count > 0 && s.share <= 0.25)
                .map(|s| Distinctive {
                    key: s.key.clone(),
                    tools: members
                        .iter()
                        .filter(|t| t.features.contains(&s.key))
                        .map(|t| t.id.clone())
                        .collect(),
                })
                .collect();
            let facet_keys = members.iter().flat_map(|t| {
                t.deployment
                    .iter()
                    .map(|k| format!("deployment:{k}"))
                    .chain(t.interfaces.iter().map(|k| format!("interfaces:{k}")))
                    .collect::<Vec<_>>()
            });
            let facet_keys: Vec<String> = facet_keys.collect();
            CategorySummary {
                slug: c.slug.clone(),
                name: c.name.clone(),
                label: c.label.clone(),
                color: c.color.clone(),
                description: c.description.clone(),
                goal: c.goal.clone(),
                problem: c.problem.clone(),
                count: n,
                stars: members.iter().filter_map(|t| t.stars).sum(),
                active_share: ratio(members.iter().filter(|t| t.status == "active").count(), n),
                tools: members.iter().map(|t| t.id.clone()).collect(),
                features,
                common,
                distinctive,
                languages: shares(members.iter().filter_map(|t| t.language.as_deref()), n, 6),
                licenses: shares(members.iter().map(|t| t.license_family.as_str()), n, 4),
                facets: shares(facet_keys.iter().map(String::as_str), n, 8),
            }
        })
        .collect()
}

#[derive(Debug, Serialize)]
pub struct HeatCell {
    pub category: String,
    pub key: String,
    pub count: usize,
    pub share: f64,
}

#[derive(Debug, Serialize)]
pub struct Stats {
    pub updated_at: String,
    pub source_refreshed: Option<String>,
    pub tools: usize,
    pub categories: usize,
    pub stars: u64,
    pub active: usize,
    pub active_share: f64,
    pub rising: usize,
    pub source_available: usize,
    pub license_families: Vec<Share>,
    pub licenses: Vec<Share>,
    pub languages: Vec<Share>,
    /// Tranches d'ancienneté du dernier push : `0-7`, `8-30`, `31-90`, `91-365`, `365+`.
    pub freshness: Vec<Share>,
    pub heatmap_keys: Vec<String>,
    pub heatmap: Vec<HeatCell>,
}

/// Clés transverses de la matrice catégories × capacités.
pub const HEATMAP_KEYS: &[&str] = &[
    "deployment:docker",
    "deployment:pip",
    "deployment:npm",
    "deployment:binary",
    "interfaces:web-ui",
    "interfaces:cli",
    "interfaces:rest-api",
    "interfaces:python-sdk",
    "interfaces:mcp",
    "openai_compatible",
    "hardware:nvidia",
    "hardware:apple-silicon",
];

pub fn tool_has(t: &Tool, key: &str) -> bool {
    match key.split_once(':') {
        Some(("deployment", k)) => t.deployment.iter().any(|v| v == k),
        Some(("interfaces", k)) => t.interfaces.iter().any(|v| v == k),
        Some(("hardware", k)) => t.hardware.iter().any(|v| v == k),
        _ if key == "openai_compatible" => t.openai_compatible,
        _ => false,
    }
}

pub fn compute_stats(
    tax: &Taxonomy,
    tools: &[Tool],
    today: &str,
    source_refreshed: Option<String>,
) -> Stats {
    let n = tools.len();
    let active = tools.iter().filter(|t| t.status == "active").count();
    let buckets = ["0-7", "8-30", "31-90", "91-365", "365+"];
    let mut fresh = [0usize; 5];
    for t in tools {
        if let Some(d) = t.last_push.as_deref().and_then(|d| days_between(d, today)) {
            let i = match d {
                ..=7 => 0,
                8..=30 => 1,
                31..=90 => 2,
                91..=365 => 3,
                _ => 4,
            };
            fresh[i] += 1;
        }
    }
    let mut heatmap = Vec::new();
    for c in &tax.categories {
        let members: Vec<&Tool> = tools.iter().filter(|t| t.category == c.slug).collect();
        for k in HEATMAP_KEYS {
            let count = members.iter().filter(|t| tool_has(t, k)).count();
            heatmap.push(HeatCell {
                category: c.slug.clone(),
                key: k.to_string(),
                count,
                share: ratio(count, members.len()),
            });
        }
    }
    Stats {
        updated_at: today.to_string(),
        source_refreshed,
        tools: n,
        categories: tax.categories.len(),
        stars: tools.iter().filter_map(|t| t.stars).sum(),
        active,
        active_share: ratio(active, n),
        rising: tools
            .iter()
            .filter(|t| t.tags.iter().any(|g| g == "rising"))
            .count(),
        source_available: tools
            .iter()
            .filter(|t| t.license_family == "source-available")
            .count(),
        license_families: shares(tools.iter().map(|t| t.license_family.as_str()), n, 10),
        licenses: shares(tools.iter().filter_map(|t| t.license.as_deref()), n, 12),
        languages: shares(tools.iter().filter_map(|t| t.language.as_deref()), n, 12),
        freshness: buckets
            .iter()
            .zip(fresh)
            .map(|(k, c)| Share {
                key: k.to_string(),
                count: c,
                share: ratio(c, n),
            })
            .collect(),
        heatmap_keys: HEATMAP_KEYS.iter().map(|s| s.to_string()).collect(),
        heatmap,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_roundtrip() {
        for d in ["1970-01-01", "2026-11-01", "2000-02-29", "2024-12-31"] {
            assert_eq!(civil_from_days(days_from_civil(d).unwrap()), d);
        }
        assert_eq!(days_between("2026-10-01", "2026-11-01"), Some(31));
    }

    #[test]
    fn families() {
        assert_eq!(license_family(Some("MIT"), false), "permissive");
        assert_eq!(license_family(Some("AGPL-3.0"), false), "copyleft");
        assert_eq!(license_family(Some("MIT"), true), "source-available");
        assert_eq!(license_family(None, false), "unknown");
    }

    #[test]
    fn status() {
        assert_eq!(status_from_push(Some("2026-10-01"), "2026-11-01"), "active");
        assert_eq!(status_from_push(Some("2026-01-01"), "2026-11-01"), "slow");
        assert_eq!(status_from_push(Some("2024-01-01"), "2026-11-01"), "stale");
    }
}
