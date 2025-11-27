//! Core library for the "Gagne Ton Papa" puzzle game.
//!
//! This library provides data structures and algorithms for solving
//! polyomino-style puzzle games where pieces must be arranged to fill a board.
//!
//! # Main Components
//!
//! - [`models`] - Core data structures (`Piece`, `Game`)
//! - [`game_data`] - Predefined game pieces and configurations
//! - [`game_resolver`] - Solver algorithm
//! - [`svg_renderer`] - SVG visualization of solutions
//! - [`matrix_tools`] - Matrix manipulation utilities

pub mod models;
pub mod game_data;
pub mod game_resolver;
pub mod svg_renderer;
mod matrix_tools;

// Re-export commonly used types for convenience
pub use models::{Piece, Game};
pub use game_data::PieceName;
pub use game_resolver::{GameResolver, GameResolverTrait};