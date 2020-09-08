use super::NodeCompiler;
use crate::{
    compiler::{scope::ScopeKind, Compiler},
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
            let Block(items, trailing_expression, _span) = block.clone();

            if trailing_expression.is_some() {
                return Err(CompilerError::InvalidLoopEnding);
            }

            for expression in items.iter() {
                self.compile_node(*expression)?;
                self.assembler.pop(span.clone());
            }
        } else {
            return Err(CompilerError::InternalCompilerError(String::from(
                "While loop bodies should only ever contain blocks.",
            )));
        }

        self.assembler.jump_back(loop_start, span.clone());
        self.assembler.patch_jump(loop_end);

        let scope_close = self.scope_stack.pop_scope()?;

        for location in scope_close.exit_points.iter() {
            self.assembler.patch_jump(*location as u64);
        }

        self.assembler.push_unit(span);

        Ok(())
    }
}
