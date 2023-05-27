use serde::Serialize;

#[derive(Serialize)]
pub struct Piece {
    piece: PieceType,
    color: PieceColor,
}

impl Piece {
    pub fn new(piece: PieceType, color: PieceColor) -> Self {
        Self { piece, color }
    }

    pub fn to_str(&self) -> String {
        self.color.to_str(&self.piece)
    }
}

#[derive(Serialize)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }
}

#[derive(Serialize)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    fn to_str(&self, piece_type: &PieceType) -> String {
        match self {
            // White font on black background
            PieceColor::White => format!("\x1b[97m\x1b[40m{}\x1b[0m", piece_type.to_char()),
            // Black font on white background
            PieceColor::Black => format!("\x1b[90m\x1b[107m{}\x1b[0m", piece_type.to_char()),
        }
    }
}
