# Solutions for GAGNE TON PAPA!

This is a rust + react-ts project to find solutions for the game [GAGNE TON PAPA!](https://www.gigamic.com/jeu/gagne-ton-papa) (= WIN YOUR DADDY in English).

This is a brick game for 2 players with wood pieces of different sizes and shapes. The winner is the quickest who places all the seleceted pieces in a rectangle with a size of 3x5, 4x5 or 5x5 depending on the pieces.

This project finds all possible solutions from a set of pieces. The actual goal is to play with Rust, Wasm and React-Typescript.

The web app is live here: https://manuroe.github.io/gagne-ton-papa/.

The repository is organised as the following (this is also the technical stack):
 - [lib](./lib): the core rust library
 - [src](.src): a sample rust terminal app to play with the lib in Rust
 - [lib-wasm](.lib-wasm): wasm bindings using [wasm_bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
 - [web](./web): the web front-end based on react and typescript
