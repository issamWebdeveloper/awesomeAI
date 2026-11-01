// Sélection du comparateur, partagée par toutes les pages et mémorisée dans le navigateur.
import { browser } from '$app/env';
import { toolById } from './data';

export const MAX_COMPARE = 4;
const KEY = 'awesome-ai:compare';

function load(): string[] {
  if (!browser) return [];
  try {
    const v = JSON.parse(localStorage.getItem(KEY) ?? '[]');
    return Array.isArray(v) ? v.filter((id) => toolById.has(id)).slice(0, MAX_COMPARE) : [];
  } catch {
    return [];
  }
}

class CompareSelection {
  ids = $state<string[]>(load());

  has(id: string) {
    return this.ids.includes(id);
  }

  get full() {
    return this.ids.length >= MAX_COMPARE;
  }

  toggle(id: string) {
    if (this.has(id)) this.remove(id);
    else if (!this.full) this.set([...this.ids, id]);
  }

  remove(id: string) {
    this.set(this.ids.filter((x) => x !== id));
  }

  set(ids: string[]) {
    this.ids = [...new Set(ids)].filter((id) => toolById.has(id)).slice(0, MAX_COMPARE);
    if (browser) {
      try {
        localStorage.setItem(KEY, JSON.stringify(this.ids));
      } catch {
        /* stockage indisponible : la sélection reste en mémoire */
      }
    }
  }

  clear() {
    this.set([]);
  }
}

export const compare = new CompareSelection();
