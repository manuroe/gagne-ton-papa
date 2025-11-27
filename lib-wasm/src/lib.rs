pub mod js_models;

use wasm_bindgen::prelude::*;

use gtp_lib::models::*;
use gtp_lib::game_data::*;

use js_models::*;

#[wasm_bindgen]
impl JSGame {
    pub fn sample_game() -> JSGame {

        let pieces = vec![
            PieceName::RedSquare1.piece(),
            PieceName::OrangeBar3.piece(),
            PieceName::BrownL3.piece(),
            PieceName::YellowZigZag4.piece(),
            PieceName::BlueT4.piece(),
            PieceName::BlueS5.piece(),
            PieceName::OrangeL5.piece(),
        ];

        Self {
            game: Game { columns: 5, pieces }
        }
    }
}