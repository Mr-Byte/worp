use super::{
    action::MacroKey,
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
    current_map: Option<MapKey>,
    macros: Vec<MacroKey>,
    #[serde(serialize_with = "toml::ser::tables_last")]
    owned_tokens: BTreeMap<TokenKey, Token>,
}

impl Player {
    pub fn new(id: impl Into<String>, owned_tokens: Vec<Token>) -> Self {
        Self {
            id: id.into(),
            current_map: None,
            owned_tokens: owned_tokens.into_iter().map(|token| (token.key(), token)).collect(),
            macros: Vec::new(),
        }
    }
}

impl Player {
    #[inline]
    pub fn token(&self, key: &TokenKey) -> Option<&Token> {
        self.owned_tokens.get(&key)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub struct PlayerTokenKey {
    pub(in crate::model::game) player: PlayerKey,
    pub(in crate::model::game) token: TokenKey,
}

impl PlayerTokenKey {
    pub fn new(player: PlayerKey, token: TokenKey) -> Self {
        Self { player, token }
    }
}
