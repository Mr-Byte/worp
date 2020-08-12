use super::{error::ErrorKind, ParseResult, Parser};
use crate::{
    runtime::core::Symbol,
    syntax::{lexer::TokenKind, Literal, ParserError, SyntaxTree},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_literal(&mut self) -> ParseResult {
        self.next();
        let token = self.current_token.clone();

        let result = match token.kind {
            TokenKind::True => SyntaxTree::Literal(Literal::Boolean(true), token.span),
            TokenKind::False => SyntaxTree::Literal(Literal::Boolean(false), token.span),
            TokenKind::Integer => SyntaxTree::Literal(Literal::Integer(token.slice().parse()?), token.span),
            TokenKind::Float => SyntaxTree::Literal(Literal::Float(token.slice().parse()?), token.span),
            TokenKind::String => SyntaxTree::Literal(Literal::String(token.slice().trim_matches('"').to_owned()), token.span),
            TokenKind::LeftParen => {
                let expression = self.parse_expression()?;

                if self.next_token.is_kind(TokenKind::RightParen) {
                    expression
                } else {
                    return Err(ParserError::new(ErrorKind::UnexpectedToken, Some(token.span)));
                }
            }
            TokenKind::None => SyntaxTree::Literal(Literal::None, token.span),
            TokenKind::Identifier => SyntaxTree::Literal(Literal::Identifier(Symbol::new(token.slice().to_owned())), token.span),
            TokenKind::LeftCurly => todo!(),
            TokenKind::LeftSquare => todo!(),
            TokenKind::Empty => return Err(ParserError::new(ErrorKind::UnexpectedEndOfInput, Some(token.span))),
            _ => return Err(ParserError::new(ErrorKind::UnexpectedToken, Some(token.span))),
        };

        Ok(result)
    }
}
