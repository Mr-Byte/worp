use super::{
    lexer::{Token, TokenIterator, TokenKind},
    BinaryOperator, Literal, ParserError, SyntaxTree,
};
use error::ErrorKind;

pub mod error;

type ParseResult = Result<SyntaxTree, ParserError>;

macro_rules! match_next_token {
    ($tokens:expr, $token:pat) => {
        matches!($tokens.peek(), Some($crate::syntax::lexer::Token { kind: $token, .. }))
    };
}

// macro_rules! is_token_kind {
//     ($token:pat, $span:pat) => {
//         Some($crate::syntax::lexer::Token { kind: $token, span: $span, .. })
//     }
// }

pub fn parse(input: &str) -> ParseResult {
    let mut token_stream = Token::tokenize(input);

    // TODO: Ensure all input is consumed.

    parse_expression(&mut token_stream)
}

pub fn parse_expression(token_stream: &mut TokenIterator<'_>) -> ParseResult {
    parse_coalesce(token_stream)
}

fn parse_coalesce(token_stream: &mut TokenIterator) -> ParseResult {
    let mut expression = parse_literal(token_stream)?;

    while match_next_token!(token_stream, TokenKind::Coalesce) {
        let operator = token_stream.next();
        let rhs = parse_literal(token_stream)?;

        expression = SyntaxTree::Binary(
            BinaryOperator::Coalesce(operator.unwrap().span.clone()),
            Box::new(expression),
            Box::new(rhs),
        );
    }

    Ok(expression)
}

fn parse_range(token_stream: &mut TokenIterator) -> ParseResult {
    todo!()
}

fn parse_literal(token_stream: &mut TokenIterator) -> ParseResult {
    if let Some(token) = token_stream.next() {
        let result = match &token.kind {
            TokenKind::True => SyntaxTree::Literal(Literal::Boolean(true), token.span),
            TokenKind::False => SyntaxTree::Literal(Literal::Boolean(false), token.span),
            TokenKind::Integer => SyntaxTree::Literal(Literal::Integer(token.slice().parse()?), token.span),
            TokenKind::Float => SyntaxTree::Literal(Literal::Float(token.slice().parse()?), token.span),
            TokenKind::String => todo!(),
            TokenKind::LeftParen => todo!(),
            TokenKind::LeftCurly => todo!(),
            TokenKind::LeftSquare => todo!(),
            _ => return Err(ParserError::new(ErrorKind::UnexpectedToken, Some(token.span))),
        };

        Ok(result)
    } else {
        Err(ParserError::new(ErrorKind::UnexpectedEndOfInput, None))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn parse_coalesce_rule() {
        let input = "5 ?? 5";
        let parsed = parse(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Binary(BinaryOperator::Coalesce(_), _, _))));
    }

    #[test]
    fn parse_integer_literal_rule() {
        let input = "5";
        let parsed = parse(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Integer(5), _))));
    }

    #[test]
    fn parse_float_literal_rule() {
        let input = "5.0";
        let parsed = parse(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Float(value), _)) if value == 5.0));
    }

    #[test]
    fn parse_boolean_true_literal_rule() {
        let input = "true";
        let parsed = parse(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Boolean(true), _))));
    }

    #[test]
    fn parse_boolean_false_literal_rule() {
        let input = "false";
        let parsed = parse(input);

        assert!(matches!(parsed, Ok(SyntaxTree::Literal(Literal::Boolean(false), _))));
    }

    #[test]
    fn parse_unexpected_end_of_input() {
        let input = "";
        let parsed = parse(input);

        assert!(matches!(
            parsed,
            Err(ParserError {
                kind: ErrorKind::UnexpectedEndOfInput,
                ..
            })
        ));
    }
}
