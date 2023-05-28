use crate::chess::board::Board;
mod board;

pub(crate) struct Game {
    board: Board
}

impl Game {
    pub(crate) fn new() -> Self {
        Self { board: Board::new() }
    }

    fn board_terminal(&self) -> String {
        self.board.to_string()
    }

    const fn board(&self) -> &Board {
        &self.board
    }
}
