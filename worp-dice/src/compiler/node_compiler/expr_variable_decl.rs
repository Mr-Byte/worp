use super::NodeCompiler;
use crate::{compiler::Compiler, syntax::VariableDeclaration, CompilerError, Symbol};

impl NodeCompiler<VariableDeclaration> for Compiler {
    fn compile_node(
        &mut self,
        VariableDeclaration(name, is_mutable, value, span): VariableDeclaration,
    ) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.scope_stack.add_local(name, is_mutable)? as u8;

        self.compile_node(value)?;
        self.assembler.store_local(slot, span.clone());
        self.assembler.pop(span);

        Ok(())
    }
}
