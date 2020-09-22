use super::NodeVisitor;
use crate::{
    compiler::Compiler,
    syntax::{Unary, UnaryOperator},
    CompilerError,
};

impl NodeVisitor<&Unary> for Compiler {
    fn visit(
        &mut self,
        Unary {
            operator: op,
            expression: expr,
            span,
        }: &Unary,
    ) -> Result<(), CompilerError> {
        self.visit(*expr)?;

        match op {
            UnaryOperator::Negate => self.context()?.assembler().neg(*span),
            UnaryOperator::Not => self.context()?.assembler().not(*span),
            UnaryOperator::DiceRoll => todo!(),
        }

        Ok(())
    }
}
