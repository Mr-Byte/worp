use crate::{compiler::Compiler, syntax::LitIdent, CompilerError, Symbol};

use super::NodeVisitor;

impl NodeVisitor<&LitIdent> for Compiler {
    fn visit(&mut self, LitIdent(name, span): &LitIdent) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.scope_stack.local(name)?.slot as u8;

        self.current_assembler().load_local(slot, span.clone());

        Ok(())
    }
}
