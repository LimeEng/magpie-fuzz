#![no_main]

use libfuzzer_sys::fuzz_target;
use magpie::othello::{Bitboard, Board, Position, Stone};

mod common;

fuzz_target!(|board: common::ShadowBoard| {
    // Check so that all moves not contained in the set of legal moves
    // actually is illegal
    let board = Board::from(board);
    let stone = Stone::Black;

    let legal_positions = board.moves_for(stone);

    let failed = Bitboard::from(u64::MAX)
        .bits()
        .filter(|pos| *pos & legal_positions == 0)
        .filter_map(|pos| Position::try_from(pos).ok())
        .map(|pos| board.is_legal_move(stone, pos))
        .any(|result| result);

    assert!(!failed);
});
