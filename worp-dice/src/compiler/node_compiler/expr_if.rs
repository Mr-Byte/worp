use super::NodeCompiler;
use crate::{compiler::Compiler, syntax::IfExpression, CompilerError};

impl NodeCompiler<&IfExpression> for Compiler {
    fn compile_node(
        &mut self,
        IfExpression(condition, primary, secondary, span): &IfExpression,
    ) -> Result<(), CompilerError> {
        self.compile_node(*condition)?;
        let if_jump = self.assembler.jump_if_false(span.clone());
        self.compile_node(*primary)?;

        let else_jump = self.assembler.jump(span.clone());

        self.assembler.patch_jump(if_jump);

        if let Some(secondary) = secondary {
            self.compile_node(*secondary)?;
        } else {
            self.assembler.push_unit(span.clone());
        }

        self.assembler.patch_jump(else_jump);

        Ok(())
    }
}
