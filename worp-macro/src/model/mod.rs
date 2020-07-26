mod definition;
mod document;
pub mod error;
mod expression;
mod link;
mod span;
mod variable;

pub use definition::{Definition, DefinitionList};
pub use document::Document;
pub use expression::Expression;
pub use link::{LabeledTarget, Link, LinkTarget};
pub use span::{Span, SpanList};
pub use variable::{Variable, VariableList};

#[macro_use]
#[doc(hidden)]
mod macros {
    #[macro_export]
    macro_rules! next_pair {
        ($input:expr => $rule:expr) => {
            $input
                .next()
                .filter(|token| token.as_rule() == $rule)
                .ok_or_else($crate::model::error::DocumentError::malformed(concat!(
                    "Unexpected rule. Expected ",
                    stringify!($rule),
                    "."
                )))
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let _result = Document::try_from_str(include_str!("../../test/data/long_sword_basic_attack.txt")).unwrap();
    }
}
