mod definition;
mod document;
pub mod error;
mod expression;
mod link;
mod parser;
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
                .unwrap_or_else(|| unreachable!())
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let inputs = &[
            include_str!("../test/data/nonsense.txt"),
            include_str!("../test/data/more_nonsense.txt"),
            include_str!("../test/data/long_sword_basic_attack.txt"),
            include_str!("../test/data/long_sword_multiple_attack.txt"),
            include_str!("../test/data/eblast.txt"),
        ];

        for input in inputs {
            let _result = Document::try_from_str(input).unwrap();
        }
    }
}
