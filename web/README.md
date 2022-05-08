
## Web app 

This web app is the React-TypeScrit front-end that uses the Rust [lib](../lib) and its [wasm bindings](../lib-wasm) in this repo to resolve the [Gagne Ton Papa](https://www.gigamic.com/jeu/gagne-ton-papa) game.

## Scripts

First, build the Rust binary in `../lib-wasm`:

### `wasm-pack build --target web`

Then, in the project directory, you can run:

### `npm start`

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.\
You will also see any lint errors in the console.

On every change on the Rust side, you need to rebuild the lib-wasm library and relauch `npm start`.

### `npm test`

Launches the test runner in the interactive watch mode.\
See the section about [running tests](https://facebook.github.io/create-react-app/docs/running-tests) for more information.

### `npm run build`

Builds the app for production to the `build` folder.\
It correctly bundles React in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.\
Your app is ready to be deployed!

See the section about [deployment](https://facebook.github.io/create-react-app/docs/deployment) for more information.

## Manu's notes

This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app).
https://create-react-app.dev/
`npx create-react-app app-name --template typescript`


https://www.pluralsight.com/guides/render-a-react-component-with-state-and-props-using-typescript
https://dev.to/krzysztofkaczy9/webassembly-rust-typescript-project-setup-4gio


TS + rust
https://github.com/aeroxy/react-typescript-webassembly-starter
https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

