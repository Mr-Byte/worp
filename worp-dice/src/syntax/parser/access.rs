use super::{error::ErrorKind, ParseResult, Parser};
use crate::{
    runtime::core::Symbol,
    syntax::{lexer::TokenKind, ParserError, SyntaxTree},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_accessor(&mut self) -> ParseResult {
        let mut expression = self.parse_literal()?;

        while self
            .next_token
            .is_any_kind(&[TokenKind::Dot, TokenKind::SafeDot, TokenKind::LeftParen, TokenKind::LeftSquare])
        {
            self.next();

            match &self.current_token.kind {
                TokenKind::Dot | TokenKind::SafeDot => expression = self.parse_field_access(expression)?,
                TokenKind::LeftParen | TokenKind::LeftSquare => todo!(),
                _ => unreachable!(),
            }
        }

        Ok(expression)
    }

    fn parse_field_access(&mut self, expression: SyntaxTree) -> ParseResult {
        let span_start = self.current_token.span.clone();
        let operator = self.current_token.clone();
        self.next();

        if self.current_token.is_kind(TokenKind::Identifier) {
            let symbol: Symbol = self.current_token.slice().into();
            let span_end = self.current_token.span.clone();

            let result = match operator.kind {
                TokenKind::Dot => SyntaxTree::FieldAccess(Box::new(expression), symbol, span_start + span_end),
                TokenKind::SafeDot => SyntaxTree::SafeAccess(Box::new(expression), symbol, span_start + span_end),
                _ => unreachable!(),
            };

            Ok(result)
        } else {
            Err(ParserError::new(
                ErrorKind::UnexpectedToken {
                    expected: vec![TokenKind::Identifier],
                    found: self.current_token.kind,
                },
                Some(self.current_token.span.clone()),
            ))
        }
    }
}
