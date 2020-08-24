use crate::{
    runtime::machine::Module,
    syntax::{Parser, SyntaxTree},
};
use bytecode::BytecodeGenerator;
use error::CompilerError;

mod bytecode;
pub mod error;
mod expression;

pub struct Compiler<'a> {
    input: &'a str,
    syntax_tree: SyntaxTree,
    bytecode: BytecodeGenerator,
}

impl<'a> Compiler<'a> {
    pub fn compile(input: &'a str) -> Result<Module, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self {
            input,
            syntax_tree,
            bytecode: BytecodeGenerator::default(),
        };

        compiler.expression(compiler.syntax_tree.root())?;

        let module = Module::new(compiler.bytecode.generate());

        Ok(module)
    }
}
