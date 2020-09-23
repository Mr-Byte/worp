use crate::{compiler::Compiler, syntax::LitIdent, CompilerError, Symbol, Value};

use super::NodeVisitor;

impl NodeVisitor<&LitIdent> for Compiler {
    fn visit(&mut self, LitIdent { name, span }: &LitIdent) -> Result<(), CompilerError> {
        let name_symbol = Symbol::new(name);

        {
            let context = self.context()?;
            if let Some(scope_variable) = context.scope_stack().local(name_symbol.clone()) {
                if !scope_variable.is_initialized() {
                    return Err(CompilerError::UnitiailizedVariable(scope_variable.name.clone()));
                }

                let slot = scope_variable.slot as u8;
                context.assembler().load_local(slot, *span);

                return Ok(());
            }
        }

        if let Some(upvalue) = self.compiler_stack.resolve_upvalue(name_symbol.clone(), 0) {
            let context = self.context()?;
            context.assembler().load_upvalue(upvalue as u8, *span);

            return Ok(());
        }

        let context = self.context()?;
        context.assembler().load_global(Value::String(name.clone()), *span)?;

        Ok(())
    }
}
