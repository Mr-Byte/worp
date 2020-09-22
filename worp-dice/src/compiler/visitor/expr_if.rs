use super::NodeVisitor;
use crate::{compiler::Compiler, syntax::IfExpression, CompilerError};

impl NodeVisitor<&IfExpression> for Compiler {
    fn visit(
        &mut self,
        IfExpression {
            condition,
            primary,
            secondary,
            span,
        }: &IfExpression,
    ) -> Result<(), CompilerError> {
        self.visit(*condition)?;
        let if_jump = self.context()?.assembler().jump_if_false(*span);
        self.visit(*primary)?;

        let else_jump = self.context()?.assembler().jump(*span);

        self.context()?.assembler().patch_jump(if_jump);

        if let Some(secondary) = secondary {
            self.visit(*secondary)?;
        } else {
            self.context()?.assembler().push_unit(*span);
        }

        self.context()?.assembler().patch_jump(else_jump);

        Ok(())
    }
}
