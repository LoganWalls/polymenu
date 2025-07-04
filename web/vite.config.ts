import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'

const devUrl = 'http://localhost:4000';

// https://vite.dev/config/
export default defineConfig({
  server: {
    proxy: {
      "/options": devUrl,
      "/command": devUrl,
      "/print": devUrl,
      "/close": devUrl,
    }
  },
  plugins: [svelte(), tailwindcss()],
})
