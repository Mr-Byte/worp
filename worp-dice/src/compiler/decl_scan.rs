use crate::{
    syntax::{Block, FnDecl, SyntaxNode},
    CompilerError, Symbol,
};

use super::{scope::State, Compiler};

impl Compiler {
    /// Scan through all the declared items in a block and add slots for any functions and classes ahead of time.
    pub(super) fn scan_item_decls(&mut self, block: &Block) -> Result<(), CompilerError> {
        for expression in &block.expressions {
            if let Some(SyntaxNode::FnDecl(fn_decl)) = self.syntax_tree.get(*expression) {
                let fn_decl = fn_decl.clone();
                self.fn_decl(fn_decl)?
            }
        }

        if let Some(trailing_expression) = block.trailing_expression {
            if let Some(SyntaxNode::FnDecl(fn_decl)) = self.syntax_tree.get(trailing_expression) {
                let fn_decl = fn_decl.clone();
                self.fn_decl(fn_decl)?
            }
        }

        Ok(())
    }

    fn fn_decl(&mut self, fn_decl: FnDecl) -> Result<(), CompilerError> {
        let name = Symbol::new(fn_decl.name);

        self.context()?
            .scope_stack()
            .add_local(name, State::Function { is_initialized: false })?;

        Ok(())
    }
}
