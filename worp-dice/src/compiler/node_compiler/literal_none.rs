use crate::{compiler::Compiler, syntax::LitNone, CompilerError};

use super::NodeCompiler;

impl NodeCompiler<&LitNone> for Compiler {
    fn compile_node(&mut self, LitNone(span): &LitNone) -> Result<(), CompilerError> {
        self.assembler.push_none(span.clone());

        Ok(())
    }
}
