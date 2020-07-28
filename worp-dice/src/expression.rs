use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Symbol(pub String);

#[derive(Debug, Clone)]
pub enum Literal {
    /// None values
    None,
    /// Integer values such as `-1`, `0`, `1`, etc
    Integer(i32),
    /// Floating point decimals such as `-1.0, `0.0`, `1.1`, etc
    Float(f32),
    /// String literals such as `"hello"`
    String(String),
    /// Boolean literals (true or false)
    Boolean(bool),
    /// Lists, such as `[ 1, x, 3 ]`
    List(Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
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

    /// The `??` operator (null-coalesce)
    Coalesce,

    /// The `;` operator (discard)
    Discard,
}

#[derive(Debug, Clone)]
pub enum RangeOperator {
    Exclusive,
    Inclusive,
}

#[derive(Debug, Clone)]
pub enum AccessType {
    Direct,
    Safe,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    /// Symbols represent some named item (variables, etc) within an expression
    Symbol(Symbol),

    //Primary operators
    /// Access to a field, (e.g. `x.y`)
    FieldAccess(AccessType, Box<Expression>, Box<Expression>), // TODO: Figure out if this should be expr -> ident, or expr -> expr
    /// Function call (e.g. `y(1, 2)`
    /// First part evaluates to a function, second part is the parameters
    FunctionCall(Box<Expression>, Vec<Expression>),
    /// Indexed access (e.g. `x.y[1]` or `y["x"]`)
    Index(AccessType, Box<Expression>, Box<Expression>),

    // Operators
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Range(RangeOperator, Box<Expression>, Box<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
}

impl Display for Expression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
