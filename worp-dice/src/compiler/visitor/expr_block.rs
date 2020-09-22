use super::NodeVisitor;
use crate::{
    compiler::{
        scope_stack::{ScopeKind, State},
        Compiler,
    },
    syntax::Block,
    CompilerError, Symbol,
};

pub enum BlockKind<'args, T: AsRef<str>> {
    Block,
    Function(&'args [T]),
}

impl<'args, T: AsRef<str>> NodeVisitor<(&Block, BlockKind<'args, T>)> for Compiler {
    fn visit(&mut self, (block, kind): (&Block, BlockKind<'args, T>)) -> Result<(), CompilerError> {
        self.context()?.scope_stack().push_scope(ScopeKind::Block, None);

        if let BlockKind::Function(args) = kind {
            for arg in args {
                self.context()?.scope_stack().add_local(
                    Symbol::new(arg.as_ref()),
                    State::Local {
                        is_mutable: false,
                        is_initialized: true,
                    },
                )?;
            }
        }

        self.scan_item_decls(block)?;

        for expression in block.expressions.iter() {
            self.visit(*expression)?;
            self.context()?.assembler().pop(block.span.clone());
        }

        match block.trailing_expression {
            Some(trailing_expression) => self.visit(trailing_expression)?,
            None => self.context()?.assembler().push_unit(block.span.clone()),
        }

        let scope = self.context()?.scope_stack().pop_scope()?;

        for variable in scope.variables {
            if variable.is_captured {
                self.context()?
                    .assembler()
                    .close_upvalue(variable.slot as u8, block.span.clone());
            }
        }

        // NOTE: If in context of a function, implicitly return the top item on the stack.
        // If the previous instruction was a return, this will never execute.
        if let BlockKind::Function(_) = kind {
            self.context()?.assembler().ret(block.span.clone())
        }

        Ok(())
    }
}
