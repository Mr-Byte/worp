use crate::{compiler::Compiler, syntax::LitIdent, CompilerError, Symbol};

use super::NodeCompiler;

impl NodeCompiler<&LitIdent> for Compiler {
    fn compile_node(&mut self, LitIdent(name, span): &LitIdent) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.scope_stack.local(name)?.slot as u8;

        self.assembler.load_local(slot, span.clone());

        Ok(())
    }
}
