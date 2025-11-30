# AI Agent Guidelines & Project Context

> **SYSTEM INSTRUCTION**: All AI agents (Gemini, Claude, ChatGPT, etc.) working on this project **MUST** read and follow these guidelines. This file is the source of truth for project standards, workflows, and context.

## 1. Project Overview
**Name**: `gagne-ton-papa`
**Description**: A solver for the board game "Gagne Ton Papa" (also known as Katamino).
**Architecture**:
- **`lib/`** (Rust): Core solver logic, pure Rust, high performance.
- **`src/`** (Rust): TUI application to quickly check the solver's output.
- **`lib-wasm/`** (Rust): WASM bindings exposing `lib` to the web.
- **`web/`** (React/TypeScript): Frontend application.

## 2. Core Rules & Behavior

### General
- **Source of Truth**: This file (`agents.md`) defines the standards. If you are unsure, check here first.
- **Continuous Improvement**: **CRITICAL**. You must amend this file (`agents.md`) if you discover new patterns, corrections, or user preferences during your interaction. Capture this knowledge for future agents.

### Pull Requests (PRs)
- **Workflow**:
    1.  **Draft**: Populate a new file in `.prs/` (e.g., `.prs/my_feature.md`) with the Title, Body, and **Prompts Used**, using `docs/PR_DESCRIPTION_TEMPLATE.md` as a base.
    2.  **Review**: Ask the user to review/edit this file.
    3.  **Send**: Only after user approval, use the content of this file to create the PR.
- **Prompts Sharing**: You **MUST** document the prompts that generated the code in the PR description. Skip resuming requests after a model hits a quota limit.

## 3. Tech Stack & Standards

### Rust (`lib`, `lib-wasm`, `src`)
- **Style**: Idiomatic Rust. No `unwrap()` in production code (use `expect` with context or proper error handling).
- **Linting**: Strict Clippy compliance. No `#[allow(clippy::...)]` without a very strong reason.
- **Testing**: High coverage for `lib`. Run `cargo test` before submitting.

### Web (`web`)
- **Framework**: React 18+, TypeScript.
- **Styling**: CSS Modules or Vanilla CSS. **NO** Tailwind unless explicitly requested.
- **UX/UI**:
    - **Target Audience**: Children (vibrant, clear, intuitive).
    - **Responsiveness**: Mobile-first or fully responsive.
    - **Visuals**: Distinct borders for game pieces. Premium feel (smooth animations).

## 4. Workflows

### Build & Test
- **Core**: `cd lib && cargo build && cargo test`
- **WASM**: `cd lib-wasm && wasm-pack build --target web`
- **Web**: `cd web && npm start`

### Common Tasks
- **New Feature**: Plan -> Implement in `lib` -> Expose in `lib-wasm` -> UI in `web`.
- **Refactor**: Ensure tests pass at every step.

### CI Notes
- When the web app depends on `lib-wasm` via a local file reference (`lib-wasm": "file:../lib-wasm/pkg"`), ensure CI downloads/builds the `lib-wasm/pkg` artifact before running `npm ci` in `web/`. Otherwise, npm cannot resolve the local dependency and the job fails.

