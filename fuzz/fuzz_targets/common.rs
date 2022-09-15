use libfuzzer_sys::arbitrary;
use libfuzzer_sys::arbitrary::{Arbitrary, Unstructured};
use magpie::othello::{Board, OthelloError};
use std::convert::{From, TryFrom};

#[derive(Debug, Clone)]
pub struct ShadowBoard {
    black_stones: u64,
    white_stones: u64,
}

impl Arbitrary for ShadowBoard {
    fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Self> {
        // Generate a random bitboard
        let bits = u64::arbitrary(u)?;

        let mut black_stones = 0;
        let mut white_stones = 0;

        // Iterate over all bits
        for i in 0..63 {
            // Extract the next bit
            let next_bit = (bits >> i) & 1;
            // Arbitrarily assign this bit to either black or white
            let assign_black = bool::arbitrary(u)?;
            if assign_black {
                black_stones |= next_bit << i;
            } else {
                white_stones |= next_bit << i;
            }
        }
        Ok(ShadowBoard::try_from((black_stones, white_stones)).unwrap())
    }
}

impl TryFrom<(u64, u64)> for ShadowBoard {
    type Error = OthelloError;

    fn try_from(stones: (u64, u64)) -> Result<Self, Self::Error> {
        let (black_stones, white_stones) = stones;
        if black_stones & white_stones != 0 {
            return Err(OthelloError::PiecesOverlapping);
        }
        let board = ShadowBoard {
            black_stones,
            white_stones,
        };
        Ok(board)
    }
}

impl From<ShadowBoard> for Board {
    fn from(board: ShadowBoard) -> Self {
        Board::try_from((board.black_stones, board.white_stones)).unwrap()
    }
}
