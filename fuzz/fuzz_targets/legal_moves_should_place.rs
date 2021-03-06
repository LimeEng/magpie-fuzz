#![no_main]

use libfuzzer_sys::fuzz_target;
use magpie::othello::{OthelloBoard, Stone, StoneExt};

mod common;

fuzz_target!(|board: common::ShadowOthelloBoard| {
    // Check so that all legal moves returned can actually be placed
    let board = OthelloBoard::from(board);
    let stone = Stone::Black;

    let result = board
        .moves_for(stone)
        .stones()
        .map(|pos| board.clone().place_stone(stone, pos))
        .all(|result| result.is_ok());
    assert!(result);
});
