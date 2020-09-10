use super::NodeVisitor;
use crate::{
    compiler::{scope::ScopeKind, Compiler},
    syntax::Block,
    CompilerError,
};

pub enum BlockKind {
    Block,
    Function,
}

impl NodeVisitor<(&Block, BlockKind)> for Compiler {
    fn visit(&mut self, (block, kind): (&Block, BlockKind)) -> Result<(), CompilerError> {
        self.scope_stack.push_scope(ScopeKind::Block, None);

        for expression in block.expressions.iter() {
            self.visit(*expression)?;
            self.assembler.pop(block.span.clone());
        }

        match block.trailing_expression {
            Some(trailing_expression) => self.visit(trailing_expression)?,
            None => self.assembler.push_unit(block.span.clone()),
        }

        // NOTE: If in context of a function, implicitly return the top item on the stack.
        // If the previous instruction was a return, this will never execute.
        if let BlockKind::Function = kind {
            self.assembler.ret(block.span.clone())
        }

        self.scope_stack.pop_scope()?;

        Ok(())
    }
}
