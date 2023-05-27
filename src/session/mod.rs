use std::collections::HashMap;
use std::sync::RwLock;
use rand::Rng;

pub struct Players {
    players: RwLock<HashMap<u64, String>>,
}

impl Players {
    pub fn new() -> Self {
        Self {
            players: RwLock::new(HashMap::new()),
        }
    }

    pub fn new_player(&self) -> u64 {
        let mut self_players = self.players.write().unwrap();
        loop {
            let id = rand::thread_rng().gen::<u64>();
            if !self_players.contains_key(&id) {
                self_players.insert(id, String::from(format!("{}", id)));
                break id;
            }
        }
    }

    pub fn player_list(&self) -> Vec<String> {
        let self_players = self.players.read().unwrap();
        self_players.values().cloned().collect()
    }

    pub fn contains(&self, id: u64) -> bool {
        let self_players = self.players.read().unwrap();
        self_players.contains_key(&id)
    }
}