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

pub mod common {
    pub mod types {
        use crate::interpreter::symbol::Symbol;

        pub const TY_NONE: Symbol = Symbol::new_static("None");
        pub const TY_BOOL: Symbol = Symbol::new_static("Bool");
        pub const TY_INT: Symbol = Symbol::new_static("Int");
        pub const TY_FLOAT: Symbol = Symbol::new_static("Float");
        pub const TY_STRING: Symbol = Symbol::new_static("String");
        pub const TY_LIST: Symbol = Symbol::new_static("List");
        pub const TY_FUNC: Symbol = Symbol::new_static("Function");
        pub const TY_OBJECT: Symbol = Symbol::new_static("Object");
    }

    pub mod operators {
        use crate::interpreter::symbol::Symbol;

        pub const OP_NEG: Symbol = Symbol::new_static("#op_neg");
        pub const OP_MUL: Symbol = Symbol::new_static("#op_mul");
        pub const OP_DIV: Symbol = Symbol::new_static("#op_div");
        pub const OP_REM: Symbol = Symbol::new_static("#op_rem");
        pub const OP_ADD: Symbol = Symbol::new_static("#op_add");
        pub const OP_SUB: Symbol = Symbol::new_static("#op_sub");
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
