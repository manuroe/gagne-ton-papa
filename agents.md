# Project Context & Standards

This file documents the standards, conventions, and context for the `gagne-ton-papa` project. AI agents and developers should refer to this to maintain consistency.

## Project Structure
- **`lib/`**: Core game logic implemented in pure Rust.
  - **Focus**: Performance, correctness, idiomatic Rust.
  - **Testing**: High unit test coverage required.
- **`lib-wasm/`**: WASM bindings for the library.
  - **Focus**: Exposing `lib` functionality to the web.
- **`web/`**: React/TypeScript frontend application.
  - **Focus**: User experience, visualization, responsiveness.

## Development Standards

### Pull Requests
- **Prompts Sharing**: All PR descriptions **MUST** include a section sharing the prompts used to generate the code. This is critical for knowledge sharing and reproducibility.
- **Description Editing**: Always offer the user the option to edit the PR description before finalizing or sending it.

### Model Interaction
- **Quota Limits**: If the model hits a quota limit (e.g., "resume" prompts), ignore the prompt to resume and offer alternative actions (like editing the PR description) instead of trying to continue blindly.

### Rust (lib & lib-wasm)
- **Code Style**:
  - Follow idiomatic Rust patterns.
  - Ensure code is Clippy-compliant (no `#[allow(clippy::...)]` unless strictly necessary).
  - Avoid unreadable literals; use named constants or clear formatting.
- **Testing**:
  - Write unit tests for all new logic in `lib`.
  - Ensure `cargo test` passes before committing.

### Web (React/TypeScript)
- **UI/UX Guidelines**:
  - **Child-Friendly**: Design for children (vibrant colors, clear typography, intuitive interactions).
  - **Responsive**: Ensure the game is playable on mobile and tablet devices.
  - **Visuals**:
    - Use clear borders for game pieces.
    - Ensure pieces are distinct and easy to distinguish.
    - Use "premium" aesthetics (gradients, rounded corners, smooth animations).
- **Tech Stack**:
  - React 18+
  - TypeScript
  - CSS Modules or Vanilla CSS (avoid adding heavy UI frameworks like Tailwind unless requested).

## Workflows

### Build Core Library
```bash
cd lib
cargo build
cargo test
```

### Build WASM
```bash
cd lib-wasm
wasm-pack build --target web
# Note: Target 'web' is often preferred for direct browser usage, but check project specific setup.
```

### Run Web App
```bash
cd web
npm start
```
