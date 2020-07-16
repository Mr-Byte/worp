key!(TokenTemplateKey);

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenTemplate {
    id: String,
}

impl TokenTemplate {
    pub fn key(&self) -> TokenTemplateKey {
        TokenTemplateKey(self.id.clone())
    }
}

pub type TokenTemplateMap = BTreeMap<TokenTemplateKey, TokenTemplate>;
