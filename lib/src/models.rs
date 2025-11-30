
use nalgebra::DMatrix;

/// Represents a game piece with a matrix pattern and color.
///
/// Each piece has a matrix where non-zero values indicate occupied cells,
/// and a color value represented as a 24-bit RGB hex value.
#[derive(Debug, Clone)]
pub struct Piece {
    pub matrix: DMatrix<u32>,
    pub color: u32,
    pub tui_color: u32,
}

impl Piece {
    /// Returns the total number of cells occupied by this piece.
    #[must_use]
    pub fn cells(&self) -> u32 {
        self.matrix.iter().sum()
    }
}


/// Represents a game board configuration with pieces to place.
///
/// The game consists of a grid with a fixed number of columns,
/// and a collection of pieces that need to be placed on the board.
pub struct Game {
    pub columns: u32,
    pub pieces: Vec<Piece>
}

impl Game {
    /// Checks if the game configuration is valid.
    ///
    /// A game is valid if:
    /// - The total cells from all pieces exactly fill the board
    /// - There are at least 2 pieces
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.cells() == self.rows() * self.columns 
            && self.pieces.len() > 1 
    }

    /// Calculates the number of rows needed for the game board.
    #[must_use]
    pub fn rows(&self) -> u32 {
        self.cells() / self.columns
    }

    /// Returns the total number of cells occupied by all pieces.
    #[must_use]
    pub fn cells(&self) -> u32 {
        self.pieces.iter().map(Piece::cells).sum()
    }

    /// Returns the number of missing cells to fill the game board.
    ///
    /// If the game is valid, returns 0. Otherwise, calculates how many
    /// cells are needed to complete the next full row.
    #[must_use]
    pub fn missing_cells(&self) -> u32 {
        if self.is_valid() {
            0
        } else {
            (self.rows() + 1) * self.columns - self.cells()
        }
    }

    /// Returns a vector of piece indices.
    #[must_use]
    pub fn piece_ids(&self) -> Vec<usize> {
        (0..self.pieces.len()).collect()
    }

    /// Returns a reference to the piece at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn piece(&self, id: usize) -> Option<&Piece> {
        self.pieces.get(id)
    }

    /// Creates a new game from a subset of pieces from an existing game.
    ///
    /// # Panics
    ///
    /// Panics if any `piece_id` is out of bounds.
    #[must_use]
    pub fn game_from_game(game: &Self, piece_ids: Vec<usize>) -> Self {
        let pieces = piece_ids.into_iter()
            .map(|id| game.piece(id).expect("Invalid piece ID").clone())
            .collect();
        Self {
            columns: game.columns,
            pieces
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    fn create_piece(rows: usize, cols: usize, values: &[u32]) -> Piece {
        Piece {
            matrix: DMatrix::from_row_slice(rows, cols, values),
            color: 0,
            tui_color: 0,
        }
    }

    #[test]
    fn test_piece_cells() {
        let piece = create_piece(2, 2, &[1, 0, 1, 1]);
        assert_eq!(piece.cells(), 3);
    }

    #[test]
    fn test_game_is_valid() {
        let p1 = create_piece(1, 2, &[1, 1]);
        let p2 = create_piece(1, 2, &[1, 1]);
        let game = Game {
            columns: 2,
            pieces: vec![p1, p2],
        };
        assert!(game.is_valid());
        assert_eq!(game.rows(), 2);
        assert_eq!(game.cells(), 4);
        assert_eq!(game.missing_cells(), 0);
    }

    #[test]
    fn test_game_invalid() {
        let p1 = create_piece(1, 2, &[1, 1]);
        let game = Game {
            columns: 2,
            pieces: vec![p1],
        };
        assert!(!game.is_valid());
    }

    #[test]
    fn test_game_missing_cells() {
        let p1 = create_piece(1, 2, &[1, 1]);
        let p2 = create_piece(1, 1, &[1]);
        let game = Game {
            columns: 2,
            pieces: vec![p1, p2],
        };
        // Total cells: 3. Columns: 2. Rows needed: 3/2 = 1.
        // But 3 cells don't fill 1*2=2 or 2*2=4.
        // rows() returns 1.
        // missing_cells logic: (rows + 1) * columns - cells
        // (1 + 1) * 2 - 3 = 4 - 3 = 1.
        assert_eq!(game.missing_cells(), 1);
    }

    #[test]
    fn test_game_from_game() {
        let p1 = create_piece(1, 1, &[1]);
        let p2 = create_piece(1, 1, &[1]);
        let game = Game {
            columns: 2,
            pieces: vec![p1, p2],
        };
        
        let sub_game = Game::game_from_game(&game, vec![0]);
        assert_eq!(sub_game.pieces.len(), 1);
        assert_eq!(sub_game.pieces[0].cells(), 1);
    }
}
