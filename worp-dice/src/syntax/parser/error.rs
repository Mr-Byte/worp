use crate::syntax::{lexer::TokenKind, span::Span};
use std::{
    error::Error,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug)]
pub enum ErrorKind {
    InvalidIntegerLiteral,
    InvalidFloatLiteral,
    ReservedKeyword { keyword: TokenKind },
    UnexpectedToken { expected: Vec<TokenKind>, found: TokenKind },
    UnknownToken { value: String },
}

#[derive(thiserror::Error, Debug)]
#[error("Parse Error.")]
pub struct ParserError {
    pub kind: ErrorKind,
    span: Option<Span>,
    source: Option<Box<dyn Error>>,
}

impl ParserError {
    pub fn new(kind: ErrorKind, span: Option<Span>) -> Self {
        Self { kind, span, source: None }
    }

    pub fn unexpected_token(found: TokenKind, expected: &[TokenKind], span: Option<Span>) -> Self {
        Self {
            kind: ErrorKind::UnexpectedToken {
                found,
                expected: expected.to_owned(),
            },
            span,
            source: None,
        }
    }
}

impl From<ParseIntError> for ParserError {
    fn from(error: ParseIntError) -> Self {
        ParserError {
            kind: ErrorKind::InvalidIntegerLiteral,
            span: None,
            source: Some(Box::new(error)),
        }
    }
}

impl From<ParseFloatError> for ParserError {
    fn from(error: ParseFloatError) -> Self {
        ParserError {
            kind: ErrorKind::InvalidFloatLiteral,
            span: None,
            source: Some(Box::new(error)),
        }
    }
}
