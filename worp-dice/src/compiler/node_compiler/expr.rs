use super::NodeCompiler;
use crate::{
    compiler::Compiler,
    syntax::{SyntaxNode, SyntaxNodeId},
    CompilerError,
};

impl NodeCompiler<SyntaxNodeId> for Compiler {
    fn compile_node(&mut self, node: SyntaxNodeId) -> Result<(), CompilerError> {
        let node = self
            .syntax_tree
            .get(node)
            .cloned()
            .expect("Node should never be empty.");

        match &node {
            SyntaxNode::Literal(literal) => self.compile_node(literal)?,
            SyntaxNode::SafeAccess(_) => todo!(),
            SyntaxNode::FieldAccess(_) => todo!(),
            SyntaxNode::Index(_) => todo!(),
            SyntaxNode::Assignment(assignment) => self.compile_node(assignment)?,
            SyntaxNode::Unary(unary) => self.compile_node(unary)?,
            SyntaxNode::Binary(binary) => self.compile_node(binary)?,
            SyntaxNode::VariableDeclaration(variable) => self.compile_node(variable)?,
            SyntaxNode::IfExpression(conditional) => self.compile_node(conditional)?,
            SyntaxNode::WhileLoop(while_loop) => self.compile_node(while_loop)?,
            SyntaxNode::ForLoop(_) => todo!(),
            SyntaxNode::Break(break_node) => self.compile_node(break_node)?,
            SyntaxNode::Continue(continue_node) => self.compile_node(continue_node)?,
            SyntaxNode::Block(block) => self.compile_node(block)?,
        }

        Ok(())
    }
}
