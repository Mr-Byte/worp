use crate::parser;

#[derive(thiserror::Error, Debug)]
pub enum DocumentError {
    #[error(transparent)]
    ParseError(#[from] pest::error::Error<parser::Rule>),
}
