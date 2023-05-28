use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};
use rand::Rng;
use crate::chess;

pub(crate) struct Games {
    games: RwLock<HashMap<u64, Arc<Game>>>,
}

impl Games {
    pub(crate) fn new() -> Self {
        Self {
            games: RwLock::new(HashMap::new()),
        }
    }

    pub(crate) fn new_game(&self) -> (u64, Weak<Game>) {
        loop {
            let id = rand::thread_rng().gen::<u64>();
            let mut games = self.games.write().unwrap();
            if let Entry::Vacant(e) = games.entry(id) {
                let game = Arc::new(Game::new());
                let game_ref = Arc::downgrade(&game);
                e.insert(game);

                break (id, game_ref);
            }
        }
    }
}

pub(crate) struct Game {
    game: chess::Game,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            game: chess::Game::new(),
        }
    }
}
