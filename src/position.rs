use crate::attacks;
use crate::bitboard::Bitboard;
use crate::piece::{Color, Piece, PieceType};
use crate::square::Square;

type Castling = u8;
const CASTLING_WHITE_KING: Castling = 0b0001;
const CASTLING_WHITE_QUEEN: Castling = 0b0010;
const CASTLING_BLACK_KING: Castling = 0b0100;
const CASTLING_BLACK_QUEEN: Castling = 0b1000;

pub struct Position {
    pieces_by_color: [Bitboard; Color::NUM],
    pieces: [[Bitboard; PieceType::NUM]; Color::NUM],
    all_pieces: Bitboard,
    board: [Option<Piece>; Square::NUM],
    side_to_move: Color,
    castling: Castling,
    en_passente: Option<Square>,
    half_move_clock: u8,
    ply: u8,
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
            en_passente: None,
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

    pub fn from_fen(fen: &str) -> Result<Self, &str> {
        let tokens: Vec<&str> = fen.split(' ').collect();

        if tokens.len() != 6 {
            return Err("wrong number of tokens");
        }

        unimplemented!()
    }
}
