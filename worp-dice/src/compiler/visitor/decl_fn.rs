use crate::{compiler::CompilationKind, compiler::Compiler, syntax::FnDecl};

use super::NodeVisitor;

impl NodeVisitor<&FnDecl> for Compiler {
    fn visit(&mut self, node: &FnDecl) -> Result<(), crate::CompilerError> {
        let _ = Self::new(
            self.syntax_tree.child(node.body).expect("Node should always exist."),
            CompilationKind::Function,
        )
        .compile()?;

        self.assembler.push_unit(node.span.clone());

        Ok(())
    }
}
