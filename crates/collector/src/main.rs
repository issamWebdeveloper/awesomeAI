//! Collecteur : lit la liste awesome-selfhosted-ai, enrichit chaque outil (README, API GitHub
//! si un jeton est fourni), applique le vocabulaire contrôlé et les corrections manuelles,
//! puis écrit `tools.json`, `categories.json`, `stats.json` et `meta.json`.
//!
//! Usage : `cargo run -p collector --release -- [options]` depuis la racine du dépôt.
//! Variables : `GITHUB_TOKEN` (facultatif) active les appels à l'API GitHub.

use awesome_core::analyze::{compute_stats, days_from_civil, summarize_categories};
use awesome_core::model::{GithubRepo, Overrides};
use awesome_core::taxonomy::Taxonomy;
use awesome_core::{build_tools, source, BuildInput, Extra};
use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, fs, thread};

const SOURCE: &str = "https://raw.githubusercontent.com/glincker/awesome-selfhosted-ai/main";

struct Args {
    projects: String,
    live: String,
    taxonomy: PathBuf,
    overrides: PathBuf,
    snapshots: PathBuf,
    out: Vec<PathBuf>,
    readmes: bool,
    threads: usize,
}

fn parse_args() -> Args {
    let mut a = Args {
        projects: format!("{SOURCE}/data/projects.json"),
        live: format!("{SOURCE}/LIVE.md"),
        taxonomy: "data/taxonomy.yaml".into(),
        overrides: "data/overrides".into(),
        snapshots: "data/snapshots".into(),
        out: vec!["web/src/lib/data".into(), "web/static/data".into()],
        readmes: true,
        threads: 16,
    };
    let mut custom_out = false;
    let mut it = env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut val = || {
            it.next()
                .unwrap_or_else(|| die(&format!("valeur manquante pour {arg}")))
        };
        match arg.as_str() {
            "--projects" => a.projects = val(),
            "--live" => a.live = val(),
            "--taxonomy" => a.taxonomy = val().into(),
            "--overrides" => a.overrides = val().into(),
            "--snapshots" => a.snapshots = val().into(),
            "--out" => {
                if !custom_out {
                    a.out.clear();
                    custom_out = true;
                }
                a.out.push(val().into());
            }
            "--threads" => a.threads = val().parse().unwrap_or(16),
            "--no-readme" => a.readmes = false,
            "-h" | "--help" => {
                println!(
                    "collector [--projects URL|FICHIER] [--live URL|FICHIER] [--taxonomy F] [--overrides DIR]\n          [--snapshots DIR] [--out DIR]... [--threads N] [--no-readme]"
                );
                std::process::exit(0);
            }
            other => die(&format!("option inconnue : {other}")),
        }
    }
    a
}

fn die(msg: &str) -> ! {
    eprintln!("erreur : {msg}");
    std::process::exit(1);
}

fn agent() -> ureq::Agent {
    use ureq::tls::{RootCerts, TlsConfig};
    ureq::Agent::config_builder()
        .tls_config(
            TlsConfig::builder()
                .root_certs(RootCerts::PlatformVerifier)
                .build(),
        )
        .timeout_global(Some(Duration::from_secs(30)))
        .user_agent("awesome-ai-collector")
        .http_status_as_error(false)
        .build()
        .into()
}

fn get_text(agent: &ureq::Agent, url: &str, token: Option<&str>) -> Result<String, String> {
    let mut req = agent.get(url);
    if let Some(t) = token {
        req = req
            .header("Authorization", &format!("Bearer {t}"))
            .header("Accept", "application/vnd.github+json");
    }
    let mut res = req.call().map_err(|e| format!("{url} : {e}"))?;
    if !res.status().is_success() {
        return Err(format!("{url} : HTTP {}", res.status()));
    }
    res.body_mut()
        .with_config()
        .limit(5 * 1024 * 1024)
        .read_to_string()
        .map_err(|e| format!("{url} : {e}"))
}

fn load(agent: &ureq::Agent, src: &str) -> String {
    let r = if src.starts_with("http://") || src.starts_with("https://") {
        get_text(agent, src, None)
    } else {
        fs::read_to_string(src).map_err(|e| format!("{src} : {e}"))
    };
    r.unwrap_or_else(|e| die(&e))
}

fn fetch_extra(agent: &ureq::Agent, repo: &str, token: Option<&str>, readmes: bool) -> Extra {
    let mut extra = Extra::default();
    if readmes {
        for name in [
            "README.md",
            "readme.md",
            "README.MD",
            "Readme.md",
            "README.rst",
            "README",
        ] {
            if let Ok(t) = get_text(
                agent,
                &format!("https://raw.githubusercontent.com/{repo}/HEAD/{name}"),
                None,
            ) {
                extra.readme = t.chars().take(200_000).collect();
                break;
            }
        }
    }
    if let Some(t) = token {
        match get_text(
            agent,
            &format!("https://api.github.com/repos/{repo}"),
            Some(t),
        ) {
            Ok(body) => match serde_json::from_str::<GithubRepo>(&body) {
                Ok(g) => extra.github = Some(g),
                Err(e) => eprintln!("avertissement : API GitHub {repo} : {e}"),
            },
            Err(e) => eprintln!("avertissement : {e}"),
        }
    }
    extra
}

