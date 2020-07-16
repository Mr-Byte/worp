use super::key::Key;
use hex::HexGridMap;
use serde::{Deserialize, Serialize};
use square::SquareGridMap;
use std::collections::BTreeMap;

pub mod hex;
pub mod instance;
pub mod square;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Map {
    Square(Box<SquareGridMap>),
    Hex(Box<HexGridMap>),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub struct MapKey(String);

impl Key for Map {
    type Key = MapKey;

    fn key(&self) -> MapKey {
        match self {
            Map::Square(inner) => inner.key(),
            Map::Hex(inner) => inner.key(),
        }
    }
}

impl Into<Map> for SquareGridMap {
    fn into(self) -> Map {
        Map::Square(Box::new(self))
    }
}

impl Into<Map> for HexGridMap {
    fn into(self) -> Map {
        Map::Hex(Box::new(self))
    }
}

pub type MapMap = BTreeMap<MapKey, Map>;
