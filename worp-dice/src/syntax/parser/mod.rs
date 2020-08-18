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
        let next_token = token_stream.next().unwrap_or_else(Token::end_of_input);

        Self {
            token_stream,
            current_token: Token::start_of_input(),
            next_token,
        }
    }

    fn next(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.token_stream.next().unwrap_or_else(Token::end_of_input);
    }

    fn consume(&mut self, kinds: &[TokenKind]) -> ParseResult<()> {
        if self.next_token.is_any_kind(kinds) {
            self.next();
            Ok(())
        } else {
            Err(ParserError::unexpected_token(
                self.next_token.kind,
                kinds,
                Some(self.next_token.span()),
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
    use crate::syntax::{lexer::TokenKind, BinaryOperator, Literal, RangeOperator, UnaryOperator};
    use error::ErrorKind;

    type TestResult = Result<(), ParserError>;

    macro_rules! assert_statement {
        ($tree:expr, $pattern:pat) => {
            if let SyntaxTree::Block(statements, _) = $tree {
                assert!(
                    matches!(statements.as_slice(), [$pattern, ..]),
                    "Unexpected syntax tree. Found: {:?}",
                    statements
                );
            } else {
                panic!("Syntax tree is not rooted with statements node.");
            }
        };
    }

    #[test]
    fn parse_variable_decl_rule() -> TestResult {
        let input = "let x = 5;";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::VariableDeclaration(_, _, _));

        Ok(())
    }

    #[test]
    fn parse_coalesce_rule() -> TestResult {
        let input = "5 ?? 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, _, _));

        Ok(())
    }

    #[test]
    fn parse_range_rule_exclusive() -> TestResult {
        let input = "5 .. 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Range(RangeOperator::Exclusive(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_range_rule_inclusive() -> TestResult {
        let input = "5 ..= 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Range(RangeOperator::Inclusive(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_lazy_and_rule() -> TestResult {
        let input = "5 && 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::LogicalAnd(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_lazy_or_rule() -> TestResult {
        let input = "5 || 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::LogicalOr(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_comparison_rule_equals() -> TestResult {
        let input = "5 == 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::Equals(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_comparison_rule_not_equals() -> TestResult {
        let input = "5 != 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::NotEquals(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_comparison_rule_less() -> TestResult {
        let input = "5 < 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::LessThan(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_comparison_rule_less_equals() -> TestResult {
        let input = "5 <= 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::LessThanOrEquals(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_comparison_rule_greater() -> TestResult {
        let input = "5 > 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::GreaterThan(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_comparison_rule_greater_equals() -> TestResult {
        let input = "5 >= 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(
            parsed,
            SyntaxTree::Binary(BinaryOperator::GreaterThanOrEquals(_), _, _, _)
        );

        Ok(())
    }

    #[test]
    fn parse_multiplicative_rule_multiply() -> TestResult {
        let input = "5 * 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::Multiply(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_multiplicative_rule_divide() -> TestResult {
        let input = "5 / 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::Divide(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_multiplicative_rule_remainder() -> TestResult {
        let input = "5 % 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::Remainder(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_multiplicative_rule_compound() -> TestResult {
        let input = "5 * 5 / 5 % 5";
        let _ = Parser::parse_str(input)?;

        Ok(())
    }

    #[test]
    fn parse_additive_rule_add() -> TestResult {
        let input = "5 + 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::Add(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_additive_rule_subtract() -> TestResult {
        let input = "5 - 5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::Subtract(_), _, _, _));
        Ok(())
    }

    #[test]
    fn parse_unary_rule_not() -> TestResult {
        let input = "!5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Unary(UnaryOperator::Not(_), _, _));
        Ok(())
    }

    #[test]
    fn parse_unary_rule_negate() -> TestResult {
        let input = "-x";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Unary(UnaryOperator::Negate(_), _, _));
        Ok(())
    }

    #[test]
    fn parse_unary_rule_dice_roll() -> TestResult {
        let input = "d5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Unary(UnaryOperator::DiceRoll(_), _, _));

        Ok(())
    }

    #[test]
    fn parse_unary_rule_dice_roll_with_arithmetic() -> TestResult {
        let input = "d4 + 4";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::Add(_), _, _, _));

        Ok(())
    }

    #[test]
    fn parse_dice_roll_rule() -> TestResult {
        let input = "6d8";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::DiceRoll(_), _, _, _));

        Ok(())
    }

    #[test]
    fn parse_access_rule_field_access() -> TestResult {
        let input = "x.y";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::FieldAccess(_, _, _));
        Ok(())
    }

    #[test]
    fn parse_access_rule_field_safe_access() -> TestResult {
        let input = "x?.y";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::SafeAccess(_, _, _));
        Ok(())
    }

    #[test]
    fn parse_access_rule_index_access() -> TestResult {
        let input = "x[y]";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Index(_, _, _));

        Ok(())
    }

    #[test]
    fn parse_access_rule_function_call() -> TestResult {
        let input = "x(y)";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::FunctionCall(_, _, _));

        Ok(())
    }

    #[test]
    fn parse_access_rule_function_call_trailing_comma() -> TestResult {
        let input = "x(y,)";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::FunctionCall(_, _, _));

        Ok(())
    }

    #[test]
    fn parse_access_rule_function_call_multiple_parameters() -> TestResult {
        let input = "x(y,z)";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::FunctionCall(_, _, _));

        Ok(())
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
    fn parse_identifier_literal_rule() -> TestResult {
        let input = "_abc";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::Identifier(_), _));

        Ok(())
    }

    #[test]
    fn parse_integer_literal_rule() -> TestResult {
        let input = "5";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::Integer(5), _));
        Ok(())
    }

    #[test]
    fn parse_float_literal_rule() -> TestResult {
        let input = "5.0";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::Float(_), _));
        Ok(())
    }

    #[test]
    fn parse_boolean_true_literal_rule() -> TestResult {
        let input = "true";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::Boolean(true), _));
        Ok(())
    }

    #[test]
    fn parse_boolean_false_literal_rule() -> TestResult {
        let input = "false";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::Boolean(false), _));
        Ok(())
    }

    #[test]
    fn parse_string_literal_rule() -> TestResult {
        let input = r#""test""#;
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::String(_), _));

        Ok(())
    }

    #[test]
    fn parse_object_literal_rule() -> TestResult {
        let input = r#"object { x: 5, "y": "test", 5: "y", z: object { "test": 5 } }"#;
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::Object(_), _));
        Ok(())
    }

    #[test]
    fn parse_object_literal_rule_with_trailing_comma() -> TestResult {
        let input = r#"object { x: 5, "y": "test", 5: "y", }"#;
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::Object(_), _));
        Ok(())
    }

    #[test]
    fn parse_object_literal_rule_with_no_commas() {
        let input = r#"object { x: 5 "y": "test" 5: "y" }"#;
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
        let input = r#"object { x: 5, "y": "test", 5: "y", "#;
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
    fn parse_list_literal_rule() -> TestResult {
        let input = r#"[1, 2, 3, 4, 5]"#;
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::List(_), _));
        Ok(())
    }

    #[test]
    fn parse_list_literal_rule_with_trailing_comma() -> TestResult {
        let input = r#"[1, 2, 3, 4, 5, ]"#;
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Literal(Literal::List(_), _));
        Ok(())
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
                kind:
                    ErrorKind::UnexpectedToken {
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
    fn parse_reserved_keyword() {
        let input = "return";
        let parsed = Parser::parse_str(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::ReservedKeyword {
                    keyword: TokenKind::Return,
                },
                ..
            })
        ));
    }

    #[test]
    fn parse_subexpression_literal_rule() -> TestResult {
        let input = "5 ?? (5 ?? 5)";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, _, _));

        Ok(())
    }

    #[test]
    fn parse_subexpression_literal_with_chained_function_call() -> TestResult {
        let input = "(d4).roll()";
        let parsed = Parser::parse_str(input)?;

        assert_statement!(parsed, SyntaxTree::FunctionCall(_, _, _));

        Ok(())
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
    fn parse_empty_expression() -> Result<(), ParserError> {
        let input = "";
        Parser::parse_str(input)?;

        Ok(())
    }
}
