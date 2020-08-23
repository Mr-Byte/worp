use super::lexer::Token;

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
}
