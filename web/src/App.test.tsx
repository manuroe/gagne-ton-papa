import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import App from './App';
import './i18n/i18n';

// No WASM in tests: provide a minimal stub for the game prop

it('renders app title', () => {
  const mockGame = { pieces: [] } as any;
  render(<App allPiecesGame={mockGame} />);
  const titleElement = screen.getByText(/GAGNE TON PAPA !/i);
  expect(titleElement).toBeInTheDocument();
});
