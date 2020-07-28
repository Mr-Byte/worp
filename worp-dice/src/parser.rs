// Remove once the parser is used via a public export.
#![allow(dead_code)]

use crate::expression::{AccessType, BinaryOperator, Expression, Literal, RangeOperator, Symbol, UnaryOperator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, alphanumeric1, digit1},
    character::complete::{char, space0},
    combinator::{map, map_res, opt},
    multi::{fold_many0, many0, separated_list0},
    number::complete::float,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn none_literal(input: &str) -> IResult<&str, Literal> {
    map(delimited(space0, tag("None"), space0), |_| Literal::None)(input)
}

fn float_literal(input: &str) -> IResult<&str, Literal> {
    map(delimited(space0, terminated(float, char('f')), space0), Literal::Float)(input)
}

fn int_literal(input: &str) -> IResult<&str, Literal> {
    let int = map_res(
        tuple((opt(alt((char('+'), char('-')))), digit1)),
        |(sign, value): (Option<char>, &str)| match sign {
            Some('-') => value.parse().map(|value: i32| -value),
            _ => value.parse(),
        },
    );

    map(delimited(space0, int, space0), Literal::Integer)(input)
}

fn boolean_literal(input: &str) -> IResult<&str, Literal> {
    let bool = map(alt((tag("true"), tag("false"))), |value| match value {
        "true" => true,
        "false" => false,
        _ => unreachable!(),
    });

    map(delimited(space0, bool, space0), Literal::Boolean)(input)
}

fn string_literal(input: &str) -> IResult<&str, Literal> {
    map(delimited(char('"'), take_till(|c| c == '"'), char('"')), |value: &str| {
        Literal::String(value.to_owned())
    })(input)
}

fn list(input: &str) -> IResult<&str, Literal> {
    map(
        delimited(char('['), separated_list0(delimited(space0, char(','), space0), expression), char(']')),
        Literal::List,
    )(input)
}

fn literal(input: &str) -> IResult<&str, Expression> {
    map(
        alt((none_literal, float_literal, int_literal, string_literal, boolean_literal, list)),
        Expression::Literal,
    )(input)
}

fn symbol(input: &str) -> IResult<&str, Expression> {
    let symbol_start = alt((tag("_"), alpha1));
    let symbol_remainder = many0(alt((tag("_"), alphanumeric1)));
    let symbol = pair(symbol_start, symbol_remainder);

    delimited(
        space0,
        map(symbol, |(start, rest): (&str, Vec<&str>)| {
            let mut result = String::new();
            result += start;

            for parts in &rest {
                result += parts;
            }

            Expression::Symbol(Symbol(result))
        }),
        space0,
    )(input)
}

fn parens(input: &str) -> IResult<&str, Expression> {
    delimited(space0, delimited(char('('), expression, char(')')), space0)(input)
}

fn primary(input: &str) -> IResult<&str, Expression> {
    alt((literal, symbol, parens))(input)
}

