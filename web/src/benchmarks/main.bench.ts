import { bench } from 'vitest';
import init, * as gtpLib from 'lib-wasm';

import fs from 'fs';
import path from 'path';

// Load WASM file manually for Node environment
const wasmPath = path.resolve(__dirname, '../../../lib-wasm/pkg/lib_wasm_bg.wasm');
const wasmBuffer = fs.readFileSync(wasmPath);
await init(wasmBuffer);

// Setup the game configuration
const allGame = gtpLib.JSGame.game_with_all_pieces();
const PIECE_IDS = Uint32Array.from([0, 4, 5, 6, 9, 15, 16]); // RedSquare1, BrownL3, OrangeBar3, PinkBar4, YellowZigZag4, PinkNotSquare5, YellowU5
const game = gtpLib.JSGame.game_from_game(allGame, PIECE_IDS);

bench('resolve_full_results', () => {
    const results = game.resolve();
    // Clean up WASM memory for the returned objects
    results.forEach(m => m.free());
});

bench('resolve_count_only', () => {
    game.resolve_count();
});

bench('resolve_and_render_first_page', () => {
    // Result pagination is not yet implemented. This benchmark is about getting the initial metrics
    const results = game.resolve();

    // Note: The SVG property is not used in the benchmark, but it is computed
    for (let result of results) {
        const _svg = result.svg;
    }
    // Clean up
    results.forEach(m => m.free());
});
