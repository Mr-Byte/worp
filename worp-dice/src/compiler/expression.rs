use super::{error::CompilerError, Compiler};
use crate::{
    runtime::core::Value,
    syntax::{Binary, Literal, SyntaxNodeId},
};

impl<'a> Compiler<'a> {
    pub(crate) fn expression(&mut self, node: SyntaxNodeId) -> Result<(), CompilerError> {
        let node = self.syntax_tree.get(node).unwrap();

        match node {
            crate::syntax::SyntaxNode::Literal(literal) => self.literal(literal.clone())?,
            crate::syntax::SyntaxNode::SafeAccess(_) => todo!(),
            crate::syntax::SyntaxNode::FieldAccess(_) => todo!(),
            crate::syntax::SyntaxNode::Index(_) => todo!(),
            crate::syntax::SyntaxNode::Unary(_) => todo!(),
            crate::syntax::SyntaxNode::Binary(binary) => self.binary_op(binary.clone())?,
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
            Literal::None(_) => self.bytecode.push_none(),
            Literal::Integer(value, _) => self.bytecode.push_int(value),
            Literal::Float(value, _) => self.bytecode.push_float(value),
            Literal::String(value, _) => self.bytecode.push_const(Value::new(value)),
            Literal::Boolean(value, _) => self.bytecode.push_bool(value),
            Literal::List(_, _) => todo!(),
            Literal::Object(_, _) => todo!(),
        };

        Ok(())
    }

    fn binary_op(&mut self, Binary(op, lhs, rhs, _): Binary) -> Result<(), CompilerError> {
        self.expression(rhs)?;
        self.expression(lhs)?;

        match op {
            crate::syntax::BinaryOperator::DiceRoll => todo!(),
            crate::syntax::BinaryOperator::Multiply => self.bytecode.mul(),
            crate::syntax::BinaryOperator::Divide => self.bytecode.div(),
            crate::syntax::BinaryOperator::Remainder => self.bytecode.rem(),
            crate::syntax::BinaryOperator::Add => self.bytecode.add(),
            crate::syntax::BinaryOperator::Subtract => self.bytecode.sub(),
            crate::syntax::BinaryOperator::GreaterThan => todo!(),
            crate::syntax::BinaryOperator::LessThan => todo!(),
            crate::syntax::BinaryOperator::GreaterThanEquals => todo!(),
            crate::syntax::BinaryOperator::LessThanEquals => todo!(),
            crate::syntax::BinaryOperator::Equals => todo!(),
            crate::syntax::BinaryOperator::NotEquals => todo!(),
            crate::syntax::BinaryOperator::LogicalAnd => todo!(),
            crate::syntax::BinaryOperator::LogicalOr => todo!(),
            crate::syntax::BinaryOperator::RangeInclusive => todo!(),
            crate::syntax::BinaryOperator::RangeExclusive => todo!(),
            crate::syntax::BinaryOperator::Coalesce => todo!(),
        }

        Ok(())
    }
}
