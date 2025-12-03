#![allow(clippy::significant_drop_tightening)]

use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use gtp_lib::{Game, PieceName, GameResolver, GameResolverTrait};

fn sample_game() -> Game {
    let pieces = vec![
        PieceName::RedSquare1.piece(),
        PieceName::BrownL3.piece(),
        PieceName::OrangeBar3.piece(),
        PieceName::PinkBar4.piece(),
        PieceName::YellowZigZag4.piece(),
        PieceName::PinkNotSquare5.piece(),
        PieceName::YellowU5.piece(),
    ];

    Game { columns: 5, pieces }
}

fn bench_resolve_specific_game(c: &mut Criterion) {
    let game = sample_game();

    let resolver = GameResolver;

    c.bench_function("resolve_specific_game", |b| {
        b.iter(|| {
            let solutions = resolver.resolve(&game);
            assert!(!solutions.is_empty());
        });
    });
}

fn bench_resolve_specific_game_first_results(c: &mut Criterion) {
    let game = sample_game();

    let resolver = GameResolver;

    c.bench_function("resolve_specific_game_first_results", |b| {
        b.iter(|| {
            // TODO: Stream the results as they are found
            // For now, we can only get the full resolution
            let solutions = resolver.resolve(&game);
            assert!(!solutions.is_empty());
        });
    });
}

criterion_group!(
    benches,
    bench_resolve_specific_game,
    bench_resolve_specific_game_first_results
);
criterion_main!(benches);
