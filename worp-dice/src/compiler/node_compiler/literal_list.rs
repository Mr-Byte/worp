use crate::{compiler::Compiler, syntax::LitList, CompilerError};

use super::NodeCompiler;

impl NodeCompiler<&LitList> for Compiler {
    fn compile_node(&mut self, LitList(value, span): &LitList) -> Result<(), CompilerError> {
        for item in value {
            self.compile_node(*item)?;
        }

        self.assembler.build_list(value.len() as u8, span.clone());

        Ok(())
    }
}
