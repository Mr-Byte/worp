use crate::{compiler::Compiler, syntax::LitInt, CompilerError, Value};

use super::NodeCompiler;

impl NodeCompiler<LitInt> for Compiler {
    fn compile_node(&mut self, LitInt(value, span): LitInt) -> Result<(), CompilerError> {
        match value {
            0 => self.assembler.push_i0(span),
            1 => self.assembler.push_i1(span),
            _ => self.assembler.push_const(Value::Int(value), span),
        }

        Ok(())
    }
}
