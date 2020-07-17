use super::{
    game_macro::MacroKey,
    map::MapKey,
    token::{Token, TokenKey},
};
use crate::model::key::Key as _;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

key!(Player::id as PlayerKey: String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    id: String,
    pub current_map_instance: Option<MapKey>,
    pub macros: Vec<MacroKey>,
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub owned_tokens: BTreeMap<TokenKey, Token>,
}

impl Player {
    pub fn new(id: impl Into<String>, owned_tokens: Vec<Token>) -> Self {
        Self {
            id: id.into(),
            current_map_instance: None,
            owned_tokens: owned_tokens.into_iter().map(|token| (token.key(), token)).collect(),
            macros: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub struct PlayerTokenKey {
    pub(super) player: PlayerKey,
    pub(super) token: TokenKey,
}

impl PlayerTokenKey {
    // TODO: Should this enforce the existence of the given token on the given player?
    pub fn new(player: PlayerKey, token: TokenKey) -> Self {
        Self { player, token }
    }
}

pub type PlayerMap = BTreeMap<PlayerKey, Player>;
