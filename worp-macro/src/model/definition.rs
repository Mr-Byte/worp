use super::{error::DocumentError, variable::VariableList, SpanList};
use crate::{next_pair, parser::Rule};
use pest::iterators::Pairs;
use std::{
    convert::{TryFrom, TryInto as _},
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct Definition {
    pub name: Option<String>,
    pub variables: VariableList,
    pub body: SpanList,
}

impl TryFrom<Pairs<'_, Rule>> for Definition {
    type Error = DocumentError;

    fn try_from(mut macro_definition_pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let name = match macro_definition_pairs.peek() {
            Some(pair) if pair.as_rule() == Rule::sub_macro_header => {
                let sub_macro_header_pair = next_pair!(macro_definition_pairs => Rule::sub_macro_header)?;
                let sub_macro_name = next_pair!(sub_macro_header_pair.into_inner() => Rule::macro_name)?.as_str().to_owned();

                Some(sub_macro_name)
            }
            _ => None,
        };

        let variables_header_pair = next_pair!(macro_definition_pairs => Rule::variable_header)?;
        let variables = variables_header_pair.into_inner().try_into()?;

        let body_pair = next_pair!(macro_definition_pairs => Rule::macro_body)?;
        let body = body_pair.into_inner().try_into()?;

        let definition = Definition { name, variables, body };

        Ok(definition)
    }
}

#[derive(Debug)]
pub struct DefinitionList(Vec<Definition>);

impl Deref for DefinitionList {
    type Target = [Definition];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DefinitionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<Pairs<'_, Rule>> for DefinitionList {
    type Error = DocumentError;

    fn try_from(pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let mut macro_definitions = vec![];

        for next_pair in pairs {
            if let Rule::sub_macro = next_pair.as_rule() {
                let sub_macro = next_pair.into_inner().try_into()?;
                macro_definitions.push(sub_macro);
            } else {
                return Err(DocumentError::Malformed(format!(
                    "Unexpected rule while parsing sub-macros: {:?}.",
                    next_pair.as_rule()
                )));
            }
        }

        Ok(DefinitionList(macro_definitions))
    }
}
