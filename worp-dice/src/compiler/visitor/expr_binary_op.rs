use super::NodeVisitor;
use crate::{
    compiler::Compiler,
    syntax::{Binary, BinaryOperator},
    CompilerError,
};

impl NodeVisitor<&Binary> for Compiler {
    fn visit(&mut self, Binary(op, lhs, rhs, span): &Binary) -> Result<(), CompilerError> {
        // TODO: Decmpose this into multiple expressions.
        match op {
            BinaryOperator::LogicalAnd => {
                self.visit(*lhs)?;
                self.context()?.assembler().dup(span.clone());

                let short_circuit_jump = self.context()?.assembler().jump_if_false(span.clone());
                self.context()?.assembler().pop(span.clone());
                self.visit(*rhs)?;
                self.compiler_stack
                    .top_mut()?
                    .assembler()
                    .patch_jump(short_circuit_jump);
            }
            BinaryOperator::LogicalOr => {
                self.visit(*lhs)?;
                self.context()?.assembler().dup(span.clone());
                self.context()?.assembler().not(span.clone());

                let short_circuit_jump = self.context()?.assembler().jump_if_false(span.clone());
                self.context()?.assembler().pop(span.clone());
                self.visit(*rhs)?;
                self.compiler_stack
                    .top_mut()?
                    .assembler()
                    .patch_jump(short_circuit_jump);
            }
            _ => {
                self.visit(*rhs)?;
                self.visit(*lhs)?;

                match op {
                    BinaryOperator::DiceRoll => todo!(),
                    BinaryOperator::Multiply => self.context()?.assembler().mul(span.clone()),
                    BinaryOperator::Divide => self.context()?.assembler().div(span.clone()),
                    BinaryOperator::Remainder => self.context()?.assembler().rem(span.clone()),
                    BinaryOperator::Add => self.context()?.assembler().add(span.clone()),
                    BinaryOperator::Subtract => self.context()?.assembler().sub(span.clone()),
                    BinaryOperator::GreaterThan => self.context()?.assembler().gt(span.clone()),
                    BinaryOperator::LessThan => self.context()?.assembler().lt(span.clone()),
                    BinaryOperator::GreaterThanEquals => self.context()?.assembler().gte(span.clone()),
                    BinaryOperator::LessThanEquals => self.context()?.assembler().lte(span.clone()),
                    BinaryOperator::Equals => self.context()?.assembler().eq(span.clone()),
                    BinaryOperator::NotEquals => self.context()?.assembler().neq(span.clone()),
                    BinaryOperator::RangeInclusive => todo!(),
                    BinaryOperator::RangeExclusive => todo!(),
                    BinaryOperator::Coalesce => todo!(),
                    _ => unreachable!(),
                }
            }
        }

        Ok(())
    }
}
