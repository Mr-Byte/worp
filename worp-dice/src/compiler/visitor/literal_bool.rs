use crate::{compiler::Compiler, syntax::LitBool, CompilerError};

use super::NodeVisitor;

impl NodeVisitor<&LitBool> for Compiler {
    fn visit(
        &mut self,
        LitBool {
            value,
            span,
        }: &LitBool,
    ) -> Result<(), CompilerError> {
        self.compiler_stack.top_mut()?.assembler().push_bool(*value, *span);

        Ok(())
    }
}
