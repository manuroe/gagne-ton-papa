## Title
Performance Optimizations: Bitboard Solver & WASM Build

## Body
This PR introduces significant performance improvements to the core solver and the WASM build configuration.

**Key Changes:**

*   **Core Solver (`lib`)**:
    *   Replaced the matrix-based collision detection with a **Bitboard (u64)** implementation.
    *   Optimized position generation to use bitwise operations.
    *   Preserved color information in the final output while using bitboards for logic.
    *   **Performance**: `resolve_specific_game` benchmark improved from **~148ms** to **~0.87ms** (approx. **170x speedup**).

*   **WASM (`lib-wasm`)**:
    *   Enabled `wasm-opt = true` in `Cargo.toml`.
    *   Added a `[profile.release]` with `lto = true`, `codegen-units = 1`, and `opt-level = 3`.
    *   **Result**: WASM binary size reduced from **~177KB** to **126KB** (~29% reduction).

*   **CI/CD (`.github/workflows`)**:
    *   Updated workflows to use `jetli/wasm-pack-action` for faster and more reliable builds.
    *   Updated Node.js version to `20.x` in CI.
    *   Updated `actions/checkout` to `v3`.

*   **Documentation**:
    *   Updated `README.md` to reflect Node.js v18+ requirement.

## Prompts Used
- As an algorithm expert, improve the performance of `resolve_specific_game`. You can change any logic but keep the high level API. Game rules have not changed.
- now you looked at the existing algo, can we use another technique to resolve this game?
- ok let's try the bit‑board implementation
- check the perf before pushing anything
- `cargo run` on the root of the project now returns black rectangles as results
- We use the wasm version of the rust lib, is there anything to do to improve the final overall performance in the web app?
- Make sure the instructions in the doc and the GH actions to build the wasm package in the best way are correct
