use crate::runtime::core::Span;
use id_arena::{Arena, Id};
use std::rc::Rc;

pub type SyntaxNodeId = Id<SyntaxNode>;

pub struct SyntaxTree {
    root: SyntaxNodeId,
    nodes: Rc<Arena<SyntaxNode>>,
}

impl SyntaxTree {
    pub(crate) fn new(root: SyntaxNodeId, nodes: Arena<SyntaxNode>) -> Self {
        Self {
            root,
            nodes: Rc::new(nodes),
        }
    }

    pub fn root(&self) -> SyntaxNodeId {
        self.root
    }

    pub fn get(&self, id: SyntaxNodeId) -> Option<&SyntaxNode> {
        self.nodes.get(id)
    }

    pub fn child(&self, id: SyntaxNodeId) -> Option<SyntaxTree> {
        self.nodes.get(id).map(|_| Self {
            root: id,
            nodes: self.nodes.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum SyntaxNode {
    Literal(Literal),
    SafeAccess(SafeAccess),
    FieldAccess(FieldAccess),
    Index(Index),

    // Operators
    Unary(Unary),
    Binary(Binary),
    Assignment(Assignment),

    // Statements
    VariableDeclaration(VariableDeclaration),
    Conditional(Conditional),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    Block(Block),
    Discard(Span),
    Break(Span),
    Continue(Span),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String, Span),
    None(Span),
    Unit(Span),
    Integer(i64, Span),
    Float(f64, Span),
    String(String, Span),
    Boolean(bool, Span),
    List(Vec<SyntaxNodeId>, Span),
    Object(Vec<(SyntaxNodeId, SyntaxNodeId)>, Span),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
    DiceRoll,
}

#[derive(Debug, Clone)]
pub struct SafeAccess(pub SyntaxNodeId, pub String, pub Span);

#[derive(Debug, Clone)]
pub struct FieldAccess(pub SyntaxNodeId, pub String, pub Span);

#[derive(Debug, Clone)]
pub struct FunctionCall(pub SyntaxNodeId, pub Vec<SyntaxNodeId>, pub Span);

#[derive(Debug, Clone)]
pub struct Index(pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct Unary(pub UnaryOperator, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct Binary(pub BinaryOperator, pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

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
pub struct Assignment(pub AssignmentOperator, pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Assignment,
    MulAssignment,
    DivAssignment,
    AddAssignment,
    SubAssignment,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration(pub String, pub bool, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct Conditional(pub SyntaxNodeId, pub SyntaxNodeId, pub Option<SyntaxNodeId>, pub Span);

#[derive(Debug, Clone)]
pub struct WhileLoop(pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct ForLoop(pub String, pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct Block(pub Vec<SyntaxNodeId>, pub Span);
