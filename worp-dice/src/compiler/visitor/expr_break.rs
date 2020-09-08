use super::NodeVisitor;
use crate::{
    compiler::{scope::ScopeKind, Compiler},
    syntax::Break,
    CompilerError,
};

impl NodeVisitor<&Break> for Compiler {
    fn visit(&mut self, Break(span): &Break) -> Result<(), crate::CompilerError> {
        if !self.scope_stack.in_context_of(ScopeKind::Loop) {
            return Err(CompilerError::InvalidBreak);
        }

        let patch_location = self.assembler.jump(span.clone());
        self.scope_stack.add_loop_exit_point(patch_location as usize)?;

        Ok(())
    }
}
