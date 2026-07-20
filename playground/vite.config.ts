import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'

export default defineConfig({
  base: '/play/',
  plugins: [svelte()],
  build: {
    outDir: '../dist/play',
    emptyOutDir: true,
    reportCompressedSize: false,
  },
  worker: {
    format: 'es',
  },
})
