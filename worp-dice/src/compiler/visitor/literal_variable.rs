use crate::{compiler::Compiler, syntax::LitIdent, CompilerError, Symbol};

use super::NodeVisitor;

impl NodeVisitor<&LitIdent> for Compiler {
    fn visit(&mut self, LitIdent(name, span): &LitIdent) -> Result<(), CompilerError> {
        let context = self.context()?;
        let name = Symbol::new(name);
        let slot = context.scope_stack().local(name).expect("Fix this").slot as u8;

        context.assembler().load_local(slot, span.clone());

        Ok(())
    }
}
