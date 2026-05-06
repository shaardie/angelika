pub type Bitboard = u64;

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
    pub const fn new(v: u8) -> Square {
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
}

#[derive(PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const NUM: usize = 2;
    pub const fn switch(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub const NUM: usize = 6;
}

pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub const NUM: usize = Color::NUM * PieceType::NUM;
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
    fn color() {
        assert_eq!(Color::White.switch(), Color::Black);
        assert_eq!(Color::Black.switch(), Color::White);
    }
}
