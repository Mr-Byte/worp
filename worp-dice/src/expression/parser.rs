use super::expression::{BinaryOperator, Expression, Literal, RangeOperator, UnaryOperator};
use crate::interpreter::{object::ObjectKey, symbol::Symbol};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1},
    combinator::{all_consuming, cut, map, map_res, not, opt, recognize, value},
    error::{context, convert_error, VerboseError},
    multi::{fold_many0, many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::str::FromStr;

fn open_paren(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    delimited(multispace0, context("openening parenthesis", value((), char('('))), multispace0)(input)
}

fn close_paren(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    delimited(multispace0, context("closing parenthesis", value((), char(')'))), multispace0)(input)
}

fn open_curly(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    delimited(multispace0, context("openening curly brace", value((), char('{'))), multispace0)(input)
}

fn close_curly(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    delimited(multispace0, context("closing curly brace", value((), char('}'))), multispace0)(input)
}

fn open_square(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    delimited(multispace0, context("openening square bracket", value((), char('['))), multispace0)(input)
}

fn close_square(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    delimited(multispace0, context("closing square bracket", value((), char(']'))), multispace0)(input)
}

fn comma(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    delimited(multispace0, context("comma", value((), char(','))), multispace0)(input)
}

fn double_quote(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    context("double quote", value((), char('"')))(input)
}

fn reserved(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    context(
        "reserved keyword",
        cut(not(alt((
            tag("while"),
            tag("do"),
            tag("loop"),
            tag("for"),
            tag("break"),
            tag("continue"),
            tag("return"),
            tag("fn"),
            tag("let"),
            tag("const"),
            tag("switch"),
            tag("match"),
            tag("when"),
            tag("table"),
            tag("struct"),
            tag("trait"),
            tag("interface"),
        )))),
    )(input)
}

fn identifier(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    let symbol_start = alt((tag("_"), alpha1));
    let symbol_remainder = cut(many0(alt((tag("_"), alphanumeric1))));
    let symbol = recognize(pair(symbol_start, symbol_remainder));

    context(
        "identifier",
        delimited(
            multispace0,
            map(symbol, |symbol: &str| Literal::Identifier(Symbol::new(symbol))),
            multispace0,
        ),
    )(input)
}

fn none_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    context("none literal", map(delimited(multispace0, tag("none"), multispace0), |_| Literal::None))(input)
}

fn float_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    let float = map_res(
        recognize(tuple((opt(alt((char('+'), char('-')))), digit1, char('.'), digit1))),
        FromStr::from_str,
    );

    context("float literal", map(delimited(multispace0, float, multispace0), Literal::Float))(input)
}

fn int_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    let int = map_res(
        tuple((opt(alt((char('+'), char('-')))), digit1)),
        |(sign, value): (Option<char>, &str)| match sign {
            Some('-') => value.parse().map(|value: i64| -value),
            _ => value.parse(),
        },
    );

    context("integer literal", map(delimited(multispace0, int, multispace0), Literal::Integer))(input)
}

fn boolean_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    let bool = map(alt((tag("true"), tag("false"))), |value| match value {
        "true" => true,
        "false" => false,
        _ => unreachable!(),
    });

    context("boolean literal", map(delimited(multispace0, bool, multispace0), Literal::Boolean))(input)
}

fn string_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    context(
        "string literal",
        delimited(
            multispace0,
            map(delimited(double_quote, take_till(|c| c == '"'), cut(double_quote)), |value: &str| {
                Literal::String(value.to_owned())
            }),
            multispace0,
        ),
    )(input)
}

fn list_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    context(
        "list literal",
        map(
            delimited(
                open_square,
                separated_list0(delimited(multispace0, comma, multispace0), expression),
                cut(close_square),
            ),
            Literal::List,
        ),
    )(input)
}

fn object_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    let field = context(
        "field",
        map(
            alt((identifier, string_literal, delimited(open_square, int_literal, close_square))),
            |value| match value {
                Literal::Integer(index) => ObjectKey::Index(index),
                Literal::Identifier(symbol) => ObjectKey::Symbol(symbol),
                Literal::String(string) => ObjectKey::Symbol(Symbol::new(string)),
                _ => unreachable!(),
            },
        ),
    );

    context(
        "object literal",
        map(
            delimited(
                open_curly,
                separated_list0(comma, separated_pair(field, delimited(multispace0, char(':'), multispace0), expression)),
                close_curly,
            ),
            |pairs| Literal::Object(pairs.into_iter().collect()),
        ),
    )(input)
}

