use std::fmt;
use std::fmt::Formatter;
use serde::Serialize;
use crate::chess::board::piece::{Piece, PieceColor, PieceType};

mod piece;

#[derive(Serialize)]
pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self { board: Default::default() };
        board.initialize_pieces();
        board
    }

    fn initialize_pieces(&mut self) {
        // Init pawns
        for i in 0..8 {
            self.board[1][i] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
            self.board[6][i] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
        }

        // Rooks
        self.board[0][0] = Some(Piece::new(PieceType::Rook, PieceColor::White));
        self.board[0][7] = Some(Piece::new(PieceType::Rook, PieceColor::White));
        self.board[7][7] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
        self.board[7][0] = Some(Piece::new(PieceType::Rook, PieceColor::Black));

        // Knights
        self.board[0][1] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        self.board[0][6] = Some(Piece::new(PieceType::Knight, PieceColor::White));
        self.board[7][6] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
        self.board[7][1] = Some(Piece::new(PieceType::Knight, PieceColor::Black));

        // Bishops
        self.board[0][2] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
        self.board[0][5] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
        self.board[7][5] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
        self.board[7][2] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));

        // Queens
        self.board[0][3] = Some(Piece::new(PieceType::Queen, PieceColor::White));
        self.board[7][3] = Some(Piece::new(PieceType::Queen, PieceColor::Black));

        // Kings
        self.board[0][4] = Some(Piece::new(PieceType::King, PieceColor::White));
        self.board[7][4] = Some(Piece::new(PieceType::King, PieceColor::Black));
    }

    fn column_name(c: usize) -> char {
        match c {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            _ => panic!("Invalid column"),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::with_capacity(1_000);

        result.push_str("  ");
        for i in 0..8 {
            result.push_str(" | ");
            result.push(Board::column_name(i));
        }
        result.push_str(" |");
        result.push('\n');

        for i in 0..8 {
            let row = &self.board[i];
            // result.push_str("   ");
            for _i in 0..9 {
                result.push_str("--- ");
            }
            result.push('\n');
            result.push(
                char::from_digit((i + 1) as u32, 10)
                    .expect("Invalid row")
            );
            result.push(' ');
            for c in 0..8 {
                result.push_str(" | ");
                if let Some(piece) = &row[c] {
                    result.push_str(&*piece.to_str());
                } else {
                    result.push(' ');
                }
            }
            result.push_str(" |");
            result.push('\n');
        }
        for _i in 0..9 {
            result.push_str("--- ");
        }
        write!(f, "{}", result)
    }
}
