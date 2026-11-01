import { categories, tools } from '#lib/data.js';

export const prerender = true;

const SITE = (process.env.SITE_URL ?? 'https://issamwebdeveloper.github.io') + (process.env.BASE_PATH ?? '');

export function GET() {
  const paths = [
    '/', '/dashboard/', '/outils/', '/categories/', '/comparer/', '/a-propos/',
    ...categories.map((c) => `/categories/${c.slug}/`),
    ...tools.map((t) => `/outils/${t.id}/`)
  ];
  const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${paths.map((p) => `  <url><loc>${SITE}${p}</loc></url>`).join('\n')}
</urlset>
`;
  return new Response(body, { headers: { 'Content-Type': 'application/xml' } });
}
