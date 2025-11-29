use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use gtp_lib::{Game, PieceName, GameResolver, GameResolverTrait};

fn resolve_specific_game(c: &mut Criterion) {
    let pieces = vec![
        PieceName::RedSquare1.piece(),
        PieceName::BrownL3.piece(),
        PieceName::OrangeBar3.piece(),
        PieceName::PinkBar4.piece(),
        PieceName::YellowZigZag4.piece(),
        PieceName::PinkNotSquare5.piece(),
        PieceName::YellowU5.piece(),
    ];

    let game = Game {
        columns: 5,
        pieces,
    };

    let resolver = GameResolver;

    c.bench_function("resolve_specific_game", |b| {
        b.iter(|| {
            let solutions = resolver.resolve(&game);
            assert!(!solutions.is_empty());
        })
    });
}

criterion_group!(benches, resolve_specific_game);
criterion_main!(benches);
