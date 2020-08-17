use super::{ParseResult, Parser};
use crate::{
    runtime::core::Symbol,
    syntax::{lexer::TokenKind, SyntaxTree},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_statements(&mut self) -> ParseResult {
        let mut statements = Vec::new();

        let span_start = self.current_token.span.clone();
        while self.next_token.kind != TokenKind::EndOfInput {
            if self.next_token.kind == TokenKind::If {
                statements.push(self.parse_if_expression()?);

                if self
                    .next_token
                    .is_any_kind(&[TokenKind::RightParen, TokenKind::RightCurly, TokenKind::EndOfInput])
                {
                    break;
                }
            } else if self.next_token.kind == TokenKind::Let {
                statements.push(self.parse_variable_decl()?);
                self.consume(&[TokenKind::Semicolon])?;
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
        let span_end = self.current_token.span.clone();

        Ok(SyntaxTree::Statements(statements, span_start + span_end))
    }

    fn parse_variable_decl(&mut self) -> ParseResult {
        self.consume(&[TokenKind::Let])?;
        let span_start = self.current_token.span.clone();

        self.consume(&[TokenKind::Identifier])?;
        let identifier = Symbol::new(self.current_token.slice());
        self.consume(&[TokenKind::Assign])?;
        let value = self.parse_expression()?;

        let span_end = self.current_token.span.clone();

        Ok(SyntaxTree::VariableDeclaration(identifier, Box::new(value), span_start + span_end))
    }

    fn parse_if_expression(&mut self) -> ParseResult {
        self.consume(&[TokenKind::If])?;

        let span_start = self.current_token.span.clone();
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
        let span_end = self.current_token.span.clone();

        Ok(SyntaxTree::Conditional(
            Box::new(condition),
            Box::new(primary_condition),
            else_condition.map(Box::new),
            span_start + span_end,
        ))
    }

    fn parse_block(&mut self) -> ParseResult {
        self.consume(&[TokenKind::LeftCurly])?;
        let result = self.parse_statements()?;
        self.consume(&[TokenKind::RightCurly])?;

        Ok(result)
    }
}
