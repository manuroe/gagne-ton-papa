
use gtp_lib::{Game, Piece, PieceName, GameResolver, GameResolverTrait};

use nalgebra::DMatrix;
use colored::*;

fn main() {
    let pieces = vec![
        PieceName::RedSquare1.piece(),
        PieceName::OrangeBar3.piece(),
        PieceName::BrownL3.piece(),
        PieceName::YellowZigZag4.piece(),
        PieceName::BlueT4.piece(),
    ];

    // Use TUI colors (high contrast) for the terminal app
    let pieces: Vec<Piece> = pieces.into_iter().map(|mut p| {
        p.color = p.tui_color;
        p
    }).collect();

    let game = Game { columns: 5, pieces };
    print_pieces(&game.pieces);

    let resolver = GameResolver;
    let solutions = resolver.resolve(&game);
    print_solutions(&game, &solutions);
}

fn print_piece(piece: &Piece) {
    let matrix = &piece.matrix * piece.tui_color;
    display(&matrix);
}

fn print_pieces(pieces: &[Piece]) {
    for piece in pieces {
        print_piece(piece);
        println!();
    }
}

fn print_solutions(game: &Game, solutions: &[DMatrix<u32>]) {
    for solution in solutions {
        display(solution);
        println!("--------------------");
    }

    println!(
        "{}x{}: {} -> {} solutions",
        game.rows(),
        game.columns,
        game.is_valid(),
        solutions.len()
    );
}

fn display(matrix: &DMatrix<u32>) {
    const DISPLAY_SIZE: usize = 2;
    const BLOCK_CHAR: &str = "█";
    const DEFAULT_COLOR: u32 = 0x0F0F0F;
    
    let block = BLOCK_CHAR.repeat(DISPLAY_SIZE * 2);
    
    for row in matrix.row_iter() {
        for _i in 0..DISPLAY_SIZE {
            for &color in row.iter() {
                let (r, g, b) = if color > 0 {
                    from_rgb_u32(color)
                } else {
                    from_rgb_u32(DEFAULT_COLOR)
                };
    
                print!("{}", block.truecolor(r, g, b));
            }
            println!();
        }
    }
}

fn from_rgb_u32(c: u32) -> (u8, u8, u8) {
    let r = ((c & 0x00FF_0000u32) >> 16) as u8;
    let g = ((c & 0x0000_FF00u32) >> 8) as u8;
    let b = (c & 0x0000_00FFu32) as u8;
    (r, g, b)
}