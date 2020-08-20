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

pub mod common {
    use super::*;

    pub mod operators {
        use super::*;

        pub const OP_NEG: Symbol = Symbol::new_static("#op_neg");
        pub const OP_MUL: Symbol = Symbol::new_static("#op_mul");
        pub const OP_DIV: Symbol = Symbol::new_static("#op_div");
        pub const OP_REM: Symbol = Symbol::new_static("#op_rem");
        pub const OP_ADD: Symbol = Symbol::new_static("#op_add");
        pub const OP_SUB: Symbol = Symbol::new_static("#op_sub");

        pub const OP_GT: Symbol = Symbol::new_static("#op_gt");
        pub const OP_LT: Symbol = Symbol::new_static("#op_lt");
        pub const OP_GTE: Symbol = Symbol::new_static("#op_gte");
        pub const OP_LTE: Symbol = Symbol::new_static("#op_lte");
        pub const OP_EQ: Symbol = Symbol::new_static("#op_eq");
        pub const OP_NEQ: Symbol = Symbol::new_static("#op_ne");

        pub const OP_NOT: Symbol = Symbol::new_static("#op_not");
    }

    pub mod methods {
        use super::*;

        pub const FN_TO_STRING: Symbol = Symbol::new_static("to_string");
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
