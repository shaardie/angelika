//! Search — explores the game tree to find the best move.
use crate::{
    chessmovelist::MoveList, evaluation, position::Position,
    principal_variation::PrincipalVariation,
};

/// Searches the position to the given depth using negamax with alpha-beta pruning.
///
/// - `alpha`: best score the side to move can guarantee so far
/// - `beta`: best score the opponent can guarantee so far
/// - `depth`: remaining depth to search
/// - `ply`: distance from the root position (used for mate scoring)
/// - `pv`: filled with the best line of play found
///
/// Returns the score from the side to move's perspective.
fn alpha_beta(
    pos: &Position,
    mut alpha: i16,
    beta: i16,
    depth: u8,
    ply: u8,
    pv: &mut PrincipalVariation,
) -> i16 {
    if depth == 0 {
        pv.clear();
        return evaluation::evaluation(pos);
    }

    let mut best_store: i16 = -evaluation::INF;
    let mut child_pv = PrincipalVariation::default();
    let mut legal_moves: u8 = 0;

    let mut moves = MoveList::default();
    pos.generate_moves(&mut moves);
    for idx in 0..moves.len() {
        let m = moves.get(idx);

        // Create new position and make move.
        // Only continue, if the new position is actually legal.
        let mut new_pos = *pos;
        new_pos.make_move(m);
        if !new_pos.is_legal() {
            continue;
        }

        legal_moves += 1;

        // Calculate the score for the new position
        let score = -alpha_beta(&new_pos, -beta, -alpha, depth - 1, ply + 1, &mut child_pv);

        // If the score is better than our best result,
        // use it as the new best result
        if score > best_store {
            best_store = score;

            // If the best result of this run is better than the best garanteed result (alpha), use
            // this as the new garanteed result and also update principal variation with the new
            // best move found
            if score > alpha {
                pv.update(m, &child_pv);
                alpha = score;
            }
        }

        // if the score is better than the best garanteed result of the opponent, he will never
        // allow that we come to this point, so we can stop the evaluation of this sub-tree.
        if score >= beta {
            break;
        }
    }

    // No legal moves: checkmate or stalemate
    if legal_moves == 0 {
        return if pos.is_check() {
            -evaluation::INF + ply as i16
        } else {
            0
        };
    }

    best_store
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;

    fn search(fen: &str, depth: u8) -> (i16, PrincipalVariation) {
        let pos = Position::from_fen(fen).unwrap();
        let mut pv = PrincipalVariation::default();
        let score = alpha_beta(&pos, -evaluation::INF, evaluation::INF, depth, 0, &mut pv);
        (score, pv)
    }

    #[test]
    fn has_best_move() {
        let (_, pv) = search(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            4,
        );
        assert!(pv.best_move().is_some());
    }

    #[test]
    fn checkmate_has_no_move() {
        let (score, pv) = search("k7/1Q6/1K6/8/8/8/8/8 b - - 0 1", 1);
        assert!(pv.best_move().is_none());
        assert!(score.abs() >= evaluation::MATE);
    }

    #[test]
    fn stalemate_is_draw() {
        let (score, _) = search("k7/2Q5/1K6/8/8/8/8/8 b - - 0 1", 1);
        assert_eq!(score, 0);
    }
}
