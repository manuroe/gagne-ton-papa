
use nalgebra::DMatrix;

/// Represents a game piece with a matrix pattern and color.
///
/// Each piece has a matrix where non-zero values indicate occupied cells,
/// and a color value represented as a 24-bit RGB hex value.
#[derive(Debug, Clone)]
pub struct Piece {
    pub matrix: DMatrix<u32>,
    pub color: u32
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
