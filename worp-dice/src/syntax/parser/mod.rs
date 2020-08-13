use super::{
    lexer::{Token, TokenIterator, TokenKind},
    ParserError, SyntaxTree,
};

mod access;
pub mod error;
mod expression;
mod literal;
mod statement;

type ParseResult<T = SyntaxTree> = Result<T, ParserError>;

pub struct Parser<'a> {
    token_stream: TokenIterator<'a>,
    current_token: Token<'a>,
    next_token: Token<'a>,
}

impl<'a> Parser<'a> {
    fn new(mut token_stream: TokenIterator<'a>) -> Self {
        let next_token = token_stream.next().unwrap_or_else(Token::empty);

        Self {
            token_stream,
            current_token: Token::empty(),
            next_token,
        }
    }

    fn next(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.token_stream.next().unwrap_or_else(Token::empty);
    }

    fn consume(&mut self, kinds: &[TokenKind]) -> ParseResult<()> {
        if self.next_token.is_any_kind(kinds) {
            self.next();
            Ok(())
        } else {
            Err(ParserError::unexpected_token(
                self.next_token.kind,
                kinds,
                Some(self.next_token.span.clone()),
            ))
        }
    }

    pub fn parse_str(input: &'a str) -> ParseResult {
        let token_stream = Token::tokenize(input);
        let mut parser = Self::new(token_stream);

        parser.parse()
    }

    fn parse(&mut self) -> ParseResult {
        self.parse_statements()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        runtime::core::Symbol,
        syntax::{lexer::TokenKind, BinaryOperator, Literal, RangeOperator, UnaryOperator},
    };
    use error::ErrorKind;

    #[test]
    fn parse_coalesce_rule() {
        let input = "5 ?? 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, _))));
    }

    #[test]
    fn parse_range_rule_exclusive() {
        let input = "5 .. 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Range(RangeOperator::Exclusive(_), _, _))));
    }

    #[test]
    fn parse_range_rule_inclusive() {
        let input = "5 ..= 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Range(RangeOperator::Inclusive(_), _, _))));
    }

    #[test]
    fn parse_lazy_and_rule() {
        let input = "5 && 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::LogicalAnd(_), _, _))));
    }

    #[test]
    fn parse_lazy_or_rule() {
        let input = "5 || 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::LogicalOr(_), _, _))));
    }

    #[test]
    fn parse_comparison_rule_equals() {
        let input = "5 == 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Equals(_), _, _))));
    }

    #[test]
    fn parse_comparison_rule_not_equals() {
        let input = "5 != 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::NotEquals(_), _, _))));
    }

    #[test]
    fn parse_comparison_rule_less() {
        let input = "5 < 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::LessThan(_), _, _))));
    }

    #[test]
    fn parse_comparison_rule_less_equals() {
        let input = "5 <= 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::LessThanOrEquals(_), _, _))));
    }

    #[test]
    fn parse_comparison_rule_greater() {
        let input = "5 > 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::GreaterThan(_), _, _))));
    }

    #[test]
    fn parse_comparison_rule_greater_equals() {
        let input = "5 >= 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::GreaterThanOrEquals(_), _, _))));
    }

    #[test]
    fn parse_multiplicative_rule_multiply() {
        let input = "5 * 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Multiply(_), _, _))));
    }

    #[test]
    fn parse_multiplicative_rule_divide() {
        let input = "5 / 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Divide(_), _, _))));
    }

    #[test]
    fn parse_multiplicative_rule_remainder() {
        let input = "5 % 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Remainder(_), _, _))));
    }

    #[test]
    fn parse_multiplicative_rule_compound() {
        let input = "5 * 5 / 5 % 5";
        let parsed = Parser::parse_str(input);

        assert!(parsed.is_ok());
    }

    #[test]
    fn parse_additive_rule_add() {
        let input = "5 + 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Add(_), _, _))));
    }

    #[test]
    fn parse_additive_rule_subtract() {
        let input = "5 - 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Subtract(_), _, _))));
    }

    #[test]
    fn parse_unary_rule_not() {
        let input = "!5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Unary(UnaryOperator::Not(_), _))));
    }

