use super::action::{MacroHotbar, MacroKey};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

key!(Token::id as TokenKey: String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    id: String,
    macro_hotbar: Vec<MacroHotbar>,
    macros: Vec<MacroKey>,
    #[serde(serialize_with = "toml::ser::tables_last")]
    attributes: BTreeMap<String, f64>,
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
