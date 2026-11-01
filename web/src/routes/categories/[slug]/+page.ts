import { error } from '@sveltejs/kit';
import { categories, categoryBySlug } from '#lib/data.js';
import type { EntryGenerator, PageLoad } from './$types';

export const entries: EntryGenerator = () => categories.map((c) => ({ slug: c.slug }));

export const load: PageLoad = ({ params }) => {
  const category = categoryBySlug.get(params.slug);
  if (!category) error(404, 'Catégorie introuvable');
  return { category };
};
