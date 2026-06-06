//! Search — explores the game tree to find the best move.
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Instant,
};

use crate::{
    chessmove::Move,
    chessmovelist::MoveList,
    evaluation::{self, is_mate},
    position::Position,
    principal_variation::PrincipalVariation,
    search_parameters::SearchParameters,
};

#[derive(Debug)]
pub struct Search {
    nodes: u64,
    pv: Option<PrincipalVariation>,
    score: Option<i16>,
    stop: Arc<AtomicBool>,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            nodes: 0,
            pv: None,
            score: None,
            stop: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Search {
    pub fn new(stop: Arc<AtomicBool>) -> Self {
        Search {
            stop,
            ..Default::default()
        }
    }

    pub fn search(&mut self, pos: &Position, search_parameters: SearchParameters) {
        if let Some(duration) = search_parameters.time_for_move(pos.side_to_move) {
            println!("info string search for {}ms", duration.as_millis());
            let stop_clone = self.stop.clone();
            thread::spawn(move || {
                thread::sleep(duration);
                stop_clone.store(true, Ordering::Relaxed);
            });
        }

        let max_depth = search_parameters.depth.unwrap_or(100);
        self.iterative_search(pos, max_depth);
        let bestmove = self
            .pv
            .clone()
            .unwrap_or_default()
            .best_move()
            .unwrap_or(Move::NULL);
        println!("bestmove {}", bestmove);
    }

    fn iterative_search(&mut self, pos: &Position, max_depth: u8) {
        let start = Instant::now();
        let alpha = -evaluation::INF;
        let beta = evaluation::INF;
        for depth in 1..max_depth + 1 {
            let mut pv = PrincipalVariation::default();
            let score = self.alpha_beta(pos, alpha, beta, depth, 0, &mut pv);
            if self.stop.load(Ordering::Relaxed) {
                return;
            }

            println!(
                "info depth {} time {} nodes {} score cp {} pv {}",
                depth,
                start.elapsed().as_millis(),
                self.nodes,
                score,
                pv
            );
            self.score = Some(score);
            self.pv = Some(pv);

            if is_mate(score) {
                return;
            }
        }
    }

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
        &mut self,
        pos: &Position,
        mut alpha: i16,
        beta: i16,
        depth: u8,
        ply: u8,
        pv: &mut PrincipalVariation,
    ) -> i16 {
        // Time is up, we need to tear down directly
        if self.stop.load(Ordering::Relaxed) {
            return 0;
        }

        self.nodes += 1;

        if depth == 0 {
            pv.clear();
            return evaluation::evaluation(pos);
        }

        let mut best_score: i16 = -evaluation::INF;
        let mut child_pv = PrincipalVariation::default();
        let mut legal_moves: u8 = 0;

        let mut moves = MoveList::default();
        pos.generate_moves(&mut moves);
        for idx in 0..moves.len() {
            // Time is up, we need to tear down directly
            if self.stop.load(Ordering::Relaxed) {
                return 0;
            }
            let m = moves.get(idx);

            // Create new position and make move.
            // Only contirue, if the new position is actually legal.
            let mut new_pos = *pos;
            new_pos.make_move(m);
            if !new_pos.is_legal() {
                continue;
            }

            legal_moves += 1;

            // Calculate the score for the new position
            let score =
                -self.alpha_beta(&new_pos, -beta, -alpha, depth - 1, ply + 1, &mut child_pv);

            // If the score is better than our best result,
            // use it as the new best result
            if score > best_score {
                best_score = score;

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

        best_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{attacks, position::Position};

    #[test]
    fn search_with_max_depth() {
        let mut search = Search::default();
        search.search(
            &Position::starting_position(),
            SearchParameters {
                depth: Some(5),
                ..Default::default()
            },
        );
        assert!(search.pv.is_some());
        assert!(search.score.is_some());
    }

    #[test]
    fn search_with_time() {
        // We need to init the attacks first, because magic takes a lot of time
        attacks::init();
        let mut search = Search::default();
        search.search(
            &Position::starting_position(),
            SearchParameters {
                wtime: Some(1000),
                winc: Some(10),
                movestogo: Some(10),
                ..Default::default()
            },
        );
        assert!(search.pv.is_some());
        assert!(search.score.is_some());
    }

    #[test]
    fn checkmate_has_no_move() {
        let mut search = Search::default();
        search.search(
            &Position::from_fen("k7/1Q6/1K6/8/8/8/8/8 b - - 0 1").unwrap(),
            SearchParameters {
                depth: Some(5),
                ..Default::default()
            },
        );
        assert!(search.pv.is_some());
        assert!(search.score.is_some());
        assert!(is_mate(search.score.unwrap()));
    }

    #[test]
    fn stalemate_is_draw() {
        let mut search = Search::default();
        search.search(
            &Position::from_fen("k7/2Q5/1K6/8/8/8/8/8 b - - 0 1").unwrap(),
            SearchParameters {
                depth: Some(5),
                ..Default::default()
            },
        );
        assert!(search.pv.is_some());
        assert!(search.score.is_some());
        assert_eq!(search.score.unwrap(), 0);
    }
}
