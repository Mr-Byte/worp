use super::{
    map::{instance::MapInstanceMap, MapMap},
    player::{PlayerMap, PlayerTokenKey},
    token::Token,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub maps: MapMap,
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub map_instances: MapInstanceMap,
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub players: PlayerMap,
}

// Functions to make accessing certain elements in the game easier.
impl Game {
    pub fn player_token(&self, key: &PlayerTokenKey) -> Option<&Token> {
        self.players.get(&key.player)?.owned_tokens.get(&key.token)
    }
}
