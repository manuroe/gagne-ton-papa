import { defineConfig } from 'vite'
import path from 'node:path'
import react from '@vitejs/plugin-react'
import type { UserConfig } from 'vite'

// https://vitejs.dev/config/
const config: UserConfig = {
  plugins: [react()],
  // No fs.allow relaxation; WASM can be copied into public for dev/preview
}

export default defineConfig(({ mode }) => {
  return {
    ...config,
    // Use base path only in production for GitHub Pages
    base: mode === 'production' ? '/gagne-ton-papa' : '/',
    test: {
      globals: true,
      environment: 'jsdom',
      setupFiles: './src/setupTests.ts',
    },
  }
})