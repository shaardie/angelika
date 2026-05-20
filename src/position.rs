use crate::attacks;
use crate::bitboard::Bitboard;
use crate::piece::{Color, Piece, PieceType};
use crate::square::{File, Rank, Square};

type Castling = u8;
const CASTLING_WHITE_KING: Castling = 0b0001;
const CASTLING_WHITE_QUEEN: Castling = 0b0010;
const CASTLING_BLACK_KING: Castling = 0b0100;
const CASTLING_BLACK_QUEEN: Castling = 0b1000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pieces_by_color: [Bitboard; Color::NUM],
    pieces: [[Bitboard; PieceType::NUM]; Color::NUM],
    all_pieces: Bitboard,
    board: [Option<Piece>; Square::NUM],
    side_to_move: Color,
    castling: Castling,
    en_passante: Option<Square>,
    half_move_clock: u8,
    ply: u8,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            pieces_by_color: [Bitboard::EMPTY; Color::NUM],
            pieces: [[Bitboard::EMPTY; PieceType::NUM]; Color::NUM],
            all_pieces: Bitboard::EMPTY,
            board: [None; Square::NUM],
            side_to_move: Color::White,
            castling: 0,
            en_passante: None,
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
            all_pieces: Bitboard::EMPTY,
            board,
            side_to_move: Color::White,
            castling: CASTLING_WHITE_KING
                | CASTLING_WHITE_QUEEN
                | CASTLING_BLACK_KING
                | CASTLING_BLACK_QUEEN,
            en_passante: None,
            half_move_clock: 0,
            ply: 0,
        };

        // Generate Bitboards
        for sq in Square::ALL {
            if let Some(piece) = pos.board[sq] {
                let bb = Bitboard::from_square(sq);
                let c = piece.color();
                pos.pieces[c][piece.piece_type()] |= bb;
                pos.all_pieces |= bb;
                pos.pieces_by_color[c] |= bb
            }
        }

        pos
    }

    fn square_attacked_by_color(&self, square: Square, color: Color) -> Bitboard {
        attacks::KING_ATTACKS[square] & self.pieces[color][PieceType::King]
            | attacks::KNIGHT_ATTACKS[square] & self.pieces[color][PieceType::Knight]
            | attacks::PAWN_ATTACKS[color][square] & self.pieces[color][PieceType::Pawn]
            | attacks::bishop_attacks(square, self.all_pieces)
                & self.pieces[color][PieceType::Bishop]
            | attacks::rook_attacks(square, self.all_pieces) & self.pieces[color][PieceType::Rook]
            | attacks::queen_attacks(square, self.all_pieces) & self.pieces[color][PieceType::Queen]
    }

    pub fn is_check(&self) -> bool {
        let sq = self.pieces[self.side_to_move][PieceType::King].lsb_square();
        self.square_attacked_by_color(sq, self.side_to_move.switch()) != Bitboard::EMPTY
    }

    // only castling consts are allowed as input
    pub fn can_do_castle(&self, castling: Castling) -> bool {
        // King and one of the Rooks hasn't moved, so castling in possible in theory
        if castling & self.castling == 0 {
            return false;
        }

        let color = if castling & (CASTLING_WHITE_KING | CASTLING_WHITE_QUEEN) > 0 {
            Color::White
        } else {
            Color::Black
        };

        let queen_side = castling & (CASTLING_WHITE_QUEEN | CASTLING_BLACK_QUEEN) > 0;

        // Correct color to castle
        if color != self.side_to_move {
            return false;
        }

        // Can not castle, if in check
        if self.is_check() {
            return false;
        }

        let mut square = Bitboard::lsb_square(self.pieces[self.side_to_move][PieceType::King]);
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
        self.all_pieces |= bb;
        self.board[square] = Some(piece);
    }

    pub fn delete_piece(&mut self, square: Square) -> Piece {
        let nbb = !Bitboard::from_square(square);
        let piece = self.board[square].unwrap();
        let piece_color = piece.color();
        let piece_type = piece.piece_type();
        self.pieces_by_color[piece_color] &= nbb;
        self.pieces[piece_color][piece_type] &= nbb;
        self.all_pieces &= nbb;
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
        pos.castling = 0;
        for c in tokens[2].chars() {
            match c {
                'K' => pos.castling |= CASTLING_WHITE_KING,
                'Q' => pos.castling |= CASTLING_WHITE_QUEEN,
                'k' => pos.castling |= CASTLING_BLACK_KING,
                'q' => pos.castling |= CASTLING_BLACK_QUEEN,
                '-' => break,
                _ => return Err("invalid castling token"),
            }
        }

        // En passante
        pos.en_passante = {
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
        if self.castling == 0 {
            s.push('-');
        } else {
            if self.castling & CASTLING_WHITE_KING != 0 {
                s.push('K');
            }
            if self.castling & CASTLING_WHITE_QUEEN != 0 {
                s.push('Q');
            }
            if self.castling & CASTLING_BLACK_KING != 0 {
                s.push('k');
            }
            if self.castling & CASTLING_BLACK_QUEEN != 0 {
                s.push('q');
            }
        };
        s.push(' ');

        // En Passant
        match self.en_passante {
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
