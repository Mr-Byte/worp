use crate::runtime::core::Span;

use super::lexer::Token;

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),

    #[error("Function {0} has too many arguments (max 255).")]
    TooManyArguments(String, Span),
}
