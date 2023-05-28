use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Piece {
    piece: Type,
    color: Color,
}

impl Piece {
    pub(crate) const fn new(piece: Type, color: Color) -> Self {
        Self { piece, color }
    }

    pub(crate) fn to_str(&self) -> String {
        self.color.to_str(&self.piece)
    }
}

#[derive(Serialize)]
pub(crate) enum Type {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Type {
    const fn to_char(&self) -> char {
        match self {
            Self::Pawn => 'p',
            Self::Rook => 'r',
            Self::Knight => 'n',
            Self::Bishop => 'b',
            Self::Queen => 'q',
            Self::King => 'k',
        }
    }
}

#[derive(Serialize)]
pub(crate) enum Color {
    White,
    Black,
}

impl Color {
    fn to_str(&self, piece_type: &Type) -> String {
        match self {
            // White font on black background
            Self::White => format!("\x1b[97m\x1b[40m{}\x1b[0m", piece_type.to_char()),
            // Black font on white background
            Self::Black => format!("\x1b[90m\x1b[107m{}\x1b[0m", piece_type.to_char()),
        }
    }
}
