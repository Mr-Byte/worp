use super::NodeCompiler;
use crate::{
    compiler::{component::scope::ScopeKind, Compiler},
    syntax::Break,
    CompilerError,
};

impl NodeCompiler<Break> for Compiler {
    fn compile_node(&mut self, Break(span): Break) -> Result<(), crate::CompilerError> {
        if !self.scope_stack.in_context_of(ScopeKind::Loop) {
            return Err(CompilerError::InvalidBreak);
        }

        let patch_location = self.assembler.jump(span);
        self.scope_stack.add_loop_exit_point(patch_location as usize)?;

        Ok(())
    }
}
