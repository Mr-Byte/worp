use super::NodeVisitor;
use crate::{
    compiler::Compiler,
    syntax::{Assignment, AssignmentOperator, LitIdent, SyntaxNode},
    CompilerError, Symbol,
};

impl NodeVisitor<&Assignment> for Compiler {
    fn visit(&mut self, Assignment(op, lhs, rhs, span): &Assignment) -> Result<(), CompilerError> {
        let lhs = self.syntax_tree.get(*lhs).expect("Node should exist.");

        match lhs {
            SyntaxNode::LitIdent(LitIdent(target, _)) => {
                let target = Symbol::new(target);
                // TODO: This should also try to resolve upvalues.
                let local = self.scope_stack.local(target.clone()).expect("Fix this");
                let slot = local.slot as u8;

                if !local.is_mutable {
                    return Err(CompilerError::ImmutableVariable(target));
                }

                self.visit(*rhs)?;

                match op {
                    AssignmentOperator::Assignment => self.current_assembler().store_local(slot, span.clone()),
                    AssignmentOperator::MulAssignment => self.current_assembler().mul_assign_local(slot, span.clone()),
                    AssignmentOperator::DivAssignment => self.current_assembler().div_assign_local(slot, span.clone()),
                    AssignmentOperator::AddAssignment => self.current_assembler().add_assign_local(slot, span.clone()),
                    AssignmentOperator::SubAssignment => self.current_assembler().sub_assign_local(slot, span.clone()),
                }
            }
            _ => return Err(CompilerError::InvalidAssignmentTarget),
        }

        Ok(())
    }
}
