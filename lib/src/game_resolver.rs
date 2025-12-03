use nalgebra::DMatrix;

use crate::models::{Game, Piece};
use crate::matrix_tools;
use crate::bitboard::{BitBoard, generate_positions};

// Helper context and DFS for paginated resolution
struct PageCtx<'a> {
    precomputed: &'a [Vec<(BitBoard, DMatrix<u32>)>],
    start: usize,
    end: usize,
}

fn dfs_page(
    ctx: &mut PageCtx,
    depth: usize,
    boards_bits: BitBoard,
    boards_matrix: &DMatrix<u32>,
    results: &mut Vec<DMatrix<u32>>,
    count: &mut usize,
) -> bool {
    if *count >= ctx.end {
        return true; // stop early
    }
    if depth == ctx.precomputed.len() {
        if *count >= ctx.start {
            results.push(boards_matrix.clone());
        }
        *count += 1;
        return *count >= ctx.end;
    }
    for (placement_bits, placement_matrix) in &ctx.precomputed[depth] {
        if boards_bits & *placement_bits == 0 {
            let new_bits = boards_bits | *placement_bits;
            let new_matrix = boards_matrix + placement_matrix;
            if dfs_page(ctx, depth + 1, new_bits, &new_matrix, results, count) {
                return true;
            }
        }
    }
    false
}

/// Trait for game puzzle solvers.
///
/// Implementors of this trait can solve puzzle games by finding all valid
/// arrangements of pieces on a game board.
pub trait GameResolverTrait {
    /// Finds all valid solutions for the given game.
    fn resolve(&self, game: &Game) -> Vec<DMatrix<u32>>;

    /// Finds the number of valid solutions for the given game without allocating solution matrices.
    fn resolve_count(&self, game: &Game) -> u32 {
        u32::try_from(self.resolve(game).len()).expect("solution count exceeds u32::MAX")
    }
    
    /// Generates all unique variants (rotations and reflections) of a piece.
    fn piece_variants(&self, piece: &Piece) -> Vec<Piece>;

    /// Returns a specific page of solutions using DFS search.
    fn resolve_page(&self, game: &Game, page_index: usize, page_size: usize) -> Vec<DMatrix<u32>>;
}

/// A solver for the "Gagne Ton Papa" puzzle game.
///
/// This solver uses a backtracking algorithm to find all possible ways
/// to arrange the given pieces on the game board.
pub struct GameResolver;

impl GameResolverTrait for GameResolver {
    fn resolve(&self, game: &Game) -> Vec<DMatrix<u32>> {
        let rows = usize::try_from(game.rows()).expect("Row count too large");
        let cols = usize::try_from(game.columns).expect("Column count too large");
        assert!(rows * cols <= 64, "Board size exceeds 64 cells (rows * cols = {}), which is the limit for the bitboard implementation.", rows * cols);
        let empty_board_bits: BitBoard = 0;
        let empty_board_matrix = DMatrix::<u32>::zeros(rows, cols);
        
        // Solutions are stored as (BitBoard, DMatrix) tuples during the search.
        // BitBoard for fast collision detection, DMatrix for preserving colors.
        let mut solutions: Vec<(BitBoard, DMatrix<u32>)> = vec![(empty_board_bits, empty_board_matrix)];

        if !game.is_valid() {
            return vec![];
        }

        // Order pieces by decreasing cell count (already done in original code).
        let mut piece_indices: Vec<_> = (0..game.pieces.len()).collect();
        piece_indices.sort_by_key(|&i| std::cmp::Reverse(game.pieces[i].cells()));

        for &piece_idx in &piece_indices {
            let piece = &game.pieces[piece_idx];
            // Encode piece index into the high 8 bits of its color (as before).
            let piece_id = u32::try_from(piece_idx).expect("Too many pieces") + 1;
            let color_with_id = piece.color | (piece_id << 24);

            let piece_with_id = Piece {
                matrix: piece.matrix.clone(),
                color: color_with_id,
                tui_color: piece.tui_color,
            };

            let mut next_solutions: Vec<(BitBoard, DMatrix<u32>)> = Vec::new();

            for variant in self.piece_variants(&piece_with_id) {
                // Generate all possible placements for this piece as (BitBoard, DMatrix) tuples.
                let placements = generate_positions(&variant, rows, cols);
                
                for (placement_bits, placement_matrix) in placements {
                    for (board_bits, board_matrix) in &solutions {
                        // Collision test: no overlapping 1 bits.
                        if board_bits & placement_bits == 0 {
                            // Merge the piece into the board.
                            let new_bits = board_bits | placement_bits;
                            // Matrix addition preserves the colors since we know there's no overlap
                            let new_matrix = board_matrix + &placement_matrix;
                            next_solutions.push((new_bits, new_matrix));
                        }
                    }
                }
            }
            solutions = next_solutions;
            if solutions.is_empty() {
                break;
            }

            #[cfg(debug_assertions)]
            println!("- Found {} possible boards for piece {}", solutions.len(), piece_idx);
        }

        // Return the colored matrices from the solutions
        solutions
            .into_iter()
            .map(|(_, matrix)| matrix)
            .collect()
    }

