use crate::runtime::core::symbol::Symbol;
use gc::{Finalize, Trace};
use std::fmt::Display;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Trace, Finalize)]
pub enum ValueKey {
    Symbol(#[unsafe_ignore_trace] Symbol),
    Index(i64),
}

impl<T> From<T> for ValueKey
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        ValueKey::Symbol(Symbol::new(value))
    }
}

impl Display for ValueKey {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueKey::Symbol(symbol) => write!(fmt, r#""{}""#, symbol),
            ValueKey::Index(index) => write!(fmt, "[{}]", index),
        }
    }
}
