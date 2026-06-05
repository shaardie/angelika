use crate::{
    piece::{Color, PieceType},
    position::Position,
};

pub const INF: i16 = i16::MAX;
pub const MAX_PLIES: i16 = 100;
pub const MATE: i16 = INF - MAX_PLIES;

const PIECE_TYPE_VALUE: [i16; PieceType::NUM] = [100, 300, 300, 500, 900, 0];

pub fn evaluation(pos: &Position) -> i16 {
    let mut r = 0;
    for piece_type in PieceType::ALL {
        r += PIECE_TYPE_VALUE[piece_type]
            * (pos.pieces[Color::White][piece_type].population_count() as i16
                - pos.pieces[Color::Black][piece_type].population_count() as i16)
    }
    match pos.side_to_move {
        Color::White => r,
        Color::Black => -r,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn startpos() {
        assert_eq!(evaluation(&Position::starting_position()), 0);
    }
}
