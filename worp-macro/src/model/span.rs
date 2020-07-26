use super::{error::DocumentError, Expression, Link};
use crate::next_pair;
use crate::parser::Rule;
use pest::iterators::Pairs;
use std::{
    convert::{TryFrom, TryInto as _},
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub enum Span {
    RawText(String),
    Expression(Expression),
    // TODO: Should there be types to represent variable names and macro names?
    // If so, do they belong in here or a more generic model crate?
    VariableReference(String),
    MacroReference(String),
    BoldText(SpanList),
    ItalicText(SpanList),
    UnderlineText(SpanList),
    StrikeThroughText(SpanList),
    Link(Link),
}

impl TryFrom<Pairs<'_, Rule>> for Span {
    type Error = DocumentError;

    fn try_from(mut span_pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let rule = span_pairs
            .peek()
            .map(|pair| pair.as_rule())
            .ok_or_else(DocumentError::malformed("Unexpected end of document."))?;

        let result = match rule {
            Rule::raw_text => {
                let raw_text = next_pair!(span_pairs => Rule::raw_text)?.as_str().to_owned();
                Span::RawText(raw_text)
            }
            Rule::macro_name => {
                let reference = next_pair!(span_pairs => Rule::macro_name)?.as_str().to_owned();
                Span::MacroReference(reference)
            }
            Rule::variable_name => {
                let reference = next_pair!(span_pairs => Rule::variable_name)?.as_str().to_owned();
                Span::VariableReference(reference)
            }
            Rule::expression => {
                let expression = span_pairs.try_into()?;
                Span::Expression(expression)
            }
            Rule::bold_text => {
                let bold_text = next_pair!(span_pairs => Rule::bold_text)?.into_inner().try_into()?;
                Span::BoldText(bold_text)
            }
            Rule::italic_text => {
                let bold_text = next_pair!(span_pairs => Rule::italic_text)?.into_inner().try_into()?;
                Span::ItalicText(bold_text)
            }
            Rule::underline_text => {
                let bold_text = next_pair!(span_pairs => Rule::underline_text)?.into_inner().try_into()?;
                Span::UnderlineText(bold_text)
            }
            Rule::strike_through_text => {
                let bold_text = next_pair!(span_pairs => Rule::strike_through_text)?.into_inner().try_into()?;
                Span::StrikeThroughText(bold_text)
            }
            Rule::macro_link => {
                let link = next_pair!(span_pairs => Rule::macro_link)?.into_inner().try_into()?;
                Span::Link(link)
            }
            _ => unreachable!(),
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct SpanList(Vec<Span>);

impl Deref for SpanList {
    type Target = [Span];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SpanList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<Pairs<'_, Rule>> for SpanList {
    type Error = DocumentError;

    fn try_from(spans_pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let mut spans = vec![];

        for spans_pair in spans_pairs {
            if spans_pair.as_rule() == Rule::macro_span {
                let span = spans_pair.into_inner().try_into()?;
                spans.push(span);
            } else {
                unreachable!()
            }
        }

        Ok(SpanList(spans))
    }
}