fn literal(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    map(
        preceded(
            reserved,
            alt((
                none_literal,
                float_literal,
                int_literal,
                string_literal,
                boolean_literal,
                identifier,
                list_literal,
                object_literal,
            )),
        ),
        Expression::Literal,
    )(input)
}

fn parens(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    context(
        "parnenthesized expression",
        delimited(multispace0, delimited(open_paren, expression, cut(close_paren)), multispace0),
    )(input)
}

fn primary(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    alt((literal, parens))(input)
}

fn dice_roll(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = primary(input)?;
    let dice_roll_op = context("dice roll operator", char('d'));

    fold_many0(preceded(delimited(multispace0, dice_roll_op, multispace0), primary), init, |acc, expr| {
        Expression::Binary(BinaryOperator::DiceRoll, Box::new(acc), Box::new(expr))
    })(input)
}

enum CallType {
    Function(Vec<Expression>),
    ArrayIndex(Expression),
    FieldAccess(Expression),
    SafeAccess,
}

fn function_call(input: &str) -> IResult<&str, CallType, VerboseError<&str>> {
    context(
        "function call",
        map(
            delimited(
                multispace0,
                delimited(open_paren, separated_list0(comma, expression), cut(close_paren)),
                multispace0,
            ),
            CallType::Function,
        ),
    )(input)
}

fn safe_access(input: &str) -> IResult<&str, CallType, VerboseError<&str>> {
    let safe_access_op = context("safe access operator", delimited(multispace0, tag("?"), multispace0));
    map(safe_access_op, |_| CallType::SafeAccess)(input)
}

fn array_index(input: &str) -> IResult<&str, CallType, VerboseError<&str>> {
    context(
        "array index",
        map(
            delimited(multispace0, delimited(open_square, expression, cut(close_square)), multispace0),
            |expr| CallType::ArrayIndex(expr),
        ),
    )(input)
}

fn field_access(input: &str) -> IResult<&str, CallType, VerboseError<&str>> {
    let field_acces_op = context("field access operator", char('.'));

    context(
        "field access",
        map(delimited(multispace0, preceded(field_acces_op, identifier), multispace0), |identifier| {
            CallType::FieldAccess(Expression::Literal(identifier))
        }),
    )(input)
}

fn access(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = dice_roll(input)?;

    let call = alt((safe_access, function_call, array_index, field_access));

    fold_many0(call, init, |acc, call_type| match call_type {
        CallType::Function(args) => Expression::FunctionCall(Box::new(acc), args),
        CallType::ArrayIndex(arg) => Expression::Index(Box::new(acc), Box::new(arg)),
        CallType::FieldAccess(field) => Expression::FieldAccess(Box::new(acc), Box::new(field)),
        CallType::SafeAccess => Expression::SafeAccess(Box::new(acc)),
    })(input)
}

// TODO: Refactor this to produce better errors?
fn unary(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let negation = context("negation operator", char('-'));
    let not = context("not operator", char('!'));

    let unary_rule = map(
        delimited(multispace0, pair(alt((negation, not)), unary), multispace0),
        |(op, expr)| match op {
            '-' => Expression::Unary(UnaryOperator::Negate, Box::new(expr)),
            '!' => Expression::Unary(UnaryOperator::Not, Box::new(expr)),
            _ => unreachable!(),
        },
    );

    alt((access, unary_rule))(input)
}

fn multiplicative(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = unary(input)?;

    let multiply = context("multiply operator", char('*'));
    let divide = context("divide operator", char('/'));
    let remainder = context("remainder operator", char('%'));

    fold_many0(pair(alt((multiply, divide, remainder)), unary), init, |acc, (op, value)| {
        let op = match op {
            '*' => BinaryOperator::Multiply,
            '/' => BinaryOperator::Divide,
            '%' => BinaryOperator::Remainder,
            _ => unreachable!(),
        };

        Expression::Binary(op, Box::new(acc), Box::new(value))
    })(input)
}

fn additive(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = multiplicative(input)?;

    let add = context("add operator", char('+'));
    let subtract = context("subtract operator", char('-'));

    fold_many0(pair(alt((add, subtract)), multiplicative), init, |acc, (op, value)| {
        let op = match op {
            '+' => BinaryOperator::Add,
            '-' => BinaryOperator::Subtract,
            _ => unreachable!(),
        };

        Expression::Binary(op, Box::new(acc), Box::new(value))
    })(input)
}

