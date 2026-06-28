import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import tailwindcss from '@tailwindcss/vite'
import vueDevTools from 'vite-plugin-vue-devtools'
import Components from 'unplugin-vue-components/vite';
import {PrimeVueResolver} from '@primevue/auto-import-resolver';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    tailwindcss(),
    vue(),
    vueJsx(),
    vueDevTools(),
    Components({
      resolvers: [
        PrimeVueResolver()
      ]
    })
  ],
  server: {
    proxy: {
      // Any request starting with /api will be forwarded to your Rust backend
      '/api': {
        target: 'http://127.0.0.1:6969',
        changeOrigin: true,
        // (Optional) secure: false if you are using https locally with self-signed certs
      }
    }
  },
  resolve: {
    alias: {
      // Use a more robust path resolution for Windows
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  // This helps Vite 8 ignore the dependency scanning logic that's crashing
  optimizeDeps: {
    entries: ['./index.html']
  }
})
