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
    // Literals
    LitIdent(LitIdent),
    LitNone(LitNone),
    LitUnit(LitUnit),
    LitInt(LitInt),
    LitFloat(LitFloat),
    LitString(LitString),
    LitBool(LitBool),
    LitList(LitList),
    LitObject(LitObject),

    // Member access
    SafeAccess(SafeAccess),
    FieldAccess(FieldAccess),
    Index(Index),

    // Operators
    Unary(Unary),
    Binary(Binary),
    Assignment(Assignment),

    // Declarations
    VariableDeclaration(VariableDeclaration),

    // Control flow
    IfExpression(IfExpression),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    Block(Block),
    Break(Break),
    Continue(Continue),
}

#[derive(Debug, Clone)]
pub struct LitList(pub Vec<SyntaxNodeId>, pub Span);

#[derive(Debug, Clone)]
pub struct LitObject(pub Vec<(SyntaxNodeId, SyntaxNodeId)>, pub Span);

#[derive(Debug, Clone)]
pub struct LitIdent(pub String, pub Span);

#[derive(Debug, Clone)]
pub struct LitUnit(pub Span);

#[derive(Debug, Clone)]
pub struct LitNone(pub Span);

#[derive(Debug, Clone)]
pub struct LitInt(pub i64, pub Span);

#[derive(Debug, Clone)]
pub struct LitFloat(pub f64, pub Span);

#[derive(Debug, Clone)]
pub struct LitString(pub String, pub Span);

#[derive(Debug, Clone)]
pub struct LitBool(pub bool, pub Span);

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
pub struct IfExpression(pub SyntaxNodeId, pub SyntaxNodeId, pub Option<SyntaxNodeId>, pub Span);

#[derive(Debug, Clone)]
pub struct WhileLoop(pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct ForLoop(pub String, pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct Block(pub Vec<SyntaxNodeId>, pub Option<SyntaxNodeId>, pub Span);

#[derive(Debug, Clone)]
pub struct Break(pub Span);

#[derive(Debug, Clone)]
pub struct Continue(pub Span);
