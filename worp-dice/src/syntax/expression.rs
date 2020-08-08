use super::parser::error::ParseError;
use crate::runtime::core::{symbol::Symbol, ValueKey};
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub enum Literal {
    /// Identifiers (e.g. _test)
    Identifier(Symbol),
    /// None values
    None,
    /// Integer values such as `-1`, `0`, `1`, etc
    Integer(i64),
    /// Floating point decimals such as `-1.0, `0.0`, `1.1`, etc
    Float(f64),
    /// String literals such as `"hello"`
    String(String),
    /// Boolean literals (true or false)
    Boolean(bool),
    /// Lists, such as `[ 1, x, 3 ]`
    List(Vec<Expression>),
    /// Objects, such as { x: 55, y: 6d6, z: { inner: 42 } }
    Object(HashMap<ValueKey, Expression>),
}

impl Display for Literal {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Identifier(symbol) => symbol.fmt(fmt),
            Literal::None => write!(fmt, "none"),
            Literal::Integer(value) => write!(fmt, "{}", value),
            Literal::Float(value) => write!(fmt, "{:.}", value),
            Literal::String(value) => write!(fmt, r#""{}""#, value),
            Literal::Boolean(value) => write!(fmt, "{}", value),
            Literal::List(value) => write!(fmt, "( [{}] )", value.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")),
            Literal::Object(object) => write!(
                fmt,
                "{{ {} }}",
                object
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key, value))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
}

impl Display for UnaryOperator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Negate => write!(fmt, "-"),
            UnaryOperator::Not => write!(fmt, "!"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    // Operators are ordered and grouped by precedence.
    /// The `d` operator (dice rolls)
    DiceRoll,

    /// The `*` operator
    Multiply,
    /// The `/` operator
    Divide,
    /// The `%` operator
    Remainder,

    /// The `+` operator
    Add,
    /// The `-` operator
    Subtract,

    /// The `=` operator
    Equals,
    /// The `/=` operator
    NotEquals,
    /// The `>` operator
    GreaterThan,
    /// The `>=` operator
    GreaterThanOrEquals,
    /// The `<` operator
    LessThan,
    /// The `<=` operator
    LessThanOrEquals,

    /// The `and` operator
    LogicalAnd,

    /// The `or` operator
    LogicalOr,

    /// The `:?` operator (null-coalesce)
    Coalesce,

    /// The `;` operator (discard)
    Discard,
}

impl Display for BinaryOperator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::DiceRoll => write!(fmt, "d"),
            BinaryOperator::Multiply => write!(fmt, "*"),
            BinaryOperator::Divide => write!(fmt, "/"),
            BinaryOperator::Remainder => write!(fmt, "%"),
            BinaryOperator::Add => write!(fmt, "+"),
            BinaryOperator::Subtract => write!(fmt, "-"),
            BinaryOperator::Equals => write!(fmt, "=="),
            BinaryOperator::NotEquals => write!(fmt, "!="),
            BinaryOperator::GreaterThan => write!(fmt, ">"),
            BinaryOperator::GreaterThanOrEquals => write!(fmt, ">="),
            BinaryOperator::LessThan => write!(fmt, "<"),
            BinaryOperator::LessThanOrEquals => write!(fmt, "<="),
            BinaryOperator::LogicalAnd => write!(fmt, "&&"),
            BinaryOperator::LogicalOr => write!(fmt, "||"),
            BinaryOperator::Coalesce => write!(fmt, ":?"),
            BinaryOperator::Discard => write!(fmt, ";"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RangeOperator {
    Exclusive,
    Inclusive,
}

impl Display for RangeOperator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeOperator::Exclusive => write!(fmt, ".."),
            RangeOperator::Inclusive => write!(fmt, "..="),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),

    /// Use of the safe access operator `?` which will short-circuit further evaluation on `none`.
    SafeAccess(Box<Expression>, Symbol),

    //Primary operators
    /// Access to a field, (e.g. `x.y`)
    FieldAccess(Box<Expression>, Symbol), // TODO: Figure out if this should be expr -> ident, or expr -> expr
    /// Function call (e.g. `y(1, 2)`
    /// First part evaluates to a function, second part is the parameters
    FunctionCall(Box<Expression>, Vec<Expression>),
    /// Indexed access (e.g. `x.y[1]` or `y["x"]`)
    Index(Box<Expression>, Box<Expression>),

    // Operators
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Range(RangeOperator, Box<Expression>, Box<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
}

impl Display for Expression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(value) => value.fmt(fmt),
            Expression::SafeAccess(expr, field) => write!(fmt, "{}?.{}", expr, field),
            Expression::FieldAccess(expr, field) => write!(fmt, "{}.{}", expr, field),
            Expression::FunctionCall(expr, args) => {
                let args = args.iter().map(ToString::to_string).collect::<Vec<_>>().join(" ");
                write!(fmt, "{}( {} )", expr, args)
            }
            Expression::Index(expr, index) => write!(fmt, "{}[ {} ]", expr, index),
            Expression::Unary(op, expr) => write!(fmt, "{}{}", op, expr),
            Expression::Binary(op, lhs, rhs) => write!(fmt, "( {} {} {} )", lhs, op, rhs),
            Expression::Range(op, lhs, rhs) => write!(fmt, "{} {} {}", lhs, op, rhs),
            Expression::Conditional(condition, body, else_body) => write!(
                fmt,
                "if {} {{ {} }} else {{ {} }}",
                condition,
                body,
                else_body.as_ref().map(ToString::to_string).unwrap_or_default()
            ),
        }
    }
}

impl FromStr for Expression {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        super::parser::parse(input)
    }
}
