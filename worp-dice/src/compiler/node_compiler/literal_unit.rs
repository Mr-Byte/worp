use crate::{compiler::Compiler, syntax::LitUnit, CompilerError};

use super::NodeCompiler;

impl NodeCompiler<&LitUnit> for Compiler {
    fn compile_node(&mut self, LitUnit(span): &LitUnit) -> Result<(), CompilerError> {
        self.assembler.push_unit(span.clone());

        Ok(())
    }
}
