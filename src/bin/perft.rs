//! Perft (Performance Test) — counts the number of leaf nodes in the move tree.
//!
//! Used to verify the correctness of the move generator by comparing
//! node counts against known results for well-known positions.
//!
//! # Usage
//!
//! ```sh
//! cargo run --bin perft "<fen>" <depth>
//! ```
//!
//! # Example
//!
//! ```sh
//! cargo run --bin perft "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" 5
//! ```

use std::env;

use angelika::{chessmovelist::MoveList, position::Position};

/// Recursively counts the number of legal positions reachable from `pos` at the given `depth`.
///
/// At `depth == 0`, returns 1 (the current position counts as one leaf node).
/// Otherwise, generates all pseudo-legal moves, applies each one,
/// discards illegal moves, and recurses with `depth - 1`.
fn perft(pos: &mut Position, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes: u64 = 0;

    let mut moves = MoveList::default();
    pos.generate_moves(&mut moves);
    for idx in 0..moves.len() {
        let mut new_pos = *pos;
        new_pos.make_move(moves.get(idx));
        if !new_pos.is_legal() {
            continue;
        }
        nodes += perft(&mut new_pos, depth - 1)
    }
    nodes
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprint!("Usage: {} <fen> <depth>", args[0]);
        return;
    }

    let fen: &str = args[1].as_str();
    let depth: u32 = args[2].parse().expect("depth should be number");

    let mut pos = Position::from_fen(fen).expect("unable to parse fen string");
    println!("{}\n", perft(&mut pos, depth))
}

#[cfg(test)]
mod test {
    use angelika::position::Position;

    use crate::perft;

    /// Perft results for "Kiwipete", a position by Peter McKenzie
    /// designed to exercise tricky move generation edge cases
    /// (castling, en passant, promotions, discovered checks).
    ///
    /// Reference: <https://www.chessprogramming.org/Perft_Results#Position_2>
    #[test]
    fn kiwipete() {
        let pos = Position::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();
        assert_eq!(perft(&mut pos.clone(), 1), 48);
        assert_eq!(perft(&mut pos.clone(), 2), 2039);
        assert_eq!(perft(&mut pos.clone(), 3), 97862);
        assert_eq!(perft(&mut pos.clone(), 4), 4085603);
    }
}
