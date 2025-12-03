// Bitboard utilities for Gagne Ton Papa solver
// Supports boards up to 11x11 (128 bits) which is sufficient for current puzzles.
// The board is stored as a u128 where bit i corresponds to cell (row, col)
// i = row * board_cols + col (row-major order).

use nalgebra::DMatrix;

pub type BitBoard = u64;

/// Convert a piece matrix (with its color already multiplied) into a `BitBoard` positioned at (`offset_row`, `offset_col`).
pub fn matrix_to_bitboard(
    matrix: &DMatrix<u32>,
    _board_rows: usize,
    board_cols: usize,
    offset_row: usize,
    offset_col: usize,
) -> BitBoard {
    let mut bits: BitBoard = 0;
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            if matrix[(r, c)] != 0 {
                let board_r = offset_row + r;
                let board_c = offset_col + c;
                // safety: board dimensions are guaranteed by caller
                let idx = board_r * board_cols + board_c;
                bits |= 1u64 << idx;
            }
        }
    }
    bits
}

/// Convert a `BitBoard` back into a `DMatrix`<u32> with the given dimensions.
#[allow(dead_code)]
pub fn bitboard_to_matrix(bits: BitBoard, rows: usize, cols: usize) -> DMatrix<u32> {
    let mut mat = DMatrix::<u32>::zeros(rows, cols);
    for idx in 0..(rows * cols) {
        if (bits >> idx) & 1 == 1 {
            let r = idx / cols;
            let c = idx % cols;
            mat[(r, c)] = 1; // we only need a non-zero marker; original color is stored elsewhere
        }
    }
    mat
}

/// Generate all possible placements for a piece on a board.
/// Returns a vector of tuples `(placement_bitboard, placed_matrix)`.
/// `placed_matrix` has the piece's color applied and is padded to board size.
pub fn generate_positions(
    piece: &crate::models::Piece,
    board_rows: usize,
    board_cols: usize,
) -> Vec<(BitBoard, DMatrix<u32>)> {
    let mut positions = Vec::new();
    if piece.matrix.nrows() > board_rows || piece.matrix.ncols() > board_cols {
        return positions;
    }
    
    // Pre-multiply the piece matrix by its color once.
    let colored = &piece.matrix * piece.color;
    
    for start_row in 0..=(board_rows - piece.matrix.nrows()) {
        for start_col in 0..=(board_cols - piece.matrix.ncols()) {
            // Bitboard representation for collision detection
            let bits = matrix_to_bitboard(
                &piece.matrix,
                board_rows,
                board_cols,
                start_row,
                start_col,
            );
            
            // Full matrix representation for the final result
            let mut placed = DMatrix::zeros(board_rows, board_cols);
            for r in 0..piece.matrix.nrows() {
                for c in 0..piece.matrix.ncols() {
                    placed[(start_row + r, start_col + c)] = colored[(r, c)];
                }
            }
            
            positions.push((bits, placed));
        }
    }
    positions
}


#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;
    use crate::models::Piece;

    fn create_matrix(rows: usize, cols: usize, values: &[u32]) -> DMatrix<u32> {
        DMatrix::from_row_slice(rows, cols, values)
    }

    #[test]
    fn test_matrix_to_bitboard() {
        // 3x3 board
        // Matrix: 1x2 [1, 1] at (1, 1)
        // Board indices:
        // 0 1 2
        // 3 4 5
        // 6 7 8
        // Placed at (1, 1) occupies (1,1) -> 4 and (1,2) -> 5.
        // Bitboard should have bits 4 and 5 set.
        let matrix = create_matrix(1, 2, &[1, 1]);
        let bits = matrix_to_bitboard(&matrix, 3, 3, 1, 1);
        
        let expected = (1u64 << 4) | (1u64 << 5);
        assert_eq!(bits, expected);
    }

    #[test]
    fn test_bitboard_to_matrix() {
        // Bits 4 and 5 set on 3x3 board
        let bits = (1u64 << 4) | (1u64 << 5);
        let matrix = bitboard_to_matrix(bits, 3, 3);
        
        assert_eq!(matrix[(1, 1)], 1);
        assert_eq!(matrix[(1, 2)], 1);
        assert_eq!(matrix[(0, 0)], 0);
        assert_eq!(matrix.nrows(), 3);
        assert_eq!(matrix.ncols(), 3);
    }

    #[test]
    fn test_generate_positions() {
        // 2x2 board
        // Piece: 1x1 [1]
        // Should have 4 positions: (0,0), (0,1), (1,0), (1,1)
        let piece = Piece {
            matrix: create_matrix(1, 1, &[1]),
            color: 0xFF_0000,
            tui_color: 0,
        };
        
        let positions = generate_positions(&piece, 2, 2);
        assert_eq!(positions.len(), 4);
        
        // Check first position (0,0) -> bit 0
        let (bits, mat) = &positions[0];
        assert_eq!(*bits, 1u64 << 0);
        assert_eq!(mat[(0, 0)], 0xFF_0000);
    }

    #[test]
    fn test_generate_positions_large_piece() {
        // 2x2 board
        // Piece: 3x3 [1...] - too big
        // Should return empty
        let piece = Piece {
            matrix: create_matrix(3, 3, &[1; 9]),
            color: 0,
            tui_color: 0,
        };
        
        let positions = generate_positions(&piece, 2, 2);
        assert!(positions.is_empty());
    }
}
