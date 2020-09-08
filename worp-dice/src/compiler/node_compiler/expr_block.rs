use super::NodeCompiler;
use crate::{
    compiler::{scope::ScopeKind, Compiler},
    syntax::Block,
    CompilerError,
};

impl NodeCompiler<Block> for Compiler {
    fn compile_node(&mut self, Block(items, trailing_expression, span): Block) -> Result<(), CompilerError> {
        self.scope_stack.push_scope(ScopeKind::Block, None);

        for expression in items.iter() {
            self.compile_node(*expression)?;
            self.assembler.pop(span.clone());
        }

        if let Some(trailing_expression) = trailing_expression {
            self.compile_node(trailing_expression)?;
        } else {
            self.assembler.push_unit(span);
        }

        self.scope_stack.pop_scope()?;

        Ok(())
    }
}
