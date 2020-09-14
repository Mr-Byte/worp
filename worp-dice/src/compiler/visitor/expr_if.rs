use super::NodeVisitor;
use crate::{compiler::Compiler, syntax::IfExpression, CompilerError};

impl NodeVisitor<&IfExpression> for Compiler {
    fn visit(&mut self, IfExpression(condition, primary, secondary, span): &IfExpression) -> Result<(), CompilerError> {
        self.visit(*condition)?;
        let if_jump = self.current_assembler().jump_if_false(span.clone());
        self.visit(*primary)?;

        let else_jump = self.current_assembler().jump(span.clone());

        self.current_assembler().patch_jump(if_jump);

        if let Some(secondary) = secondary {
            self.visit(*secondary)?;
        } else {
            self.current_assembler().push_unit(span.clone());
        }

        self.current_assembler().patch_jump(else_jump);

        Ok(())
    }
}
