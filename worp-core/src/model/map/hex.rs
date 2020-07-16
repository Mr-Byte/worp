use super::MapKey;
use crate::model::key::Key;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HexGridMap {
    id: String,
    pub image: String,
}

impl HexGridMap {
    pub fn new(id: impl Into<String>, image: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            image: image.into(),
        }
    }
}

impl Key for HexGridMap {
    type Key = MapKey;

    fn key(&self) -> Self::Key {
        MapKey(self.id.clone())
    }
}
