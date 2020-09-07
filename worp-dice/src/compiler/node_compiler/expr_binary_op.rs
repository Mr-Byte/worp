use super::NodeCompiler;
use crate::{
    compiler::Compiler,
    syntax::{Binary, BinaryOperator},
    CompilerError,
};

impl NodeCompiler<Binary> for Compiler {
    fn compile_node(&mut self, Binary(op, lhs, rhs, span): Binary) -> Result<(), CompilerError> {
        // TODO: Decmpose this into multiple expressions.
        match op {
            BinaryOperator::LogicalAnd => {
                self.compile_node(lhs)?;
                self.assembler.dup(span.clone());

                let short_circuit_jump = self.assembler.jump_if_false(span.clone());
                self.assembler.pop(span);
                self.compile_node(rhs)?;
                self.assembler.patch_jump(short_circuit_jump);
            }
            BinaryOperator::LogicalOr => {
                self.compile_node(lhs)?;
                self.assembler.dup(span.clone());
                self.assembler.not(span.clone());

                let short_circuit_jump = self.assembler.jump_if_false(span.clone());
                self.assembler.pop(span);
                self.compile_node(rhs)?;
                self.assembler.patch_jump(short_circuit_jump);
            }
            _ => {
                self.compile_node(rhs)?;
                self.compile_node(lhs)?;

                match op {
                    BinaryOperator::DiceRoll => todo!(),
                    BinaryOperator::Multiply => self.assembler.mul(span),
                    BinaryOperator::Divide => self.assembler.div(span),
                    BinaryOperator::Remainder => self.assembler.rem(span),
                    BinaryOperator::Add => self.assembler.add(span),
                    BinaryOperator::Subtract => self.assembler.sub(span),
                    BinaryOperator::GreaterThan => self.assembler.gt(span),
                    BinaryOperator::LessThan => self.assembler.lt(span),
                    BinaryOperator::GreaterThanEquals => self.assembler.gte(span),
                    BinaryOperator::LessThanEquals => self.assembler.lte(span),
                    BinaryOperator::Equals => self.assembler.eq(span),
                    BinaryOperator::NotEquals => self.assembler.neq(span),
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
