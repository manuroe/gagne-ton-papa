
use nalgebra::DMatrix;

#[derive(Debug, Clone)]
pub struct Piece {
    pub matrix: DMatrix<u32>,
    pub color: u32
}

impl Piece {
    pub fn cells(&self) -> u32 {
        self.matrix.iter().sum()
    }
}



pub struct Game {
    pub columns: u32,
    pub pieces: Vec<Piece>
}

impl Game {
    pub fn is_valid(&self) -> bool {
        self.cells() == self.rows() * self.columns 
            && self.pieces.len() > 1 
    }

    pub fn rows(&self) -> u32 {
        self.cells() / self.columns
    }

    // Total number of pieces celles
    pub fn cells(&self) -> u32 {
        self.pieces.iter().map(|s| s.cells()).sum()
    }

    // Number of missing cells to fill the game board
    pub fn missing_cells(&self) -> u32 {
        if self.is_valid() {
            return 0;
        }

        (self.rows() + 1) * self.columns - self.cells()
    }

    pub fn piece_ids(&self) -> Vec<usize> {
        (0..self.pieces.len()).collect::<Vec<usize>>()
    }

    pub fn piece(&self, id: usize) -> &Piece {
        &self.pieces[id]
    }

    pub fn game_from_game(game: &Game, piece_ids: Vec<usize>) -> Self {
        let pieces = piece_ids.into_iter().map(|id| game.piece(id).clone()).collect();
        Self {
            columns: game.columns,
            pieces: pieces
        }
    }
}
