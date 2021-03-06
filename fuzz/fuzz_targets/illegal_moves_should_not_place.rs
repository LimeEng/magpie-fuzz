#![no_main]

use libfuzzer_sys::fuzz_target;
use magpie::othello::{OthelloBoard, SquareExt, Stone};

mod common;

fuzz_target!(|board: common::ShadowOthelloBoard| {
    // Check so that all moves not contained in the set of legal moves
    // cannot actually be placed
    let board = OthelloBoard::from(board);
    let stone = Stone::Black;

    let legal_positions = board.moves_for(stone);

    let failed = u64::MAX
        .squares()
        .filter(|pos| *pos & legal_positions == 0)
        .map(|pos| board.clone().place_stone(stone, pos))
        .any(|result| result.is_ok());

    assert!(!failed);
});
