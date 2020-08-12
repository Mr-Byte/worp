use super::{error::ErrorKind, ParseResult, Parser};
use crate::{
    runtime::core::Symbol,
    syntax::{lexer::TokenKind, ParserError, SyntaxTree},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_field_access(&mut self) -> ParseResult {
        let mut expression = self.parse_literal()?;

        while self.next_token.is_any_kind(&[TokenKind::Dot, TokenKind::SafeDot]) {
            let span_start = self.current_token.span.clone();
            self.next();
            let operator = self.current_token.clone();
            self.next();

            if self.current_token.is_kind(TokenKind::Identifier) {
                let symbol: Symbol = self.current_token.slice().into();
                let span_end = self.current_token.span.clone();

                expression = match operator.kind {
                    TokenKind::Dot => SyntaxTree::FieldAccess(Box::new(expression), symbol, span_start + span_end),
                    TokenKind::SafeDot => SyntaxTree::SafeAccess(Box::new(expression), symbol, span_start + span_end),
                    _ => unreachable!(),
                };
            } else {
                return Err(ParserError::new(
                    ErrorKind::UnexpectedToken {
                        expected: vec![TokenKind::Identifier],
                        found: self.current_token.kind.clone(),
                    },
                    Some(self.current_token.span.clone()),
                ));
            }
        }

        Ok(expression)
    }
}
