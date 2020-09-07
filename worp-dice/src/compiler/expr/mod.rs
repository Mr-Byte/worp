use super::{component::scope::ScopeKind, Compiler};
use crate::{
    runtime::core::{Span, Symbol, Value},
    syntax::{Assignment, AssignmentOperator, Block, Literal, SyntaxNode, SyntaxNodeId, VariableDeclaration},
    CompilerError,
};

mod expr_if;
mod expr_loop;
mod expr_op;

impl Compiler {
    pub fn expression(&mut self, node: SyntaxNodeId) -> Result<(), CompilerError> {
        let node = self
            .syntax_tree
            .get(node)
            .cloned()
            .expect("Node should never be empty.");

        match node {
            SyntaxNode::Literal(literal) => self.literal(literal)?,
            SyntaxNode::SafeAccess(_) => todo!(),
            SyntaxNode::FieldAccess(_) => todo!(),
            SyntaxNode::Index(_) => todo!(),
            SyntaxNode::Assignment(assignment) => self.assignment(assignment)?,
            SyntaxNode::Unary(unary) => self.unary_op(unary)?,
            SyntaxNode::Binary(binary) => self.binary_op(binary)?,
            SyntaxNode::VariableDeclaration(variable) => self.variable(variable)?,
            SyntaxNode::Conditional(conditional) => self.conditional(conditional)?,
            SyntaxNode::WhileLoop(while_loop) => self.while_loop(while_loop)?,
            SyntaxNode::ForLoop(_) => todo!(),
            SyntaxNode::Break(span) => self.break_statement(span)?,
            SyntaxNode::Continue(span) => self.continue_statement(span)?,
            SyntaxNode::Block(block) => self.block(block, ScopeKind::Block)?,
            SyntaxNode::Discard(span) => self.assembler.pop(span.clone()),
        }

        Ok(())
    }

    fn block(&mut self, Block(items, span): Block, kind: ScopeKind) -> Result<(), CompilerError> {
        self.scope_stack.push_scope(kind, None);

        for expression in items.iter() {
            self.expression(*expression)?;
        }

        // If the block is empty or the last element is a discard of variable, push unit onto the stack.
        match items.last() {
            Some(node) => match self.syntax_tree.get(*node) {
                Some(SyntaxNode::Discard(_)) => self.assembler.push_unit(span),
                Some(SyntaxNode::VariableDeclaration(_)) => self.assembler.push_unit(span),
                _ => {}
            },
            None => self.assembler.push_unit(span),
        }

        self.scope_stack.pop_scope()?;

        Ok(())
    }

    fn literal(&mut self, node: Literal) -> Result<(), CompilerError> {
        match node {
            Literal::Identifier(name, span) => self.load_variable(name, span)?,
            Literal::None(span) => self.assembler.push_none(span),
            Literal::Unit(span) => self.assembler.push_unit(span),
            Literal::Integer(value, span) => match value {
                0 => self.assembler.push_i0(span),
                1 => self.assembler.push_i1(span),
                _ => self.assembler.push_const(Value::Int(value), span),
            },
            Literal::Float(value, span) => {
                if value == 0.0 {
                    self.assembler.push_f0(span);
                } else if value == 1.0 {
                    self.assembler.push_f1(span);
                } else {
                    self.assembler.push_const(Value::Float(value), span);
                }
            }
            Literal::String(value, span) => self.assembler.push_const(Value::String(value), span),
            Literal::Boolean(value, span) => self.assembler.push_bool(value, span),
            Literal::List(list, span) => {
                for item in &list {
                    self.expression(*item)?;
                }

                self.assembler.build_list(list.len() as u8, span);
            }
            Literal::Object(_, _) => todo!(),
        };

        Ok(())
    }

    fn variable(
        &mut self,
        VariableDeclaration(name, is_mutable, value, span): VariableDeclaration,
    ) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.scope_stack.add_local(name, is_mutable)? as u8;

        self.expression(value)?;
        self.assembler.store_local(slot, span.clone());
        self.assembler.pop(span);

        Ok(())
    }

    fn load_variable(&mut self, name: String, span: Span) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.scope_stack.local(name)?.slot as u8;

        self.assembler.load_local(slot, span);

        Ok(())
    }

    fn assignment(&mut self, Assignment(op, lhs, rhs, span): Assignment) -> Result<(), CompilerError> {
        let lhs = self.syntax_tree.get(lhs).expect("Node should exist.");

        match lhs {
            SyntaxNode::Literal(Literal::Identifier(target, _)) => {
                let target = Symbol::new(target);
                let local = self.scope_stack.local(target.clone())?;
                let slot = local.slot as u8;

                if !local.is_mutable {
                    return Err(CompilerError::ImmutableVariable(target));
                }

                self.expression(rhs)?;

                match op {
                    AssignmentOperator::Assignment => self.assembler.store_local(slot, span),
                    AssignmentOperator::MulAssignment => self.assembler.mul_assign_local(slot, span),
                    AssignmentOperator::DivAssignment => self.assembler.div_assign_local(slot, span),
                    AssignmentOperator::AddAssignment => self.assembler.add_assign_local(slot, span),
                    AssignmentOperator::SubAssignment => self.assembler.sub_assign_local(slot, span),
                }
            }
            _ => return Err(CompilerError::InvalidAssignmentTarget),
        }

        Ok(())
    }
}
