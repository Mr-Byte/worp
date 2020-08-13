use super::{ParseResult, Parser};
use crate::syntax::{lexer::TokenKind, SyntaxTree};

impl<'a> Parser<'a> {
    pub(super) fn parse_statements(&mut self) -> ParseResult {
        let mut statements = Vec::new();

        loop {
            if self.next_token.kind == TokenKind::If {
                statements.push(self.parse_if_expression()?);

                if self
                    .next_token
                    .is_any_kind(&[TokenKind::RightParen, TokenKind::RightCurly, TokenKind::EndOfInput])
                {
                    break;
                }
            } else {
                statements.push(self.parse_expression()?);

                if self.next_token.is_kind(TokenKind::Semicolon) {
                    self.next();
                }

                if self
                    .next_token
                    .is_any_kind(&[TokenKind::RightParen, TokenKind::RightCurly, TokenKind::EndOfInput])
                {
                    break;
                }
            }
        }

        Ok(SyntaxTree::Statements(statements))
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
