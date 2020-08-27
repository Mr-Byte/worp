use super::{error::CompilerError, Compiler};
use crate::{
    runtime::core::Value,
    syntax::{Binary, Block, Conditional, Literal, SyntaxNodeId, Unary},
};

impl Compiler {
    pub(crate) fn expression(&mut self, node: SyntaxNodeId) -> Result<(), CompilerError> {
        let node = self.syntax_tree.get(node).unwrap();

        match node {
            crate::syntax::SyntaxNode::Literal(literal) => {
                let literal = literal.clone();
                self.literal(literal)?;
            }
            crate::syntax::SyntaxNode::SafeAccess(_) => todo!(),
            crate::syntax::SyntaxNode::FieldAccess(_) => todo!(),
            crate::syntax::SyntaxNode::Index(_) => todo!(),
            crate::syntax::SyntaxNode::Unary(unary) => {
                let unary = unary.clone();
                self.unary_op(unary)?;
            }
            crate::syntax::SyntaxNode::Binary(binary) => {
                let binary = binary.clone();
                self.binary_op(binary)?;
            }
            crate::syntax::SyntaxNode::VariableDeclaration(_) => todo!(),
            crate::syntax::SyntaxNode::Assignment(_) => todo!(),
            crate::syntax::SyntaxNode::Conditional(conditional) => {
                let conditional = conditional.clone();
                self.conditional(conditional)?;
            }
            crate::syntax::SyntaxNode::WhileLoop(_) => todo!(),
            crate::syntax::SyntaxNode::ForLoop(_) => todo!(),
            crate::syntax::SyntaxNode::Block(Block(items, _)) => {
                let items = items.clone();
                for expression in items.iter() {
                    self.expression(*expression)?;
                }
            }
            crate::syntax::SyntaxNode::Discard(span) => self.bytecode.pop(span.clone()),
        }

        Ok(())
    }

    fn conditional(
        &mut self,
        Conditional(condition, primary, secondary, span): Conditional,
    ) -> Result<(), CompilerError> {
        self.expression(condition)?;
        let if_jump = self.bytecode.jump_if_false(span.clone());
        self.expression(primary)?;

        let else_jump = self.bytecode.jump(span);
        // -2 accounts for the jump offset itself.

        self.bytecode.patch_jump_with_current_pos(if_jump);

        if let Some(secondary) = secondary {
            self.expression(secondary)?;
        }

        self.bytecode.patch_jump_with_current_pos(else_jump);

        Ok(())
    }

    fn literal(&mut self, node: Literal) -> Result<(), CompilerError> {
        match node {
            Literal::Identifier(_, _) => todo!(),
            Literal::None(span) => self.bytecode.push_none(span),
            Literal::Integer(value, span) => self.bytecode.push_int(value, span),
            Literal::Float(value, span) => self.bytecode.push_float(value, span),
            Literal::String(value, span) => self.bytecode.push_const(Value::new(value), span),
            Literal::Boolean(value, span) => self.bytecode.push_bool(value, span),
            Literal::List(_, _) => todo!(),
            Literal::Object(_, _) => todo!(),
        };

        Ok(())
    }

    fn unary_op(&mut self, Unary(op, expr, span): Unary) -> Result<(), CompilerError> {
        self.expression(expr)?;

        match op {
            crate::syntax::UnaryOperator::Negate => self.bytecode.neg(span),
            crate::syntax::UnaryOperator::Not => self.bytecode.not(span),
            crate::syntax::UnaryOperator::DiceRoll => todo!(),
        }

        Ok(())
    }

    fn binary_op(&mut self, Binary(op, lhs, rhs, span): Binary) -> Result<(), CompilerError> {
        match op {
            crate::syntax::BinaryOperator::LogicalAnd => {
                self.expression(lhs)?;
                self.bytecode.dup(span.clone());
                let short_circuit_jump = self.bytecode.jump_if_false(span.clone());
                self.bytecode.pop(span);
                self.expression(rhs)?;
                self.bytecode.patch_jump_with_current_pos(short_circuit_jump);
            }
            crate::syntax::BinaryOperator::LogicalOr => {
                self.expression(lhs)?;
                self.bytecode.dup(span.clone());
                self.bytecode.not(span.clone());
                let short_circuit_jump = self.bytecode.jump_if_false(span.clone());
                self.bytecode.pop(span);
                self.expression(rhs)?;
                self.bytecode.patch_jump_with_current_pos(short_circuit_jump);
            }
            _ => {
                self.expression(rhs)?;
                self.expression(lhs)?;

                match op {
                    crate::syntax::BinaryOperator::DiceRoll => todo!(),
                    crate::syntax::BinaryOperator::Multiply => self.bytecode.mul(span),
                    crate::syntax::BinaryOperator::Divide => self.bytecode.div(span),
                    crate::syntax::BinaryOperator::Remainder => self.bytecode.rem(span),
                    crate::syntax::BinaryOperator::Add => self.bytecode.add(span),
                    crate::syntax::BinaryOperator::Subtract => self.bytecode.sub(span),
                    crate::syntax::BinaryOperator::GreaterThan => self.bytecode.gt(span),
                    crate::syntax::BinaryOperator::LessThan => self.bytecode.lt(span),
                    crate::syntax::BinaryOperator::GreaterThanEquals => self.bytecode.gte(span),
                    crate::syntax::BinaryOperator::LessThanEquals => self.bytecode.lte(span),
                    crate::syntax::BinaryOperator::Equals => self.bytecode.eq(span),
                    crate::syntax::BinaryOperator::NotEquals => self.bytecode.neq(span),
                    crate::syntax::BinaryOperator::RangeInclusive => todo!(),
                    crate::syntax::BinaryOperator::RangeExclusive => todo!(),
                    crate::syntax::BinaryOperator::Coalesce => todo!(),
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }
}
