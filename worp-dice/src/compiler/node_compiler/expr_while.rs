use super::NodeCompiler;
use crate::{
    compiler::{component::scope::ScopeKind, Compiler},
    syntax::{Block, SyntaxNode, WhileLoop},
    CompilerError,
};

impl NodeCompiler<WhileLoop> for Compiler {
    fn compile_node(&mut self, WhileLoop(condition, body, span): WhileLoop) -> Result<(), CompilerError> {
        let loop_start = self.assembler.current_position();

        self.scope_stack.push_scope(ScopeKind::Loop, Some(loop_start as usize));

        self.compile_node(condition)?;
        let loop_end = self.assembler.jump_if_false(span.clone());

        if let Some(SyntaxNode::Block(block)) = self.syntax_tree.get(body) {
            let Block(items, _span) = block.clone();

            // NOTE: Loops should always end in a discard.  The stack should be left unaltered after each iteration.
            // This means some detection should be done to enforce that the loop ends in some operation that leaves the stack
            // in its original state.

            if let Some(node) = items.last() {
                if !matches!(self.syntax_tree.get(*node),
                      Some(SyntaxNode::Discard(_))
                    | Some(SyntaxNode::VariableDeclaration(_))
                    | Some(SyntaxNode::Break(_))
                    | Some(SyntaxNode::Continue(_))
                    | None)
                {
                    return Err(CompilerError::InvalidLoopEnding);
                }
            }

            for expression in items.iter() {
                self.compile_node(*expression)?;
            }
        } else {
            return Err(CompilerError::InternalCompilerError(String::from(
                "While loop bodies should only ever contain blocks.",
            )));
        }

        self.assembler.jump_back(loop_start, span.clone());
        self.assembler.pop(span.clone());

        let scope_close = self.scope_stack.pop_scope()?;
        self.assembler.patch_jump(loop_end);

        for location in scope_close.exit_points.iter() {
            self.assembler.patch_jump(*location as u64);
        }

        self.assembler.push_unit(span);

        Ok(())
    }
}
