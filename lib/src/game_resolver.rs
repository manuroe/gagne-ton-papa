
use nalgebra::{DMatrix};
use std::time::Instant;

use crate::models::*;
use crate::matrix_tools;

pub trait GameResolverTrait {
    fn resolve(&self, game: &Game) -> Vec<DMatrix<u32>>;
    fn piece_variants(&self, piece: &Piece) -> Vec<Piece>;
}


pub struct GameResolver {
}

impl GameResolverTrait for GameResolver {
    fn resolve(&self, game: &Game) -> Vec<DMatrix<u32>>  {
        let start = Instant::now();

        let rows = usize::try_from(game.rows()).unwrap();
        let columns = usize::try_from(game.columns).unwrap();
        let board = DMatrix::<u32>::zeros(rows, columns);

        if !game.is_valid() {
            return vec![];
        }

        // Optimisation #1: Start placing bigger pieces first
        let mut sorted_pieces = game.pieces.clone();
        sorted_pieces.sort_by_key(|p| p.size());
        sorted_pieces.reverse();

        // Optimisation #2: Reduce the results of the first call of resolve_board()
        // We can remove the horizonal and vertical symmetries and divide the possible combinations by 4.
        // TODO

        let mut solutions: Vec<DMatrix<u32>> = vec![board.clone()];
        for piece in sorted_pieces.iter() {
            let mut next_solutions: Vec<DMatrix<u32>> = vec![];
            for variant in self.piece_variants(piece) {
                for solution in &solutions {
                    next_solutions.extend(self.resolve_board(&solution, &variant));
                }
            }

            solutions = next_solutions;
            if solutions.len() == 0 {
                break;
            }

            println!("- Found {} posible boards after {:.2?}", solutions.len(), start.elapsed());
        }
        solutions
    }

    fn piece_variants(&self, piece: &Piece) -> Vec<Piece> {
        matrix_tools::rotation_variants(&piece.matrix).iter().map( |matrix|
            Piece{
                color: piece.color,
                matrix: matrix.clone()
            }
        ).collect()
    }
}


impl GameResolver {
    fn resolve_board(&self, board: &DMatrix::<u32>, piece: &Piece) -> Vec<DMatrix<u32>> {
        let mut solutions: Vec<DMatrix<u32>> = vec![];

        let board_maxed = matrix_tools::max_matrix(&board, 1);


        for piece_board in  self.boards_with_piece(piece, board) {
            let piece_board_maxed = matrix_tools::max_matrix(&piece_board, 1);
            let merged_board_maxed = board_maxed.clone() + piece_board_maxed;

            if merged_board_maxed.max() == 1 {
                let merged_board = board + piece_board.clone();
                solutions.push(merged_board);
            }
        }

        solutions
    }

    // Return all boards when the piece can fit
    fn boards_with_piece(&self, piece: &Piece, board: &DMatrix::<u32>) -> Vec<DMatrix<u32>> {
        if piece.matrix.nrows() > board.nrows() {
            return Vec::new();
        }

        let horizontal_position_count: u32 = (board.ncols() - piece.matrix.ncols() + 1).try_into().unwrap();

        // Find all posible horizontal positions (the piece is at the top)
        let range: Vec<u32> = (0..horizontal_position_count).collect();
        let mut horizontal_positions: Vec<DMatrix<u32>> = range.into_iter().map(|pos| {
            piece.matrix.clone().insert_columns(0, pos.try_into().unwrap(), 0) * piece.color
        }).collect();

        // Add empty rows at the top 
        let mut more_horizontal_positions: Vec<DMatrix<u32>> = Vec::new();
        for position in horizontal_positions.iter() {
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
        let horizontal_positions = horizontal_positions.iter().map(|m| {
            m.clone().resize(board.nrows(), board.ncols(), 0)
        }).collect();
        
        horizontal_positions
    }

}
