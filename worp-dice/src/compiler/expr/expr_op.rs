use crate::{
    compiler::Compiler,
    syntax::{Binary, BinaryOperator, Unary, UnaryOperator},
    CompilerError,
};

impl Compiler {
    pub(super) fn unary_op(&mut self, Unary(op, expr, span): Unary) -> Result<(), CompilerError> {
        self.expression(expr)?;

        match op {
            UnaryOperator::Negate => self.assembler.neg(span),
            UnaryOperator::Not => self.assembler.not(span),
            UnaryOperator::DiceRoll => todo!(),
        }

        Ok(())
    }

    pub(super) fn binary_op(&mut self, Binary(op, lhs, rhs, span): Binary) -> Result<(), CompilerError> {
        // TODO: Decmpose this into multiple expressions.
        match op {
            BinaryOperator::LogicalAnd => {
                self.expression(lhs)?;
                self.assembler.dup(span.clone());

                let short_circuit_jump = self.assembler.jump_if_false(span.clone());
                self.assembler.pop(span);
                self.expression(rhs)?;
                self.assembler.patch_jump(short_circuit_jump);
            }
            BinaryOperator::LogicalOr => {
                self.expression(lhs)?;
                self.assembler.dup(span.clone());
                self.assembler.not(span.clone());

                let short_circuit_jump = self.assembler.jump_if_false(span.clone());
                self.assembler.pop(span);
                self.expression(rhs)?;
                self.assembler.patch_jump(short_circuit_jump);
            }
            _ => {
                self.expression(rhs)?;
                self.expression(lhs)?;

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
