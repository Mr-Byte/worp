// Remove once the parser is used via a public export.
#![allow(dead_code)]

use crate::expression::{BinaryOperator, Expression, Literal, RangeOperator, Symbol, UnaryOperator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, alphanumeric1, digit1, multispace1},
    character::complete::{char, multispace0},
    combinator::{all_consuming, cut, map, map_res, not, opt},
    error::{context, convert_error, VerboseError},
    multi::{fold_many0, many0, separated_list0},
    number::complete::float,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

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
            tag("fn"),
            tag("let"),
            tag("const"),
            tag("switch"),
            tag("match"),
            tag("when"),
        )))),
    )(input)
}

fn none_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    map(delimited(multispace0, tag("none"), multispace0), |_| Literal::None)(input)
}

fn float_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    map(delimited(multispace0, terminated(float, char('f')), multispace0), Literal::Float)(input)
}

fn int_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    let int = map_res(
        tuple((opt(alt((char('+'), char('-')))), digit1)),
        |(sign, value): (Option<char>, &str)| match sign {
            Some('-') => value.parse().map(|value: i32| -value),
            _ => value.parse(),
        },
    );

    map(delimited(multispace0, int, multispace0), Literal::Integer)(input)
}

fn boolean_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    let bool = map(alt((tag("true"), tag("false"))), |value| match value {
        "true" => true,
        "false" => false,
        _ => unreachable!(),
    });

    map(delimited(multispace0, bool, multispace0), Literal::Boolean)(input)
}

fn string_literal(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    map(delimited(char('"'), take_till(|c| c == '"'), char('"')), |value: &str| {
        Literal::String(value.to_owned())
    })(input)
}

fn list(input: &str) -> IResult<&str, Literal, VerboseError<&str>> {
    map(
        delimited(
            char('['),
            separated_list0(delimited(multispace0, char(','), multispace0), expression),
            char(']'),
        ),
        Literal::List,
    )(input)
}

fn literal(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    map(
        preceded(
            reserved,
            alt((none_literal, float_literal, int_literal, string_literal, boolean_literal, list)),
        ),
        Expression::Literal,
    )(input)
}

fn symbol(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let symbol_start = alt((tag("_"), alpha1));
    let symbol_remainder = many0(alt((tag("_"), alphanumeric1)));
    let symbol = pair(symbol_start, symbol_remainder);

    delimited(
        multispace0,
        map(symbol, |(start, rest): (&str, Vec<&str>)| {
            let mut result = String::new();
            result += start;

            for parts in &rest {
                result += parts;
            }

            Expression::Symbol(Symbol(result))
        }),
        multispace0,
    )(input)
}

fn parens(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    delimited(multispace0, delimited(char('('), expression, char(')')), multispace0)(input)
}

fn primary(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    alt((literal, symbol, parens))(input)
}

fn dice_roll(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = primary(input)?;

    fold_many0(preceded(delimited(multispace0, tag("d"), multispace0), primary), init, |acc, expr| {
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
    map(
        delimited(
            multispace0,
            delimited(tag("("), separated_list0(tag(","), expression), tag(")")),
            multispace0,
        ),
        CallType::Function,
    )(input)
}

fn safe_access(input: &str) -> IResult<&str, CallType, VerboseError<&str>> {
    map(delimited(multispace0, tag("?"), multispace0), |_| CallType::SafeAccess)(input)
}

fn array_index(input: &str) -> IResult<&str, CallType, VerboseError<&str>> {
    map(delimited(multispace0, delimited(tag("["), expression, tag("]")), multispace0), |expr| {
        CallType::ArrayIndex(expr)
    })(input)
}

fn field_access(input: &str) -> IResult<&str, CallType, VerboseError<&str>> {
    map(delimited(multispace0, preceded(tag("."), symbol), multispace0), |symbol| {
        CallType::FieldAccess(symbol)
    })(input)
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

fn unary(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let unary_rule = map(
        delimited(multispace0, pair(alt((char('-'), char('!'))), unary), multispace0),
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

    fold_many0(pair(alt((char('*'), char('/'), char('%'))), unary), init, |acc, (op, value)| {
        let op = match op {
            '*' => BinaryOperator::Multiply,
            '/' => BinaryOperator::Divide,
            '%' => BinaryOperator::Remainder,
            _ => unreachable!(), // TODO: make this an error?
        };

        Expression::Binary(op, Box::new(acc), Box::new(value))
    })(input)
}

fn additive(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = multiplicative(input)?;

    fold_many0(pair(alt((char('+'), char('-'))), multiplicative), init, |acc, (op, value)| {
        let op = match op {
            '+' => BinaryOperator::Add,
            '-' => BinaryOperator::Subtract,
            _ => unreachable!(), // TODO: make this an error?
        };

        Expression::Binary(op, Box::new(acc), Box::new(value))
    })(input)
}

fn comparison(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = additive(input)?;

    fold_many0(
        pair(alt((tag("=="), tag("!="), tag(">"), tag("<"), tag(">="), tag("<="))), additive),
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

    fold_many0(preceded(tag("&&"), comparison), init, |acc, expr| {
        Expression::Binary(BinaryOperator::LogicalAnd, Box::new(acc), Box::new(expr))
    })(input)
}

fn logical_or(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = logical_and(input)?;

    fold_many0(preceded(tag("||"), logical_and), init, |acc, expr| {
        Expression::Binary(BinaryOperator::LogicalOr, Box::new(acc), Box::new(expr))
    })(input)
}

fn range(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    alt((
        map(tuple((logical_or, alt((tag("..="), tag(".."))), logical_or)), |(lhs, op, rhs)| match op {
            ".." => Expression::Range(RangeOperator::Exclusive, Box::new(lhs), Box::new(rhs)),
            "..=" => Expression::Range(RangeOperator::Inclusive, Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        }),
        logical_or,
    ))(input)
}

fn coalesce(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = range(input)?;

    fold_many0(preceded(tag(":?"), range), init, |acc, expr| {
        Expression::Binary(BinaryOperator::Coalesce, Box::new(acc), Box::new(expr))
    })(input)
}

fn if_expression(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    map(
        tuple((
            preceded(delimited(multispace0, tag("if"), multispace1), discard),
            delimited(tag("{"), discard, tag("}")),
            opt(alt((
                preceded(
                    delimited(multispace0, tag("else"), multispace1),
                    delimited(tag("{"), expression, tag("}")),
                ),
                preceded(delimited(multispace0, tag("else"), multispace1), if_expression),
            ))),
        )),
        |(condition, body, alt)| Expression::Conditional(Box::new(condition), Box::new(body), alt.map(Box::new)),
    )(input)
}

fn conditional(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    alt((if_expression, coalesce))(input)
}

fn discard(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, init) = conditional(input)?;

    fold_many0(preceded(tag(";"), conditional), init, |acc, expr| {
        Expression::Binary(BinaryOperator::Discard, Box::new(acc), Box::new(expr))
    })(input)
}

fn expression(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    discard(input)
}

pub fn parse(input: &str) -> Result<Expression, String> {
    match all_consuming(terminated(expression, multispace0))(input) {
        Ok((_, result)) => Ok(result),
        Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => Err(convert_error(input, err)),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let result = parse("2d8.reroll(8)");

        match result {
            Ok(ok) => println!("{}", ok),
            Err(err) => println!("{}", err),
        }
    }
}
