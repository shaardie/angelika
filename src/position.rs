use crate::attacks;
use crate::bitboard::Bitboard;
use crate::castling::{CastleMove, CastleSide, Castling};
use crate::chessmove::{Move, MoveType};
use crate::chessmovelist::MoveList;
use crate::piece::{Color, Piece, PieceType};
use crate::pushes::pushes;
use crate::square::{File, Rank, Square};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pieces_by_color: [Bitboard; Color::NUM],
    pieces: [[Bitboard; PieceType::NUM]; Color::NUM],
    occupied: Bitboard,
    board: [Option<Piece>; Square::NUM],
    side_to_move: Color,
    castling: Castling,
    en_passant: Option<Square>,
    half_move_clock: u8,
    ply: u8,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            pieces_by_color: [Bitboard::EMPTY; Color::NUM],
            pieces: [[Bitboard::EMPTY; PieceType::NUM]; Color::NUM],
            occupied: Bitboard::EMPTY,
            board: [None; Square::NUM],
            side_to_move: Color::White,
            castling: Castling::NONE,
            en_passant: None,
            half_move_clock: 0,
            ply: 0,
        }
    }
}

impl Position {
    pub fn starting_position() -> Self {
        #[rustfmt::skip]
        let board = [
                Some(Piece::WhiteRook), Some(Piece::WhiteKnight), Some(Piece::WhiteBishop), Some(Piece::WhiteQueen), Some(Piece::WhiteKing), Some(Piece::WhiteBishop), Some(Piece::WhiteKnight), Some(Piece::WhiteRook),
                Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn), Some(Piece::WhitePawn),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn), Some(Piece::BlackPawn),
                Some(Piece::BlackRook), Some(Piece::BlackKnight), Some(Piece::BlackBishop), Some(Piece::BlackQueen), Some(Piece::BlackKing), Some(Piece::BlackBishop), Some(Piece::BlackKnight), Some(Piece::BlackRook),
        ];
        let mut pos = Position {
            pieces_by_color: [Bitboard::EMPTY; Color::NUM],
            pieces: [[Bitboard::EMPTY; PieceType::NUM]; Color::NUM],
            occupied: Bitboard::EMPTY,
            board,
            side_to_move: Color::White,
            castling: Castling::ANY,
            en_passant: None,
            half_move_clock: 0,
            ply: 0,
        };

        // Generate Bitboards
        for sq in Square::ALL {
            if let Some(piece) = pos.board[sq] {
                let bb = Bitboard::from_square(sq);
                let c = piece.color();
                pos.pieces[c][piece.piece_type()] |= bb;
                pos.occupied |= bb;
                pos.pieces_by_color[c] |= bb
            }
        }

        pos
    }

    fn square_attacked_by_color(&self, square: Square, color: Color) -> Bitboard {
        attacks::KING_ATTACKS[square] & self.pieces[color][PieceType::King]
            | attacks::KNIGHT_ATTACKS[square] & self.pieces[color][PieceType::Knight]
            // "Which pawns of `color` attack `square`?"
            // Pawn attacks are asymmetric: a white pawn on d3 attacks e4, but not vice versa.
            // To find attacking pawns, look in the opposite direction.
            | attacks::PAWN_ATTACKS[color.switch()][square] & self.pieces[color][PieceType::Pawn]
            | attacks::bishop_attacks(square, self.occupied) & self.pieces[color][PieceType::Bishop]
            | attacks::rook_attacks(square, self.occupied) & self.pieces[color][PieceType::Rook]
            | attacks::queen_attacks(square, self.occupied) & self.pieces[color][PieceType::Queen]
    }

    fn color_in_check(&self, color: Color) -> bool {
        let sq = self.pieces[color][PieceType::King].lsb_square().unwrap();
        self.square_attacked_by_color(sq, color.switch()) != Bitboard::EMPTY
    }

    pub fn is_check(&self) -> bool {
        self.color_in_check(self.side_to_move)
    }

    pub fn is_legal(&self) -> bool {
        !self.color_in_check(self.side_to_move.switch())
    }

    pub fn can_do_castle_move(&self, castle_move: CastleMove) -> bool {
        // King and one of the Rooks hasn't moved, so castling in possible in theory
        if !self.castling.contains(Castling::from(castle_move)) {
            return false;
        }

        // Correct color to castle
        if castle_move.color() != self.side_to_move {
            return false;
        }

        let queen_side = castle_move.side() == CastleSide::Queen;

        // Can not castle, if in check
        if self.is_check() {
            return false;
        }

        let mut square =
            Bitboard::lsb_square(self.pieces[self.side_to_move][PieceType::King]).unwrap();
        let mut attacked_files = 2;
        let mut free_files = if queen_side { 3 } else { 2 };
        while attacked_files > 0 || free_files > 0 {
            // Check the next square, either in queen or in king side direction
            square = if queen_side {
                square.previous()
            } else {
                square.next()
            };

            // Check, if there is a piece in between
            if free_files > 0 {
                if self.board[square].is_some() {
                    return false;
                }
                free_files -= 1
            }

            // Check, if a square in beween is attacked
            if attacked_files > 0 {
                if self.square_attacked_by_color(square, self.side_to_move.switch())
                    != Bitboard::EMPTY
                {
                    return false;
                }
                attacked_files -= 1;
            }
        }
        true
    }

    pub fn get_piece(&self, square: Square) -> Piece {
        self.board[square].unwrap()
    }

    pub fn set_piece(&mut self, piece: Piece, square: Square) {
        let bb = Bitboard::from_square(square);
        let piece_color = piece.color();
        let piece_type = piece.piece_type();
        self.pieces_by_color[piece_color] |= bb;
        self.pieces[piece_color][piece_type] |= bb;
        self.occupied |= bb;
        self.board[square] = Some(piece);
    }

    pub fn delete_piece(&mut self, square: Square) -> Piece {
        let nbb = !Bitboard::from_square(square);
        let piece = self.board[square].unwrap();
        let piece_color = piece.color();
        let piece_type = piece.piece_type();
        self.pieces_by_color[piece_color] &= nbb;
        self.pieces[piece_color][piece_type] &= nbb;
        self.occupied &= nbb;
        self.board[square] = None;
        piece
    }

    pub fn move_piece(&mut self, from_square: Square, to_square: Square) -> Piece {
        let piece = self.delete_piece(from_square);
        self.set_piece(piece, to_square);
        piece
    }

    pub fn from_fen(fen: &str) -> Result<Self, &str> {
        let mut pos: Position = Default::default();

        let tokens: Vec<&str> = fen.split(' ').collect();

        if tokens.len() != 6 {
            return Err("Invalid number of tokens");
        }

        // Pieces
        // use as u8 because during calculation is exceeds the
        let mut sq = Square::A8 as u8;
        // values of valid squares
        for c in tokens[0].chars() {
            if c.is_ascii_digit() {
                sq += c.to_digit(10).unwrap() as u8;
                continue;
            } else if c == '/' {
                sq -= 2 * 8;
                continue;
            }
            pos.set_piece(Piece::from_char(c)?, Square::new(sq));
            sq += 1;
        }

        // Side to move
        match tokens[1] {
            "w" => pos.side_to_move = Color::White,
            "b" => pos.side_to_move = Color::Black,
            _ => return Err("Invalid side to move"),
        }

        // Castling
        pos.castling = Castling::NONE;
        for c in tokens[2].chars() {
            match c {
                'K' => pos.castling = pos.castling.add(Castling::WHITE_KING),
                'Q' => pos.castling = pos.castling.add(Castling::WHITE_QUEEN),
                'k' => pos.castling = pos.castling.add(Castling::BLACK_KING),
                'q' => pos.castling = pos.castling.add(Castling::BLACK_QUEEN),
                '-' => break,
                _ => return Err("invalid castling token"),
            }
        }

        // En passante
        pos.en_passant = {
            if tokens[3] == "-" {
                None
            } else {
                Some(Square::from_chars(tokens[3])?)
            }
        };

        // Half move clock
        pos.half_move_clock = tokens[4]
            .parse::<u8>()
            .map_err(|_| "invalid half move clock")?;

        // Plys
        let number_of_full_moves = tokens[5]
            .parse::<u8>()
            .map_err(|_| "invalid number of full moves")?;
        pos.ply = 2 * number_of_full_moves - 1;
        if pos.side_to_move == Color::White {
            pos.ply -= 1
        }

        Ok(pos)
    }

    pub fn to_fen(&self) -> String {
        let mut s = String::new();

        // Pieces
        for rank in Rank::ALL.iter().rev() {
            let mut empty_count = 0;
            for file in File::ALL.iter() {
                let sq = Square::from_rank_and_file(*rank, *file);
                match self.board[sq] {
                    None => empty_count += 1,
                    Some(piece) => {
                        if empty_count > 0 {
                            s.push_str(&empty_count.to_string());
                            empty_count = 0;
                        }
                        s.push(piece.to_char());
                    }
                }
            }

            if empty_count > 0 {
                s.push_str(&empty_count.to_string());
            }

            if *rank != Rank::R1 {
                s.push('/');
            }
        }

        // Side to move
        match self.side_to_move {
            Color::White => s.push_str(" w "),
            Color::Black => s.push_str(" b "),
        }

        // Castling
        if self.castling == Castling::NONE {
            s.push('-');
        } else {
            if self.castling.contains(Castling::WHITE_KING) {
                s.push('K');
            }
            if self.castling.contains(Castling::WHITE_QUEEN) {
                s.push('Q');
            }
            if self.castling.contains(Castling::BLACK_KING) {
                s.push('k');
            }
            if self.castling.contains(Castling::BLACK_QUEEN) {
                s.push('q');
            }
        };
        s.push(' ');

        // En Passant
        match self.en_passant {
            None => s.push('-'),
            Some(sq) => s.push_str(&sq.to_str()),
        }
        s.push(' ');

        // Half move clock
        s.push_str(&self.half_move_clock.to_string());
        s.push(' ');

        // Number of full Moves
        let number_of_full_moves = self.ply / 2 + 1;
        s.push_str(&number_of_full_moves.to_string());

        s
    }

    pub fn generate_moves(&self, moves: &mut MoveList) {
        let us = self.pieces_by_color[self.side_to_move];
        let them = self.pieces_by_color[self.side_to_move.switch()];
        let occupied = self.occupied;
        let destinations = !us; // Everything, but our own pieces

        Self::generate_move_helper(
            moves,
            self.pieces[self.side_to_move][PieceType::Bishop],
            occupied,
            destinations,
            attacks::bishop_attacks,
        );
        Self::generate_move_helper(
            moves,
            self.pieces[self.side_to_move][PieceType::Rook],
            occupied,
            destinations,
            attacks::rook_attacks,
        );

        Self::generate_move_helper(
            moves,
            self.pieces[self.side_to_move][PieceType::Queen],
            occupied,
            destinations,
            attacks::queen_attacks,
        );
        Self::generate_move_helper(
            moves,
            self.pieces[self.side_to_move][PieceType::King],
            occupied,
            destinations,
            |sq, _| attacks::KING_ATTACKS[sq],
        );
        Self::generate_move_helper(
            moves,
            self.pieces[self.side_to_move][PieceType::Knight],
            occupied,
            destinations,
            |sq, _| attacks::KNIGHT_ATTACKS[sq],
        );
        Self::generate_pawn_moves(
            moves,
            self.side_to_move,
            self.pieces[self.side_to_move][PieceType::Pawn],
            them,
            occupied,
            self.en_passant,
        );
        self.generate_castling_moves(moves);
    }

    fn generate_move_helper(
        moves: &mut MoveList,
        mut froms: Bitboard,
        occupied: Bitboard,
        destinations: Bitboard,
        attacks: fn(Square, Bitboard) -> Bitboard,
    ) {
        while let Some(from) = froms.pop_lsb_square() {
            let mut tos = attacks(from, occupied) & destinations;
            while let Some(to) = tos.pop_lsb_square() {
                moves.push(Move::new(from, to, MoveType::Normal, None));
            }
        }
    }

    fn generate_pawn_moves(
        moves: &mut MoveList,
        color: Color,
        mut froms: Bitboard,
        them: Bitboard,
        occupied: Bitboard,
        en_passant: Option<Square>,
    ) {
        // Attacks and pushes
        while let Some(from) = froms.pop_lsb_square() {
            let mut tos = (attacks::PAWN_ATTACKS[color][from] & them)
                | pushes(color, Bitboard::from_square(from), occupied);
            while let Some(to) = tos.pop_lsb_square() {
                if (color == Color::White && to.rank() == Rank::R8)
                    || (color == Color::Black && to.rank() == Rank::R1)
                {
                    moves.push(Move::new(
                        from,
                        to,
                        MoveType::Promotion,
                        Some(PieceType::Knight),
                    ));
                    moves.push(Move::new(
                        from,
                        to,
                        MoveType::Promotion,
                        Some(PieceType::Bishop),
                    ));
                    moves.push(Move::new(
                        from,
                        to,
                        MoveType::Promotion,
                        Some(PieceType::Rook),
                    ));
                    moves.push(Move::new(
                        from,
                        to,
                        MoveType::Promotion,
                        Some(PieceType::Queen),
                    ));
                } else {
                    moves.push(Move::new(from, to, MoveType::Normal, None));
                }
            }
            // En Passant
            if let Some(en_passant_square) = en_passant {
                let mut attacking_pawns =
                    attacks::PAWN_ATTACKS[color][from] & Bitboard::from_square(en_passant_square);
                while let Some(to) = attacking_pawns.pop_lsb_square() {
                    moves.push(Move::new(from, to, MoveType::EnPassant, None));
                }
            }
        }
    }

    fn generate_castling_moves(&self, moves: &mut MoveList) {
        for castle_move in CastleMove::ALL {
            if !self.can_do_castle_move(castle_move) {
                continue;
            }

            let from = self.pieces[self.side_to_move][PieceType::King]
                .lsb_square()
                .unwrap();
            let to = {
                if castle_move.side() == CastleSide::King {
                    from.next().next()
                } else {
                    from.previous().previous()
                }
            };
            moves.push(Move::new(from, to, MoveType::Castling, None));
        }
    }

    pub fn make_move(&mut self, m: Move) {
        let mut reset_half_move_clock = false;
        let from = m.from();
        let to = m.to();

        // Update en passant
        self.en_passant = None;

        // Remove target piece
        if self.board[to].is_some() {
            self.delete_piece(to);
            reset_half_move_clock = true;
        }

        // Update castling
        for sq in [from, to] {
            match sq {
                Square::A1 => self.castling = self.castling.remove(Castling::WHITE_QUEEN),
                Square::H1 => self.castling = self.castling.remove(Castling::WHITE_KING),
                Square::A8 => self.castling = self.castling.remove(Castling::BLACK_QUEEN),
                Square::H8 => self.castling = self.castling.remove(Castling::BLACK_KING),
                Square::E1 => {
                    self.castling = self
                        .castling
                        .remove(Castling::WHITE_QUEEN)
                        .remove(Castling::WHITE_KING)
                }
                Square::E8 => {
                    self.castling = self
                        .castling
                        .remove(Castling::BLACK_QUEEN)
                        .remove(Castling::BLACK_KING)
                }
                _ => {}
            }
        }

        // Move piece
        let p = self.move_piece(from, to);
        if p.piece_type() == PieceType::Pawn {
            reset_half_move_clock = true;
            if (from as i8 - to as i8).unsigned_abs() as usize == 2 * File::NUM {
                let en_passant = match self.side_to_move {
                    Color::White => Square::new(to as u8 - File::NUM as u8),
                    Color::Black => Square::new(to as u8 + File::NUM as u8),
                };
                self.en_passant = Some(en_passant)
            }
        }

        // custom handling for special move types
        match m.move_type() {
            MoveType::Normal => {}
            MoveType::Castling => {
                match to {
                    Square::C1 => {
                        self.move_piece(Square::A1, Square::D1);
                    }
                    Square::G1 => {
                        self.move_piece(Square::H1, Square::F1);
                    }
                    Square::C8 => {
                        self.move_piece(Square::A8, Square::D8);
                    }
                    Square::G8 => {
                        self.move_piece(Square::H8, Square::F8);
                    }
                    _ => {}
                };
            }
            MoveType::EnPassant => {
                let pawn_to_remove = {
                    match self.side_to_move {
                        Color::White => Square::new(to as u8 - File::NUM as u8),
                        Color::Black => Square::new(to as u8 + File::NUM as u8),
                    }
                };
                self.delete_piece(pawn_to_remove);
            }
            MoveType::Promotion => {
                self.delete_piece(to);
                self.set_piece(
                    Piece::new_from_color_and_type(self.side_to_move, m.promotion().unwrap()),
                    to,
                );
            }
        }

        self.ply += 1;
        self.side_to_move = self.side_to_move.switch();
        self.half_move_clock = {
            if reset_half_move_clock {
                0
            } else {
                self.half_move_clock + 1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_fen_starting_position() {
        let pos =
            Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let starting = Position::starting_position();

        assert_eq!(pos, starting);
    }

    #[test]
    fn test_fen_roundtrip() {
        let fens = ["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"];

        for fen in fens {
            assert_eq!(
                Position::from_fen(fen).unwrap().to_fen(),
                fen,
                "Roundtrip failed for: {}",
                fen
            );
        }
    }
}
