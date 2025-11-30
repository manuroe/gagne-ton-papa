// Bitboard utilities for Gagne Ton Papa solver
// Supports boards up to 11x11 (128 bits) which is sufficient for current puzzles.
// The board is stored as a u128 where bit i corresponds to cell (row, col)
// i = row * board_cols + col (row‑major order).

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
            mat[(r, c)] = 1; // we only need a non‑zero marker; original color is stored elsewhere
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
