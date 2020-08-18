use super::{error::ErrorKind, ParseResult, Parser};
use crate::{
    runtime::core::{Symbol, ValueKey},
    syntax::{lexer::TokenKind, Literal, ParserError, SyntaxTree},
};
use std::collections::HashMap;

impl<'a> Parser<'a> {
    pub(super) fn parse_literal(&mut self) -> ParseResult {
        if self.next_token.is_kind(TokenKind::LeftCurly) {
            return self.parse_block();
        }

        self.next();

        let result = match self.current_token.kind {
            TokenKind::True => SyntaxTree::Literal(Literal::Boolean(true), self.current_token.span()),
            TokenKind::False => SyntaxTree::Literal(Literal::Boolean(false), self.current_token.span()),
            TokenKind::Integer => SyntaxTree::Literal(
                Literal::Integer(self.current_token.slice().parse()?),
                self.current_token.span(),
            ),
            TokenKind::Float => SyntaxTree::Literal(
                Literal::Float(self.current_token.slice().parse()?),
                self.current_token.span(),
            ),
            TokenKind::String => SyntaxTree::Literal(
                Literal::String(self.current_token.slice().trim_matches('"').to_owned()),
                self.current_token.span(),
            ),
            TokenKind::None => SyntaxTree::Literal(Literal::None, self.current_token.span()),
            TokenKind::Identifier => SyntaxTree::Literal(
                Literal::Identifier(Symbol::new(self.current_token.slice().to_owned())),
                self.current_token.span(),
            ),
            TokenKind::Object => self.parse_object_literal()?,
            TokenKind::LeftSquare => self.parse_list_literal()?,
            TokenKind::LeftParen => {
                let expression = self.parse_expression()?;
                self.consume(&[TokenKind::RightParen])?;
                expression
            }
            TokenKind::EndOfInput => {
                return Err(ParserError::unexpected_token(
                    self.current_token.kind,
                    &[TokenKind::String],
                    Some(self.current_token.span()),
                ))
            }
            TokenKind::Error => {
                return Err(ParserError::new(
                    ErrorKind::UnknownToken {
                        value: self.current_token.slice().to_owned(),
                    },
                    Some(self.current_token.span()),
                ))
            }
            kind if kind.is_reserved() => {
                return Err(ParserError::new(
                    ErrorKind::ReservedKeyword { keyword: kind },
                    Some(self.current_token.span()),
                ))
            }
            _ => unreachable!("Invalid self.current_token kind found: {:?}", self.current_token.kind),
        };

        Ok(result)
    }

    fn parse_object_literal(&mut self) -> ParseResult {
        let mut properties = HashMap::new();
        let span_start = self.current_token.span();

        self.consume(&[TokenKind::LeftCurly])?;

        while !self.next_token.is_kind(TokenKind::RightCurly) {
            let (key, value) = self.parse_object_literal_property()?;
            properties.insert(key, value);
        }

        self.consume(&[TokenKind::RightCurly])?;
        let span_end = self.current_token.span();

        Ok(SyntaxTree::Literal(Literal::Object(properties), span_start + span_end))
    }

    fn parse_object_literal_property(&mut self) -> ParseResult<(ValueKey, SyntaxTree)> {
        let key = self.parse_object_literal_key()?;

        if self.next_token.is_kind(TokenKind::Colon) {
            self.next();
        } else {
            return Err(ParserError::unexpected_token(
                self.current_token.kind,
                &[TokenKind::Colon],
                Some(self.next_token.span()),
            ));
        }

        let value = self.parse_expression()?;

        if self.next_token.is_kind(TokenKind::Comma) {
            self.next();
        } else if !self.next_token.is_kind(TokenKind::RightCurly) {
            return Err(ParserError::unexpected_token(
                self.next_token.kind,
                &[TokenKind::Comma, TokenKind::RightCurly],
                Some(self.next_token.span()),
            ));
        }

        Ok((key, value))
    }

    fn parse_object_literal_key(&mut self) -> ParseResult<ValueKey> {
        self.next();

        let result = match self.current_token.kind {
            TokenKind::Identifier => ValueKey::Symbol(self.current_token.slice().into()),
            TokenKind::String => ValueKey::Symbol(self.current_token.slice().trim_matches('"').into()),
            TokenKind::Integer => ValueKey::Index(self.current_token.slice().parse()?),
            _ => {
                return Err(ParserError::unexpected_token(
                    self.next_token.kind,
                    &[TokenKind::Identifier, TokenKind::String, TokenKind::Integer],
                    Some(self.current_token.span()),
                ));
            }
        };

        Ok(result)
    }

    fn parse_list_literal(&mut self) -> ParseResult {
        let mut items = Vec::new();
        let span_start = self.current_token.span();

        while !self.next_token.is_kind(TokenKind::RightSquare) {
            items.push(self.parse_expression()?);

            if self.next_token.is_kind(TokenKind::Comma) {
                self.next();
            } else if !self.next_token.is_kind(TokenKind::RightSquare) {
                return Err(ParserError::unexpected_token(
                    self.next_token.kind,
                    &[TokenKind::Comma, TokenKind::RightSquare],
                    Some(self.next_token.span()),
                ));
            }
        }

        self.consume(&[TokenKind::RightSquare])?;
        let span_end = self.current_token.span();

        Ok(SyntaxTree::Literal(Literal::List(items), span_start + span_end))
    }
}
