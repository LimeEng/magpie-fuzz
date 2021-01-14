#![no_main]

use libfuzzer_sys::fuzz_target;
use magpie::othello::{OthelloBoard, PositionExt, Stone};

mod common;

fuzz_target!(|board: common::ShadowOthelloBoard| {
    // Check so that all legal moves returned can actually be placed
    let board = OthelloBoard::from(board);
    let stone = Stone::Black;

    let result = board
        .legal_moves_for(stone)
        .positions()
        .map(|pos| board.clone().place_stone(stone, pos))
        .all(|result| result.is_ok());
    assert!(result);
});
