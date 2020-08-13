use super::{ParseResult, Parser};
use crate::syntax::{lexer::TokenKind, SyntaxTree};

impl<'a> Parser<'a> {
    pub(super) fn parse_statements(&mut self) -> ParseResult {
        // TODO: Parse list of expression statements.
        match self.next_token.kind {
            TokenKind::If => self.parse_if_expression(),
            _ => self.parse_expression(),
        }
    }

    fn parse_if_expression(&mut self) -> ParseResult {
        self.consume(&[TokenKind::If])?;

        let condition = self.parse_expression()?;
        let primary_condition = self.parse_block()?;
        let else_condition = {
            if self.next_token.is_kind(TokenKind::Else) {
                self.consume(&[TokenKind::Else])?;

                if self.next_token.is_kind(TokenKind::If) {
                    Some(self.parse_if_expression()?)
                } else {
                    Some(self.parse_block()?)
                }
            } else {
                None
            }
        };

        Ok(SyntaxTree::Conditional(
            Box::new(condition),
            Box::new(primary_condition),
            else_condition.map(Box::new),
        ))
    }

    fn parse_block(&mut self) -> ParseResult {
        self.consume(&[TokenKind::LeftCurly])?;
        let result = self.parse_statements()?;
        self.consume(&[TokenKind::RightCurly])?;

        Ok(result)
    }
}
