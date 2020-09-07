use super::NodeCompiler;
use crate::{
    compiler::{component::scope::ScopeKind, Compiler},
    syntax::{Block, SyntaxNode},
    CompilerError,
};

impl NodeCompiler<Block> for Compiler {
    fn compile_node(&mut self, Block(items, span): Block) -> Result<(), CompilerError> {
        self.scope_stack.push_scope(ScopeKind::Block, None);

        for expression in items.iter() {
            self.compile_node(*expression)?;
        }

        // NOTE: If the block is empty or the last element is a discard of variable, push unit onto the stack.
        match items.last() {
            Some(node) => match self.syntax_tree.get(*node) {
                Some(SyntaxNode::Discard(_)) => self.assembler.push_unit(span),
                Some(SyntaxNode::VariableDeclaration(_)) => self.assembler.push_unit(span),
                _ => {}
            },
            None => self.assembler.push_unit(span),
        }

        self.scope_stack.pop_scope()?;

        Ok(())
    }
}
