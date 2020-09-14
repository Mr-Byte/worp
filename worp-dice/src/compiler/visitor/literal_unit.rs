use crate::{compiler::Compiler, syntax::LitUnit, CompilerError};

use super::NodeVisitor;

impl NodeVisitor<&LitUnit> for Compiler {
    fn visit(&mut self, LitUnit(span): &LitUnit) -> Result<(), CompilerError> {
        self.current_assembler().push_unit(span.clone());

        Ok(())
    }
}
