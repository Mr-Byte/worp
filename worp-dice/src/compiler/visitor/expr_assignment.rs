use super::NodeVisitor;
use crate::{
    compiler::Compiler,
    syntax::LitIdent,
    syntax::{Assignment, AssignmentOperator, SyntaxNode},
    CompilerError, Symbol,
};

impl NodeVisitor<&Assignment> for Compiler {
    fn visit(&mut self, Assignment(op, lhs, rhs, span): &Assignment) -> Result<(), CompilerError> {
        let lhs = self.syntax_tree.get(*lhs).expect("Node should exist.");

        match lhs {
            SyntaxNode::LitIdent(LitIdent(target, _)) => {
                let target = Symbol::new(target);
                let local = self.scope_stack.local(target.clone())?;
                let slot = local.slot as u8;

                if !local.is_mutable {
                    return Err(CompilerError::ImmutableVariable(target));
                }

                self.visit(*rhs)?;

                match op {
                    AssignmentOperator::Assignment => self.assembler.store_local(slot, span.clone()),
                    AssignmentOperator::MulAssignment => self.assembler.mul_assign_local(slot, span.clone()),
                    AssignmentOperator::DivAssignment => self.assembler.div_assign_local(slot, span.clone()),
                    AssignmentOperator::AddAssignment => self.assembler.add_assign_local(slot, span.clone()),
                    AssignmentOperator::SubAssignment => self.assembler.sub_assign_local(slot, span.clone()),
                }
            }
            _ => return Err(CompilerError::InvalidAssignmentTarget),
        }

        Ok(())
    }
}
