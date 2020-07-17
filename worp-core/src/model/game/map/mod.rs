use super::player;
use super::token;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

key!(Map::id as MapKey: String);

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum MapType {
    Square,
    Hex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    id: String,
    pub map: MapKey,
    pub map_type: MapType,
    pub player_tokens: BTreeMap<player::PlayerTokenKey, token::TokenPosition>,
    pub tokens: BTreeMap<token::TokenKey, token::TokenPosition>,
}
