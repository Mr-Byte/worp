use super::{error::CompilerError, Compiler};
use crate::{
    runtime::core::{Span, Symbol, Value},
    syntax::{
        Binary, BinaryOperator, Block, Conditional, Literal, SyntaxNode, SyntaxNodeId, Unary, UnaryOperator,
        VariableDeclaration, WhileLoop,
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
            SyntaxNode::Block(Block(items, span)) => {
                let span = span.clone();
                let items = items.clone();

                self.begin_scope();

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

                self.end_scope();
            }
            SyntaxNode::Discard(span) => self.bytecode.pop(span.clone()),
        }

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

        // TODO: Have a special compile for the body that allows for breaks?
        // TODO: Have this take a boolean to indicate it's in a loop body?
        // Return a list of jump locations to patch for loop breaks.
        // Handle breaks nested inside blocks that aren't other loops.
        self.compile(body)?;
        // Cleanup the stack at the end of each iteration of the loop.
        // TODO: Figure out how to optimize out the DUP/POP needed for variable assignments in the final position?
        self.bytecode.pop(span.clone());
        self.bytecode.jump_back(loop_start, span.clone());
        self.bytecode.patch_jump(loop_end);
        // Push a unit onto the stack after a loop finishes executing.
        // TODO: Maybe only do this if the next node is a discard.
        // If the next node is a discard, consume it and don't push to the stack?
        // ^ This could help improve performance.
        self.bytecode.push_unit(span);

        Ok(())
    }

    fn literal(&mut self, node: Literal) -> Result<(), CompilerError> {
        match node {
            Literal::Identifier(name, span) => self.load_variable(name, span)?,
            Literal::None(span) => self.bytecode.push_none(span),
            Literal::Unit(span) => self.bytecode.push_unit(span),
            Literal::Integer(value, span) => self.bytecode.push_int(value, span),
            Literal::Float(value, span) => self.bytecode.push_float(value, span),
            Literal::String(value, span) => self.bytecode.push_const(Value::String(value), span),
            Literal::Boolean(value, span) => self.bytecode.push_bool(value, span),
            Literal::List(_, _) => todo!(),
            Literal::Object(_, _) => todo!(),
        };

        Ok(())
    }

    fn variable(
        &mut self,
        VariableDeclaration(name, is_mutable, value, span): VariableDeclaration,
    ) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.add_local(name.clone(), is_mutable);

        self.compile(value)?;
        self.bytecode.store_local(slot, span);

        Ok(())
    }

    fn load_variable(&mut self, name: String, span: Span) -> Result<(), CompilerError> {
        let name = Symbol::new(name);
        let slot = self.local(name)?.slot;

        self.bytecode.load_local(slot, span);

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
            // TODO: Separate assignments out into their own class of operators.
            BinaryOperator::AddAssignment => {
                let lhs = self.syntax_tree.get(lhs).expect("Node should exist.");

                if let SyntaxNode::Literal(Literal::Identifier(target, _)) = lhs {
                    let target = Symbol::new(target);
                    let local = self.local(target.clone())?;
                    let slot = local.slot;

                    if !local.is_mutable {
                        return Err(CompilerError::ImmutableVariable(target));
                    }

                    self.compile(rhs)?;
                    // Since assignments are always followed by a discard, make this a no-op?
                    self.bytecode.add_assign_local(slot, span);
                } else {
                    return Err(CompilerError::InvalidAssignmentTarget);
                }
            }
            BinaryOperator::Assignment => {
                let lhs = self.syntax_tree.get(lhs).expect("Node should exist.");

                if let SyntaxNode::Literal(Literal::Identifier(target, _)) = lhs {
                    let target = Symbol::new(target);
                    let local = self.local(target.clone())?;
                    let slot = local.slot;

                    if !local.is_mutable {
                        return Err(CompilerError::ImmutableVariable(target));
                    }

                    self.compile(rhs)?;
                    self.bytecode.store_local(slot, span);
                } else {
                    return Err(CompilerError::InvalidAssignmentTarget);
                }
            }
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
