use crate::{compiler::Compiler, syntax::LitFloat, CompilerError, Value};

use super::NodeCompiler;

impl NodeCompiler<LitFloat> for Compiler {
    fn compile_node(&mut self, LitFloat(value, span): LitFloat) -> Result<(), CompilerError> {
        if value == 0.0 {
            self.assembler.push_f0(span);
        } else if value == 1.0 {
            self.assembler.push_f1(span);
        } else {
            self.assembler.push_const(Value::Float(value), span);
        }

        Ok(())
    }
}
