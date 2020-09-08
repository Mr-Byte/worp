use crate::{compiler::Compiler, syntax::LitString, CompilerError, Value};

use super::NodeCompiler;

impl NodeCompiler<&LitString> for Compiler {
    fn compile_node(&mut self, LitString(value, span): &LitString) -> Result<(), CompilerError> {
        self.assembler.push_const(Value::String(value.clone()), span.clone());

        Ok(())
    }
}
