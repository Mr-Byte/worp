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
    pub mod operator {
        use crate::interpreter::symbol::Symbol;

        pub const OP_NEG: Symbol = Symbol::new_static("#op_neg");
        pub const OP_MUL: Symbol = Symbol::new_static("#op_mul");
        pub const OP_DIV: Symbol = Symbol::new_static("#op_div");
        pub const OP_REM: Symbol = Symbol::new_static("#op_rem");
        pub const OP_ADD: Symbol = Symbol::new_static("#op_add");
        pub const OP_SUB: Symbol = Symbol::new_static("#op_sub");
    }
}
