use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
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

impl<T> Index<Color> for [T] {
    type Output = T;
    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Color> for [T] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
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

impl<T> Index<PieceType> for [T] {
    type Output = T;
    fn index(&self, index: PieceType) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<PieceType> for [T] {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
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
    pub const fn piece_type(&self) -> PieceType {
        match self {
            Self::WhitePawn | Self::BlackPawn => PieceType::Pawn,
            Self::WhiteKnight | Self::BlackKnight => PieceType::Knight,
            Self::WhiteBishop | Self::BlackBishop => PieceType::Bishop,
            Self::WhiteRook | Self::BlackRook => PieceType::Rook,
            Self::WhiteQueen | Self::BlackQueen => PieceType::Queen,
            Self::WhiteKing | Self::BlackKing => PieceType::King,
        }
    }
    pub const fn color(&self) -> Color {
        match self {
            #[rustfmt::skip]
            Self::WhitePawn | Self::WhiteKnight | Self::WhiteBishop | Self::WhiteRook | Self::WhiteQueen | Self::WhiteKing => Color::White,
            #[rustfmt::skip]
            Self::BlackPawn | Self::BlackKnight | Self::BlackBishop | Self::BlackRook | Self::BlackQueen | Self::BlackKing => Color::Black,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color() {
        assert_eq!(Color::White.switch(), Color::Black);
        assert_eq!(Color::Black.switch(), Color::White);
    }
}
