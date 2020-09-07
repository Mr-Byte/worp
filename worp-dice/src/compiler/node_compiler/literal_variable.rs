use crate::{compiler::Compiler, runtime::core::Span, CompilerError, Symbol};

// TODO: Convert this to a NodeCompiler once literals are decomposed into structs-in-variants.
impl Compiler {
    pub fn load_variable(&mut self, name: String, span: Span) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.scope_stack.local(name)?.slot as u8;

        self.assembler.load_local(slot, span);

        Ok(())
    }
}
