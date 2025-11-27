
use nalgebra::DMatrix;

use crate::models::{Game, Piece};
use crate::matrix_tools;

/// Trait for game puzzle solvers.
///
/// Implementors of this trait can solve puzzle games by finding all valid
/// arrangements of pieces on a game board.
pub trait GameResolverTrait {
    /// Finds all valid solutions for the given game.
    fn resolve(&self, game: &Game) -> Vec<DMatrix<u32>>;
    
    /// Generates all unique variants (rotations and reflections) of a piece.
    fn piece_variants(&self, piece: &Piece) -> Vec<Piece>;
}

/// A solver for the "Gagne Ton Papa" puzzle game.
///
/// This solver uses a backtracking algorithm to find all possible ways
/// to arrange the given pieces on the game board.
pub struct GameResolver;

impl GameResolverTrait for GameResolver {
    fn resolve(&self, game: &Game) -> Vec<DMatrix<u32>> {
        let rows = usize::try_from(game.rows()).expect("Row count too large");
        let columns = usize::try_from(game.columns).expect("Column count too large");
        let board = DMatrix::<u32>::zeros(rows, columns);

        if !game.is_valid() {
            return vec![];
        }

        // Optimization: Start placing bigger pieces first to prune the search space earlier
        let mut piece_indices: Vec<_> = (0..game.pieces.len()).collect();
        piece_indices.sort_by_key(|&i| std::cmp::Reverse(game.pieces[i].cells()));

        let mut solutions: Vec<DMatrix<u32>> = vec![board];
        for &piece_idx in &piece_indices {
            let piece = &game.pieces[piece_idx];
            let mut next_solutions: Vec<DMatrix<u32>> = vec![];
            
            for variant in self.piece_variants(piece) {
                for solution in &solutions {
                    next_solutions.extend(Self::resolve_board(solution, &variant));
                }
            }

            solutions = next_solutions;
            if solutions.is_empty() {
                break;
            }

            #[cfg(debug_assertions)]
            println!("- Found {} possible boards for piece {}", solutions.len(), piece_idx);
        }
        solutions
    }

    fn piece_variants(&self, piece: &Piece) -> Vec<Piece> {
        matrix_tools::rotation_variants(&piece.matrix)
            .iter()
            .map(|matrix| Piece {
                color: piece.color,
                matrix: matrix.clone()
            })
            .collect()
    }
}


impl GameResolver {
    /// Finds all possible boards where the piece fits in the passed board.
    /// 
    /// The algorithm:
    /// 1. Generate all possible positions for the piece on the board
    /// 2. Filter out positions that collide with existing pieces
    /// 3. Merge valid positions with the current board state
    ///
    /// Collision detection works by normalizing both matrices to binary (0/1),
    /// adding them, and checking if any cell has a value > 1.
    fn resolve_board(board: &DMatrix<u32>, piece: &Piece) -> Vec<DMatrix<u32>> {
        let mut solutions: Vec<DMatrix<u32>> = vec![];

        let normalised_board = matrix_tools::max_matrix(board, 1);

        for piece_board in Self::boards_with_piece(piece, board) {
            let normalised_piece = matrix_tools::max_matrix(&piece_board, 1);
            let normalised_merged_board = normalised_board.clone() + normalised_piece;

            if normalised_merged_board.max() == 1 {
                let merged_board = board + piece_board.clone();
                solutions.push(merged_board);
            }
        }

        solutions
    }

    /// Returns all boards where the piece can fit.
    ///
    /// Generates all possible positions for a piece on a board by:
    /// 1. Creating horizontal positions (left to right)
    /// 2. Adding vertical positions (top to bottom)
    /// 3. Padding to match board dimensions
    fn boards_with_piece(piece: &Piece, board: &DMatrix<u32>) -> Vec<DMatrix<u32>> {
        if piece.matrix.nrows() > board.nrows() {
            return Vec::new();
        }

        let horizontal_position_count: u32 = (board.ncols() - piece.matrix.ncols() + 1)
            .try_into()
            .expect("Position count too large");

        // Find all possible horizontal positions (piece at the top)
        let mut horizontal_positions: Vec<DMatrix<u32>> = (0..horizontal_position_count)
            .map(|pos| {
                piece.matrix.clone().insert_columns(0, pos.try_into().unwrap(), 0) * piece.color
            })
            .collect();

        // Add empty rows at the top
        let mut more_horizontal_positions: Vec<DMatrix<u32>> = Vec::new();
        for position in &horizontal_positions {
            if board.nrows() < position.nrows() {
                continue;
            }

            let remaining_height = board.nrows() - position.nrows();
            for h in 0..remaining_height {
                more_horizontal_positions.push(position.clone().insert_rows(0, h + 1, 0));
            }
        }
        horizontal_positions.extend(more_horizontal_positions);

        // Add empty rows at the bottom if needed
        horizontal_positions
            .iter()
            .map(|m| m.clone().resize(board.nrows(), board.ncols(), 0))
            .collect()
    }
}
