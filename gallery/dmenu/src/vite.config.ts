import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'

const apiPort = Number(((globalThis as any).process?.env.API_PORT) ?? 7777)
const port = Number(((globalThis as any).process?.env.DEV_SERVER_PORT) ?? 5173)
const apiUrl = `http://localhost:${apiPort}`;

// https://vite.dev/config/
export default defineConfig({
  server: {
    port,
    proxy: {
      "/api": apiUrl,
      "/files": apiUrl,
      "/session": apiUrl,
    }
  },
  plugins: [svelte(), tailwindcss()],
})
