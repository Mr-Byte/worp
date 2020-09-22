use super::NodeVisitor;
use crate::{
    compiler::Compiler,
    syntax::{Assignment, AssignmentOperator, LitIdent, SyntaxNode},
    CompilerError, Symbol,
};

impl NodeVisitor<&Assignment> for Compiler {
    fn visit(&mut self, Assignment(op, lhs, rhs, span): &Assignment) -> Result<(), CompilerError> {
        let lhs = self.syntax_tree.get(*lhs).expect("Node should exist.");

        // TODO: Decompose this down into smaller functions.
        match lhs {
            SyntaxNode::LitIdent(LitIdent(target, _)) => {
                let target = Symbol::new(target);
                {
                    if let Some(local) = self.context()?.scope_stack().local(target.clone()) {
                        let slot = local.slot as u8;

                        if !local.is_mutable() {
                            return Err(CompilerError::ImmutableVariable(target));
                        }

                        self.visit(*rhs)?;

                        // TODO: optimize in-place assignment operators to mutate their target, without needing to be loaded onto the stack.
                        match op {
                            AssignmentOperator::Assignment => {
                                self.context()?.assembler().store_local(slot, *span);
                            }
                            AssignmentOperator::MulAssignment => {
                                self.context()?.assembler().load_local(slot, *span);
                                self.context()?.assembler().mul(*span);
                                self.context()?.assembler().store_local(slot, *span);
                            }
                            AssignmentOperator::DivAssignment => {
                                self.context()?.assembler().load_local(slot, *span);
                                self.context()?.assembler().div(*span);
                                self.context()?.assembler().store_local(slot, *span);
                            }
                            AssignmentOperator::AddAssignment => {
                                self.context()?.assembler().load_local(slot, *span);
                                self.context()?.assembler().add(*span);
                                self.context()?.assembler().store_local(slot, *span);
                            }
                            AssignmentOperator::SubAssignment => {
                                self.context()?.assembler().load_local(slot, *span);
                                self.context()?.assembler().sub(*span);
                                self.context()?.assembler().store_local(slot, *span);
                            }
                        }

                        return Ok(());
                    }

                    if let Some(upvalue) = self.compiler_stack.resolve_upvalue(target.clone(), 0) {
                        if !self.context()?.upvalues()[upvalue].is_mutable() {
                            return Err(CompilerError::ImmutableVariable(target));
                        }

                        self.visit(*rhs)?;

                        match op {
                            AssignmentOperator::Assignment => {
                                self.context()?.assembler().store_upvalue(upvalue as u8, *span);
                            }
                            AssignmentOperator::MulAssignment => {
                                self.context()?.assembler().load_upvalue(upvalue as u8, *span);
                                self.context()?.assembler().mul(*span);
                                self.context()?.assembler().store_upvalue(upvalue as u8, *span);
                            }
                            AssignmentOperator::DivAssignment => {
                                self.context()?.assembler().load_upvalue(upvalue as u8, *span);
                                self.context()?.assembler().div(*span);
                                self.context()?.assembler().store_upvalue(upvalue as u8, *span);
                            }
                            AssignmentOperator::AddAssignment => {
                                self.context()?.assembler().load_upvalue(upvalue as u8, *span);
                                self.context()?.assembler().add(*span);
                                self.context()?.assembler().store_upvalue(upvalue as u8, *span);
                            }
                            AssignmentOperator::SubAssignment => {
                                self.context()?.assembler().load_upvalue(upvalue as u8, *span);
                                self.context()?.assembler().sub(*span);
                                self.context()?.assembler().store_upvalue(upvalue as u8, *span);
                            }
                        }

                        return Ok(());
                    }

                    Err(CompilerError::UndeclaredVariable(target))
                }
            }
            _ => Err(CompilerError::InvalidAssignmentTarget),
        }
    }
}
