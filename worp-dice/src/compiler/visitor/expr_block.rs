use super::NodeVisitor;
use crate::{
    compiler::{scope::ScopeKind, Compiler},
    syntax::Block,
    CompilerError,
};

impl NodeVisitor<&Block> for Compiler {
    fn visit(&mut self, Block(items, trailing_expression, span): &Block) -> Result<(), CompilerError> {
        self.scope_stack.push_scope(ScopeKind::Block, None);

        for expression in items.iter() {
            self.visit(*expression)?;
            self.assembler.pop(span.clone());
        }

        match trailing_expression {
            Some(trailing_expression) => self.visit(*trailing_expression)?,
            None => self.assembler.push_unit(span.clone()),
        }

        self.scope_stack.pop_scope()?;

        Ok(())
    }
}
