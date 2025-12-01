import { bench, describe, beforeAll } from "vitest";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import init, * as gtpLib from "lib-wasm";

describe("game solver", () => {
  beforeAll(async () => {
    // Initialize the WASM module before running benchmarks
    // In Node.js environment, we need to load the WASM file manually
    const wasmPath = join(process.cwd(), "node_modules/lib-wasm/lib_wasm_bg.wasm");
    const wasmBuffer = await readFile(wasmPath);
    await init(wasmBuffer);
  });

  bench("resolve_specific_game_ts", () => {
    // Create the same game as in the Rust benchmark:
    // 7 pieces (RedSquare1, BrownL3, OrangeBar3, PinkBar4, 
    // YellowZigZag4, PinkNotSquare5, YellowU5) on a 5-column board
    // These correspond to piece IDs: 0, 4, 5, 6, 9, 15, 16 in game_with_all_pieces
    // (RedSquare1, BrownL3, OrangeBar3, PinkBar4, YellowZigZag4, PinkNotSquare5, YellowU5)
    const allPiecesGame = gtpLib.JSGame.game_with_all_pieces();
    const game = gtpLib.JSGame.game_from_game(
      allPiecesGame,
      Uint32Array.from([0, 4, 5, 6, 9, 15])
    );
    
    const solutions = game.resolve();
    
    // Verify we got solutions (same assertion as Rust benchmark)
    if (solutions.length === 0) {
      throw new Error("Expected solutions but got none");
    }
  });
});
