use super::NodeVisitor;
use crate::{
    compiler::{compiler::CompilerKind, Compiler},
    syntax::Return,
    CompilerError,
};

impl NodeVisitor<&Return> for Compiler {
    fn visit(&mut self, expr_return: &Return) -> Result<(), crate::CompilerError> {
        let context = self.context()?;

        if context.kind() != CompilerKind::Function {
            return Err(CompilerError::InvalidReturn);
        }

        match expr_return.result {
            Some(expr) => self.visit(expr)?,
            None => context.assembler().push_unit(expr_return.span.clone()),
        }

        self.context()?.assembler().ret(expr_return.span.clone());

        Ok(())
    }
}
