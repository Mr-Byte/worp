use super::NodeVisitor;
use crate::{
    compiler::{scope::ScopeKind, Compiler},
    syntax::Return,
    CompilerError,
};

impl NodeVisitor<&Return> for Compiler {
    fn visit(&mut self, expr_return: &Return) -> Result<(), crate::CompilerError> {
        if !self.scope_stack.in_context_of(ScopeKind::Function) {
            return Err(CompilerError::InvalidReturn);
        }

        match expr_return.result {
            Some(expr) => self.visit(expr)?,
            None => self.current_assembler().push_unit(expr_return.span.clone()),
        }

        self.current_assembler().ret(expr_return.span.clone());

        Ok(())
    }
}
