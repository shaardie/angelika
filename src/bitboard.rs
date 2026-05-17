use std::ops::{BitAnd, BitOr, BitOrAssign, Mul, Not, Shl, Shr};

use crate::square::Square;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bitboard(pub u64);

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

impl Shr for Bitboard {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        Self(self.0 >> rhs.0)
    }
}

impl<T> Shl<T> for Bitboard
where
    u64: Shl<T, Output = u64>,
{
    type Output = Self;
    fn shl(self, rhs: T) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl<T> Shr<T> for Bitboard
where
    u64: Shr<T, Output = u64>,
{
    type Output = Self;
    fn shr(self, rhs: T) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Mul for Bitboard {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_mul(rhs.0))
    }
}

impl Bitboard {
    pub const EMPTY: Self = Self(0);
    pub const ONE: Self = Self(1);

    // Rank Bitboards
    pub const RANK_1: Self = Self(0x00000000000000ff);
    pub const RANK_2: Self = Self(Self::RANK_1.0 << 8);
    pub const RANK_3: Self = Self(Self::RANK_2.0 << 8);
    pub const RANK_4: Self = Self(Self::RANK_3.0 << 8);
    pub const RANK_5: Self = Self(Self::RANK_4.0 << 8);
    pub const RANK_6: Self = Self(Self::RANK_5.0 << 8);
    pub const RANK_7: Self = Self(Self::RANK_6.0 << 8);
    pub const RANK_8: Self = Self(Self::RANK_7.0 << 8);

    // File Bitboards
    pub const FILE_A: Self = Self(0x0101010101010101);
    pub const FILE_B: Self = Self(Self::FILE_A.0 << 1);
    pub const FILE_C: Self = Self(Self::FILE_B.0 << 1);
    pub const FILE_D: Self = Self(Self::FILE_C.0 << 1);
    pub const FILE_E: Self = Self(Self::FILE_D.0 << 1);
    pub const FILE_F: Self = Self(Self::FILE_E.0 << 1);
    pub const FILE_G: Self = Self(Self::FILE_F.0 << 1);
    pub const FILE_H: Self = Self(Self::FILE_G.0 << 1);

    pub const NOT_FILE_A: Self = Self(!Self::FILE_A.0);
    pub const NOT_FILE_H: Self = Self(!Self::FILE_H.0);

    pub const fn north_one(self) -> Self {
        Self(self.0 << 8)
    }
    pub const fn south_one(self) -> Self {
        Self(self.0 >> 8)
    }
    pub const fn west_one(self) -> Self {
        Self((self.0 & Self::NOT_FILE_A.0) >> 1)
    }
    pub const fn east_one(self) -> Self {
        Self((self.0 & Self::NOT_FILE_H.0) << 1)
    }
    pub const fn north_west_one(self) -> Self {
        self.north_one().west_one()
    }
    pub const fn north_east_one(self) -> Self {
        self.north_one().east_one()
    }
    pub const fn south_west_one(self) -> Self {
        self.south_one().west_one()
    }
    pub const fn south_east_one(self) -> Self {
        self.south_one().east_one()
    }

    // from_square explicitly not as a traint to be able to define this as const fn and also use it
    // in other const fn
    pub const fn from_square(sq: Square) -> Self {
        Self(1u64 << (sq as u64))
    }

    pub fn subsets(self) -> impl Iterator<Item = Self> {
        SubsetIterator {
            current: Bitboard::EMPTY,
            mask: self,
            done: false,
        }
    }

    pub fn population_count(self) -> u32 {
        self.0.count_ones()
    }
}

struct SubsetIterator {
    mask: Bitboard,
    current: Bitboard,
    done: bool,
}

impl Iterator for SubsetIterator {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.current;
        if self.done {
            return None;
        }
        self.current = Bitboard(self.current.0.wrapping_sub(self.mask.0)) & self.mask;
        if self.current == Bitboard::EMPTY {
            self.done = true;
        }
        Some(s)
    }
}

#[cfg(test)]
mod test {
    use crate::{bitboard::Bitboard, square::Square};

    #[test]
    fn and() {
        assert_eq!(Bitboard(0b1010) & Bitboard(0b0010), Bitboard(0b0010));
    }

    #[test]
    fn or() {
        assert_eq!(Bitboard(0b1010) | Bitboard(0b0101), Bitboard(0b1111));
    }

    #[test]
    fn shl() {
        assert_eq!(Bitboard(0b1010) << Bitboard::ONE, Bitboard(0b10100));
        assert_eq!(Bitboard(0b1010) << 1, Bitboard(0b10100));
    }

    #[test]
    fn shr() {
        assert_eq!(Bitboard(0b1010) >> Bitboard::ONE, Bitboard(0b101));
        assert_eq!(Bitboard(0b1010) >> 1, Bitboard(0b101))
    }

    #[test]
    fn from_square() {
        assert_eq!(Bitboard::from_square(Square::E2), Bitboard(0b1000000000000));
    }

    #[test]
    fn test_subsets_empty() {
        assert_eq!(
            Bitboard::EMPTY.subsets().collect::<Vec<Bitboard>>(),
            vec![Bitboard::EMPTY]
        );
    }

    #[test]
    fn test_subsets() {
        assert_eq!(
            Bitboard(0b1010).subsets().collect::<Vec<Bitboard>>(),
            vec![
                Bitboard::EMPTY,
                Bitboard(0b10),
                Bitboard(0b1000),
                Bitboard(0b1010)
            ]
        );
    }

    #[test]
    fn test_population_count() {
        assert_eq!(Bitboard(0b0100111).population_count(), 4);
    }
}
