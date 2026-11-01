// Définition des critères du comparateur, groupés comme dans le cahier des charges (§6.6).
import {
  categoryBySlug, facetKeys, facetLabel, featureLabel, fmt, fmtDate, LICENSE_FAMILIES, relativeDays,
  STATUS_LABELS, TAG_LABELS, type Facet, type Tool
} from '#lib/data.js';

export type Cell =
  | { kind: 'bool'; value: boolean }
  | { kind: 'text'; value: string; sort?: number | string | null };

export interface Criterion {
  id: string;
  group: GroupId;
  label: string;
  /** Valeur affichée pour un outil. */
  get: (t: Tool) => Cell;
  /** Le plus grand gagne (étoiles, date de push). */
  best?: 'max';
}

export type GroupId = 'general' | 'features' | 'deployment' | 'compat' | 'community' | 'license';

export const GROUPS: { id: GroupId; label: string }[] = [
  { id: 'general', label: 'Généralités' },
  { id: 'features', label: 'Fonctionnalités' },
  { id: 'deployment', label: 'Déploiement' },
  { id: 'compat', label: 'Compatibilité' },
  { id: 'community', label: 'Communauté et activité' },
  { id: 'license', label: 'Licence' }
];

const text = (value: string, sort?: number | string | null): Cell => ({ kind: 'text', value, sort });
const bool = (value: boolean): Cell => ({ kind: 'bool', value });

function facetCriteria(facet: Facet, group: GroupId, prefix: string): Criterion[] {
  return facetKeys(facet).map((k) => ({
    id: `${prefix}:${k}`,
    group,
    label: facetLabel(facet, k),
    get: (t: Tool) => bool(t[facet].includes(k))
  }));
}

/** Critères disponibles pour une sélection d'outils (les fonctionnalités dépendent des catégories). */
export function buildCriteria(selected: Tool[]): Criterion[] {
  const featureKeys: string[] = [];
  for (const t of selected) {
    for (const s of categoryBySlug.get(t.category)?.features ?? []) {
      if (!featureKeys.includes(s.key)) featureKeys.push(s.key);
    }
  }
  return [
    { id: 'gen:category', group: 'general', label: 'Catégorie', get: (t) => text(categoryBySlug.get(t.category)?.label ?? t.category) },
    { id: 'gen:summary', group: 'general', label: 'Résumé', get: (t) => text(t.summary) },
    { id: 'gen:goal', group: 'general', label: 'Objectif', get: (t) => text(t.goal ?? categoryBySlug.get(t.category)?.goal ?? '—') },
    { id: 'gen:problem', group: 'general', label: 'Problème résolu', get: (t) => text(t.problem_solved ?? categoryBySlug.get(t.category)?.problem ?? '—') },
    { id: 'gen:tags', group: 'general', label: 'Étiquettes', get: (t) => text(t.tags.map((g) => TAG_LABELS[g] ?? g).join(', ') || '—') },
    { id: 'gen:language', group: 'general', label: 'Langage', get: (t) => text(t.language ?? '—') },

    ...featureKeys.map((k) => ({
      id: `feat:${k}`,
      group: 'features' as const,
      label: featureLabel(k),
      get: (t: Tool) => bool(t.features.includes(k))
    })),

    ...facetCriteria('deployment', 'deployment', 'dep'),

    { id: 'openai', group: 'compat', label: 'API compatible OpenAI', get: (t) => bool(t.openai_compatible) },
    ...facetCriteria('interfaces', 'compat', 'iface'),
    ...facetCriteria('hardware', 'compat', 'hw'),

    { id: 'com:stars', group: 'community', label: 'Étoiles GitHub', best: 'max', get: (t) => text(fmt(t.stars), t.stars) },
    { id: 'com:last_push', group: 'community', label: 'Dernier push', best: 'max', get: (t) => text(t.last_push ? `${fmtDate(t.last_push)} (${relativeDays(t.last_push)})` : '—', t.last_push) },
    { id: 'com:status', group: 'community', label: 'Activité', get: (t) => text(STATUS_LABELS[t.status] ?? t.status) },
    { id: 'com:issues', group: 'community', label: 'Issues ouvertes', get: (t) => text(fmt(t.open_issues)) },
    { id: 'com:created', group: 'community', label: 'Création du dépôt', get: (t) => text(fmtDate(t.created_at)) },
    { id: 'com:first_seen', group: 'community', label: 'Entrée dans la liste', get: (t) => text(fmtDate(t.first_seen)) },

    { id: 'lic:license', group: 'license', label: 'Licence', get: (t) => text(t.license ?? '—') },
    { id: 'lic:family', group: 'license', label: 'Famille', get: (t) => text(LICENSE_FAMILIES[t.license_family] ?? t.license_family) },
    { id: 'lic:note', group: 'license', label: 'Conditions particulières', get: (t) => text(t.license_note ?? '—') }
  ];
}

export const PRESETS = {
  essentiel: {
    label: 'Essentiel',
    includes: (c: Criterion) =>
      ['gen:category', 'gen:summary', 'gen:goal', 'com:stars', 'com:last_push', 'com:status', 'lic:license', 'lic:family', 'openai', 'dep:docker', 'iface:web-ui', 'iface:rest-api'].includes(c.id) ||
      c.group === 'features'
  },
  technique: {
    label: 'Technique',
    includes: (c: Criterion) =>
      ['features', 'deployment', 'compat'].includes(c.group) || ['gen:language', 'lic:license'].includes(c.id)
  },
  tout: { label: 'Tout', includes: () => true }
} as const;

export type PresetId = keyof typeof PRESETS;

export function cellKey(c: Cell): string {
  return c.kind === 'bool' ? String(c.value) : c.value;
}
