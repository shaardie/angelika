use crate::chessmove::Move;

#[derive(Debug)]
pub struct MoveList {
    moves: [Move; 255],
    len: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [Move::NULL; 255],
            len: 0,
        }
    }
}

impl MoveList {
    pub fn reset(&mut self) {
        self.len = 0;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Move {
        self.moves[index]
    }

    pub fn push(&mut self, m: Move) {
        self.moves[self.len] = m;
        self.len += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chessmove::{Move, MoveType};
    use crate::square::Square;

    #[test]
    fn test_movelist_new() {
        let ml = MoveList::default();
        assert_eq!(ml.len(), 0);
    }

    #[test]
    fn test_movelist_push_and_get() {
        let mut ml = MoveList::default();
        let m = Move::new(Square::E2, Square::E4, MoveType::Normal, None);
        ml.push(m);
        assert_eq!(ml.len(), 1);
        assert_eq!(ml.get(0), m);
    }

    #[test]
    fn test_movelist_reset() {
        let mut ml = MoveList::default();
        ml.push(Move::new(Square::E2, Square::E4, MoveType::Normal, None));
        ml.push(Move::new(Square::D2, Square::D4, MoveType::Normal, None));
        assert_eq!(ml.len(), 2);
        ml.reset();
        assert_eq!(ml.len(), 0);
    }

    #[test]
    fn test_movelist_multiple_pushes() {
        let mut ml = MoveList::default();
        let moves = [
            Move::new(Square::E2, Square::E4, MoveType::Normal, None),
            Move::new(Square::D2, Square::D4, MoveType::Normal, None),
            Move::new(Square::E1, Square::G1, MoveType::Castling, None),
        ];
        for m in moves {
            ml.push(m);
        }
        assert_eq!(ml.len(), 3);
        for (i, m) in moves.iter().enumerate() {
            assert_eq!(ml.get(i), *m);
        }
    }
}
