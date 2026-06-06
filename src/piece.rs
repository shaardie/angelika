use std::{
    fmt::Display,
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

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TO_CHAR.as_bytes()[*self as usize] as char)
    }
}

impl PieceType {
    pub const NUM: usize = 6;
    const TO_CHAR: &str = " pkbrqK";
    pub const ALL: [Self; Self::NUM] = [
        Self::Pawn,
        Self::Knight,
        Self::Bishop,
        Self::Rook,
        Self::Queen,
        Self::King,
    ];
    pub fn new(i: u8) -> Self {
        unsafe { transmute(i) }
    }
}

impl<T> Index<PieceType> for [T] {
    type Output = T;
    fn index(&self, index: PieceType) -> &Self::Output {
        &self[index as usize - 1]
    }
}

impl<T> IndexMut<PieceType> for [T] {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        &mut self[index as usize - 1]
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

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::TO_CHAR.as_bytes()[*self as usize] as char)
    }
}

impl Piece {
    pub const NUM: usize = Color::NUM * PieceType::NUM;
    const TO_CHAR: &str = " PNBRQK  pnbrqk";

    pub fn new(i: u8) -> Self {
        unsafe { transmute(i) }
    }

    pub fn new_from_color_and_type(c: Color, pt: PieceType) -> Self {
        Self::new((c as u8 * 8) + pt as u8)
    }

    pub fn from_char(c: char) -> Result<Self, &'static str> {
        let idx = Self::TO_CHAR
            .chars()
            .position(|candidate| candidate == c)
            .ok_or("invalid piece char")?;
        if idx == 0 {
            return Err("Invalid piece char");
        }
        Ok(Piece::new(idx as u8))
    }

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

    #[test]
    fn test_from_char() {
        assert_eq!(Piece::from_char('P'), Ok(Piece::WhitePawn));
        assert_eq!(Piece::from_char('N'), Ok(Piece::WhiteKnight));
        assert_eq!(Piece::from_char('B'), Ok(Piece::WhiteBishop));
        assert_eq!(Piece::from_char('R'), Ok(Piece::WhiteRook));
        assert_eq!(Piece::from_char('Q'), Ok(Piece::WhiteQueen));
        assert_eq!(Piece::from_char('K'), Ok(Piece::WhiteKing));
        assert_eq!(Piece::from_char('p'), Ok(Piece::BlackPawn));
        assert_eq!(Piece::from_char('n'), Ok(Piece::BlackKnight));
        assert_eq!(Piece::from_char('b'), Ok(Piece::BlackBishop));
        assert_eq!(Piece::from_char('r'), Ok(Piece::BlackRook));
        assert_eq!(Piece::from_char('q'), Ok(Piece::BlackQueen));
        assert_eq!(Piece::from_char('k'), Ok(Piece::BlackKing));

        // Invalid Chars
        assert!(Piece::from_char('x').is_err());
        assert!(Piece::from_char('1').is_err());
        assert!(Piece::from_char(' ').is_err());
    }
}
