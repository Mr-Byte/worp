use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Symbol(Cow<'static, str>);

impl Symbol {
    pub fn new(value: impl Into<String>) -> Self {
        Symbol(Cow::Owned(value.into()))
    }

    pub const fn new_static(value: &'static str) -> Self {
        Symbol(Cow::Borrowed(value))
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(fmt)
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Symbol::new(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn static_borrowed_should_equal_owned_value() {
        let borrowed = Symbol::new_static("test");
        let owned = Symbol::new("test");

        assert_eq!(borrowed, owned);
    }

    #[test]
    fn to_string_should_produce_internal_strings_value() {
        let symbol = Symbol::new("test");
        let as_string = symbol.to_string();

        assert_eq!("test", as_string);
    }
}
