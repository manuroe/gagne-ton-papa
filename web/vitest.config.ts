import { defineConfig } from 'vitest/config';
import path from 'path';

export default defineConfig(async () => {
    const { default: codspeedPlugin } = await import('@codspeed/vitest-plugin');
    return {
        plugins: [codspeedPlugin()],
        test: {
            environment: 'node',
        },
        resolve: {
            alias: {
                'lib-wasm': path.resolve(__dirname, '../lib-wasm/pkg'),
            },
        },
    };
});
