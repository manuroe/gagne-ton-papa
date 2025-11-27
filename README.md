# Gagne Ton Papa! Solver

A Rust + React/TypeScript project to find solutions for the game [GAGNE TON PAPA!](https://www.gigamic.com/jeu/gagne-ton-papa) ("Win Your Daddy").

This project implements a solver for the puzzle game where players must fit wooden pieces of different shapes (pentaminos) into a specific rectangular area.

## Features

- **Core Logic**: Efficient solver written in Rust.
- **Web Interface**: Interactive web application built with React and TypeScript.
- **WASM Integration**: Runs the Rust solver directly in the browser using WebAssembly.
- **CLI Tool**: Terminal-based application for testing and running the solver locally.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v14+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer.html)

### Quick Start

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/manuroe/gagne-ton-papa.git
    cd gagne-ton-papa
    ```

2.  **Run the CLI app:**
    ```bash
    cargo run
    ```

3.  **Run the Web App:**
    First, build the WASM library:
    ```bash
    cd lib-wasm
    wasm-pack build --target web
    ```
    Then, start the web server:
    ```bash
    cd ../web
    npm install
    npm start
    ```

## Project Structure

The repository is organized into the following components:

- **[lib](./lib)**: The core Rust library containing the game logic and solver algorithms.
- **[src](./src)**: A CLI application to interact with the library from the terminal.
- **[lib-wasm](./lib-wasm)**: WASM bindings for the core library, enabling usage in web applications.
- **[web](./web)**: The web frontend application built with React and TypeScript.

## Live Demo

Check out the live web application here: [https://manuroe.github.io/gagne-ton-papa/](https://manuroe.github.io/gagne-ton-papa/)