fn fetch_all(
    agent: &ureq::Agent,
    repos: Vec<String>,
    token: Option<String>,
    readmes: bool,
    threads: usize,
) -> HashMap<String, Extra> {
    let queue = Arc::new(Mutex::new(repos));
    let out = Arc::new(Mutex::new(HashMap::new()));
    let handles: Vec<_> = (0..threads.max(1))
        .map(|_| {
            let (queue, out, agent, token) =
                (queue.clone(), out.clone(), agent.clone(), token.clone());
            thread::spawn(move || loop {
                let Some(repo) = queue.lock().unwrap().pop() else {
                    break;
                };
                let extra = fetch_extra(&agent, &repo, token.as_deref(), readmes);
                out.lock().unwrap().insert(repo.to_lowercase(), extra);
            })
        })
        .collect();
    for h in handles {
        let _ = h.join();
    }
    Arc::try_unwrap(out).ok().unwrap().into_inner().unwrap()
}

fn load_overrides(dir: &Path) -> Overrides {
    let mut all = Overrides::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return all;
    };
    let mut files: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    files.sort();
    for f in files {
        if !matches!(f.extension().and_then(|e| e.to_str()), Some("yaml" | "yml")) {
            continue;
        }
        let text =
            fs::read_to_string(&f).unwrap_or_else(|e| die(&format!("{} : {e}", f.display())));
        let map: Overrides =
            serde_yaml::from_str(&text).unwrap_or_else(|e| die(&format!("{} : {e}", f.display())));
        all.extend(map);
    }
    all
}

fn today() -> String {
    if let Ok(d) = env::var("COLLECT_DATE") {
        return d;
    }
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    awesome_core::analyze::civil_from_days((secs / 86_400) as i64)
}

fn write_json(dirs: &[PathBuf], name: &str, value: &serde_json::Value) {
    let text = serde_json::to_string(value).expect("sérialisation JSON");
    for d in dirs {
        fs::create_dir_all(d).unwrap_or_else(|e| die(&format!("{} : {e}", d.display())));
        let path = d.join(name);
        fs::write(&path, &text).unwrap_or_else(|e| die(&format!("{} : {e}", path.display())));
    }
}

fn main() {
    let args = parse_args();
    let agent = agent();
    let today = today();

    let tax_text = fs::read_to_string(&args.taxonomy)
        .unwrap_or_else(|e| die(&format!("{} : {e}", args.taxonomy.display())));
    let tax = Taxonomy::from_yaml(&tax_text).unwrap_or_else(|e| die(&format!("taxonomie : {e}")));
    tax.validate().unwrap_or_else(|e| die(&e));

    let projects = source::parse_projects(&load(&agent, &args.projects))
        .unwrap_or_else(|e| die(&format!("projects.json : {e}")));
    let live_md = load(&agent, &args.live);
    let live = source::parse_live(&live_md);
    let source_refreshed = live_md
        .lines()
        .find_map(|l| l.split("Refreshed ").nth(1))
        .map(|s| s.chars().take(10).collect::<String>())
        .filter(|s| source::is_date(s));
    if projects.len() < 50 {
        die(&format!(
            "seulement {} outils dans la source : données conservées",
            projects.len()
        ));
    }
    eprintln!("{} outils, {} lignes LIVE.md", projects.len(), live.len());

    let token = env::var("GITHUB_TOKEN").ok().filter(|t| !t.is_empty());
    eprintln!(
        "enrichissement : README {}, API GitHub {}",
        if args.readmes { "oui" } else { "non" },
        if token.is_some() {
            "oui"
        } else {
            "non (pas de GITHUB_TOKEN)"
        }
    );
    let extras = if args.readmes || token.is_some() {
        fetch_all(
            &agent,
            projects.iter().map(|p| p.repo.clone()).collect(),
            token,
            args.readmes,
            args.threads,
        )
    } else {
        HashMap::new()
    };

    let overrides = load_overrides(&args.overrides);
    let first_seen_path = args.snapshots.join("first-seen.json");
    let mut first_seen: BTreeMap<String, String> = fs::read_to_string(&first_seen_path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default();

    let tools = build_tools(&BuildInput {
        taxonomy: &tax,
        projects: &projects,
        live: &live,
        extras: &extras,
        overrides: &overrides,
        first_seen: &first_seen,
        today: &today,
    })
    .unwrap_or_else(|e| die(&e));

    for t in &tools {
        first_seen
            .entry(t.id.clone())
            .or_insert_with(|| today.clone());
    }
    fs::create_dir_all(&args.snapshots).ok();
    fs::write(
        &first_seen_path,
        serde_json::to_string_pretty(&first_seen).unwrap() + "\n",
    )
    .unwrap_or_else(|e| die(&format!("{} : {e}", first_seen_path.display())));

    let categories = summarize_categories(&tax, &tools);
    let stats = compute_stats(&tax, &tools, &today, source_refreshed);
    let _ = days_from_civil(&today).unwrap_or_else(|| die(&format!("date invalide : {today}")));

    write_json(
        &args.out,
        "tools.json",
        &serde_json::to_value(&tools).unwrap(),
    );
    write_json(
        &args.out,
        "categories.json",
        &serde_json::to_value(&categories).unwrap(),
    );
    write_json(
        &args.out,
        "stats.json",
        &serde_json::to_value(&stats).unwrap(),
    );
    write_json(
        &args.out,
        "meta.json",
        &json!({ "features": tax.features, "facets": tax.facets, "openai_compatible": tax.openai_compatible }),
    );
    eprintln!(
        "écrit : {} outils, {} catégories, {} corrections manuelles → {}",
        tools.len(),
        categories.len(),
        overrides.len(),
        args.out
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}
