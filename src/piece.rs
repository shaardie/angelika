use std::{
    mem::transmute,
    ops::{Index, IndexMut},
};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const NUM: usize = 2;
    pub const fn switch(self) -> Self {
        unsafe { transmute(self as u8 ^ Self::Black as u8) }
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
    Pawn = 1,
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
    WhitePawn = 1,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn = Self::WhitePawn as u8 + 8,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub const NUM: usize = Color::NUM * PieceType::NUM;
    pub const fn piece_type(self) -> PieceType {
        unsafe { transmute(self as u8 & 7) }
    }
    pub const fn color(self) -> Color {
        unsafe { transmute(self as u8 >> 3) }
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

    #[test]
    fn piece_type() {
        for piece in &[
            Piece::WhitePawn,
            Piece::WhiteKnight,
            Piece::WhiteBishop,
            Piece::WhiteRook,
            Piece::WhiteQueen,
            Piece::WhiteKing,
        ] {
            assert_eq!(piece.color(), Color::White);
        }
        for piece in &[
            Piece::BlackPawn,
            Piece::BlackKnight,
            Piece::BlackBishop,
            Piece::BlackRook,
            Piece::BlackQueen,
            Piece::BlackKing,
        ] {
            assert_eq!(piece.color(), Color::Black);
        }
        for piece in &[Piece::WhitePawn, Piece::BlackPawn] {
            assert_eq!(piece.piece_type(), PieceType::Pawn);
        }
        for piece in &[Piece::WhiteKnight, Piece::BlackKnight] {
            assert_eq!(piece.piece_type(), PieceType::Knight);
        }
        for piece in &[Piece::WhiteBishop, Piece::BlackBishop] {
            assert_eq!(piece.piece_type(), PieceType::Bishop);
        }
        for piece in &[Piece::WhiteRook, Piece::BlackRook] {
            assert_eq!(piece.piece_type(), PieceType::Rook);
        }
        for piece in &[Piece::WhiteQueen, Piece::BlackQueen] {
            assert_eq!(piece.piece_type(), PieceType::Queen);
        }
        for piece in &[Piece::WhiteKing, Piece::BlackKing] {
            assert_eq!(piece.piece_type(), PieceType::King);
        }
    }
}
