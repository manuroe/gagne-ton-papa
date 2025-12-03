import { defineConfig } from 'vite'
import path from 'node:path'
import react from '@vitejs/plugin-react'
import type { UserConfig } from 'vite'

// https://vitejs.dev/config/
const config: UserConfig = {
  base: '/gagne-ton-papa',
  plugins: [react()],
  // No fs.allow relaxation; WASM can be copied into public for dev/preview
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