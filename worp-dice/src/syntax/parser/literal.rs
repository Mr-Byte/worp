use super::{error::ErrorKind, ParseResult, Parser};
use crate::{
    runtime::core::{Symbol, ValueKey},
    syntax::{lexer::TokenKind, Literal, ParserError, SyntaxTree},
};
use std::collections::HashMap;

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
                    return Err(ParserError::new(
                        ErrorKind::UnexpectedToken {
                            expected: vec![TokenKind::RightParen],
                            found: token.kind,
                        },
                        Some(token.span),
                    ));
                }
            }
            TokenKind::None => SyntaxTree::Literal(Literal::None, token.span),
            TokenKind::Identifier => SyntaxTree::Literal(Literal::Identifier(Symbol::new(token.slice().to_owned())), token.span),
            TokenKind::LeftCurly => self.parse_object_literal()?,
            TokenKind::LeftSquare => self.parse_list_literal()?,
            TokenKind::Empty => return Err(ParserError::new(ErrorKind::UnexpectedEndOfInput, Some(token.span))),
            TokenKind::Error => return Err(ParserError::new(ErrorKind::UnexpectedEndOfInput, Some(token.span))),
            _ => unreachable!("Invalid token kind found: {:?}", token.kind),
        };

        Ok(result)
    }

    // TODO: Improve this to detect a missing, closing curly.
    fn parse_object_literal(&mut self) -> ParseResult {
        let mut properties = HashMap::new();
        let span_start = self.current_token.span.clone();

        while !self.next_token.is_kind(TokenKind::RightCurly) {
            let (key, value) = self.parse_object_literal_property()?;
            properties.insert(key, value);
        }

        self.next();
        let span_end = self.current_token.span.clone();

        Ok(SyntaxTree::Literal(Literal::Object(properties), span_start + span_end))
    }

    fn parse_object_literal_property(&mut self) -> ParseResult<(ValueKey, SyntaxTree)> {
        let key = self.parse_object_literal_key()?;

        if self.next_token.is_kind(TokenKind::Colon) {
            self.next();
        } else {
            return Err(ParserError::new(
                ErrorKind::UnexpectedToken {
                    expected: vec![TokenKind::Colon],
                    found: self.current_token.kind,
                },
                Some(self.current_token.span.clone()),
            ));
        }

        let value = self.parse_expression()?;

        if self.next_token.is_kind(TokenKind::Comma) {
            self.next();
        } else if !self.next_token.is_kind(TokenKind::RightCurly) {
            return Err(ParserError::new(
                ErrorKind::UnexpectedToken {
                    expected: vec![TokenKind::Comma, TokenKind::RightCurly],
                    found: self.next_token.kind,
                },
                Some(self.next_token.span.clone()),
            ));
        }

        Ok((key, value))
    }

    fn parse_object_literal_key(&mut self) -> ParseResult<ValueKey> {
        self.next();
        let token = self.current_token.clone();

        let result = match token.kind {
            TokenKind::Identifier => ValueKey::Symbol(token.slice().into()),
            TokenKind::String => ValueKey::Symbol(token.slice().trim_matches('"').into()),
            TokenKind::Integer => ValueKey::Index(token.slice().parse()?),
            _ => {
                return Err(ParserError::new(
                    ErrorKind::UnexpectedToken {
                        expected: vec![TokenKind::Identifier, TokenKind::String, TokenKind::Integer],
                        found: self.current_token.kind,
                    },
                    Some(token.span),
                ))
            }
        };

        Ok(result)
    }

    // TODO: Improve this to detect a missing, closing square brace.
    fn parse_list_literal(&mut self) -> ParseResult {
        let mut items = Vec::new();
        let span_start = self.current_token.span.clone();

        while !self.next_token.is_kind(TokenKind::RightSquare) {
            items.push(self.parse_expression()?);

            if self.next_token.is_kind(TokenKind::Comma) {
                self.next();
            } else if !self.next_token.is_kind(TokenKind::RightSquare) {
                return Err(ParserError::new(
                    ErrorKind::UnexpectedToken {
                        expected: vec![TokenKind::Comma, TokenKind::RightSquare],
                        found: self.next_token.kind,
                    },
                    Some(self.next_token.span.clone()),
                ));
            }
        }

        self.next();
        let span_end = self.current_token.span.clone();

        Ok(SyntaxTree::Literal(Literal::List(items), span_start + span_end))
    }
}
