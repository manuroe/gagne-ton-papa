# Gagne Ton Papa! Core Library

This crate (`gtp-lib`) contains the core logic for the Gagne Ton Papa solver. It is a pure Rust library that implements the game rules, data structures, and the solving algorithm.

## Features

- **Game Data**: Definitions for pieces (pentaminos) and the game board.
- **Solver**: A recursive backtracking algorithm to find solutions.
- **Platform Agnostic**: Designed to be used by both the CLI application and the WASM bindings for the web app.

## Usage

Add this crate as a dependency in your `Cargo.toml`:

```toml
[dependencies]
gtp-lib = { path = "../lib" }
```

(Note: This is currently part of a workspace and intended for internal use within this repository.)
