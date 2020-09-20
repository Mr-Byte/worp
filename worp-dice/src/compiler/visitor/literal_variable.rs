use crate::{compiler::Compiler, syntax::LitIdent, CompilerError, Symbol};

use super::NodeVisitor;

impl NodeVisitor<&LitIdent> for Compiler {
    // TODO: Decompose this down into smaller functions.
    fn visit(&mut self, LitIdent(name, span): &LitIdent) -> Result<(), CompilerError> {
        let name = Symbol::new(name);

        {
            let context = self.context()?;
            if let Some(scope_variable) = context.scope_stack().local(name.clone()) {
                if !scope_variable.is_initialized() {
                    return Err(CompilerError::UnitiailizedVariable(scope_variable.name.clone()));
                }

                let slot = scope_variable.slot as u8;
                context.assembler().load_local(slot, span.clone());

                return Ok(());
            }
        }

        if let Some(upvalue) = self.resolve_upvalue(name.clone(), 0) {
            let context = self.context()?;
            context.assembler().load_upvalue(upvalue as u8, span.clone());

            return Ok(());
        }

        // TODO: Resolve to a global variable.
        Err(CompilerError::UndeclaredVariable(name))
    }
}
