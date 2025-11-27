use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys;

use nalgebra::DMatrix;

use gtp_lib::models::*;
use gtp_lib::svg_renderer::*;
use gtp_lib::game_resolver::*;


// https://github.com/rustwasm/wasm-bindgen/pull/2633
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JSMatrix {
    pub svg: String,
    pub width: usize,
    pub height: usize,
}

impl JSMatrix {
    pub fn new(matrix: &DMatrix<u32>) -> Self {
        Self {
            svg: svg_from_matrix(matrix),
            width: matrix.ncols(),
            height: matrix.nrows()
        }
    }
}



#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JSPiece {
    pub id: usize,
    pub matrix: JSMatrix,
    pub color: u32,
}

impl JSPiece {
    pub fn new(id: usize, piece: &Piece) -> Self {
        Self {
            id,
            matrix: JSMatrix::new(&(piece.matrix.clone() * piece.color)),
            color: piece.color,
        }
    }
}



#[wasm_bindgen(getter_with_clone)]
pub struct JSGame {
    #[wasm_bindgen(skip)]
    pub game: Game,
}

#[wasm_bindgen]
impl JSGame {
    pub fn is_valid(&self) -> bool {
        self.game.is_valid()
    }

    pub fn missing_cells(&self) -> u32 {
        self.game.missing_cells()
    }

    pub fn piece(&self, id: usize) -> JSPiece {
        let piece = self.game.piece(id).expect("Invalid piece ID");
        JSPiece::new(id, piece)
    }

    #[wasm_bindgen(getter)]
    pub fn pieces(&self) -> JSPieceArray {
        self.game.piece_ids().into_iter()
        .map(|id| {
            let piece = self.game.piece(id).expect("Invalid piece ID");
            JSPiece::new(id, piece)
        })
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<JSPieceArray>()
    }
}

#[wasm_bindgen]
impl JSGame {
    pub fn game_with_all_pieces() -> JSGame {
        Self {
            game: Game::game_with_all_pieces()
        }
    }

    pub fn game_from_game(game: &JSGame, piece_ids: Vec<usize>) -> Self {
        Self {
            game: Game::game_from_game(&game.game, piece_ids)
        }
    }
}

#[wasm_bindgen]
impl JSGame {
    pub fn resolve(&self) -> JSMatrixArray {
        let resolver = GameResolver {};
        resolver.resolve(&self.game).iter()
        .map(JSMatrix::new)
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<JSMatrixArray>()
    }
}



#[wasm_bindgen]
extern "C" {
    // Beurk. Not really generic...
    // https://github.com/rustwasm/wasm-bindgen/issues/111#issuecomment-625729949,
    // https://github.com/Kinrany/likelike-online-rs/commit/d87b391ec4715a1c01e057fb0b0dee225a03a447
    #[wasm_bindgen(typescript_type = "Array<JSMatrix>")]
    pub type JSMatrixArray;

    #[wasm_bindgen(typescript_type = "Array<JSPiece>")]
    pub type JSPieceArray;
}
