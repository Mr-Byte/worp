use super::NodeVisitor;
use crate::{
    compiler::{scope::ScopeKind, Compiler},
    syntax::{SyntaxNode, WhileLoop},
    CompilerError,
};

impl NodeVisitor<&WhileLoop> for Compiler {
    fn visit(&mut self, WhileLoop(condition, body, span): &WhileLoop) -> Result<(), CompilerError> {
        if let Some(SyntaxNode::Block(block)) = self.syntax_tree.get(*body) {
            let block = block.clone();
            let loop_start = self.assembler.current_position();

            self.scope_stack.push_scope(ScopeKind::Loop, Some(loop_start as usize));
            // TODO: Scan for any function or class declarations and create local slots, before visiting all children.

            self.visit(*condition)?;
            let loop_end = self.assembler.jump_if_false(span.clone());

            for expression in block.expressions.iter() {
                self.visit(*expression)?;
                self.assembler.pop(span.clone());
            }

            // NOTE: While loops allow a trailing expression, but the value is discarded at the end of each iteration.
            if let Some(trailing_expression) = block.trailing_expression {
                self.visit(trailing_expression)?;
                self.assembler.pop(span.clone());
            }

            self.assembler.jump_back(loop_start, span.clone());
            self.assembler.patch_jump(loop_end);

            let scope_close = self.scope_stack.pop_scope()?;

            for location in scope_close.exit_points.iter() {
                self.assembler.patch_jump(*location as u64);
            }

            self.assembler.push_unit(span.clone());
        } else {
            return Err(CompilerError::InternalCompilerError(String::from(
                "While loop bodies should only ever contain blocks.",
            )));
        }

        Ok(())
    }
}
