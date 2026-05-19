use std::sync::LazyLock;

use crate::bitboard::Bitboard;
use crate::magic::Magic;
use crate::piece::Color;
use crate::square::Square;

pub const KNIGHT_ATTACKS: [Bitboard; Square::NUM] = {
    let mut r = [Bitboard::EMPTY; Square::NUM];
    let mut i: u8 = 0;
    while (i as usize) < Square::NUM {
        let sq = Square::new(i);
        r[i as usize] = knight_attacks(sq);
        i += 1;
    }
    r
};

const fn knight_attacks(sq: Square) -> Bitboard {
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

pub const KING_ATTACKS: [Bitboard; Square::NUM] = {
    let mut r = [Bitboard::EMPTY; Square::NUM];
    let mut i: u8 = 0;
    while (i as usize) < Square::NUM {
        let sq = Square::new(i);
        r[i as usize] = king_attacks(sq);
        i += 1;
    }
    r
};

const fn king_attacks(sq: Square) -> Bitboard {
    let mut bb = Bitboard::from_square(sq);
    let attacks = Bitboard(bb.west_one().0 | bb.east_one().0);
    bb = Bitboard(bb.0 | attacks.0);
    Bitboard(attacks.0 | bb.north_one().0 | bb.south_one().0)
}

pub const PAWN_ATTACKS: [[Bitboard; Square::NUM]; Color::NUM] = {
    let mut r = [[Bitboard::EMPTY; Square::NUM]; Color::NUM];
    let mut i: u8 = 0;
    while (i as usize) < Square::NUM {
        let sq = Square::new(i);
        r[Color::White as usize][i as usize] =
            pawn_attacks(Color::White, Bitboard::from_square(sq));
        r[Color::Black as usize][i as usize] =
            pawn_attacks(Color::Black, Bitboard::from_square(sq));
        i += 1;
    }
    r
};

const fn pawn_attacks(color: Color, pawns: Bitboard) -> Bitboard {
    match color {
        Color::White => Bitboard(pawns.north_west_one().0 | pawns.north_east_one().0),
        Color::Black => Bitboard(pawns.south_west_one().0 | pawns.south_east_one().0),
    }
}

fn sliding_attacks(
    sq: Square,
    occupied: Bitboard,
    directions: &[fn(Bitboard) -> Bitboard],
) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    for direction in directions {
        let mut bb = Bitboard::from_square(sq);
        loop {
            let next_move = direction(bb);
            // no square to move
            if next_move == Bitboard::EMPTY {
                break;
            }
            attacks |= next_move;

            // square was occupied
            if next_move & occupied != Bitboard::EMPTY {
                break;
            }
            bb = next_move;
        }
    }
    attacks
}

fn bishop_attacks_slow(sq: Square, occupied: Bitboard) -> Bitboard {
    sliding_attacks(
        sq,
        occupied,
        &[
            Bitboard::north_west_one,
            Bitboard::south_west_one,
            Bitboard::north_east_one,
            Bitboard::south_east_one,
        ],
    )
}

fn rook_attacks_slow(sq: Square, occupied: Bitboard) -> Bitboard {
    sliding_attacks(
        sq,
        occupied,
        &[
            Bitboard::north_one,
            Bitboard::south_one,
            Bitboard::west_one,
            Bitboard::east_one,
        ],
    )
}

static BISHOP_MAGICS: LazyLock<[Magic; Square::NUM]> =
    LazyLock::new(|| Magic::init_magics(bishop_attacks_slow, BISHOP_SEED));
const BISHOP_SEED: u64 = 281954;
static ROOK_MAGICS: LazyLock<[Magic; Square::NUM]> =
    LazyLock::new(|| Magic::init_magics(rook_attacks_slow, ROOK_SEED));
const ROOK_SEED: u64 = 121146;

pub fn init() {
    // Access once to initialize directly
    let _ = &*ROOK_MAGICS;
    let _ = &*BISHOP_MAGICS;
}

pub fn bishop_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    let magic = &BISHOP_MAGICS[sq];
    let idx = magic.index(occupied);
    magic.attacks[idx]
}

pub fn rook_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    let magic = &ROOK_MAGICS[sq];
    let idx = magic.index(occupied);
    magic.attacks[idx]
}

