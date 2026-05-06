mod color;
mod types;

use types::{Bitboard, Color, Piece};

pub struct Position {
    pieces_by_color: [Bitboard; Color::NUM],
    pieces: [Bitboard; Piece::NUM],
    all_pieces: Bitboard,
}

#[cfg(test)]
mod test {
    #[test]
    fn name() {
        todo!();
    }
}
