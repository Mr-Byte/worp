use super::{error::CompilerError, scope::ScopeKind, Compiler};
use crate::{
    runtime::core::{Span, Symbol, Value},
    syntax::{
        Assignment, AssignmentOperator, Binary, BinaryOperator, Block, Conditional, Literal, SyntaxNode, SyntaxNodeId,
        Unary, UnaryOperator, VariableDeclaration, WhileLoop,
    },
};

impl Compiler {
    pub(crate) fn compile(&mut self, node: SyntaxNodeId) -> Result<(), CompilerError> {
        let node = self.syntax_tree.get(node).unwrap();

        match node {
            SyntaxNode::Literal(literal) => {
                let literal = literal.clone();
                self.literal(literal)?;
            }
            SyntaxNode::SafeAccess(_) => todo!(),
            SyntaxNode::FieldAccess(_) => todo!(),
            SyntaxNode::Index(_) => todo!(),
            SyntaxNode::Assignment(assignment) => {
                let assignment = assignment.clone();
                self.assignment(assignment)?;
            }
            SyntaxNode::Unary(unary) => {
                let unary = unary.clone();
                self.unary_op(unary)?;
            }
            SyntaxNode::Binary(binary) => {
                let binary = binary.clone();
                self.binary_op(binary)?;
            }
            SyntaxNode::VariableDeclaration(variable) => {
                let variable = variable.clone();
                self.variable(variable)?;
            }
            SyntaxNode::Conditional(conditional) => {
                let conditional = conditional.clone();
                self.conditional(conditional)?;
            }
            SyntaxNode::WhileLoop(while_loop) => {
                let while_loop = while_loop.clone();
                self.while_loop(while_loop)?;
            }
            SyntaxNode::ForLoop(_) => todo!(),
            SyntaxNode::Break(span) => {
                let span = span.clone();
                self.break_statement(span)?;
            }
            SyntaxNode::Continue(span) => {
                let span = span.clone();
                self.continue_statement(span)?;
            }
            SyntaxNode::Block(block) => {
                let block = block.clone();
                self.block(block, ScopeKind::Block)?;
            }
            SyntaxNode::Discard(span) => self.bytecode.pop(span.clone()),
        }

        Ok(())
    }

    fn block(&mut self, Block(items, span): Block, kind: ScopeKind) -> Result<(), CompilerError> {
        self.begin_scope(kind);

        for expression in items.iter() {
            self.compile(*expression)?;
        }

        // If the block is empty or the last element is a discard of variable, push unit onto the stack.
        match items.last() {
            Some(node) => match self.syntax_tree.get(*node) {
                Some(SyntaxNode::Discard(_)) => self.bytecode.push_unit(span),
                Some(SyntaxNode::VariableDeclaration(_)) => self.bytecode.push_unit(span),
                _ => {}
            },
            None => self.bytecode.push_unit(span),
        }

        self.end_scope()?;

        Ok(())
    }

    fn conditional(
        &mut self,
        Conditional(condition, primary, secondary, span): Conditional,
    ) -> Result<(), CompilerError> {
        self.compile(condition)?;
        let if_jump = self.bytecode.jump_if_false(span.clone());
        self.compile(primary)?;

        let else_jump = self.bytecode.jump(span);

        self.bytecode.patch_jump(if_jump);

        if let Some(secondary) = secondary {
            self.compile(secondary)?;
        }

        self.bytecode.patch_jump(else_jump);

        Ok(())
    }

    fn while_loop(&mut self, WhileLoop(condition, body, span): WhileLoop) -> Result<(), CompilerError> {
        let loop_start = self.bytecode.current_position();
        self.compile(condition)?;
        let loop_end = self.bytecode.jump_if_false(span.clone());

        if let Some(SyntaxNode::Block(block)) = self.syntax_tree.get(body) {
            let block = block.clone();
            self.block(block, ScopeKind::Loop)?;
        } else {
            return Err(CompilerError::InternalCompilerError(String::from(
                "While loop bodies should only ever contain blocks.",
            )));
        }

        self.bytecode.pop(span.clone());
        self.bytecode.jump_back(loop_start, span.clone());
        self.bytecode.patch_jump(loop_end);

        self.bytecode.push_unit(span);

        Ok(())
    }

    fn break_statement(&mut self, span: Span) -> Result<(), CompilerError> {
        if !self.scope_stack.in_context_of(ScopeKind::Loop) {
            return Err(CompilerError::InvalidBreak);
        }

        Ok(())
    }

    fn continue_statement(&mut self, span: Span) -> Result<(), CompilerError> {
        if !self.scope_stack.in_context_of(ScopeKind::Loop) {
            return Err(CompilerError::InvalidContinue);
        }

        Ok(())
    }

