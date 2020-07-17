pub mod game_macro;
pub mod map;
pub mod player;
pub mod token;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub maps: BTreeMap<map::MapKey, map::Map>,
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub players: player::PlayerMap,
}

// Functions to make accessing certain elements in the game easier.
impl Game {
    pub fn player_token(&self, key: &player::PlayerTokenKey) -> Option<&token::Token> {
        self.players.get(&key.player)?.owned_tokens.get(&key.token)
    }
}
