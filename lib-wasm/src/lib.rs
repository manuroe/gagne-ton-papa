use wasm_bindgen::prelude::*;

use js_sys;


use gtp_lib::models::*;
use gtp_lib::game_data::*;
use gtp_lib::game_resolver::*;
use gtp_lib::svg_renderer::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn resolve_in_svg() -> js_sys::Array {
    let pieces = vec![
        PieceName::RedSquare1.piece(),
        PieceName::OrangeBar3.piece(),
        PieceName::BrownL3.piece(),
        PieceName::YellowZigZag4.piece(),
        PieceName::BlueT4.piece(),

        PieceName::BlueS5.piece(),
        //PieceName::PinkNotSquare5.piece(),

        PieceName::OrangeL5.piece(),
        // PieceName::BrownT5.piece(),
        // PieceName::VioletZigZag5.piece(),
        ];

    let game = Game { pieces: pieces, ..Default::default() };
    let resolver = GameResolver {};


    println!("The env vars are as follows.");

    resolver.resolve(&game).iter()
        .map(svg_from_matrix)
        .map(JsValue::from)
        .collect::<js_sys::Array>()
}

#[wasm_bindgen]
pub fn all_pieces_svg() -> js_sys::Array {
    let game = Game {..Default::default() };

    println!("The env vars are as follows.");


    game.all_pieces.iter()
        .map(|p| { svg_from_matrix(&(&p.matrix * p.color)) })
        .map(JsValue::from)
        .collect::<js_sys::Array>()
}
