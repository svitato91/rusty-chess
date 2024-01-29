use dashmap::DashMap;

pub(crate) struct AppState {
    players: DashMap<u64, Player>,
    games: DashMap<u64, Game>,
}

pub(crate) struct Player {
    name: Option<String>,
    games: Vec<u64>,
}

pub(crate) struct Game {
    white: u64,
    black: u64,
    state: GameState,
}

pub(crate) struct GameState {
    // todo
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self {
            players: DashMap::new(),
            games: DashMap::new(),
        }
    }
}
