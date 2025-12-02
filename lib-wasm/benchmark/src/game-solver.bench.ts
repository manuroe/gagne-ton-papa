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

  bench("resolve_specific_game_wasm", () => {
    // Create the same game as in the Rust benchmark:
    // 7 pieces (RedSquare1, BrownL3, OrangeBar3, PinkBar4, 
    // YellowZigZag4, PinkNotSquare5, YellowU5) on a 5-column board
    // These correspond to piece IDs: 0, 4, 5, 6, 9, 15, 16 in game_with_all_pieces
    // (RedSquare1, BrownL3, OrangeBar3, PinkBar4, YellowZigZag4, PinkNotSquare5, YellowU5)
    const allPiecesGame = gtpLib.JSGame.game_with_all_pieces();
    const game = gtpLib.JSGame.game_from_game(
      allPiecesGame,
      Uint32Array.from([0, 4, 5, 6, 9, 15, 16])
    );
    
    const solutions = game.resolve();
    
    // Verify we got solutions (same assertion as Rust benchmark)
    if (solutions.length === 0) {
      throw new Error("Expected solutions but got none");
    }
  });


  bench('resolve_count_only', () => {
    const allPiecesGame = gtpLib.JSGame.game_with_all_pieces();
    const game = gtpLib.JSGame.game_from_game(
      allPiecesGame,
      Uint32Array.from([0, 4, 5, 6, 9, 15, 16])
    );
    
    // Result pagination is not yet implemented. This benchmark is about getting the initial metrics
    const solutions_count = game.resolve_count();

        // Verify we got solutions (same assertion as Rust benchmark)
    if (solutions_count === 0) {
      throw new Error("Expected solutions but got none");
    }
  });

  bench('resolve_and_render_first_page', () => {
    const allPiecesGame = gtpLib.JSGame.game_with_all_pieces();
    const game = gtpLib.JSGame.game_from_game(
      allPiecesGame,
      Uint32Array.from([0, 4, 5, 6, 9, 15, 16])
    );
    
    // Result pagination is not yet implemented. This benchmark is about getting the initial metrics
    const solutions = game.resolve();

    // Accumulate total SVG length to ensure the SVG content is actually loaded
    // and not optimized away by the JavaScript engine
    let totalSvgLength = 0;
    for (const result of solutions) {
        totalSvgLength += result.svg.length;
    }
    // Validate to detect unexpected empty SVG content and prevent dead code elimination
    if (totalSvgLength === 0 && solutions.length > 0) {
        throw new Error('Unexpected empty SVG content');
    }
  });
});
