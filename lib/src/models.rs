
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
    pub all_pieces: Vec<Piece>,
    pub pieces: Vec<Piece>
}

impl Game {
    pub fn is_valid(&self) -> bool {
        self.piece_count() == self.rows() * self.columns
    }

    pub fn rows(&self) -> u32 {
        self.piece_count() / self.columns
    }

    pub fn piece_count(&self) -> u32 {
        return self.pieces.iter().map(|s| s.size()).sum()
    }
}