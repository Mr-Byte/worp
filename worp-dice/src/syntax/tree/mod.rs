use crate::runtime::core::Span;
use id_arena::{Arena, Id};
use std::fmt::Display;

pub type SyntaxNodeId = Id<SyntaxNode>;

pub struct SyntaxTree {
    root: SyntaxNodeId,
    nodes: Arena<SyntaxNode>,
}

impl SyntaxTree {
    pub(crate) fn new(root: SyntaxNodeId, nodes: Arena<SyntaxNode>) -> Self {
        Self { root, nodes }
    }

    pub fn root(&self) -> SyntaxNodeId {
        self.root
    }

    pub fn get(&self, id: SyntaxNodeId) -> Option<&SyntaxNode> {
        self.nodes.get(id)
    }
}

impl Display for SyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = self.get(self.root) {
            fmt_node(root, &self.nodes, f)
        } else {
            write!(f, "")
        }
    }
}

fn fmt_node(node: &SyntaxNode, nodes: &Arena<SyntaxNode>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match node {
        SyntaxNode::Literal(literal) => match literal {
            Literal::Integer(integer, _) => write!(f, "{}", integer),
            _ => todo!(),
        },
        SyntaxNode::Unary(Unary(operator, expr, _)) => {
            match operator {
                UnaryOperator::Negate => write!(f, "-")?,
                UnaryOperator::Not => write!(f, "!")?,
                UnaryOperator::DiceRoll => write!(f, "d")?,
            }

            fmt_node(nodes.get(*expr).unwrap(), nodes, f)?;

            Ok(())
        }
        SyntaxNode::Binary(Binary(operator, lhs, rhs, _)) => {
            write!(f, "(")?;
            match operator {
                BinaryOperator::Multiply => write!(f, "*")?,
                BinaryOperator::Divide => write!(f, "/")?,
                BinaryOperator::Remainder => write!(f, "%")?,
                BinaryOperator::Add => write!(f, "+")?,
                BinaryOperator::Subtract => write!(f, "-")?,
                BinaryOperator::DiceRoll => write!(f, "d")?,
                BinaryOperator::GreaterThan => write!(f, ">")?,
                BinaryOperator::LessThan => write!(f, "<")?,
                BinaryOperator::GreaterThanEquals => write!(f, ">=")?,
                BinaryOperator::LessThanEquals => write!(f, "<=")?,
                BinaryOperator::Equals => write!(f, "==")?,
                BinaryOperator::NotEquals => write!(f, "!=")?,
                BinaryOperator::LogicalAnd => write!(f, "&&")?,
                BinaryOperator::LogicalOr => write!(f, "||")?,
                BinaryOperator::RangeInclusive => write!(f, "..=")?,
                BinaryOperator::RangeExclusive => write!(f, "..")?,
                BinaryOperator::Coalesce => write!(f, "??")?,
            }
            write!(f, " ")?;
            fmt_node(nodes.get(*lhs).unwrap(), nodes, f)?;
            write!(f, " ")?;
            fmt_node(nodes.get(*rhs).unwrap(), nodes, f)?;
            write!(f, ")")?;

            Ok(())
        }
        _ => todo!(),
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

    // Statements
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),
    Conditional(Conditional),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    Block(Block),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String, Span),
    None(Span),
    Integer(i64, Span),
    Float(f64, Span),
    String(String, Span),
    Boolean(bool, Span),
    // TODO: Pull these out as their own expression types.
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
pub struct VariableDeclaration(pub String, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct Assignment(pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct Conditional(pub SyntaxNodeId, pub SyntaxNodeId, pub Option<SyntaxNodeId>, pub Span);

#[derive(Debug, Clone)]
pub struct WhileLoop(pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct ForLoop(pub String, pub SyntaxNodeId, pub SyntaxNodeId, pub Span);

#[derive(Debug, Clone)]
pub struct Block(pub Vec<SyntaxNodeId>, pub Span);
