use std::env;

use angelika::{chessmovelist::MoveList, position::Position};

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
    print!("{}\n", perft(&mut pos, depth))
}
