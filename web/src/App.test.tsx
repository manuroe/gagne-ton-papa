import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';
import * as gtpLib from 'lib-wasm';
import './i18n/i18n';

test('renders app title', () => {
  render(<App allPiecesGame={gtpLib.JSGame.game_with_all_pieces()}/>);
  const titleElement = screen.getByText(/GAGNE TON PAPA !/i);
  expect(titleElement).toBeInTheDocument();
});
