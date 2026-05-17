use rand::prelude::*;

use crate::bitboard::Bitboard;
use crate::square::Square;

pub struct Magic {
    pub mask: Bitboard,
    pub magic: Bitboard,
    pub attacks: Vec<Bitboard>,
    pub shift: u32,
}

impl Magic {
    pub fn index(&self, occupied: Bitboard) -> usize {
        (((occupied & self.mask) * self.magic) >> self.shift).0 as usize
    }

    pub fn init_magics<F>(attack_fn: F, seed: u64) -> [Magic; 64]
    where
        F: Fn(Square, Bitboard) -> Bitboard,
    {
        let mut rng = SmallRng::seed_from_u64(seed);
        std::array::from_fn(|i| {
            let square = Square::new(i as u8);

            // First we calculate the mask and shift
            // The edges are not relevant for the occupancy,
            // because the squares can be accessed independent from the occupency.
            let square_rank_mask = Bitboard::RANK_1 << Bitboard(8 * square.rank() as u64);
            let rank_edges = (Bitboard::RANK_1 | Bitboard::RANK_8) & !square_rank_mask;
            let square_file_mask = Bitboard::FILE_A << Bitboard(square.file() as u64);
            let file_edges = (Bitboard::FILE_A | Bitboard::FILE_H) & !square_file_mask;
            let edges = rank_edges | file_edges;
            let mask = attack_fn(square, Bitboard::EMPTY) & !edges;
            let mask_population_count = mask.population_count();
            let shift = 64 - mask_population_count;

            // Calculate attacks
            let occupancies: Vec<Bitboard> = mask.subsets().collect();
            let all_attacks: Vec<Bitboard> = occupancies
                .iter()
                .map(|&occupied| attack_fn(square, occupied))
                .collect();

            // Find attacks and magic
            let mut complete = false;
            let mut m = Magic {
                mask,
                magic: Bitboard::EMPTY,
                attacks: vec![Bitboard::EMPTY; all_attacks.len()],
                shift,
            };
            while !complete {
                loop {
                    // Find small magic
                    m.magic =
                        Bitboard(rng.random::<u64>() & rng.random::<u64>() & rng.random::<u64>());
                    if ((m.magic * mask) >> 56).population_count() < 6 {
                        break;
                    }
                }
                m.attacks.fill(Bitboard::EMPTY);
                for (i, &occupied) in occupancies.iter().enumerate() {
                    let idx = m.index(occupied);
                    if m.attacks[idx] != Bitboard::EMPTY {
                        break;
                    }
                    m.attacks[idx] = all_attacks[i];
                    if i == all_attacks.len() - 1 {
                        complete = true;
                    }
                }
            }
            for (i, &occupied) in occupancies.iter().enumerate() {
                let idx = m.index(occupied);
                debug_assert_eq!(
                    m.attacks[idx], all_attacks[i],
                    "Magic verification failed for square {:?}",
                    square
                );
            }
            m
        })
    }
}
