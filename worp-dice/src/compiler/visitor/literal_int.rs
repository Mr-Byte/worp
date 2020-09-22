use crate::{compiler::Compiler, syntax::LitInt, CompilerError, Value};

use super::NodeVisitor;

impl NodeVisitor<&LitInt> for Compiler {
    fn visit(
        &mut self,
        LitInt {
            value,
            span,
        }: &LitInt,
    ) -> Result<(), CompilerError> {
        let context = self.context()?;

        match value {
            0 => context.assembler().push_i0(*span),
            1 => context.assembler().push_i1(*span),
            _ => context.assembler().push_const(Value::Int(*value), *span)?,
        }

        Ok(())
    }
}
