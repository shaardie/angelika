use crate::bitboard::Bitboard;
use crate::square::Square;

pub const KNIGHT_ATTACKS: [Bitboard; Square::NUM] = generate_knight_attacks();

const fn generate_knight_attacks() -> [Bitboard; Square::NUM] {
    let mut r = [Bitboard::EMPTY; Square::NUM];
    let mut i: u8 = 0;
    while (i as usize) < Square::NUM {
        let sq = Square::new(i);
        r[i as usize] = _generate_knight_attacks(sq);
        i += 1;
    }
    r
}

const fn _generate_knight_attacks(sq: Square) -> Bitboard {
    let bb = Bitboard::from_square(sq);
    let east = bb.east_one();
    let west = bb.west_one();

    // 1 west or east and 2 north or south
    let att1 = (west.0 | east.0) << 16 | (west.0 | east.0) >> 16;

    let west2 = west.west_one();
    let east2 = east.east_one();
    let att2 =
        Bitboard(west2.0 | east2.0).north_one().0 | Bitboard(west2.0 | east2.0).south_one().0;

    Bitboard(att1 | att2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knight_attacks() {
        assert_eq!(
            KNIGHT_ATTACKS[Square::E4],
            Bitboard::from_square(Square::D2)
                | Bitboard::from_square(Square::F2)
                | Bitboard::from_square(Square::C3)
                | Bitboard::from_square(Square::G3)
                | Bitboard::from_square(Square::C5)
                | Bitboard::from_square(Square::G5)
                | Bitboard::from_square(Square::D6)
                | Bitboard::from_square(Square::F6)
        );
        assert_eq!(
            KNIGHT_ATTACKS[Square::A1],
            Bitboard::from_square(Square::C2) | Bitboard::from_square(Square::B3)
        );
    }
}
