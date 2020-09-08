use super::NodeVisitor;
use crate::{compiler::Compiler, syntax::VariableDeclaration, CompilerError, Symbol};

impl NodeVisitor<&VariableDeclaration> for Compiler {
    fn visit(
        &mut self,
        VariableDeclaration(name, is_mutable, value, span): &VariableDeclaration,
    ) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.scope_stack.add_local(name, *is_mutable)? as u8;

        self.visit(*value)?;
        self.assembler.store_local(slot, span.clone());

        Ok(())
    }
}
