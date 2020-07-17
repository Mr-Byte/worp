use serde::{Deserialize, Serialize};

key!(Macro::id as MacroKey: String);

#[derive(Debug, Deserialize, Serialize)]
pub struct Macro {
    id: String,
}

pub type MacroHotbar = [MacroKey; 12];
