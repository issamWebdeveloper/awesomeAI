// Données générées par le collecteur Rust (crates/collector). Ne pas modifier à la main :
// relancer `cargo run -p collector --release` depuis la racine du dépôt.
import toolsJson from './data/tools.json';
import categoriesJson from './data/categories.json';
import statsJson from './data/stats.json';
import metaJson from './data/meta.json';

export interface Tool {
  id: string;
  name: string;
  repo: string;
  repo_slug: string;
  category: string;
  tags: string[];
  summary: string;
  goal: string | null;
  problem_solved: string | null;
  features: string[];
  deployment: string[];
  interfaces: string[];
  hardware: string[];
  openai_compatible: boolean;
  language: string | null;
  topics: string[];
  homepage: string | null;
  license: string | null;
  license_note: string | null;
  license_family: string;
  stars: number | null;
  last_push: string | null;
  status: string;
  created_at: string | null;
  open_issues: number | null;
  first_seen: string;
  updated_at: string;
  reviewed: string[];
}

export interface Share {
  key: string;
  count: number;
  share: number;
}

export interface Category {
  slug: string;
  name: string;
  label: string;
  color: string;
  description: string;
  goal: string;
  problem: string;
  count: number;
  stars: number;
  active_share: number;
  tools: string[];
  features: Share[];
  common: Share[];
  distinctive: { key: string; tools: string[] }[];
  languages: Share[];
  licenses: Share[];
  facets: Share[];
}

export interface Stats {
  updated_at: string;
  source_refreshed: string | null;
  tools: number;
  categories: number;
  stars: number;
  active: number;
  active_share: number;
  rising: number;
  source_available: number;
  license_families: Share[];
  licenses: Share[];
  languages: Share[];
  freshness: Share[];
  heatmap_keys: string[];
  heatmap: { category: string; key: string; count: number; share: number }[];
}

type Labels = Record<string, { label: string }>;

export const tools = toolsJson as Tool[];
export const categories = categoriesJson as Category[];
export const stats = statsJson as Stats;
const meta = metaJson as unknown as {
  features: Labels;
  facets: Record<string, Labels>;
  openai_compatible: { label: string };
};

export const toolById = new Map(tools.map((t) => [t.id, t]));
export const categoryBySlug = new Map(categories.map((c) => [c.slug, c]));

export const FACETS = ['deployment', 'interfaces', 'hardware'] as const;
export type Facet = (typeof FACETS)[number];

export const FACET_LABELS: Record<Facet, string> = {
  deployment: 'Déploiement',
  interfaces: 'Interfaces',
  hardware: 'Matériel'
};

export function featureLabel(key: string): string {
  return meta.features[key]?.label ?? key;
}

export function facetLabel(facet: string, key: string): string {
  return meta.facets[facet]?.[key]?.label ?? key;
}

export function facetKeys(facet: Facet): string[] {
  return Object.keys(meta.facets[facet] ?? {});
}

/** Libellé d'une clé transverse (`deployment:docker`, `openai_compatible`…). */
export function capabilityLabel(key: string): string {
  if (key === 'openai_compatible') return meta.openai_compatible.label;
  const [facet, k] = key.split(':');
  return facetLabel(facet, k);
}

export function toolHas(t: Tool, key: string): boolean {
  if (key === 'openai_compatible') return t.openai_compatible;
  const [facet, k] = key.split(':') as [Facet, string];
  return (t[facet] as string[] | undefined)?.includes(k) ?? false;
}

export const LICENSE_FAMILIES: Record<string, string> = {
  permissive: 'Permissive',
  copyleft: 'Copyleft',
  'source-available': 'Source disponible',
  unknown: 'Autre / inconnue'
};

export const STATUS_LABELS: Record<string, string> = {
  active: 'Actif',
  slow: 'Ralenti',
  stale: 'Inactif',
  unknown: 'Inconnu'
};

export const TAG_LABELS: Record<string, string> = {
  rising: 'Rising',
  'source-available': 'Source disponible',
  archived: 'Archivé'
};

export const FRESHNESS_LABELS: Record<string, string> = {
  '0-7': 'Moins d’une semaine',
  '8-30': '8 à 30 jours',
  '31-90': '1 à 3 mois',
  '91-365': '3 à 12 mois',
  '365+': 'Plus d’un an'
};

const nf = new Intl.NumberFormat('fr-FR');
const cf = new Intl.NumberFormat('fr-FR', { notation: 'compact', maximumFractionDigits: 1 });
const df = new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'long', year: 'numeric', timeZone: 'UTC' });

export const fmt = (n: number | null | undefined) => (n == null ? '—' : nf.format(n));
export const compact = (n: number | null | undefined) => (n == null ? '—' : cf.format(n));
export const pct = (x: number) => `${Math.round(x * 100)} %`;
export const fmtDate = (d: string | null | undefined) => (d ? df.format(new Date(d + 'T00:00:00Z')) : '—');

/** Jours écoulés depuis une date AAAA-MM-JJ, par rapport à la date de collecte. */
export function daysSince(d: string | null): number | null {
  if (!d) return null;
  return Math.round((Date.parse(stats.updated_at) - Date.parse(d)) / 86_400_000);
}

export function relativeDays(d: string | null): string {
  const n = daysSince(d);
  if (n == null) return '—';
  if (n <= 0) return 'aujourd’hui';
  if (n === 1) return 'hier';
  if (n < 30) return `il y a ${n} jours`;
  if (n < 365) return `il y a ${Math.round(n / 30)} mois`;
  const y = Math.round(n / 365);
  return `il y a ${y} an${y > 1 ? 's' : ''}`;
}

/** Retire accents et casse pour la recherche. */
export function normalize(s: string): string {
  return s
    .normalize('NFD')
    .replace(/[̀-ͯ]/g, '')
    .toLowerCase();
}

const searchIndex = new Map(
  tools.map((t) => {
    const cat = categoryBySlug.get(t.category);
    const text = [
      t.name,
      t.summary,
      t.goal ?? '',
      t.problem_solved ?? '',
      t.language ?? '',
      t.license ?? '',
      cat?.label ?? '',
      cat?.name ?? '',
      ...t.topics,
      ...t.features.map(featureLabel),
      ...t.features,
      ...FACETS.flatMap((f) => t[f].map((k) => facetLabel(f, k)))
    ].join(' ');
    return [t.id, { name: normalize(t.name), text: normalize(text) }];
  })
);

/** Score de pertinence (0 = pas de correspondance). Tous les mots doivent être trouvés. */
export function searchScore(t: Tool, query: string): number {
  const words = normalize(query).split(/\s+/).filter(Boolean);
  if (!words.length) return 1;
  const idx = searchIndex.get(t.id)!;
  let score = 0;
  for (const w of words) {
    if (idx.name === w) score += 10;
    else if (idx.name.startsWith(w)) score += 6;
    else if (idx.name.includes(w)) score += 4;
    else if (idx.text.includes(w)) score += 1;
    else return 0;
  }
  return score;
}

/** Outils de la même catégorie qui partagent le plus de fonctionnalités. */
export function similarTools(t: Tool, n = 4): Tool[] {
  return tools
    .filter((o) => o.id !== t.id && o.category === t.category)
    .map((o) => ({
      o,
      s:
        o.features.filter((f) => t.features.includes(f)).length * 2 +
        o.interfaces.filter((f) => t.interfaces.includes(f)).length +
        o.deployment.filter((f) => t.deployment.includes(f)).length
    }))
    .sort((a, b) => b.s - a.s || (b.o.stars ?? 0) - (a.o.stars ?? 0))
    .slice(0, n)
    .map((x) => x.o);
}
