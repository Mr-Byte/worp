use super::error::DocumentError;
use crate::{next_pair, parser::Rule};
use pest::iterators::Pairs;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Expression(String);

impl TryFrom<Pairs<'_, Rule>> for Expression {
    type Error = DocumentError;

    fn try_from(mut expression_pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let expression_pair = next_pair!(expression_pairs => Rule::expression)?;
        let expression = expression_pair.into_inner().as_str().to_owned();

        Ok(Expression(expression))
    }
}
