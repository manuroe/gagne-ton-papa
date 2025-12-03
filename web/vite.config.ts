import { defineConfig } from 'vite'
import path from 'node:path'
import react from '@vitejs/plugin-react'
import type { UserConfig } from 'vite'

// https://vitejs.dev/config/
const config: UserConfig = {
  base: '/gagne-ton-papa',
  plugins: [react()],
  server: {
    fs: {
      // Allow serving files from the monorepo root so lib-wasm/pkg works in dev
      allow: [path.resolve(__dirname, '..'), path.resolve(__dirname)],
    },
  },
}

export default defineConfig(({ mode }) => {
  return {
    ...config,
    test: {
      globals: true,
      environment: 'jsdom',
      setupFiles: './src/setupTests.ts',
    },
  }
})