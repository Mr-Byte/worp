use super::{
    lexer::{Token, TokenIterator},
    ParserError, SyntaxTree,
};

pub mod error;
mod expr;
mod literal;

type ParseResult = Result<SyntaxTree, ParserError>;

pub struct Parser<'a> {
    token_stream: TokenIterator<'a>,
    current_token: Token<'a>,
    next_token: Token<'a>,
}

impl<'a> Parser<'a> {
    fn new(mut token_stream: TokenIterator<'a>) -> Self {
        let next_token = token_stream.peek().cloned().unwrap_or_else(Token::empty);

        Self {
            token_stream,
            current_token: Token::empty(),
            next_token,
        }
    }

    fn next(&mut self) {
        let current_token = self.token_stream.next().unwrap_or_else(Token::empty);
        let next_token = self.token_stream.peek().cloned().unwrap_or_else(Token::empty);

        self.current_token = current_token;
        self.next_token = next_token;
    }

    pub fn parse_str(input: &'a str) -> ParseResult {
        let token_stream = Token::tokenize(input);
        let mut parser = Self::new(token_stream);

        parser.parse()
    }

    fn parse(&mut self) -> ParseResult {
        self.parse_expression()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{
        runtime::core::Symbol,
        syntax::{BinaryOperator, Literal},
    };
    use error::ErrorKind;

    #[test]
    fn parse_coalesce_rule() {
        let input = "5 ?? 5";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, _))));
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

        assert!(matches!(parsed, Ok(SyntaxTree::Unary(crate::syntax::UnaryOperator::Not(_), _))));
    }

    #[test]
    fn parse_unary_rule_negate() {
        let input = "-x";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Unary(crate::syntax::UnaryOperator::Negate(_), _))));
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
    fn parse_subexpression_literal_rule() {
        let input = "5 ?? (5 ?? 5)";
        let parsed = Parser::parse_str(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, _))));

        let is_sub_match = match parsed {
            Ok(SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, rhs)) => match *rhs {
                SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, _) => true,
                _ => false,
            },
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
                kind: ErrorKind::UnexpectedToken,
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
                kind: ErrorKind::UnexpectedEndOfInput,
                ..
            })
        ));
    }
}
