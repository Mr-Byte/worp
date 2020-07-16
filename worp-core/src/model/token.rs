use super::game_macro::{MacroHotbar, MacroKey};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

key!(TokenKey, Token => id);

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    id: String,
    pub macro_hotbar: Vec<MacroHotbar>,
    pub macros: Vec<MacroKey>,
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub attributes: BTreeMap<String, f64>,
}

impl Token {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            macro_hotbar: Default::default(),
            attributes: BTreeMap::new(),
            macros: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPosition(pub u64, pub u64);
