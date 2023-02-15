#![no_main]

use libfuzzer_sys::fuzz_target;
use magpie::othello::{Bitboard, Game, Position};

mod common;

fuzz_target!(|game: common::ShadowGame| {
    // Check so that all moves not contained in the set of legal moves
    // cannot actually be placed
    let game = Game::try_from(game).unwrap();

    let legal_positions = game.moves();

    let failed = Bitboard::from(u64::MAX)
        .bits()
        .filter(|pos| *pos & legal_positions == 0)
        .filter_map(|pos| Position::try_from(pos).ok())
        .map(|pos| game.clone().play(pos))
        .any(|result| result.is_ok());

    assert!(!failed);
});
