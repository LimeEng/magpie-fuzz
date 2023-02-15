#![no_main]

use libfuzzer_sys::fuzz_target;
use magpie::othello::Game;

mod common;

fuzz_target!(|game: common::ShadowGame| {
    // Check so that all legal moves returned can actually be placed
    let game = Game::try_from(game).unwrap();

    let result = game
        .moves()
        .hot_bits()
        .map(|pos| game.clone().play(pos))
        .all(|result| result.is_ok());
    assert!(result);
});
