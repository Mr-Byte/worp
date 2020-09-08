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
                self.assembler.dup(span.clone());

                let short_circuit_jump = self.assembler.jump_if_false(span.clone());
                self.assembler.pop(span.clone());
                self.visit(*rhs)?;
                self.assembler.patch_jump(short_circuit_jump);
            }
            BinaryOperator::LogicalOr => {
                self.visit(*lhs)?;
                self.assembler.dup(span.clone());
                self.assembler.not(span.clone());

                let short_circuit_jump = self.assembler.jump_if_false(span.clone());
                self.assembler.pop(span.clone());
                self.visit(*rhs)?;
                self.assembler.patch_jump(short_circuit_jump);
            }
            _ => {
                self.visit(*rhs)?;
                self.visit(*lhs)?;

                match op {
                    BinaryOperator::DiceRoll => todo!(),
                    BinaryOperator::Multiply => self.assembler.mul(span.clone()),
                    BinaryOperator::Divide => self.assembler.div(span.clone()),
                    BinaryOperator::Remainder => self.assembler.rem(span.clone()),
                    BinaryOperator::Add => self.assembler.add(span.clone()),
                    BinaryOperator::Subtract => self.assembler.sub(span.clone()),
                    BinaryOperator::GreaterThan => self.assembler.gt(span.clone()),
                    BinaryOperator::LessThan => self.assembler.lt(span.clone()),
                    BinaryOperator::GreaterThanEquals => self.assembler.gte(span.clone()),
                    BinaryOperator::LessThanEquals => self.assembler.lte(span.clone()),
                    BinaryOperator::Equals => self.assembler.eq(span.clone()),
                    BinaryOperator::NotEquals => self.assembler.neq(span.clone()),
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
