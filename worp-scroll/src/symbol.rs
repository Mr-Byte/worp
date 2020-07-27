use crate::{error::DocumentError, next_pair, parser::Rule};
use pest::iterators::Pairs;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Symbol {
    Variable(String),
    Macro(String),
}

impl TryFrom<Pairs<'_, Rule>> for Symbol {
    type Error = DocumentError;

    fn try_from(mut value: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let result = match value.next().map(|pair| pair.as_rule()) {
            Some(Rule::macro_name_indicator) => Symbol::Macro(next_pair!(value => Rule::identifier).as_str().to_owned()),
            Some(Rule::variable_name_indicator) => Symbol::Variable(next_pair!(value => Rule::identifier).as_str().to_owned()),
            _ => unreachable!(),
        };

        Ok(result)
    }
}
