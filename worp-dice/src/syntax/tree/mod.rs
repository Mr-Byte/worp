mod node;

use crate::runtime::core::Span;
use id_arena::{Arena, Id};
pub use node::*;
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
    LitAnonymousFn(LitAnonymousFn),

    // Member access
    SafeAccess(SafeAccess),
    FieldAccess(FieldAccess),
    Index(Index),

    // Operators
    Unary(Unary),
    Binary(Binary),
    Assignment(Assignment),

    // Declarations
    VarDecl(VarDecl),
    FnDecl(FnDecl),

    // Control flow
    IfExpression(IfExpression),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    Block(Block),
    Break(Break),
    Return(Return),
    Continue(Continue),
    FunctionCall(FunctionCall),
}

impl SyntaxNode {
    pub fn span(&self) -> Span {
        match self {
            SyntaxNode::LitIdent(LitIdent { span, .. }) => *span,
            SyntaxNode::LitNone(LitNone { span, .. }) => *span,
            SyntaxNode::LitUnit(LitUnit { span, .. }) => *span,
            SyntaxNode::LitInt(LitInt { span, .. }) => *span,
            SyntaxNode::LitFloat(LitFloat { span, .. }) => *span,
            SyntaxNode::LitString(LitString { span, .. }) => *span,
            SyntaxNode::LitBool(LitBool { span, .. }) => *span,
            SyntaxNode::LitList(LitList { span, .. }) => *span,
            SyntaxNode::LitObject(LitObject { span, .. }) => *span,
            SyntaxNode::LitAnonymousFn(LitAnonymousFn { span, .. }) => *span,
            SyntaxNode::SafeAccess(SafeAccess { span, .. }) => *span,
            SyntaxNode::FieldAccess(FieldAccess { span, .. }) => *span,
            SyntaxNode::Index(Index { span, .. }) => *span,
            SyntaxNode::Unary(Unary { span, .. }) => *span,
            SyntaxNode::Binary(Binary { span, .. }) => *span,
            SyntaxNode::Assignment(Assignment { span, .. }) => *span,
            SyntaxNode::VarDecl(VarDecl { span, .. }) => *span,
            SyntaxNode::FnDecl(FnDecl { span, .. }) => *span,
            SyntaxNode::IfExpression(IfExpression { span, .. }) => *span,
            SyntaxNode::WhileLoop(WhileLoop { span, .. }) => *span,
            SyntaxNode::ForLoop(ForLoop { span, .. }) => *span,
            SyntaxNode::Block(Block { span, .. }) => *span,
            SyntaxNode::Break(Break { span, .. }) => *span,
            SyntaxNode::Return(Return { span, .. }) => *span,
            SyntaxNode::Continue(Continue { span, .. }) => *span,
            SyntaxNode::FunctionCall(FunctionCall { span, .. }) => *span,
        }
    }
}
