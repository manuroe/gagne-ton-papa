# Build WASM Composite Action

This reusable composite action installs Rust and `wasm-pack`, restores cache for the WASM build outputs, builds the `lib-wasm` package, and uploads the built `pkg` as an artifact.

## Inputs / Env
- `WASM_ARTIFACT_NAME` (env): Artifact name to use when uploading. Set at the workflow or job level, e.g.

```yaml
env:
  WASM_ARTIFACT_NAME: lib-wasm-pkg
```

## Usage

```yaml
jobs:
  build_wasm:
    runs-on: ubuntu-latest
    env:
      WASM_ARTIFACT_NAME: lib-wasm-pkg
    steps:
      - uses: actions/checkout@v4
      - uses: ./.github/actions/build-wasm
```

Then in downstream jobs:

```yaml
  web_job:
    needs: build_wasm
    steps:
      - uses: actions/checkout@v4
      - uses: actions/download-artifact@v4
        with:
          name: lib-wasm-pkg
          path: lib-wasm/pkg
```

## Caching
- Uses `actions/cache@v4` for `lib-wasm/pkg` and `lib-wasm/target` with keys based on OS, `lib-wasm/Cargo.lock` and `lib-wasm/src/**`.

## Notes
- Assumes the repository structure contains `lib-wasm/` at the root.
- The artifact contains the built `pkg` directory which includes JS bindings and the `.wasm` binary for `--target web`.
