use super::NodeCompiler;
use crate::{compiler::Compiler, syntax::Discard};

impl NodeCompiler<Discard> for Compiler {
    fn compile_node(&mut self, Discard(span): Discard) -> Result<(), crate::CompilerError> {
        self.assembler.pop(span.clone());

        Ok(())
    }
}
