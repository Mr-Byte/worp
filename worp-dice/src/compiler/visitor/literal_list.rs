use crate::{compiler::Compiler, syntax::LitList, CompilerError};

use super::NodeVisitor;

impl NodeVisitor<&LitList> for Compiler {
    fn visit(&mut self, LitList { items: value, span }: &LitList) -> Result<(), CompilerError> {
        for item in value {
            self.visit(*item)?;
        }

        self.context()?.assembler().build_list(value.len() as u8, *span);

        Ok(())
    }
}
