use crate::chess::board::Board;
mod board;

struct ChessGame {
    board: Board
}

impl ChessGame {
    fn new() -> Self {
        Self { board: Board::new() }
    }

    fn board_terminal(&self) -> String {
        self.board.to_string()
    }

    const fn board(&self) -> &Board {
        &self.board
    }
}
