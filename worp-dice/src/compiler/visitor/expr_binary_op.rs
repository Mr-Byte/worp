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
                self.current_assembler().dup(span.clone());

                let short_circuit_jump = self.current_assembler().jump_if_false(span.clone());
                self.current_assembler().pop(span.clone());
                self.visit(*rhs)?;
                self.current_assembler().patch_jump(short_circuit_jump);
            }
            BinaryOperator::LogicalOr => {
                self.visit(*lhs)?;
                self.current_assembler().dup(span.clone());
                self.current_assembler().not(span.clone());

                let short_circuit_jump = self.current_assembler().jump_if_false(span.clone());
                self.current_assembler().pop(span.clone());
                self.visit(*rhs)?;
                self.current_assembler().patch_jump(short_circuit_jump);
            }
            _ => {
                self.visit(*rhs)?;
                self.visit(*lhs)?;

                match op {
                    BinaryOperator::DiceRoll => todo!(),
                    BinaryOperator::Multiply => self.current_assembler().mul(span.clone()),
                    BinaryOperator::Divide => self.current_assembler().div(span.clone()),
                    BinaryOperator::Remainder => self.current_assembler().rem(span.clone()),
                    BinaryOperator::Add => self.current_assembler().add(span.clone()),
                    BinaryOperator::Subtract => self.current_assembler().sub(span.clone()),
                    BinaryOperator::GreaterThan => self.current_assembler().gt(span.clone()),
                    BinaryOperator::LessThan => self.current_assembler().lt(span.clone()),
                    BinaryOperator::GreaterThanEquals => self.current_assembler().gte(span.clone()),
                    BinaryOperator::LessThanEquals => self.current_assembler().lte(span.clone()),
                    BinaryOperator::Equals => self.current_assembler().eq(span.clone()),
                    BinaryOperator::NotEquals => self.current_assembler().neq(span.clone()),
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