    fn piece_variants(&self, piece: &Piece) -> Vec<Piece> {
        matrix_tools::rotation_variants(&piece.matrix)
            .into_iter()
            .map(|matrix| Piece {
                color: piece.color,
                tui_color: piece.tui_color,
                matrix,
            })
            .collect()
    }

    fn resolve_page(&self, game: &Game, page_index: usize, page_size: usize) -> Vec<DMatrix<u32>> {
        if page_size == 0 { return Vec::new(); }

        let rows = usize::try_from(game.rows()).expect("Row count too large");
        let cols = usize::try_from(game.columns).expect("Column count too large");
        assert!(rows * cols <= 64, "Board size exceeds 64 cells (rows * cols = {}), which is the limit for the bitboard implementation.", rows * cols);
        
        if !game.is_valid() {
            return Vec::new();
        }

        // Order pieces by decreasing cell count for stronger pruning
        let mut ordered_indices: Vec<_> = (0..game.pieces.len()).collect();
        ordered_indices.sort_by_key(|&i| std::cmp::Reverse(game.pieces[i].cells()));

        // Precompute variants and placements for each ordered piece
        let mut precomputed: Vec<Vec<(BitBoard, DMatrix<u32>)>> = Vec::with_capacity(ordered_indices.len());
        for (order_pos, &piece_idx) in ordered_indices.iter().enumerate() {
            let piece = &game.pieces[piece_idx];
            let piece_id = u32::try_from(order_pos).expect("Too many pieces") + 1;
            let color_with_id = piece.color | (piece_id << 24);
            let piece_with_id = Piece {
                matrix: piece.matrix.clone(),
                color: color_with_id,
                tui_color: piece.tui_color,
            };
            let mut list: Vec<(BitBoard, DMatrix<u32>)> = Vec::new();
            for variant in self.piece_variants(&piece_with_id) {
                list.extend(generate_positions(&variant, rows, cols));
            }
            list.sort_by_key(|(bits, _)| *bits);
            precomputed.push(list);
        }

        let mut results: Vec<DMatrix<u32>> = Vec::new();
        let start = page_index.saturating_mul(page_size);
        let end = start.saturating_add(page_size);
        let mut count = 0usize;

        let empty_board_bits: BitBoard = 0;
        let empty_board_matrix = DMatrix::<u32>::zeros(rows, cols);
        let mut ctx = PageCtx { precomputed: &precomputed, start, end };
        dfs_page(&mut ctx, 0, empty_board_bits, &empty_board_matrix, &mut results, &mut count);
        results
    }
}


impl GameResolver {
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Game, Piece};
    use nalgebra::DMatrix;

    fn create_piece(rows: usize, cols: usize, values: &[u32]) -> Piece {
        Piece {
            matrix: DMatrix::from_row_slice(rows, cols, values),
            color: 1,
            tui_color: 1,
        }
    }

    #[test]
    fn test_resolve_simple_game() {
        // 2x2 board
        // Piece 1: 1x2 [1, 1]
        // Piece 2: 1x2 [1, 1]
        // Should have solutions.
        let p1 = create_piece(1, 2, &[1, 1]);
        let p2 = create_piece(1, 2, &[1, 1]);
        let game = Game {
            columns: 2,
            pieces: vec![p1, p2],
        };

        let resolver = GameResolver;
        let solutions = resolver.resolve(&game);
        assert!(!solutions.is_empty());
        
        // Check that the solution is valid (filled with non-zero)
        let solution = &solutions[0];
        assert_eq!(solution.nrows(), 2);
        assert_eq!(solution.ncols(), 2);
        assert!(solution.iter().all(|&x| x > 0));
    }

    #[test]
    fn test_resolve_impossible_game() {
        // 2x2 board
        // Piece 1: 3x1 [1, 1, 1] - too tall
        // Piece 2: 1x1 [1]
        // Total cells 4, but piece 1 doesn't fit in 2x2.
        let p1 = create_piece(3, 1, &[1, 1, 1]);
        let p2 = create_piece(1, 1, &[1]);
        let game = Game {
            columns: 2,
            pieces: vec![p1, p2],
        };

        let resolver = GameResolver;
        let solutions = resolver.resolve(&game);
        assert!(solutions.is_empty());
    }

    #[test]
    #[should_panic(expected = "Board size exceeds 64 cells")]
    fn test_resolve_too_large_board() {
        // To trigger the panic, we need total_cells / cols * cols > 64.
        // Let's use 65 pieces of size 1, and 1 column.
        // Rows = 65 / 1 = 65.
        // 65 * 1 = 65 > 64.
        
        let p1 = create_piece(1, 1, &[1]);
        let game = Game {
            columns: 1,
            pieces: vec![p1; 65],
        };
        
        let resolver = GameResolver;
        resolver.resolve(&game);
    }
}
