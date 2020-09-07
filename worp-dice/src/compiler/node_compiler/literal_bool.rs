use crate::{compiler::Compiler, syntax::LitBool, CompilerError};

use super::NodeCompiler;

impl NodeCompiler<LitBool> for Compiler {
    fn compile_node(&mut self, LitBool(value, span): LitBool) -> Result<(), CompilerError> {
        self.assembler.push_bool(value, span);

        Ok(())
    }
}
