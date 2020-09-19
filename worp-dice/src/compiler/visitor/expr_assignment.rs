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

                        if !local.is_mutable {
                            return Err(CompilerError::ImmutableVariable(target));
                        }

                        self.visit(*rhs)?;

                        match op {
                            AssignmentOperator::Assignment => {
                                self.context()?.assembler().store_local(slot, span.clone())
                            }
                            AssignmentOperator::MulAssignment => {
                                self.context()?.assembler().mul_assign_local(slot, span.clone())
                            }
                            AssignmentOperator::DivAssignment => {
                                self.context()?.assembler().div_assign_local(slot, span.clone())
                            }
                            AssignmentOperator::AddAssignment => {
                                self.context()?.assembler().add_assign_local(slot, span.clone())
                            }
                            AssignmentOperator::SubAssignment => {
                                self.context()?.assembler().sub_assign_local(slot, span.clone())
                            }
                        }

                        return Ok(());
                    }

                    if let Some(upvalue) = self.resolve_upvalue(target.clone(), 0) {
                        let context = self.context()?;

                        if !context.upvalues()[upvalue].is_mutable() {
                            return Err(CompilerError::ImmutableVariable(target));
                        }

                        context.assembler().store_upvalue(upvalue as u8, span.clone());

                        return Ok(());
                    }

                    return Err(CompilerError::UndeclaredVariable(target));
                }
            }
            _ => return Err(CompilerError::InvalidAssignmentTarget),
        }
    }
}