    #[test]
    fn parse_unary_rule_negate() {
        let input = "-x";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Unary(UnaryOperator::Negate(_), _))));
    }

    #[test]
    fn parse_unary_rule_dice_roll() {
        let input = "d5";
        let parsed = Parser::parse_str(input);

        assert!(
            matches!(parsed, Ok(SyntaxTree::Unary(UnaryOperator::DiceRoll(_), _))),
            "Unexpected syntax tree: {:?}",
            parsed
        );
    }

    #[test]
    fn parse_unary_rule_dice_roll_with_arithmetic() {
        let input = "d4 + 4";
        let parsed = Parser::parse_str(input);

        assert!(
            matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Add(_), _, _))),
            "Unexpected syntax tree: {:?}",
            parsed
        );
    }

    #[test]
    fn parse_dice_roll_rule() {
        let input = "6d8";
        let parsed = Parser::parse_str(input);

        assert!(
            matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::DiceRoll(_), _, _))),
            "Unexpected syntax tree: {:?}",
            parsed
        );
    }

    #[test]
    fn parse_access_rule_field_access() {
        let input = "x.y";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::FieldAccess(_, symbol, _)) if symbol == Symbol::new_static("y")));
    }

    #[test]
    fn parse_access_rule_field_safe_access() {
        let input = "x?.y";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::SafeAccess(_, symbol, _)) if symbol == Symbol::new_static("y")));
    }

    #[test]
    fn parse_access_rule_index_access() {
        let input = "x[y]";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Index(_, _))), "Unexpexted syntax tree {:?}", parsed);
    }

    #[test]
    fn parse_access_rule_function_call() {
        let input = "x(y)";
        let parsed = Parser::parse_str(input);

        assert!(
            matches!(parsed, Ok(SyntaxTree::FunctionCall(_, _))),
            "Unexpexted syntax tree {:?}",
            parsed
        );
    }

    #[test]
    fn parse_access_rule_function_call_trailing_comma() {
        let input = "x(y,)";
        let parsed = Parser::parse_str(input);

        assert!(
            matches!(parsed, Ok(SyntaxTree::FunctionCall(_, _))),
            "Unexpexted syntax tree {:?}",
            parsed
        );
    }

    #[test]
    fn parse_access_rule_function_call_multiple_parameters() {
        let input = "x(y,z)";
        let parsed = Parser::parse_str(input);

        assert!(
            matches!(parsed, Ok(SyntaxTree::FunctionCall(_, _))),
            "Unexpexted syntax tree {:?}",
            parsed
        );
    }

    #[test]
    fn parse_access_rule_function_call_no_closing_paren() {
        let input = "x(y,z";
        let parsed = Parser::parse_str(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::UnexpectedToken { found, expected },
                ..
            }) if found == TokenKind::EndOfInput
                && expected.contains(&TokenKind::Comma)
                && expected.contains(&TokenKind::RightParen)
        ));
    }

    #[test]
    fn parse_identifier_literal_rule() {
        let input = "_abc";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed,
            Ok(SyntaxTree::Literal(Literal::Identifier(identifier), _))
                if identifier == Symbol::new_static("_abc")
        ));
    }

    #[test]
    fn parse_integer_literal_rule() {
        let input = "5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Integer(5), _))));
    }

    #[test]
    fn parse_float_literal_rule() {
        let input = "5.0";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Float(value), _)) if value == 5.0));
    }

    #[test]
    fn parse_boolean_true_literal_rule() {
        let input = "true";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Boolean(true), _))));
    }

    #[test]
    fn parse_boolean_false_literal_rule() {
        let input = "false";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Boolean(false), _))));
    }

    #[test]
    fn parse_string_literal_rule() {
        let input = r#""test""#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::String(value), _)) if value == "test"));
    }

    #[test]
    fn parse_object_literal_rule() {
        let input = r#"{ x: 5, "y": "test", 5: "y", z: { "test": 5 } }"#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Object(_), _))));
    }

    #[test]
    fn parse_object_literal_rule_with_trailing_comma() {
        let input = r#"{ x: 5, "y": "test", 5: "y", }"#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Object(_), _))));
    }

    #[test]
    fn parse_object_literal_rule_with_no_commas() {
        let input = r#"{ x: 5 "y": "test" 5: "y" }"#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::UnexpectedToken { found, expected },
                ..
            }) if found == TokenKind::String
                && expected.contains(&TokenKind::Comma)
                && expected.contains(&TokenKind::RightCurly)
        ));
    }

    #[test]
    fn parse_object_literal_rule_with_no_closing_brace() {
        let input = r#"{ x: 5, "y": "test", 5: "y", "#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::UnexpectedToken { .. },
                ..
            })
        ));
    }

    #[test]
    fn parse_list_literal_rule() {
        let input = r#"[1, 2, 3, 4, 5]"#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::List(_), _))));
    }

    #[test]
    fn parse_list_literal_rule_with_trailing_comma() {
        let input = r#"[1, 2, 3, 4, 5, ]"#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::List(_), _))));
    }

    #[test]
    fn parse_list_literal_rule_with_no_commas() {
        let input = r#"[1 2 3 4 5 ]"#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::UnexpectedToken { found, expected },
                ..
            }) if found == TokenKind::Integer
                && expected.contains(&TokenKind::Comma)
                && expected.contains(&TokenKind::RightSquare)
        ));
    }

    #[test]
    fn parse_list_literal_rule_with_no_closing_brace() {
        let input = r#"[1, 2, 3, 4, 5"#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::UnexpectedToken {
                    found: TokenKind::EndOfInput,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn parse_string_literal_unclosed_rule() {
        let input = r#""test"#;
        let parsed = Parser::parse_str(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::UnknownToken { .. },
                ..
            })
        ));
    }

    #[test]
    fn parse_subexpression_literal_rule() {
        let input = "5 ?? (5 ?? 5)";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, _))));

        let is_sub_match = match parsed {
            Ok(SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, rhs)) => matches!(*rhs, SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, _)),
            _ => false,
        };

        assert!(is_sub_match)
    }

    #[test]
    fn parse_subexpression_literal_rule_not_properly_closed() {
        let input = "5 ?? (5 ?? 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::UnexpectedToken { .. },
                ..
            })
        ));
    }

    #[test]
    fn parse_unexpected_end_of_input() {
        let input = "";
        let parsed = Parser::parse_str(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::UnexpectedToken { .. },
                ..
            })
        ));
    }
}
