import React from 'react'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { vi, it, expect } from 'vitest'
import './i18n/i18n'

vi.mock('lib-wasm', () => {
  return {
    JSGame: {
      game_with_all_pieces: () => ({
        pieces: [
          { id: 1, cells: 10, matrix: { svg: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10" fill="black"/></svg>', width: 2 } },
        ],
      }),
      game_from_game: (_game: any, ids: Uint32Array) => {
        const hasSelection = ids && ids.length > 0
        return {
          is_valid: () => false,
          missing_cells: () => (hasSelection ? 5 : 0),
          resolve: () => [],
        }
      },
    },
  }
})

// Import after mocking
import App from './App'
import * as libWasm from 'lib-wasm'

it('shows missing cells after selecting a piece', async () => {
  const allPiecesGame = libWasm.JSGame.game_with_all_pieces()

  render(<App allPiecesGame={allPiecesGame} />)

  const pieceContainers = document.querySelectorAll('.piece-container')
  expect(pieceContainers.length).toBeGreaterThan(0)

  await userEvent.click(pieceContainers[0] as Element)

  const solutionsArea = await screen.findByText(/5/) // count value rendered
  expect(solutionsArea).toBeInTheDocument()
})
