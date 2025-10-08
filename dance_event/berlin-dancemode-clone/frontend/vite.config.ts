import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  server: {
    port: 5000,
    host: true
  },
  build: {
    outDir: 'dist'
  },
  publicDir: 'static'
});
