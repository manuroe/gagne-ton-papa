
use nalgebra::DMatrix;

use crate::models::{Game, Piece};

// Color constants for game pieces (24-bit RGB hex values)
const COLOR_RED: u32 = 0x00DA_0022;
const COLOR_TAN: u32 = 0x00F1_955A;
const COLOR_BROWN: u32 = 0x0057_1C11;
const COLOR_BROWN_DARK: u32 = 0x0057_0C01; // Original: 0x571C11 (changed for terminal UI)
const COLOR_ORANGE: u32 = 0x00EB_700F;
const COLOR_ORANGE_DARK: u32 = 0x00E0_6000; // Original: 0xEB700F (changed for terminal UI)
const COLOR_PINK: u32 = 0x00E1_6BA4;
const COLOR_PINK_DARK: u32 = 0x00E0_0BA4; // Original: 0xE16BA4 (changed for terminal UI)
const COLOR_GREEN: u32 = 0x008D_C69E;
const COLOR_BLUE: u32 = 0x0036_B0EA;
const COLOR_BLUE_DARK: u32 = 0x0006_3679;
const COLOR_BLUE_LIGHT: u32 = 0x0026_A0EA; // Original: 0x36B0EA (changed for terminal UI)
const COLOR_YELLOW: u32 = 0x00FE_DA3C;
const COLOR_YELLOW_DARK: u32 = 0x00EE_CA2C; // Original: 0xFEDA3C (changed for terminal UI)
const COLOR_VIOLET: u32 = 0x00A3_6FAD;
const COLOR_VIOLET_GREEN: u32 = 0x0003_6F0D; // Original: 0xA36FAD (changed for terminal UI)

/// Represents the different piece types in the "Gagne Ton Papa" game.
///
/// Reference: <https://www.gigamic-adds.com/game/gagne-ton-papa>
///
/// Each piece is identified by its color and the number of cells it occupies.
/// The number suffix indicates how many cells the piece covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Creates a `Piece` instance from this piece name.
    #[must_use]
    pub fn piece(&self) -> Piece {
        match *self {
            Self::RedSquare1 => Piece { 
                matrix: DMatrix::from_row_slice(1, 1, &[1]),
                color: COLOR_RED,
                tui_color: COLOR_RED
            },
            Self::TanBar2 => Piece { 
                matrix: DMatrix::from_row_slice(2, 1, &[1, 1]),
                color: COLOR_TAN,
                tui_color: COLOR_TAN
            },
            Self::BrownL3 => Piece { 
                matrix: DMatrix::from_row_slice(2, 2, &[1, 0, 1, 1]),
                color: COLOR_BROWN,
                tui_color: COLOR_BROWN
            },
            Self::OrangeBar3 => Piece { 
                matrix: DMatrix::from_row_slice(3, 1, &[1, 1, 1]),
                color: COLOR_ORANGE,
                tui_color: COLOR_ORANGE
            },
            Self::PinkBar4 => Piece { 
                matrix: DMatrix::from_row_slice(4, 1, &[1, 1, 1, 1]),
                color: COLOR_PINK,
                tui_color: COLOR_PINK
            },
            Self::GreenL4 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2, &[1, 0, 1, 0, 1, 1]),
                color: COLOR_GREEN,
                tui_color: COLOR_GREEN
            },
            Self::BlueT4 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2, &[1, 0, 1, 1, 1, 0]),
                color: COLOR_BLUE,
                tui_color: COLOR_BLUE
            },
            Self::YellowZigZag4 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2, &[0, 1, 1, 1, 1, 0]),
                color: COLOR_YELLOW,
                tui_color: COLOR_YELLOW
            },
            Self::VioletSquare4 => Piece { 
                matrix: DMatrix::from_row_slice(2, 2, &[1, 1, 1, 1]),
                color: COLOR_VIOLET,
                tui_color: COLOR_VIOLET
            },
            Self::OrangeL5 => Piece { 
                matrix: DMatrix::from_row_slice(4, 2, &[1, 0, 1, 0, 1, 0, 1, 1]),
                color: COLOR_ORANGE,
                tui_color: COLOR_ORANGE_DARK
            },
            Self::BrownT5 => Piece { 
                matrix: DMatrix::from_row_slice(4, 2, &[0, 1, 1, 1, 0, 1, 0, 1]),
                color: COLOR_BROWN,
                tui_color: COLOR_BROWN_DARK
            },
            Self::VioletZigZag5 => Piece { 
                matrix: DMatrix::from_row_slice(4, 2, &[0, 1, 0, 1, 1, 1, 1, 0]),
                color: COLOR_VIOLET,
                tui_color: COLOR_VIOLET_GREEN
            },
            Self::BlueL5 => Piece { 
                matrix: DMatrix::from_row_slice(3, 3, &[1, 0, 0, 1, 0, 0, 1, 1, 1]),
                color: COLOR_BLUE_DARK,
                tui_color: COLOR_BLUE_DARK
            },
            Self::PinkNotSquare5 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2, &[0, 1, 1, 1, 1, 1]),
                color: COLOR_PINK,
                tui_color: COLOR_PINK_DARK
            },
            Self::YellowU5 => Piece { 
                matrix: DMatrix::from_row_slice(3, 2, &[1, 1, 1, 0, 1, 1]),
                color: COLOR_YELLOW,
                tui_color: COLOR_YELLOW_DARK
            },
            Self::BlueS5 => Piece {  
                matrix: DMatrix::from_row_slice(3, 3, &[0, 1, 1, 0, 1, 0, 1, 1, 0]),
                color: COLOR_BLUE,
                tui_color: COLOR_BLUE_LIGHT
            },
        }
    }
}

impl Game {
    /// Creates a game with all available pieces.
    ///
    /// This represents the complete "Gagne Ton Papa" puzzle with all 18 pieces.
    #[must_use]
    pub fn game_with_all_pieces() -> Self {
        Self {
            columns: 5,
            pieces: vec![
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

