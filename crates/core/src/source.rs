//! Parsing des fichiers de la liste source.

use crate::model::{LiveRow, SourceProject};
use std::collections::HashMap;

pub fn parse_projects(json: &str) -> Result<Vec<SourceProject>, serde_json::Error> {
    serde_json::from_str(json)
}

/// Convertit « 22k », « 2.3k », « 1.2M » ou « 450 » en entier.
pub fn parse_stars(s: &str) -> Option<u64> {
    let s = s.trim().replace(',', "");
    if s.is_empty() {
        return None;
    }
    let (num, mult) = match s.chars().last()? {
        'k' | 'K' => (&s[..s.len() - 1], 1_000.0),
        'm' | 'M' => (&s[..s.len() - 1], 1_000_000.0),
        _ => (s.as_str(), 1.0),
    };
    let v: f64 = num.parse().ok()?;
    Some((v * mult).round() as u64)
}

/// Extrait `owner/nom` d'une URL GitHub.
pub fn repo_slug(url: &str) -> Option<String> {
    let rest = url.split("github.com/").nth(1)?;
    let mut parts = rest.trim_end_matches('/').split('/');
    let owner = parts.next()?;
    let name = parts.next()?;
    Some(format!("{owner}/{name}").to_lowercase())
}

/// Lit les lignes de `LIVE.md` : `| <img> | [Nom](url) | 22k | 2026-10-31 | MIT | ![active](…) |`.
/// Les clés sont les `owner/nom` en minuscules.
pub fn parse_live(md: &str) -> HashMap<String, LiveRow> {
    let mut out = HashMap::new();
    for line in md.lines() {
        let line = line.trim();
        if !line.starts_with('|') || !line.contains("](https://github.com/") {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() < 6 {
            continue;
        }
        let Some(url) = cells[1].split("](").nth(1).map(|s| s.trim_end_matches(')')) else {
            continue;
        };
        let Some(repo) = repo_slug(url) else { continue };
        let status = cells[5]
            .strip_prefix("![")
            .and_then(|s| s.split(']').next())
            .map(str::to_string);
        let row = LiveRow {
            repo: repo.clone(),
            stars: parse_stars(cells[2]),
            last_push: Some(cells[3].to_string()).filter(|d| is_date(d)),
            license: Some(cells[4].to_string())
                .filter(|l| !l.is_empty() && l != "-" && l != "NOASSERTION" && l != "Other"),
            status,
        };
        out.entry(repo).or_insert(row);
    }
    out
}

pub fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && s.chars().filter(char::is_ascii_digit).count() == 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stars() {
        assert_eq!(parse_stars("22k"), Some(22_000));
        assert_eq!(parse_stars("2.3k"), Some(2_300));
        assert_eq!(parse_stars("1.2M"), Some(1_200_000));
        assert_eq!(parse_stars("450"), Some(450));
        assert_eq!(parse_stars(""), None);
    }

    #[test]
    fn live_row() {
        let md = "| <img src=\"x\" alt=\"\"> | [CrewAI](https://github.com/crewAIInc/crewAI) | 60k | 2026-10-30 | MIT | ![active](https://img.shields.io/badge/active-brightgreen) |";
        let rows = parse_live(md);
        let r = &rows["crewaiinc/crewai"];
        assert_eq!(r.stars, Some(60_000));
        assert_eq!(r.last_push.as_deref(), Some("2026-10-30"));
        assert_eq!(r.license.as_deref(), Some("MIT"));
        assert_eq!(r.status.as_deref(), Some("active"));
    }
}
