use super::{error::CompilerError, Compiler};
use crate::{
    runtime::core::Value,
    syntax::{Binary, Literal, SyntaxNodeId},
};

impl<'a> Compiler<'a> {
    pub(crate) fn expression(&mut self, node: SyntaxNodeId) -> Result<(), CompilerError> {
        let node = self.syntax_tree.get(node).unwrap();

        match node {
            crate::syntax::SyntaxNode::Literal(literal) => {
                let literal = literal.clone();
                self.literal(literal)?
            }
            crate::syntax::SyntaxNode::SafeAccess(_) => todo!(),
            crate::syntax::SyntaxNode::FieldAccess(_) => todo!(),
            crate::syntax::SyntaxNode::Index(_) => todo!(),
            crate::syntax::SyntaxNode::Unary(_) => todo!(),
            crate::syntax::SyntaxNode::Binary(binary) => {
                let binary = binary.clone();
                self.binary_op(binary)?
            }
            crate::syntax::SyntaxNode::VariableDeclaration(_) => todo!(),
            crate::syntax::SyntaxNode::Assignment(_) => todo!(),
            crate::syntax::SyntaxNode::Conditional(_) => todo!(),
            crate::syntax::SyntaxNode::WhileLoop(_) => todo!(),
            crate::syntax::SyntaxNode::ForLoop(_) => todo!(),
            crate::syntax::SyntaxNode::Block(_) => todo!(),
        }

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

    fn binary_op(&mut self, Binary(op, lhs, rhs, span): Binary) -> Result<(), CompilerError> {
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
            crate::syntax::BinaryOperator::LogicalAnd => todo!(),
            crate::syntax::BinaryOperator::LogicalOr => todo!(),
            crate::syntax::BinaryOperator::RangeInclusive => todo!(),
            crate::syntax::BinaryOperator::RangeExclusive => todo!(),
            crate::syntax::BinaryOperator::Coalesce => todo!(),
        }

        Ok(())
    }
}
