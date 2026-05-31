use crate::{piece::PieceType, square::Square};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MoveType {
    Normal,
    Promotion,
    EnPassant,
    Castling,
}
impl MoveType {
    pub const NUM: usize = 4;
    pub const fn new(v: u8) -> Self {
        debug_assert!(v < Self::NUM as u8);
        unsafe { std::mem::transmute(v) }
    }
}

// Move represents a move from one position to another
// 0-5 is the source square
// 6-11 is the destination square
// 12-13 is the Move Type
// 14-15 is the Promotion Piece Type
// 16-31 are for the score
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move(u32);

impl Move {
    pub const NULL: Self = Move(0);

    pub fn new(
        from: Square,
        to: Square,
        move_type: MoveType,
        promotion: Option<PieceType>,
    ) -> Self {
        let mut m = Self::NULL;

        // set from
        m.0 |= from as u32;

        // set to
        m.0 |= (to as u32) << 6;

        // set move type
        m.0 |= (move_type as u32) << 12;

        // set promotion
        if let Some(pt) = promotion {
            m.0 |= ((pt as u32 - 2) & 0b11) << 14;
        }
        m
    }

    pub fn from(self) -> Square {
        Square::new((self.0 & 0b111111) as u8)
    }

    pub fn to(self) -> Square {
        Square::new((self.0 >> 6 & 0b111111) as u8)
    }

    pub fn move_type(self) -> MoveType {
        MoveType::new((self.0 >> 12 & 0b11) as u8)
    }

    pub fn promotion(self) -> Option<PieceType> {
        let bits = (self.0 >> 14 & 0b11) as u8;
        if self.move_type() == MoveType::Promotion {
            Some(PieceType::new(bits + 2))
        } else {
            None
        }
    }
    pub fn score(self) -> u16 {
        (self.0 >> 16) as u16
    }

    pub fn set_score(&mut self, score: u16) {
        self.0 = (self.0 & !0xFFFF0000) | ((score as u32) << 16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::PieceType;
    use crate::square::Square;

    #[test]
    fn test_move_normal() {
        let m = Move::new(Square::E2, Square::E4, MoveType::Normal, None);
        assert_eq!(m.from(), Square::E2);
        assert_eq!(m.to(), Square::E4);
        assert_eq!(m.move_type(), MoveType::Normal);
        assert_eq!(m.promotion(), None);
        assert_eq!(m.score(), 0);
    }

    #[test]
    fn test_move_promotion() {
        let m = Move::new(
            Square::E7,
            Square::E8,
            MoveType::Promotion,
            Some(PieceType::Queen),
        );
        assert_eq!(m.from(), Square::E7);
        assert_eq!(m.to(), Square::E8);
        assert_eq!(m.move_type(), MoveType::Promotion);
        assert_eq!(m.promotion(), Some(PieceType::Queen));
    }

    #[test]
    fn test_move_promotion_all_pieces() {
        for pt in [
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
        ] {
            let m = Move::new(Square::A7, Square::A8, MoveType::Promotion, Some(pt));
            assert_eq!(m.promotion(), Some(pt));
        }
    }

    #[test]
    fn test_move_en_passant() {
        let m = Move::new(Square::E5, Square::D6, MoveType::EnPassant, None);
        assert_eq!(m.from(), Square::E5);
        assert_eq!(m.to(), Square::D6);
        assert_eq!(m.move_type(), MoveType::EnPassant);
        assert_eq!(m.promotion(), None);
    }

    #[test]
    fn test_move_castling() {
        let m = Move::new(Square::E1, Square::G1, MoveType::Castling, None);
        assert_eq!(m.from(), Square::E1);
        assert_eq!(m.to(), Square::G1);
        assert_eq!(m.move_type(), MoveType::Castling);
    }

    #[test]
    fn test_move_score() {
        let mut m = Move::new(Square::E2, Square::E4, MoveType::Normal, None);
        assert_eq!(m.score(), 0);
        m.set_score(1000);
        assert_eq!(m.score(), 1000);
        m.set_score(u16::MAX);
        assert_eq!(m.score(), u16::MAX);
    }

    #[test]
    fn test_null_move() {
        assert_eq!(Move::NULL.from(), Square::A1);
        assert_eq!(Move::NULL.to(), Square::A1);
        assert_eq!(Move::NULL.move_type(), MoveType::Normal);
        assert_eq!(Move::NULL.score(), 0);
    }
}
