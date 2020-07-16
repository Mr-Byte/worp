use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Macro {
    id: String,
}

key!(MacroKey, Macro => id);

pub type MacroHotbar = [MacroKey; 12];
