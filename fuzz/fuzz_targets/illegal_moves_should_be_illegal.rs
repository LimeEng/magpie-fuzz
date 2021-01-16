#![no_main]

use libfuzzer_sys::fuzz_target;
use magpie::othello::{OthelloBoard, SquareExt, Stone};

mod common;

fuzz_target!(|board: common::ShadowOthelloBoard| {
    // Check so that all moves not contained in the set of legal moves
    // actually is illegal
    let board = OthelloBoard::from(board);
    let stone = Stone::Black;

    let legal_positions = board.legal_moves_for(stone);

    let failed = u64::MAX
        .squares()
        .filter(|pos| *pos & legal_positions == 0)
        .map(|pos| board.is_legal_move(stone, pos))
        .any(|result| result);

    assert!(!failed);
});
