import { bench } from 'vitest';
import init, * as gtpLib from 'lib-wasm';

import fs from 'fs';
import path from 'path';

// Load WASM file manually for Node environment
const wasmPath = path.resolve(__dirname, '../../../lib-wasm/pkg/lib_wasm_bg.wasm');
const wasmBuffer = fs.readFileSync(wasmPath);

// Initialize WASM before running benchmarks using an async IIFE
let game: gtpLib.JSGame;

// Use beforeAll-style initialization via an async IIFE that blocks module evaluation
const initPromise = (async () => {
    try {
        await init(wasmBuffer);
        const allGame = gtpLib.JSGame.game_with_all_pieces();
        const PIECE_IDS = Uint32Array.from([0, 4, 5, 6, 9, 15, 16]); // RedSquare1, BrownL3, OrangeBar3, PinkBar4, YellowZigZag4, PinkNotSquare5, YellowU5
        game = gtpLib.JSGame.game_from_game(allGame, PIECE_IDS);
    } catch (e) {
        console.error('Failed to initialize WASM:', e);
        throw e;
    }
})();

bench('resolve_full_results', async () => {
    await initPromise;
    const results = game.resolve();
    // Clean up WASM memory for the returned objects
    results.forEach(m => m.free());
});

bench('resolve_count_only', async () => {
    await initPromise;
    game.resolve_count();
});

bench('resolve_and_render_first_page', async () => {
    await initPromise;
    // Result pagination is not yet implemented. This benchmark is about getting the initial metrics
    const results = game.resolve();

    // Accumulate total SVG length to ensure the SVG content is actually loaded
    // and not optimized away by the JavaScript engine
    let totalSvgLength = 0;
    for (const result of results) {
        totalSvgLength += result.svg.length;
    }
    // Use the value to prevent dead code elimination
    if (totalSvgLength < 0) throw new Error('Unexpected negative SVG length');

    // Clean up
    results.forEach(m => m.free());
});
