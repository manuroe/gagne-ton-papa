
use nalgebra::DMatrix;

#[derive(Debug, Clone)]
pub struct Piece {
    pub matrix: DMatrix<u32>,
    pub color: u32
}

impl Piece {
    pub fn size(&self) -> u32 {
        self.matrix.iter().sum()
    }
}



pub struct Game {
    pub columns: u32,
    pub pieces: Vec<Piece>
}

impl Game {
    pub fn is_valid(&self) -> bool {
        self.size() == self.rows() * self.columns 
            && self.pieces.len() > 1 
    }

    pub fn rows(&self) -> u32 {
        self.size() / self.columns
    }

    pub fn size(&self) -> u32 {
        return self.pieces.iter().map(|s| s.size()).sum()
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
