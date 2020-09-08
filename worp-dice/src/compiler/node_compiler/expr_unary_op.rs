use super::NodeCompiler;
use crate::{
    compiler::Compiler,
    syntax::{Unary, UnaryOperator},
    CompilerError,
};

impl NodeCompiler<&Unary> for Compiler {
    fn compile_node(&mut self, Unary(op, expr, span): &Unary) -> Result<(), CompilerError> {
        self.compile_node(*expr)?;

        match op {
            UnaryOperator::Negate => self.assembler.neg(span.clone()),
            UnaryOperator::Not => self.assembler.not(span.clone()),
            UnaryOperator::DiceRoll => todo!(),
        }

        Ok(())
    }
}
