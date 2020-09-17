use super::NodeVisitor;
use crate::{
    compiler::{scope::ScopeKind, Compiler},
    syntax::Continue,
    CompilerError,
};

impl NodeVisitor<&Continue> for Compiler {
    fn visit(&mut self, Continue(span): &Continue) -> Result<(), crate::CompilerError> {
        let context = self.context()?;
        if !context.scope_stack().in_context_of(ScopeKind::Loop) {
            return Err(CompilerError::InvalidContinue);
        }

        let loop_start = context.scope_stack().entry_point(ScopeKind::Loop)?;
        context.assembler().jump_back(loop_start as u64, span.clone());

        Ok(())
    }
}
