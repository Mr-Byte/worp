use crate::{compiler::Compiler, syntax::LitFloat, CompilerError, Value};

use super::NodeVisitor;

impl NodeVisitor<&LitFloat> for Compiler {
    fn visit(&mut self, LitFloat(value, span): &LitFloat) -> Result<(), CompilerError> {
        let context = self.context()?;

        if *value == 0.0 {
            context.assembler().push_f0(span.clone());
        } else if *value == 1.0 {
            context.assembler().push_f1(span.clone());
        } else {
            context.assembler().push_const(Value::Float(*value), span.clone())?;
        }

        Ok(())
    }
}