    fn literal(&mut self, node: Literal) -> Result<(), CompilerError> {
        match node {
            Literal::Identifier(name, span) => self.load_variable(name, span)?,
            Literal::None(span) => self.bytecode.push_none(span),
            Literal::Unit(span) => self.bytecode.push_unit(span),
            Literal::Integer(value, span) => match value {
                0 => self.bytecode.push_i0(span),
                1 => self.bytecode.push_i1(span),
                _ => self.bytecode.push_const(Value::Int(value), span),
            },
            Literal::Float(value, span) => {
                if value == 0.0 {
                    self.bytecode.push_f0(span);
                } else if value == 1.0 {
                    self.bytecode.push_f1(span);
                } else {
                    self.bytecode.push_const(Value::Float(value), span);
                }
            }
            Literal::String(value, span) => self.bytecode.push_const(Value::String(value), span),
            Literal::Boolean(value, span) => self.bytecode.push_bool(value, span),
            Literal::List(list, span) => {
                for item in &list {
                    self.compile(*item)?;
                }

                self.bytecode.build_list(list.len() as u8, span);
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
        let slot = self.add_local(name, is_mutable)?;

        self.compile(value)?;
        self.bytecode.store_local(slot, span.clone());
        self.bytecode.pop(span);

        Ok(())
    }

    fn load_variable(&mut self, name: String, span: Span) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.local(name)?.slot;

        self.bytecode.load_local(slot, span);

        Ok(())
    }

    fn assignment(&mut self, Assignment(op, lhs, rhs, span): Assignment) -> Result<(), CompilerError> {
        let lhs = self.syntax_tree.get(lhs).expect("Node should exist.");

        match lhs {
            SyntaxNode::Literal(Literal::Identifier(target, _)) => {
                let target = Symbol::new(target);
                let local = self.local(target.clone())?;
                let slot = local.slot;

                if !local.is_mutable {
                    return Err(CompilerError::ImmutableVariable(target));
                }

                self.compile(rhs)?;

                match op {
                    AssignmentOperator::Assignment => self.bytecode.store_local(slot, span),
                    AssignmentOperator::MulAssignment => self.bytecode.mul_assign_local(slot, span),
                    AssignmentOperator::DivAssignment => self.bytecode.div_assign_local(slot, span),
                    AssignmentOperator::AddAssignment => self.bytecode.add_assign_local(slot, span),
                    AssignmentOperator::SubAssignment => self.bytecode.sub_assign_local(slot, span),
                }
            }
            _ => return Err(CompilerError::InvalidAssignmentTarget),
        }

        Ok(())
    }

    fn unary_op(&mut self, Unary(op, expr, span): Unary) -> Result<(), CompilerError> {
        self.compile(expr)?;

        match op {
            UnaryOperator::Negate => self.bytecode.neg(span),
            UnaryOperator::Not => self.bytecode.not(span),
            UnaryOperator::DiceRoll => todo!(),
        }

        Ok(())
    }

    fn binary_op(&mut self, Binary(op, lhs, rhs, span): Binary) -> Result<(), CompilerError> {
        match op {
            BinaryOperator::LogicalAnd => {
                self.compile(lhs)?;
                self.bytecode.dup(span.clone());
                let short_circuit_jump = self.bytecode.jump_if_false(span.clone());
                self.bytecode.pop(span);
                self.compile(rhs)?;
                self.bytecode.patch_jump(short_circuit_jump);
            }
            BinaryOperator::LogicalOr => {
                self.compile(lhs)?;
                self.bytecode.dup(span.clone());
                self.bytecode.not(span.clone());
                let short_circuit_jump = self.bytecode.jump_if_false(span.clone());
                self.bytecode.pop(span);
                self.compile(rhs)?;
                self.bytecode.patch_jump(short_circuit_jump);
            }
            _ => {
                self.compile(rhs)?;
                self.compile(lhs)?;

                match op {
                    BinaryOperator::DiceRoll => todo!(),
                    BinaryOperator::Multiply => self.bytecode.mul(span),
                    BinaryOperator::Divide => self.bytecode.div(span),
                    BinaryOperator::Remainder => self.bytecode.rem(span),
                    BinaryOperator::Add => self.bytecode.add(span),
                    BinaryOperator::Subtract => self.bytecode.sub(span),
                    BinaryOperator::GreaterThan => self.bytecode.gt(span),
                    BinaryOperator::LessThan => self.bytecode.lt(span),
                    BinaryOperator::GreaterThanEquals => self.bytecode.gte(span),
                    BinaryOperator::LessThanEquals => self.bytecode.lte(span),
                    BinaryOperator::Equals => self.bytecode.eq(span),
                    BinaryOperator::NotEquals => self.bytecode.neq(span),
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
