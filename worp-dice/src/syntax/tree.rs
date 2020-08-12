use super::{parser::error::ParserError, span::Span};
use crate::runtime::core::{symbol::Symbol, ValueKey};
use std::{collections::HashMap, str::FromStr};

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
    List(Vec<SyntaxTree>),
    /// Objects, such as { x: 55, y: 6d6, z: { inner: 42 } }
    Object(HashMap<ValueKey, SyntaxTree>),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate(Span),
    Not(Span),
    DiceRoll(Span),
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    // Operators are ordered and grouped by precedence.
    /// The `d` operator (dice rolls)
    DiceRoll(Span),

    /// The `*` operator
    Multiply(Span),
    /// The `/` operator
    Divide(Span),
    /// The `%` operator
    Remainder(Span),

    /// The `+` operator
    Add(Span),
    /// The `-` operator
    Subtract(Span),

    /// The `=` operator
    Equals(Span),
    /// The `/=` operator
    NotEquals(Span),
    /// The `>` operator
    GreaterThan(Span),
    /// The `>=` operator
    GreaterThanOrEquals(Span),
    /// The `<` operator
    LessThan(Span),
    /// The `<=` operator
    LessThanOrEquals(Span),

    /// The `and` operator
    LogicalAnd(Span),

    /// The `or` operator
    LogicalOr(Span),

    /// The `:?` operator (null-coalesce)
    Coalesce(Span),
}

#[derive(Debug, Clone)]
pub enum RangeOperator {
    Exclusive(Span),
    Inclusive(Span),
}

#[derive(Debug, Clone)]
pub enum SyntaxTree {
    Literal(Literal, Span),

    /// Use of the safe access operator `?` which will short-circuit further evaluation on `none`.
    SafeAccess(Box<SyntaxTree>, Symbol, Span),

    //Primary operators
    /// Access to a field, (e.g. `x.y`)
    FieldAccess(Box<SyntaxTree>, Symbol, Span),
    /// Function call (e.g. `y(1, 2)`
    /// First part evaluates to a function, second part is the parameters
    FunctionCall(Box<SyntaxTree>, Vec<SyntaxTree>),
    /// Indexed access (e.g. `x.y[1]` or `y["x"]`)
    Index(Box<SyntaxTree>, Box<SyntaxTree>),

    // Operators
    Unary(UnaryOperator, Box<SyntaxTree>),
    Binary(BinaryOperator, Box<SyntaxTree>, Box<SyntaxTree>),
    Range(RangeOperator, Box<SyntaxTree>, Box<SyntaxTree>),
    Conditional(Box<SyntaxTree>, Box<SyntaxTree>, Option<Box<SyntaxTree>>),

    // Statements
    Statements(Vec<SyntaxTree>),
}

impl SyntaxTree {
    // fn span(&self) -> Span {
    //     match self {
    //         SyntaxTree::Literal(_, span) => span.clone(),
    //         SyntaxTree::SafeAccess(lhs, _, span) => lhs.span() + span,
    //         SyntaxTree::FieldAccess(lhs, _, span) => lhs.span() + span,
    //         SyntaxTree::FunctionCall(call, args) => call.span() + Self::sum_spans(args),
    //         SyntaxTree::Index(_, _) => {}
    //         SyntaxTree::Unary(_, _) => {}
    //         SyntaxTree::Binary(_, _, _) => {}
    //         SyntaxTree::Range(_, _, _) => {}
    //         SyntaxTree::Conditional(_, _, _) => {}
    //         SyntaxTree::Statements(_) => {}
    //     }
    // }

    // fn sum_spans(spans: &[SyntaxTree]) -> Span {
    //     spans.iter().map(|tree| tree.span()).sum()
    // }
}

impl FromStr for SyntaxTree {
    type Err = ParserError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        super::parser::Parser::parse_str(input)
    }
}
