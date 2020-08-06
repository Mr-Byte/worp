use crate::runtime::symbol::Symbol;
use std::fmt::Display;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ObjectKey {
    Symbol(Symbol),
    Index(i64),
}

impl<T> From<T> for ObjectKey
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        ObjectKey::Symbol(Symbol::new(value))
    }
}

impl Display for ObjectKey {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectKey::Symbol(symbol) => write!(fmt, r#""{}""#, symbol),
            ObjectKey::Index(index) => write!(fmt, "[{}]", index),
        }
    }
}
