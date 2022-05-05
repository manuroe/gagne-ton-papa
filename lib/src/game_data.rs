
use nalgebra::DMatrix;

use crate::models::*;

// https://www.gigamic-adds.com/game/gagne-ton-papa
pub enum PieceName {
    RedSquare1,
    TanBar2,
    BrownL3,
    OrangeBar3,
    PinkBar4,
    GreenL4,
    BlueT4,
    YellowZigZag4,
    VioletSquare4,
    OrangeL5,
    BrownT5,
    VioletZigZag5,
    BlueL5,
    PinkNotSquare5,
    YellowU5,
    BlueS5
}

impl PieceName {
    pub fn piece(&self) -> Piece {
        match *self {
            Self::RedSquare1 => Piece { 
                matrix: DMatrix::from_row_slice(1, 1, &[1]),
                color: 0xDA00022
            },
            Self::TanBar2 => Piece { 
                matrix: DMatrix::from_row_slice(2, 1, &[1, 1]),
                color: 0xF1955A
            },
            Self::BrownL3 => Piece { 
                matrix: DMatrix::from_row_slice(2, 2, &[1, 0, 1, 1]),
                color: 0x571C11
            },
            Self::OrangeBar3 => Piece { 
                matrix: DMatrix::from_row_slice(3, 1, &[1, 1, 1]),
                color: 0xEB700F
            },
            Self::PinkBar4 => Piece { 
                matrix: DMatrix::from_row_slice(4, 1, &[1, 1, 1, 1]),
                color: 0xE16BA4
            },
            Self::GreenL4 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2, &[1, 0, 1, 0, 1, 1]),
                color: 0x8DC69E
            },
            Self::BlueT4 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2, &[1, 0, 1, 1, 1, 0]),
                color: 0x36B0EA
            },
            Self::YellowZigZag4 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2, &[0, 1, 1, 1, 1, 0]),
                color: 0xFEDA3C
            },
            Self::VioletSquare4 => Piece { 
                matrix: DMatrix::from_row_slice(2, 2, &[1, 1, 1, 1]),
                color: 0xA36FAD
            },
            Self::OrangeL5 => Piece { 
                matrix: DMatrix::from_row_slice(4, 2,  &[1, 0, 1, 0, 1, 0, 1, 1]),
                color: 0xE06000 // 0xEB700F
            },
            Self::BrownT5 => Piece { 
                matrix: DMatrix::from_row_slice(4, 2,  &[0, 1, 1, 1, 0, 1, 0, 1]),
                color: 0x570C01 // 0x571C11
            },
            Self::VioletZigZag5 => Piece { 
                matrix: DMatrix::from_row_slice(4, 2,  &[0, 1, 0, 1, 1, 1, 1, 0]),
                color: 0x036F0D // 0xA36FAD
            },
            Self::BlueL5 => Piece { 
                matrix: DMatrix::from_row_slice(3, 3,  &[1, 0, 0, 1, 0, 0, 1, 1, 1]),
                color: 0x063679
            },
            Self::PinkNotSquare5 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2,  &[0, 1, 1, 1, 1, 1]),
                color: 0xE00BA4 // 0xE16BA4
            },
            Self::YellowU5 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2,  &[1, 1, 1, 0, 1, 1]),
                color: 0xEECA2C // 0xFEDA3C
            },
            Self::BlueS5 => Piece {  
                matrix: DMatrix::from_row_slice(3, 3,  &[0, 1, 1, 0, 1, 0, 1, 1, 0]),
                color: 0x26A0EA // 0x36B0EA
            },
        }
    }
}

impl Game {
    pub fn game_with_all_pieces() -> Game {
        Game {
            columns: 5,
            pieces:vec![
                PieceName::RedSquare1.piece(),
                PieceName::RedSquare1.piece(),
                PieceName::TanBar2.piece(),
                PieceName::TanBar2.piece(),
                PieceName::BrownL3.piece(),
                PieceName::OrangeBar3.piece(),
                PieceName::PinkBar4.piece(),
                PieceName::GreenL4.piece(),
                PieceName::BlueT4.piece(),
                PieceName::YellowZigZag4.piece(),
                PieceName::VioletSquare4.piece(),
                PieceName::OrangeL5.piece(),
                PieceName::BrownT5.piece(),
                PieceName::VioletZigZag5.piece(),
                PieceName::BlueL5.piece(),
                PieceName::PinkNotSquare5.piece(),
                PieceName::YellowU5.piece(),
                PieceName::BlueS5.piece(),
            ]
        }
     }
}
