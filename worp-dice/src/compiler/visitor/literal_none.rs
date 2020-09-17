use crate::{compiler::Compiler, syntax::LitNone, CompilerError};

use super::NodeVisitor;

impl NodeVisitor<&LitNone> for Compiler {
    fn visit(&mut self, LitNone(span): &LitNone) -> Result<(), CompilerError> {
        self.context()?.assembler().push_none(span.clone());

        Ok(())
    }
}
