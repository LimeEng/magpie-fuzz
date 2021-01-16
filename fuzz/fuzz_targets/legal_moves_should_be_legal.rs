#![no_main]

use libfuzzer_sys::fuzz_target;
use magpie::othello::{OthelloBoard, Stone, StoneExt};

mod common;

fuzz_target!(|board: common::ShadowOthelloBoard| {
    // Check so that all legal moves returned can be individually verified as legal
    let board = OthelloBoard::from(board);
    let stone = Stone::Black;

    let result = board
        .legal_moves_for(stone)
        .stones()
        .map(|pos| board.is_legal_move(stone, pos))
        .all(|result| result);
    assert!(result);
});