pub fn queen_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    bishop_attacks(sq, occupied) | rook_attacks(sq, occupied)
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_knight_attacks() {
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

    #[test]
    fn test_king_attacks() {
        assert_eq!(
            KING_ATTACKS[Square::E4],
            Bitboard::from_square(Square::D3)
                | Bitboard::from_square(Square::E3)
                | Bitboard::from_square(Square::F3)
                | Bitboard::from_square(Square::D4)
                | Bitboard::from_square(Square::F4)
                | Bitboard::from_square(Square::D5)
                | Bitboard::from_square(Square::E5)
                | Bitboard::from_square(Square::F5)
        );
        assert_eq!(
            KING_ATTACKS[Square::A1],
            Bitboard::from_square(Square::B1)
                | Bitboard::from_square(Square::A2)
                | Bitboard::from_square(Square::B2)
        );
    }
    #[test]
    fn test_pawn_attacks() {
        assert_eq!(
            PAWN_ATTACKS[Color::White][Square::E4],
            Bitboard::from_square(Square::D5) | Bitboard::from_square(Square::F5)
        );
        assert_eq!(
            pawn_attacks(
                Color::Black,
                Bitboard::from_square(Square::C4) | Bitboard::from_square(Square::E4)
            ),
            Bitboard::from_square(Square::B3)
                | Bitboard::from_square(Square::D3)
                | Bitboard::from_square(Square::F3)
        );
    }

    #[test]
    fn test_bishop_attacks_slow_empty_board() {
        assert_eq!(
            bishop_attacks_slow(Square::A1, Bitboard::EMPTY),
            Bitboard::from_square(Square::B2)
                | Bitboard::from_square(Square::C3)
                | Bitboard::from_square(Square::D4)
                | Bitboard::from_square(Square::E5)
                | Bitboard::from_square(Square::F6)
                | Bitboard::from_square(Square::G7)
                | Bitboard::from_square(Square::H8)
        );
    }

    #[test]
    fn test_bishop_attacks_slow_with_blockers() {
        assert_eq!(
            bishop_attacks_slow(
                Square::E4,
                Bitboard::from_square(Square::C2)
                    | Bitboard::from_square(Square::G6)
                    | Bitboard::from_square(Square::C6)
                    | Bitboard::from_square(Square::G2)
            ),
            Bitboard::from_square(Square::D3)
                | Bitboard::from_square(Square::C2)
                | Bitboard::from_square(Square::F5)
                | Bitboard::from_square(Square::G6)
                | Bitboard::from_square(Square::D5)
                | Bitboard::from_square(Square::C6)
                | Bitboard::from_square(Square::F3)
                | Bitboard::from_square(Square::G2)
        );
    }

    #[test]
    fn test_rook_attacks_slow_empty_board() {
        assert_eq!(
            rook_attacks_slow(Square::A1, Bitboard::EMPTY),
            Bitboard::from_square(Square::B1)
                | Bitboard::from_square(Square::C1)
                | Bitboard::from_square(Square::D1)
                | Bitboard::from_square(Square::E1)
                | Bitboard::from_square(Square::F1)
                | Bitboard::from_square(Square::G1)
                | Bitboard::from_square(Square::H1)
                | Bitboard::from_square(Square::A2)
                | Bitboard::from_square(Square::A3)
                | Bitboard::from_square(Square::A4)
                | Bitboard::from_square(Square::A5)
                | Bitboard::from_square(Square::A6)
                | Bitboard::from_square(Square::A7)
                | Bitboard::from_square(Square::A8)
        );
    }

    #[test]
    fn test_rook_attacks_slow_with_blockers() {
        assert_eq!(
            rook_attacks_slow(
                Square::E4,
                Bitboard::from_square(Square::D4)
                    | Bitboard::from_square(Square::E2)
                    | Bitboard::from_square(Square::E6)
                    | Bitboard::from_square(Square::F4)
            ),
            Bitboard::from_square(Square::D4)
                | Bitboard::from_square(Square::E2)
                | Bitboard::from_square(Square::E3)
                | Bitboard::from_square(Square::E5)
                | Bitboard::from_square(Square::E6)
                | Bitboard::from_square(Square::F4)
        );
    }

    #[test]
    fn test_bishop_attacks() {
        let mut rng = rand::rng();
        init();
        for sq in Square::ALL {
            for _ in 0..1024 {
                let occupied = Bitboard(rng.random::<u64>());
                assert_eq!(
                    bishop_attacks(sq, occupied),
                    bishop_attacks_slow(sq, occupied)
                );
            }
        }
    }

    #[test]
    fn test_rook_attacks() {
        let mut rng = rand::rng();
        init();
        for sq in Square::ALL {
            for _ in 0..1024 {
                let occupied = Bitboard(rng.random::<u64>());
                assert_eq!(rook_attacks(sq, occupied), rook_attacks_slow(sq, occupied));
            }
        }
    }

    #[test]
    fn test_queen_attacks() {
        let mut rng = rand::rng();
        init();
        for sq in Square::ALL {
            for _ in 0..1024 {
                let occupied = Bitboard(rng.random::<u64>());
                assert_eq!(
                    queen_attacks(sq, occupied),
                    bishop_attacks_slow(sq, occupied) | rook_attacks(sq, occupied)
                );
            }
        }
    }
}
