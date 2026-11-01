import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    sveltekit({
      adapter: adapter({ fallback: '404.html' }),
      paths: {
        // GitHub Pages sert le site sous /awesomeAI ; vide en développement.
        base: /** @type {'' | `/${string}`} */ (process.env.BASE_PATH ?? '')
      }
    })
  ]
});
