// Vitest benchmark for game resolution using CodSpeed
import { bench } from 'vitest';
import init, * as gtpLib from 'lib-wasm';

// IDs of the specific pieces matching the Rust benchmark
const PIECE_IDS = Uint32Array.from([0, 4, 5, 6, 9, 15, 16]); // RedSquare1, BrownL3, OrangeBar3, PinkBar4, YellowZigZag4, PinkNotSquare5, YellowU5

async function runResolution(): Promise<number> {
    // Ensure the WASM module is initialized
    await init();
    // Create a game with all pieces, then select the subset we need
    const allGame = gtpLib.JSGame.game_with_all_pieces();
    const game = gtpLib.JSGame.game_from_game(allGame, PIECE_IDS);
    const start = performance.now();
    // Resolve the game (this is the heavy computation)
    game.resolve();
    const duration = performance.now() - start;
    return duration;
}

// @ts-ignore: CodSpeed expects a numeric return value
bench('resolve_specific_game', async () => {
    const ms = await runResolution();
    return ms;
});
