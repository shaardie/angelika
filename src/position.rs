use crate::bitboard::Bitboard;
use crate::castling::Castling;
use crate::piece::{Color, Piece, PieceType};
use crate::square::Square;

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
            castling: Castling::Any,
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
}
