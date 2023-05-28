use std::fmt;
use std::fmt::Formatter;
use serde::Serialize;
use crate::chess::board::piece::Piece;
use crate::errors::Error;

mod piece;

#[derive(Serialize)]
pub(crate) struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub(crate) fn new() -> Self {
        let mut board = Self { board: Default::default() };
        board.initialize_pieces();
        board
    }

    #[allow(clippy::indexing_slicing)]
    fn initialize_pieces(&mut self) {
        // Init pawns
        for i in 0..8 {
            self.board[1][i] = Some(Piece::new(piece::Type::Pawn, piece::Color::White));
            self.board[6][i] = Some(Piece::new(piece::Type::Pawn, piece::Color::Black));
        }

        // Rooks
        self.board[0][0] = Some(Piece::new(piece::Type::Rook, piece::Color::White));
        self.board[0][7] = Some(Piece::new(piece::Type::Rook, piece::Color::White));
        self.board[7][7] = Some(Piece::new(piece::Type::Rook, piece::Color::Black));
        self.board[7][0] = Some(Piece::new(piece::Type::Rook, piece::Color::Black));

        // Knights
        self.board[0][1] = Some(Piece::new(piece::Type::Knight, piece::Color::White));
        self.board[0][6] = Some(Piece::new(piece::Type::Knight, piece::Color::White));
        self.board[7][6] = Some(Piece::new(piece::Type::Knight, piece::Color::Black));
        self.board[7][1] = Some(Piece::new(piece::Type::Knight, piece::Color::Black));

        // Bishops
        self.board[0][2] = Some(Piece::new(piece::Type::Bishop, piece::Color::White));
        self.board[0][5] = Some(Piece::new(piece::Type::Bishop, piece::Color::White));
        self.board[7][5] = Some(Piece::new(piece::Type::Bishop, piece::Color::Black));
        self.board[7][2] = Some(Piece::new(piece::Type::Bishop, piece::Color::Black));

        // Queens
        self.board[0][3] = Some(Piece::new(piece::Type::Queen, piece::Color::White));
        self.board[7][3] = Some(Piece::new(piece::Type::Queen, piece::Color::Black));

        // Kings
        self.board[0][4] = Some(Piece::new(piece::Type::King, piece::Color::White));
        self.board[7][4] = Some(Piece::new(piece::Type::King, piece::Color::Black));
    }

    fn column_name(c: usize) -> Result<char, Error> {
        match c {
            0 => Ok('A'),
            1 => Ok('B'),
            2 => Ok('C'),
            3 => Ok('D'),
            4 => Ok('E'),
            5 => Ok('F'),
            6 => Ok('G'),
            7 => Ok('H'),
            _ => Err(Error::Internal(format!("Invalid column: {c}"))),
        }
    }
}

impl fmt::Display for Board {
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::map_err_ignore)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::with_capacity(1_000);

        result.push_str("  ");
        for i in 0..8 {
            result.push_str(" | ");
            result.push(Self::column_name(i)
                .map_err(|_| fmt::Error {})?);
        }
        result.push_str(" |");
        result.push('\n');

        for i in 0..8 {
            let row = &self.board[i];
            // result.push_str("   ");
            for _i in 0..9_u8 {
                result.push_str("--- ");
            }
            result.push('\n');
            #[allow(clippy::as_conversions)]
            #[allow(clippy::cast_possible_truncation)]
            result.push(
                char::from_digit((i + 1) as u32, 10)
                    .ok_or(fmt::Error {})?
            );
            result.push(' ');
            for piece in row {
                result.push_str(" | ");
                match piece {
                    Some(piece) => result.push_str(&piece.to_str()),
                    None => result.push(' ')
                }
            }
            result.push_str(" |");
            result.push('\n');
        }
        for _i in 0..9 {
            result.push_str("--- ");
        }
        write!(f, "{result}")
    }
}
