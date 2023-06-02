use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time;
use std::time::UNIX_EPOCH;
use log::debug;
use rand::Rng;
use crate::errors::Error;

const LOGIN_TIMEOUT: u64 = 10 * 60; // seconds

pub(crate) struct Players {
    players: RwLock<HashMap<u64, Player>>,
}

impl Players {
    pub(crate) fn new() -> Self {
        Self {
            players: RwLock::new(HashMap::new()),
        }
    }

    pub(crate) fn new_player(&self) -> Result<u64, Error> {
        loop {
            let id = rand::thread_rng().gen::<u64>();
            let mut players = self.players.write().unwrap();
            if let Entry::Vacant(e) = players.entry(id) {
                e.insert(Player::new(id)?);
                break Ok(id);
            }
        }
    }

    pub(crate) fn player_list(&self) -> Vec<String> {
        let players = self.players.read().unwrap();
        players.values().map(|player| player.name.clone()).collect()
    }

    pub(crate) fn contains(&self, id: u64) -> bool {
        let players = self.players.read().unwrap();
        players.contains_key(&id)
    }

    #[allow(clippy::significant_drop_tightening)]
    pub(crate) fn rename(&self, id: u64, name: String) -> Result<(), Error> {
        let mut guard = self.players.write().unwrap();
        let  player = guard.get_mut(&id);
        match player {
            Some(player) => {
                player.update_name(name)?;
                Ok(())
            }
            None => Err(Error::Internal(format!("Player not found: {id}")))
        }
    }

    pub(crate) fn cleanup(&self) {
        let mut players = self.players.write().unwrap();
        let epoch = time::SystemTime::now().duration_since(UNIX_EPOCH)
                .map_err(Error::SystemTime).unwrap().as_secs();
        players.retain(|_, player| {
            let result = player.last_seen + LOGIN_TIMEOUT > epoch;
            if !result {debug!("Player {} removed", player.name);}
            result
        });
    }
}

#[derive(Clone)]
struct Player {
    name: String,
    games: Vec<u64>,
    last_seen: u64,
}

impl Player {
    fn new(id: u64) -> Result<Self, Error> {
        let epoch = time::SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(Error::SystemTime)?.as_secs();
        Ok(Self {
            name: format!("{id}"),
            games: Vec::new(),
            last_seen: epoch,
        })
    }

    fn update_name(&mut self, name: String) -> Result<(), Error> {
        let re = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
        if re.is_match(&name) {
            self.name = name;
            Ok(())
        } else {
            Err(Error::Internal(format!("Invalid name: {name}")))
        }
    }
}
