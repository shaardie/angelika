use std::ops::{AddAssign, Index, IndexMut, SubAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Rank {
    R1, R2, R3, R4, R5, R6, R7, R8,
}

impl Rank {
    pub const NUM: usize = 8;
    pub const fn new(v: u8) -> Rank {
        debug_assert!(v < Self::NUM as u8);
        unsafe { std::mem::transmute(v) }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
#[rustfmt::skip]
pub enum File {
    A, B, C, D, E, F, G, H,
}

impl File {
    pub const NUM: usize = 8;
    pub const fn new(v: u8) -> File {
        debug_assert!(v < Self::NUM as u8);
        unsafe { std::mem::transmute(v) }
    }
}

impl Square {
    pub const NUM: usize = 64;
    // calculate all Squares ahead and iterate over that.
    // Seems not that great
    pub const ALL: [Self; Self::NUM] = {
        let mut arr = [Self::A1; Self::NUM];
        let mut i: u8 = 0;
        while (i as usize) < Self::NUM {
            arr[i as usize] = Square::new(i);
            i += 1;
        }
        arr
    };

    pub const fn new(v: u8) -> Self {
        debug_assert!(v < Self::NUM as u8);
        unsafe { std::mem::transmute(v) }
    }
    pub const fn rank(self) -> Rank {
        Rank::new(self as u8 >> 3)
    }
    pub const fn file(self) -> File {
        File::new(self as u8 & 0b0111)
    }
    pub const fn from_rank_and_file(r: Rank, f: File) -> Square {
        Self::new((r as u8) << 3 | (f as u8))
    }

    pub const fn next(self) -> Self {
        Self::new((self as u8).wrapping_add(1))
    }

    pub const fn previous(self) -> Self {
        Self::new((self as u8).wrapping_sub(1))
    }
}

impl<T> Index<Square> for [T] {
    type Output = T;
    fn index(&self, index: Square) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Square> for [T] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_rank_and_file() {
        let sq = Square::G6;
        let r = Rank::R6;
        let f = File::G;
        assert_eq!(sq.rank(), r);
        assert_eq!(sq.file(), f);
        assert_eq!(Square::from_rank_and_file(r, f), sq);
    }
}
