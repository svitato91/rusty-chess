use crate::chess::board::Board;
mod board;

pub struct ChessGame {
    board: Board
}

impl ChessGame {
    pub fn new() -> Self {
        Self { board: Board::new() }
    }

    pub fn board_terminal(&self) -> String {
        self.board.to_string()
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}