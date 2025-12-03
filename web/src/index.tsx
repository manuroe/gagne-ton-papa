import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import './i18n/i18n';
import App from './App';
import reportWebVitals from './reportWebVitals';

import init, * as gtpLib from 'lib-wasm';
// Import the WASM file as a Vite asset - Vite will handle the path automatically
import wasmUrl from 'lib-wasm/lib_wasm_bg.wasm?url';

init(wasmUrl).then(() => {
  const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
  );
  root.render(
    <React.StrictMode>
      <App allPiecesGame={gtpLib.JSGame.game_with_all_pieces()} />
    </React.StrictMode>
  );
});

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
