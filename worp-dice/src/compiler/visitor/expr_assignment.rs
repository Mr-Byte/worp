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
                let local = self.context()?.scope_stack().local(target.clone()).expect("Fix this");
                let slot = local.slot as u8;

                if !local.is_mutable {
                    return Err(CompilerError::ImmutableVariable(target));
                }

                self.visit(*rhs)?;

                match op {
                    AssignmentOperator::Assignment => self.context()?.assembler().store_local(slot, span.clone()),
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
            }
            _ => return Err(CompilerError::InvalidAssignmentTarget),
        }

        Ok(())
    }
}
