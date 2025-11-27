# Gagne Ton Papa! Web App

This directory contains the React/TypeScript frontend for the [Gagne Ton Papa!](https://www.gigamic.com/jeu/gagne-ton-papa) solver. It uses the Rust library via WASM bindings to run the solver logic directly in the browser.

## Setup

Before running the web application, you must build the WASM library.

1.  **Build WASM Library:**
    Navigate to the `lib-wasm` directory and build the package:
    ```bash
    cd ../lib-wasm
    wasm-pack build --target web
    ```

2.  **Install Dependencies:**
    Install the Node.js dependencies for the web app:
    ```bash
    npm install
    ```

## Available Scripts

In the project directory, you can run:

### `npm start`

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.\
You will also see any lint errors in the console.

> **Note:** If you make changes to the Rust code, you must rebuild the WASM library (`wasm-pack build --target web` in `lib-wasm`) and then restart the development server.

### `npm test`

Launches the test runner in the interactive watch mode.

### `npm run build`

Builds the app for production to the `build` folder.\
It correctly bundles React in production mode and optimizes the build for the best performance.

## References

- [Create React App Documentation](https://facebook.github.io/create-react-app/docs/getting-started)
- [Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
- [wasm-bindgen Documentation](https://rustwasm.github.io/docs/wasm-bindgen/)
