use dashmap::DashMap;
use rand::Rng;
use crate::errors::Error;

pub(crate) struct Players {
    players: DashMap<u64, Player>,
}

impl Players {
    pub(crate) fn new() -> Self {
        Self {
            players: DashMap::new(),
        }
    }

    pub(crate) fn new_player(&self) -> u64 {
        loop {
            let id = rand::thread_rng().gen::<u64>();
            if !self.players.contains_key(&id) {
                self.players.insert(id, Player::new(&id));
                break id;
            }
        }
    }

    pub(crate) fn player_list(&self) -> Vec<String> {
        self.players.iter().map(|player| player.name.clone()).collect()
    }

    pub(crate) fn contains(&self, id: u64) -> bool {
        self.players.contains_key(&id)
    }

    pub(crate) fn rename(&self, id: u64, name: String) -> Result<(), Error> {
        match self.players.get_mut(&id) {
            Some(mut player) => {
                player.value_mut().update_name(name)?;
                Ok(())
            }
            None => Err(Error::Internal(format!("Player not found: {}", id)))
        }
    }
}

#[derive(Clone)]
struct Player {
    name: String,
    games: Vec<u64>,
}

impl Player {
    fn new(id: &u64) -> Self {
        Self {
            name: format!("{}", id),
            games: Vec::new(),
        }
    }

    fn update_name(&mut self, name: String) -> Result<(), Error> {
        let re = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
        if re.is_match(&name) {
            self.name = name;
            Ok(())
        } else {
            Err(Error::Internal(format!("Invalid name: {}", name)))
        }
    }
}
