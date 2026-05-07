use std::ops::{BitAnd, BitOr, BitOrAssign, Shl};

use crate::square::Square;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bitboard(u64);

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl Shl for Bitboard {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0 << rhs.0)
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Self::ONE << Self(value as u8 as u64) // this is stupid casting, maybe there is a better solution
    }
}

impl Bitboard {
    pub const EMPTY: Self = Self(0);
    pub const ONE: Self = Self(1);
}

#[cfg(test)]
mod test {
    use crate::{bitboard::Bitboard, square::Square};

    #[test]
    fn and() {
        assert_eq!(Bitboard(0b1010) & Bitboard(0b0010), Bitboard(0b0010))
    }

    #[test]
    fn or() {
        assert_eq!(Bitboard(0b1010) | Bitboard(0b0101), Bitboard(0b1111))
    }

    #[test]
    fn shl() {
        assert_eq!(Bitboard(0b1010) << Bitboard::ONE, Bitboard(0b10100))
    }

    #[test]
    fn from_square() {
        assert_eq!(Bitboard::from(Square::E2), Bitboard(0b1000000000000))
    }
}
