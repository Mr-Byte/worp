pub mod action;
pub mod map;
pub mod player;
pub mod token;

use player::{Player, PlayerKey, PlayerTokenKey};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use token::Token;

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    #[serde(serialize_with = "toml::ser::tables_last")]
    maps: BTreeMap<map::MapKey, map::Map>,
    #[serde(serialize_with = "toml::ser::tables_last")]
    players: BTreeMap<PlayerKey, Player>,
}

impl Game {
    pub fn player_token(&self, key: &PlayerTokenKey) -> Option<&Token> {
        self.players.get(&key.player)?.token(&key.token)
    }
}
