use super::{
    player::PlayerTokenKey,
    token::{TokenKey, TokenPosition},
};
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
    map_type: MapType,
    player_tokens: BTreeMap<PlayerTokenKey, TokenPosition>,
    tokens: BTreeMap<TokenKey, TokenPosition>,
}
