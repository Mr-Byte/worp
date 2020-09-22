use crate::{runtime::core::Span, syntax::SyntaxNodeId};

#[derive(Debug, Clone)]
pub struct LitAnonymousFn {
    pub args: Vec<String>,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitList {
    pub items: Vec<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitObject {
    pub items: Vec<(SyntaxNodeId, SyntaxNodeId)>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitIdent {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitUnit {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitNone {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitInt {
    pub value: i64,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitFloat {
    pub value: f64,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitString {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitBool {
    pub value: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
    DiceRoll,
}

#[derive(Debug, Clone)]
pub struct SafeAccess {
    pub expression: SyntaxNodeId,
    pub field: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FieldAccess {
    pub expression: SyntaxNodeId,
    pub field: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub target: SyntaxNodeId,
    pub args: Vec<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Index {
    pub expression: SyntaxNodeId,
    pub index_expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub operator: BinaryOperator,
    pub lhs_expression: SyntaxNodeId,
    pub rhs_expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    DiceRoll,
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    Equals,
    NotEquals,
    LogicalAnd,
    LogicalOr,
    RangeInclusive,
    RangeExclusive,
    Coalesce,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub operator: AssignmentOperator,
    pub lhs_expression: SyntaxNodeId,
    pub rhs_expression: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Assignment,
    MulAssignment,
    DivAssignment,
    AddAssignment,
    SubAssignment,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub is_mutable: bool,
    pub expr: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub args: Vec<String>,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: SyntaxNodeId,
    pub primary: SyntaxNodeId,
    pub secondary: Option<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: SyntaxNodeId,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub variable: String,
    pub source: SyntaxNodeId,
    pub body: SyntaxNodeId,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub expressions: Vec<SyntaxNodeId>,
    pub trailing_expression: Option<SyntaxNodeId>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Break {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Continue {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub result: Option<SyntaxNodeId>,
    pub span: Span,
}
