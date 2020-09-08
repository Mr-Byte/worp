use super::NodeCompiler;
use crate::{compiler::Compiler, syntax::Literal, CompilerError};

impl NodeCompiler<&Literal> for Compiler {
    fn compile_node(&mut self, node: &Literal) -> Result<(), CompilerError> {
        match node {
            Literal::Ident(identifer) => self.compile_node(identifer)?,
            Literal::None(none) => self.compile_node(none)?,
            Literal::Unit(unit) => self.compile_node(unit)?,
            Literal::Integer(int) => self.compile_node(int)?,
            Literal::Float(float) => self.compile_node(float)?,
            Literal::String(string) => self.compile_node(string)?,
            Literal::Bool(bool) => self.compile_node(bool)?,
            Literal::List(list) => self.compile_node(list)?,
            Literal::Object(_) => todo!(),
        };

        Ok(())
    }
}
