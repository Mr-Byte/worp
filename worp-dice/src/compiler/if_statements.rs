use super::Compiler;
use crate::{syntax::Conditional, CompilerError};

impl Compiler {
    pub(super) fn conditional(
        &mut self,
        Conditional(condition, primary, secondary, span): Conditional,
    ) -> Result<(), CompilerError> {
        self.expression(condition)?;
        let if_jump = self.assembler.jump_if_false(span.clone());
        self.expression(primary)?;

        let else_jump = self.assembler.jump(span);

        self.assembler.patch_jump(if_jump);

        if let Some(secondary) = secondary {
            self.expression(secondary)?;
        }

        self.assembler.patch_jump(else_jump);

        Ok(())
    }
}
