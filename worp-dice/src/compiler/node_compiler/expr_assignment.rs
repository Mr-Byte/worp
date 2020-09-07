use super::NodeCompiler;
use crate::{
    compiler::Compiler,
    syntax::{Assignment, AssignmentOperator, Literal, SyntaxNode},
    CompilerError, Symbol,
};

impl NodeCompiler<Assignment> for Compiler {
    fn compile_node(&mut self, Assignment(op, lhs, rhs, span): Assignment) -> Result<(), CompilerError> {
        let lhs = self.syntax_tree.get(lhs).expect("Node should exist.");

        match lhs {
            SyntaxNode::Literal(Literal::Identifier(target, _)) => {
                let target = Symbol::new(target);
                let local = self.scope_stack.local(target.clone())?;
                let slot = local.slot as u8;

                if !local.is_mutable {
                    return Err(CompilerError::ImmutableVariable(target));
                }

                self.compile_node(rhs)?;

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
