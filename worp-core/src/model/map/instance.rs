use super::super::player::PlayerTokenKey;
use super::super::token::TokenKey;
use super::MapKey;
use crate::model::token::TokenPosition;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

key!(MapInstanceKey, MapInstance => id);

#[derive(Debug, Serialize, Deserialize)]
pub struct MapInstance {
    id: String,
    pub map: MapKey,
    pub player_tokens: BTreeMap<PlayerTokenKey, TokenPosition>,
    pub tokens: BTreeMap<TokenKey, TokenPosition>,
}

pub type MapInstanceMap = BTreeMap<MapInstanceKey, MapInstance>;
