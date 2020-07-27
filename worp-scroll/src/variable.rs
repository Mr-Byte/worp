use super::{error::DocumentError, Expression};
use crate::{next_pair, parser::Rule, Symbol};
use pest::iterators::Pairs;
use std::{
    convert::{TryFrom, TryInto as _},
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct Variable {
    pub name: Symbol,
    pub expression: Expression,
}

impl TryFrom<Pairs<'_, Rule>> for Variable {
    type Error = DocumentError;

    fn try_from(mut variable_pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let name = next_pair!(variable_pairs => Rule::variable_name).into_inner().try_into()?;
        let expression = variable_pairs.try_into()?;

        Ok(Variable { name, expression })
    }
}

#[derive(Debug)]
pub struct VariableList(Vec<Variable>);

impl Deref for VariableList {
    type Target = [Variable];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VariableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
