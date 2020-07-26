use crate::parser;

#[derive(thiserror::Error, Debug)]
pub enum DocumentError {
    #[error(transparent)]
    ParseError(#[from] pest::error::Error<parser::Rule>),
    #[error("The macro document was unexpectedly malformed. Reason: {0}")]
    Malformed(String),
}

impl DocumentError {
    pub(super) fn malformed(reason: &'static str) -> impl Fn() -> Self {
        move || Self::Malformed(String::from(reason))
    }
}
