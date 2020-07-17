use serde::{Deserialize, Serialize};

key!(TokenTemplate::id as TokenTemplateKey: String);

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenTemplate {
    id: String,
}

impl TokenTemplate {
    pub fn key(&self) -> TokenTemplateKey {
        TokenTemplateKey(self.id.clone())
    }
}
