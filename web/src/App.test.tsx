import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';
import * as gtpLib from 'lib-wasm'; 

test('renders learn react link', () => {
  render(<App allPiecesGame={gtpLib.JSGame.game_with_all_pieces()}/>);
  const linkElement = screen.getByText(/learn react/i);
  expect(linkElement).toBeInTheDocument();
});
