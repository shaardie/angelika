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
        m.set_from(from);
        m.set_to(to);
        m.set_move_type(move_type);
        if let Some(pt) = promotion {
            m.set_promotion(pt);
        }
        m
    }

    pub fn from(self) -> Square {
        Square::new((self.0 & 0b111111) as u8)
    }

    pub fn set_from(&mut self, square: Square) {
        self.0 |= square as u32;
    }

    pub fn to(self) -> Square {
        Square::new((self.0 >> 6 & 0b111111) as u8)
    }

    pub fn set_to(&mut self, square: Square) {
        self.0 |= (square as u32) << 6;
    }

    pub fn move_type(self) -> MoveType {
        MoveType::new((self.0 >> 12 & 0b11) as u8)
    }

    pub fn set_move_type(&mut self, move_type: MoveType) {
        self.0 |= (move_type as u32) << 12;
    }

    pub fn promotion(self) -> PieceType {
        PieceType::new(((self.0 >> 14 & 0b11) + 1) as u8)
    }

    pub fn set_promotion(&mut self, piece_type: PieceType) {
        self.0 |= ((piece_type as u32 - 1) & 0b11) << 14;
    }

    pub fn score(self) -> u16 {
        (self.0 >> 16) as u16
    }

    pub fn set_score(&mut self, score: u16) {
        self.0 |= (score as u32) << 16
    }
}
