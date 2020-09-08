use crate::{compiler::Compiler, syntax::LitInt, CompilerError, Value};

use super::NodeCompiler;

impl NodeCompiler<&LitInt> for Compiler {
    fn compile_node(&mut self, LitInt(value, span): &LitInt) -> Result<(), CompilerError> {
        match value {
            0 => self.assembler.push_i0(span.clone()),
            1 => self.assembler.push_i1(span.clone()),
            _ => self.assembler.push_const(Value::Int(*value), span.clone()),
        }

        Ok(())
    }
}
