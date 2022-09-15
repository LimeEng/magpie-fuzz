#![no_main]

use libfuzzer_sys::fuzz_target;
use magpie::othello::{Board, Stone};

mod common;

fuzz_target!(|board: common::ShadowBoard| {
    // Check so that all legal moves returned can actually be placed
    let board = Board::from(board);
    let stone = Stone::Black;

    let result = board
        .moves_for(stone)
        .hot_bits()
        .map(|pos| board.clone().place_stone(stone, pos))
        .all(|result| result.is_ok());
    assert!(result);
});
