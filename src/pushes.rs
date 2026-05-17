use crate::bitboard::Bitboard;
use crate::piece::Color;

pub fn pushes(color: Color, pawns: Bitboard, occupied: Bitboard) -> Bitboard {
    single_push(color, pawns, occupied) | double_push(color, pawns, occupied)
}

fn single_push(color: Color, pawns: Bitboard, occupied: Bitboard) -> Bitboard {
    match color {
        Color::White => pawns.north_one() & !occupied,
        Color::Black => pawns.south_one() & !occupied,
    }
}

fn double_push(color: Color, pawns: Bitboard, occupied: Bitboard) -> Bitboard {
    match color {
        Color::White => {
            single_push(color, pawns, occupied).north_one() & !occupied & Bitboard::RANK_4
        }
        Color::Black => {
            single_push(color, pawns, occupied).south_one() & !occupied & Bitboard::RANK_5
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::square::Square;

    #[test]
    fn start_position() {
        assert_eq!(
            pushes(
                Color::White,
                Bitboard::from_square(Square::E2),
                Bitboard::EMPTY,
            ),
            Bitboard::from_square(Square::E3) | Bitboard::from_square(Square::E4)
        );
    }

    #[test]
    fn start_position_blocked() {
        assert_eq!(
            pushes(
                Color::White,
                Bitboard::from_square(Square::E2),
                Bitboard::from_square(Square::E3),
            ),
            Bitboard::EMPTY,
        );
        assert_eq!(
            pushes(
                Color::White,
                Bitboard::from_square(Square::E2),
                Bitboard::from_square(Square::E4),
            ),
            Bitboard::from_square(Square::E3),
        );
    }

    #[test]
    fn rank_3() {
        assert_eq!(
            pushes(
                Color::White,
                Bitboard::from_square(Square::E3),
                Bitboard::EMPTY,
            ),
            Bitboard::from_square(Square::E4),
        );
    }
}