fn access(input: &str) -> IResult<&str, Expression> {
    let (input, init) = primary(input)?;

    enum CallType<'a> {
        Function(Vec<Expression>),
        ArrayIndex(&'a str, Expression),
        FieldAccess(&'a str, Expression),
    }

    let function_call = map(
        delimited(space0, delimited(tag("("), separated_list0(tag(","), expression), tag(")")), space0),
        CallType::Function,
    );
    let array_index = map(
        delimited(space0, pair(alt((tag("["), tag("?["))), terminated(expression, tag("]"))), space0),
        |(op, symbol)| CallType::ArrayIndex(op, symbol),
    );
    let field_access = map(delimited(space0, pair(alt((tag("."), tag("?."))), symbol), space0), |(op, symbol)| {
        CallType::FieldAccess(op, symbol)
    });

    let call = alt((function_call, array_index, field_access));

    fold_many0(call, init, |acc, call_type| match call_type {
        CallType::Function(args) => Expression::FunctionCall(Box::new(acc), args),
        CallType::ArrayIndex("[", arg) => Expression::Index(AccessType::Direct, Box::new(acc), Box::new(arg)),
        CallType::ArrayIndex("?[", arg) => Expression::Index(AccessType::Safe, Box::new(acc), Box::new(arg)),
        CallType::FieldAccess(".", field) => Expression::FieldAccess(AccessType::Direct, Box::new(acc), Box::new(field)),
        CallType::FieldAccess("?.", field) => Expression::FieldAccess(AccessType::Safe, Box::new(acc), Box::new(field)),
        _ => unreachable!(),
    })(input)
}

// TODO: Fix the bug here where unaries don't seem to parse correctly in scenarios like `!true && !true`
fn unary(input: &str) -> IResult<&str, Expression> {
    let unary_rule = map(
        delimited(space0, pair(alt((char('-'), char('!'))), unary), space0),
        |(op, expr)| match op {
            '-' => Expression::Unary(UnaryOperator::Negate, Box::new(expr)),
            '!' => Expression::Unary(UnaryOperator::Not, Box::new(expr)),
            _ => unreachable!(),
        },
    );

    alt((access, unary_rule))(input)
}

fn dice_roll(input: &str) -> IResult<&str, Expression> {
    alt((
        map(separated_pair(unary, tag("d"), unary), |(lhs, rhs)| {
            Expression::Binary(BinaryOperator::DiceRoll, Box::new(lhs), Box::new(rhs))
        }),
        unary,
    ))(input)
}

fn multiplicative(input: &str) -> IResult<&str, Expression> {
    let (input, init) = dice_roll(input)?;

    fold_many0(pair(alt((char('*'), char('/'), char('%'))), dice_roll), init, |acc, (op, value)| {
        let op = match op {
            '*' => BinaryOperator::Multiply,
            '/' => BinaryOperator::Divide,
            '%' => BinaryOperator::Remainder,
            _ => unreachable!(), // TODO: make this an error?
        };

        Expression::Binary(op, Box::new(acc), Box::new(value))
    })(input)
}

fn additive(input: &str) -> IResult<&str, Expression> {
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

fn comparison(input: &str) -> IResult<&str, Expression> {
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

fn logical_and(input: &str) -> IResult<&str, Expression> {
    let (input, init) = comparison(input)?;

    fold_many0(preceded(tag("&&"), comparison), init, |acc, expr| {
        Expression::Binary(BinaryOperator::LogicalAnd, Box::new(acc), Box::new(expr))
    })(input)
}

fn logical_or(input: &str) -> IResult<&str, Expression> {
    let (input, init) = logical_and(input)?;

    fold_many0(preceded(tag("||"), logical_and), init, |acc, expr| {
        Expression::Binary(BinaryOperator::LogicalOr, Box::new(acc), Box::new(expr))
    })(input)
}

fn range(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tuple((logical_or, alt((tag("..="), tag(".."))), logical_or)), |(lhs, op, rhs)| match op {
            ".." => Expression::Range(RangeOperator::Exclusive, Box::new(lhs), Box::new(rhs)),
            "..=" => Expression::Range(RangeOperator::Inclusive, Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        }),
        logical_or,
    ))(input)
}

fn coalesce(input: &str) -> IResult<&str, Expression> {
    let (input, init) = range(input)?;

    fold_many0(preceded(tag("??"), range), init, |acc, expr| {
        Expression::Binary(BinaryOperator::Coalesce, Box::new(acc), Box::new(expr))
    })(input)
}

fn discard(input: &str) -> IResult<&str, Expression> {
    let (input, init) = coalesce(input)?;

    fold_many0(preceded(tag(";"), coalesce), init, |acc, expr| {
        Expression::Binary(BinaryOperator::Discard, Box::new(acc), Box::new(expr))
    })(input)
}

fn expression(input: &str) -> IResult<&str, Expression> {
    discard(input)
}

pub fn parse<'a>(input: &'a str) -> Result<Expression, Box<dyn std::error::Error + 'a>> {
    let (_, result) = expression(input)?;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn int_parses() -> Result<(), Box<dyn std::error::Error>> {
        if let (_, Literal::Integer(literal)) = int_literal("1")? {
            assert_eq!(1, literal);
            Ok(())
        } else {
            panic!("Invalid type.")
        }
    }

    #[test]
    fn float_parses() -> Result<(), Box<dyn std::error::Error>> {
        if let (_, Literal::Float(literal)) = float_literal("-1f")? {
            assert_eq!(-1.0f32, literal);
            Ok(())
        } else {
            panic!("Invalid type.")
        }
    }

    #[test]
    fn test() {
        let output = parse(r#"[None, None]?[1]?.xyz"#).unwrap();
        println!("{:#?}", output);
    }
}
