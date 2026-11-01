import { error } from '@sveltejs/kit';
import { toolById, tools } from '#lib/data.js';
import type { EntryGenerator, PageLoad } from './$types';

export const entries: EntryGenerator = () => tools.map((t) => ({ id: t.id }));

export const load: PageLoad = ({ params }) => {
  const tool = toolById.get(params.id);
  if (!tool) error(404, 'Outil introuvable');
  return { tool };
};
