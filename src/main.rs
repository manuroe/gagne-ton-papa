
use gtp_lib::models::*;
use gtp_lib::game_data::*;
use gtp_lib::game_resolver::*;

use nalgebra::DMatrix;
use colored::*;

fn main() {
    let pieces = vec![
        PieceName::RedSquare1.piece(),
        PieceName::OrangeBar3.piece(),
        PieceName::BrownL3.piece(),
        PieceName::YellowZigZag4.piece(),
        PieceName::BlueT4.piece(),

        // PieceName::BlueS5.piece(),
        // PieceName::PinkNotSquare5.piece(),

        // PieceName::OrangeL5.piece(),
        // PieceName::BrownT5.piece(),
        // PieceName::VioletZigZag5.piece(),
        ];

    let game = Game { pieces: pieces, ..Default::default() };
    print_pieces(&game.pieces);

    let resolver = GameResolver {};
    let solutions = resolver.resolve(&game);
    print_solutions(&game, &solutions);

    // for piece in game.all_pieces.iter().rev() {        
    //     for variant in resolver.piece_variants(piece) {
    //         print_piece(&variant);
    //         println!("");
    //     }
    //     println!("----------");
    // }

    // for variant in resolver.piece_variants(&game.all_pieces[2]) {
    //     //print_piece(&variant);
    //     println!("{}", variant.matrix);
    //     println!("");
    // }
}


fn print_piece(piece: &Piece) {
    let matrix = &piece.matrix * piece.color;
    display(&matrix);
}

fn print_pieces(pieces: &Vec<Piece>) {
    pieces.iter().for_each(|piece|{
        print_piece(piece);
        println!("");
    })
}


fn print_solutions(game: &Game, solutions: &Vec<DMatrix<u32>>) {
    for solution in solutions.iter() {
        display(solution);
        println!("--------------------");
    }

    println!("{}x{}: {} -> {} solutions", game.rows(), game.columns, game.is_valid(), solutions.len());
}

// fn print_variants(piece: &Piece) {
//     for variant in resolver.piece_variants(piece) {
//         print_piece(&variant);
//         println!("");
//     }
// }





fn display(matrix: &DMatrix<u32>) {
    const DISPLAY_SIZE: usize = 2;
    for row in matrix.row_iter() {
        for _i in 0..DISPLAY_SIZE {
            for &color in row.iter() {
                let char ="█".repeat(DISPLAY_SIZE * 2);
                let (r, g, b) = if color > 0 { from_rgb_u32(color) } else { from_rgb_u32(0x0F0F0F) };
    
                print!("{}", char.truecolor(r, g, b));
            }
            println!("");
        }
    }
}

pub fn from_rgb_u32(c: u32) -> (u8, u8, u8 ) {
    let r = ((c & 0x00FF_0000u32) >> 16) as u8;
    let g = ((c & 0x0000_FF00u32) >> 8) as u8;
    let b = (c & 0x0000_00FFu32) as u8;
    (r, g, b)
}