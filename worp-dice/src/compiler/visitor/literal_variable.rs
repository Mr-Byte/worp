use crate::{compiler::Compiler, syntax::LitIdent, CompilerError, Symbol};

use super::NodeVisitor;

impl NodeVisitor<&LitIdent> for Compiler {
    // TODO: Decompose this down into smaller functions.
    fn visit(&mut self, LitIdent(name, span): &LitIdent) -> Result<(), CompilerError> {
        let name = Symbol::new(name);

        {
            let context = self.context()?;
            if let Some(scope_variable) = context.scope_stack().local(name.clone()) {
                let slot = scope_variable.slot as u8;

                context.assembler().load_local(slot, span.clone());
                return Ok(());
            }
        }

        if let Some(upvalue) = self.resolve_upvalue(name, 0) {
            let context = self.context()?;
            context.assembler().load_upvalue(upvalue as u8, span.clone());
        }

        Ok(())
    }
}
