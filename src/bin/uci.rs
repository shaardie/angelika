use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use angelika::{attacks, position::Position, search::Search, search_parameters::SearchParameters};

fn main() {
    attacks::init();

    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut pos = Position::starting_position();
    let mut stop: Option<Arc<AtomicBool>> = None;

    loop {
        stdin.read_line(&mut buffer).unwrap();
        let input = buffer.trim();
        let tokens: Vec<&str> = input.split_whitespace().collect();
        match tokens[0] {
            "uci" => {
                println!("id name Angelika");
                println!("id author Sven Haardiek <sven@haardiek.de>");
                println!("uciok");
            }
            "isready" => {
                println!("readyok")
            }
            "ucinewgame" => {
                pos = Position::starting_position();
            }
            "quit" => {
                return;
            }
            "position" => {
                let mut rest = &tokens[1..];
                match rest[0] {
                    "startpos" => {
                        pos = Position::starting_position();
                        rest = &rest[1..];
                    }
                    "fen" => {
                        pos = Position::from_fen(&rest[1..7].join(" ")).unwrap();
                        rest = &rest[7..];
                    }
                    _ => {
                        panic!("wrong position string");
                    }
                }
                if rest.is_empty() {
                    continue;
                }
                if rest[0] != "moves" {
                    panic!("wrong position string, moves expected")
                }
                for m_str in &rest[1..] {
                    let m = pos.move_from_string(m_str).unwrap();
                    pos.make_move(m);
                }
            }
            "go" => {
                let mut search_parameters = SearchParameters::default();
                let rest = &tokens[1..];
                let mut i = 0;
                while i < rest.len() {
                    match rest[i] {
                        "wtime" => {
                            search_parameters.wtime = tokens.get(i + 1).and_then(|s| s.parse().ok())
                        }
                        "btime" => {
                            search_parameters.btime = tokens.get(i + 1).and_then(|s| s.parse().ok())
                        }
                        "winc" => {
                            search_parameters.winc = tokens.get(i + 1).and_then(|s| s.parse().ok())
                        }
                        "binc" => {
                            search_parameters.binc = tokens.get(i + 1).and_then(|s| s.parse().ok())
                        }
                        "movestogo" => {
                            search_parameters.movestogo =
                                tokens.get(i + 1).and_then(|s| s.parse().ok())
                        }
                        "depth" => {
                            search_parameters.depth = tokens.get(i + 1).and_then(|s| s.parse().ok())
                        }
                        "movetime" => {
                            search_parameters.movetime =
                                tokens.get(i + 1).and_then(|s| s.parse().ok())
                        }
                        "infinite" => {
                            search_parameters.infinite = true;
                            i += 1;
                            continue;
                        }
                        _ => {}
                    }
                    i += 2
                }
                let s = Arc::new(AtomicBool::new(false));
                let s_clone = s.clone();
                thread::spawn(move || {
                    let mut search = Search::new(s_clone);
                    search.search(&pos, search_parameters);
                });
                stop = Some(s);
            }

            "stop" => {
                if let Some(ref s) = stop {
                    s.store(true, Ordering::Relaxed);
                }
            }

            _ => {}
        };

        buffer.clear();
    }
}
