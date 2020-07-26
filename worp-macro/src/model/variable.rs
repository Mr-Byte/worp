use super::{error::DocumentError, Expression};
use crate::{next_pair, parser::Rule};
use pest::iterators::Pairs;
use std::convert::{TryFrom, TryInto as _};

#[derive(Debug)]
pub struct Variable {
    name: String,
    expression: Expression,
}

impl TryFrom<Pairs<'_, Rule>> for Variable {
    type Error = DocumentError;

    fn try_from(mut variable_pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let variable_name_pair = next_pair!(variable_pairs => Rule::variable_name)?;
        let name = variable_name_pair.as_str().to_owned();
        let expression = variable_pairs.try_into()?;

        Ok(Variable { name, expression })
    }
}

#[derive(Debug)]
pub struct VariableList(Vec<Variable>);

impl TryFrom<Pairs<'_, Rule>> for VariableList {
    type Error = DocumentError;

    fn try_from(variables_pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let mut variables = Vec::new();

        for variable_pair in variables_pairs {
            let variable = variable_pair.into_inner().try_into()?;
            variables.push(variable);
        }

        Ok(VariableList(variables))
    }
}
