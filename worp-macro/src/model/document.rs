use super::{error::DocumentError, Definition, DefinitionList};
use crate::{
    next_pair,
    parser::{MacroDocumentParser, Rule},
};
use pest::iterators::Pairs;
use pest::Parser as _;
use std::convert::{TryFrom, TryInto as _};

#[derive(Debug)]
pub struct Document {
    pub main_macro: Definition,
    pub sub_macros: DefinitionList,
}

impl Document {
    pub fn try_from_str(input: &str) -> Result<Document, DocumentError> {
        let mut parsed_input = MacroDocumentParser::parse(Rule::document, input)?;
        let document_pair = next_pair!(parsed_input => Rule::document)?;

        document_pair.into_inner().try_into()
    }
}

impl TryFrom<Pairs<'_, Rule>> for Document {
    type Error = DocumentError;

    fn try_from(mut document_pairs: Pairs<Rule>) -> Result<Self, Self::Error> {
        let main_macro_pair = next_pair!(document_pairs => Rule::main_macro)?;
        let main_macro = main_macro_pair.into_inner().try_into()?;

        let sub_macros_list_pair = next_pair!(document_pairs => Rule::sub_macro_list)?;
        let sub_macros = sub_macros_list_pair.into_inner().try_into()?;

        let document = Document { main_macro, sub_macros };

        Ok(document)
    }
}
