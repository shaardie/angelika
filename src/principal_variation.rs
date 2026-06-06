//! Principal Variation (PV) — the best line of play found during search.
//!
//! The PV is the sequence of moves that both sides would play
//! if they follow the best path found by the search so far.

use std::fmt::Display;

use crate::chessmove::Move;

const MAX_DEPTH: usize = 64;

/// The best sequence of moves found during search.
///
/// Updated during alpha-beta search: whenever a move improves alpha,
/// the PV is rebuilt as that move followed by the child's PV.
#[derive(Debug, Clone)]
pub struct PrincipalVariation {
    moves: [Option<Move>; MAX_DEPTH],
    len: usize,
}

impl Default for PrincipalVariation {
    fn default() -> Self {
        PrincipalVariation {
            moves: [None; MAX_DEPTH],
            len: 0,
        }
    }
}

impl Display for PrincipalVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for idx in 0..self.len {
            if idx > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", self.moves[idx].unwrap())?;
        }
        Ok(())
    }
}

impl PrincipalVariation {
    /// Returns the best move, or `None` if the PV is empty.
    pub fn best_move(&self) -> Option<Move> {
        if self.len == 0 {
            return None;
        }
        self.moves[0]
    }

    /// Replaces this PV with `m` followed by `child`'s PV.
    pub fn update(&mut self, m: Move, child: &PrincipalVariation) {
        self.moves[0] = Some(m);
        self.len = child.len + 1;
        for idx in 0..child.len {
            self.moves[idx + 1] = child.moves[idx];
        }
    }

    /// Clear PV
    pub fn clear(&mut self) {
        self.len = 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chessmove::{Move, MoveType};
    use crate::square::Square;

    fn make_move(from: Square, to: Square) -> Move {
        Move::new(from, to, MoveType::Normal, None)
    }

    #[test]
    fn empty_pv_has_no_best_move() {
        let pv = PrincipalVariation::default();
        assert!(pv.best_move().is_none());
    }

    #[test]
    fn single_move_pv() {
        let mut pv = PrincipalVariation::default();
        let child = PrincipalVariation::default();
        let m = make_move(Square::E2, Square::E4);

        pv.update(m, &child);

        assert_eq!(pv.best_move(), Some(m));
        assert_eq!(pv.len, 1);
    }

    #[test]
    fn clear_pv() {
        let mut pv = PrincipalVariation::default();
        let child = PrincipalVariation::default();
        let m = make_move(Square::E2, Square::E4);

        pv.update(m, &child);

        pv.clear();
        assert_eq!(pv.len, 0)
    }

    #[test]
    fn pv_builds_from_leaf_to_root() {
        let e2e4 = make_move(Square::E2, Square::E4);
        let e7e5 = make_move(Square::E7, Square::E5);
        let g1f3 = make_move(Square::G1, Square::F3);

        let mut pv_depth1 = PrincipalVariation::default();
        let mut pv_depth2 = PrincipalVariation::default();
        let mut pv_depth3 = PrincipalVariation::default();

        pv_depth1.update(g1f3, &PrincipalVariation::default());
        pv_depth2.update(e7e5, &pv_depth1);
        pv_depth3.update(e2e4, &pv_depth2);

        assert_eq!(pv_depth3.len, 3);
        assert_eq!(pv_depth3.moves[0], Some(e2e4));
        assert_eq!(pv_depth3.moves[1], Some(e7e5));
        assert_eq!(pv_depth3.moves[2], Some(g1f3));
    }
}
