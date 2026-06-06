use std::{
    char,
    fmt::Display,
    ops::{Index, IndexMut},
};

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

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Self::RANK_TO_CHAR.as_bytes()[*self as usize] as char
        )
    }
}

impl Rank {
    pub const NUM: usize = 8;
    const RANK_TO_CHAR: &str = "12345678";
    // calculate all Ranks ahead and iterate over that.
    // Seems not that great
    pub const ALL: [Self; Self::NUM] = {
        let mut arr = [Self::R1; Self::NUM];
        let mut i: u8 = 0;
        while (i as usize) < Self::NUM {
            arr[i as usize] = Rank::new(i);
            i += 1;
        }
        arr
    };
    pub const fn new(v: u8) -> Rank {
        debug_assert!(v < Self::NUM as u8);
        unsafe { std::mem::transmute(v) }
    }
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        let digit = c.to_digit(10).ok_or("Invalid rank number")? as u8;
        if !(1..=8).contains(&digit) {
            return Err("Invalid rank number");
        }
        Ok(Self::new(digit - 1))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
#[rustfmt::skip]
pub enum File {
    A, B, C, D, E, F, G, H,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Self::FILE_TO_CHAR.as_bytes()[*self as usize] as char
        )
    }
}

impl File {
    pub const NUM: usize = 8;
    // calculate all Ranks ahead and iterate over that.
    // Seems not that great
    pub const ALL: [Self; Self::NUM] = {
        let mut arr = [Self::A; Self::NUM];
        let mut i: u8 = 0;
        while (i as usize) < Self::NUM {
            arr[i as usize] = File::new(i);
            i += 1;
        }
        arr
    };
    const FILE_TO_CHAR: &str = "abcdefgh";
    pub const fn new(v: u8) -> Self {
        debug_assert!(v < Self::NUM as u8);
        unsafe { std::mem::transmute(v) }
    }
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        Ok(Self::new(
            Self::FILE_TO_CHAR
                .chars()
                .position(|candidate| candidate == c)
                .ok_or("Invalid file char")? as u8,
        ))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
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

    pub fn from_chars(s: &str) -> Result<Self, &'static str> {
        let mut chars = s.chars();
        let file = File::from_char(chars.next().ok_or("Empty string")?)?;
        let rank = Rank::from_char(chars.next().ok_or("Missing rank")?)?;
        Ok(Self::from_rank_and_file(rank, file))
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

    #[test]
    fn test_file_from_char() {
        assert_eq!(File::from_char('a'), Ok(File::A));
        assert_eq!(File::from_char('b'), Ok(File::B));
        assert_eq!(File::from_char('c'), Ok(File::C));
        assert_eq!(File::from_char('d'), Ok(File::D));
        assert_eq!(File::from_char('e'), Ok(File::E));
        assert_eq!(File::from_char('f'), Ok(File::F));
        assert_eq!(File::from_char('g'), Ok(File::G));
        assert_eq!(File::from_char('h'), Ok(File::H));

        assert!(File::from_char('x').is_err());
        assert!(File::from_char('1').is_err());
    }

    #[test]
    fn test_rank_from_char() {
        assert_eq!(Rank::from_char('1'), Ok(Rank::R1));
        assert_eq!(Rank::from_char('2'), Ok(Rank::R2));
        assert_eq!(Rank::from_char('3'), Ok(Rank::R3));
        assert_eq!(Rank::from_char('4'), Ok(Rank::R4));
        assert_eq!(Rank::from_char('5'), Ok(Rank::R5));
        assert_eq!(Rank::from_char('6'), Ok(Rank::R6));
        assert_eq!(Rank::from_char('7'), Ok(Rank::R7));
        assert_eq!(Rank::from_char('8'), Ok(Rank::R8));

        assert!(Rank::from_char('0').is_err());
        assert!(Rank::from_char('9').is_err());
        assert!(Rank::from_char('a').is_err());
    }

    #[test]
    fn test_square_from_chars() {
        assert_eq!(Square::from_chars("a1"), Ok(Square::A1));
        assert_eq!(Square::from_chars("e4"), Ok(Square::E4));
        assert_eq!(Square::from_chars("h8"), Ok(Square::H8));
        assert_eq!(Square::from_chars("a8"), Ok(Square::A8));
        assert_eq!(Square::from_chars("h1"), Ok(Square::H1));

        assert!(Square::from_chars("").is_err());
        assert!(Square::from_chars("a").is_err());
        assert!(Square::from_chars("x9").is_err());
    }
}
