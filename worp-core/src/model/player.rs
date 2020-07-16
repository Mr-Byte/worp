use super::{
    game_macro::MacroKey,
    key::Key as _,
    map::instance::MapInstanceKey,
    token::{Token, TokenKey},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    id: String,
    pub current_map_instance: Option<MapInstanceKey>,
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

key!(PlayerKey, Player => id);

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
