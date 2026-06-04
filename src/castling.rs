//! Castling rights and castle moves for chess positions.
//!
//! [`Castling`] is a bitflag type that tracks which castling rights are still available.
//! [`CastleMove`] represents one specific castling move (e.g. white king-side).
//! [`CastleSide`] distinguishes between king-side and queen-side castling.

use crate::piece::Color;

/// A set of castling rights, stored as bitflags.
///
/// Each bit represents one castling right:
/// - Bit 0: White king-side
/// - Bit 1: White queen-side
/// - Bit 2: Black king-side
/// - Bit 3: Black queen-side
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Castling(u8);

impl Castling {
    /// No castling rights.
    pub const NONE: Self = Self(0);
    /// White can castle king-side.
    pub const WHITE_KING: Self = Self(0b0001);
    /// White can castle queen-side.
    pub const WHITE_QUEEN: Self = Self(0b0010);
    /// Black can castle king-side.
    pub const BLACK_KING: Self = Self(0b0100);
    /// Black can castle queen-side.
    pub const BLACK_QUEEN: Self = Self(0b1000);
    /// All castling rights.
    pub const ANY: Self = Self(0b1111);

    /// Returns `true` if `self` has any of the rights in `castling` set.
    pub const fn contains(self, castling: Self) -> bool {
        self.0 & castling.0 != 0
    }

    /// Returns a new `Castling` with the rights from both `self` and `castling`.
    pub const fn add(self, castling: Self) -> Castling {
        Castling(self.0 | castling.0)
    }

    /// Returns a new `Castling` with the rights in `castling` removed from `self`.
    pub const fn remove(self, castling: Self) -> Castling {
        Castling(self.0 & !castling.0)
    }
}

impl From<CastleMove> for Castling {
    fn from(value: CastleMove) -> Self {
        Castling(value as u8)
    }
}

/// King-side or queen-side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CastleSide {
    King,
    Queen,
}

/// One specific castling move.
///
/// Unlike [`Castling`], which can represent multiple rights at once,
/// a `CastleMove` always represents exactly one castling move.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CastleMove {
    WhiteKing = Castling::WHITE_KING.0,
    WhiteQueen = Castling::WHITE_QUEEN.0,
    BlackKing = Castling::BLACK_KING.0,
    BlackQueen = Castling::BLACK_QUEEN.0,
}

impl CastleMove {
    pub const NUM: usize = 4;
    pub const ALL: [CastleMove; Self::NUM] = [
        Self::WhiteKing,
        Self::WhiteQueen,
        Self::BlackKing,
        Self::BlackQueen,
    ];

    /// The color that performs this castling move.
    pub const fn color(self) -> Color {
        match self {
            Self::WhiteKing | Self::WhiteQueen => Color::White,
            Self::BlackKing | Self::BlackQueen => Color::Black,
        }
    }

    /// The side of the board (king-side or queen-side).
    pub const fn side(self) -> CastleSide {
        match self {
            Self::WhiteKing | Self::BlackKing => CastleSide::King,
            Self::WhiteQueen | Self::BlackQueen => CastleSide::Queen,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn castling() {
        for castle_move in CastleMove::ALL {
            let castling = Castling::from(castle_move);
            assert!(Castling::NONE.add(castling).contains(castling));
            assert!(!Castling::ANY.remove(castling).contains(castling));
        }
    }

    #[test]
    fn castle_move() {
        // Color
        assert_eq!(CastleMove::WhiteKing.color(), Color::White);
        assert_eq!(CastleMove::WhiteQueen.color(), Color::White);
        assert_eq!(CastleMove::BlackKing.color(), Color::Black);
        assert_eq!(CastleMove::BlackQueen.color(), Color::Black);

        // Side
        assert_eq!(CastleMove::WhiteKing.side(), CastleSide::King);
        assert_eq!(CastleMove::BlackKing.side(), CastleSide::King);
        assert_eq!(CastleMove::WhiteQueen.side(), CastleSide::Queen);
        assert_eq!(CastleMove::BlackQueen.side(), CastleSide::Queen);
    }
}