fn comparison(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = additive(input)?;

    let eq_op = context("equals operator", tag("=="));
    let ne_op = context("not equals operator", tag("!="));
    let gt_op = context("greater than operator", tag(">"));
    let lt_op = context("less than operator", tag("<"));
    let gte_op = context("greater than or equals operator", tag(">="));
    let lte_op = context("less than or equals operator", tag("<="));

    fold_many0(
        pair(alt((eq_op, ne_op, gt_op, lt_op, gte_op, lte_op)), additive),
        init,
        |acc, (op, value)| {
            let op = match op {
                "==" => BinaryOperator::Equals,
                "!=" => BinaryOperator::NotEquals,
                ">" => BinaryOperator::GreaterThan,
                "<" => BinaryOperator::LessThan,
                ">=" => BinaryOperator::GreaterThanOrEquals,
                "<=" => BinaryOperator::LessThanOrEquals,
                _ => unreachable!(), // TODO: make this an error?
            };

            Expression::Binary(op, Box::new(acc), Box::new(value))
        },
    )(input)
}

fn logical_and(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = comparison(input)?;

    let logical_and_op = context("logical and operator", tag("&&"));

    fold_many0(preceded(logical_and_op, comparison), init, |acc, expr| {
        Expression::Binary(BinaryOperator::LogicalAnd, Box::new(acc), Box::new(expr))
    })(input)
}

fn logical_or(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = logical_and(input)?;

    let logical_or_op = context("logical or operator", tag("||"));

    fold_many0(preceded(logical_or_op, logical_and), init, |acc, expr| {
        Expression::Binary(BinaryOperator::LogicalOr, Box::new(acc), Box::new(expr))
    })(input)
}

fn range(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let exclusive_range_op = context("exclusive range operator", tag(".."));
    let inclusive_range_op = context("inclusive range operator", tag("..="));

    alt((
        map(
            tuple((logical_or, alt((exclusive_range_op, inclusive_range_op)), logical_or)),
            |(lhs, op, rhs)| match op {
                ".." => Expression::Range(RangeOperator::Exclusive, Box::new(lhs), Box::new(rhs)),
                "..=" => Expression::Range(RangeOperator::Inclusive, Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        ),
        logical_or,
    ))(input)
}

fn coalesce(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = range(input)?;

    let coalesce_op = context("none coalesce operator", tag(":?"));

    fold_many0(preceded(coalesce_op, range), init, |acc, expr| {
        Expression::Binary(BinaryOperator::Coalesce, Box::new(acc), Box::new(expr))
    })(input)
}

fn if_expression(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let if_keyword = context("if keyword", delimited(multispace0, tag("if"), multispace1));
    let else_keyword = context("else keyword", delimited(multispace0, tag("else"), multispace1));

    // TODO: Figure out better error handling here.
    map(
        tuple((
            preceded(if_keyword, discard),
            context("primary condition", delimited(open_curly, discard, cut(close_curly))),
            opt(context(
                "alternate condition",
                alt((
                    preceded(else_keyword, delimited(open_curly, expression, cut(close_curly))),
                    preceded(delimited(multispace0, tag("else"), multispace1), if_expression),
                )),
            )),
        )),
        |(condition, body, alt)| Expression::Conditional(Box::new(condition), Box::new(body), alt.map(Box::new)),
    )(input)
}

fn conditional(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    alt((if_expression, coalesce))(input)
}

fn discard(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = conditional(input)?;

    let discard_op = context("discard operator", char(';'));

    fold_many0(preceded(discard_op, conditional), init, |acc, expr| {
        Expression::Binary(BinaryOperator::Discard, Box::new(acc), Box::new(expr))
    })(input)
}

fn expression(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    context("expression", discard)(input)
}

pub fn parse(input: &str) -> Result<Expression, error::ParseError> {
    match all_consuming(terminated(expression, multispace0))(input) {
        Ok((_, result)) => Ok(result),
        Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => Err(error::ParseError(convert_error(input, err))),
        _ => unreachable!(),
    }
}

pub mod error {
    #[derive(thiserror::Error, Debug)]
    #[error("{0}")]
    pub struct ParseError(pub(super) String);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let result = parse(r#"{ roll: 6d8, "test": 5;55, [5]: test.roll() }[5]"#);

        match result {
            Ok(ok) => println!("{}", ok),
            Err(err) => println!("{}", err),
        }
    }
}
