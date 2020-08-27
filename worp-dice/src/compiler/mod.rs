use crate::{
    runtime::{
        core::Symbol,
        machine::{Module, Script},
    },
    syntax::{Parser, SyntaxTree},
};
use bytecode::BytecodeGenerator;
use error::CompilerError;

mod bytecode;
pub mod error;
mod expression;

pub struct Compiler {
    input: String,
    syntax_tree: SyntaxTree,
    bytecode: BytecodeGenerator,
}

impl Compiler {
    pub fn compile_module(input: &str) -> Result<Module, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self {
            input: String::from(input),
            syntax_tree,
            bytecode: BytecodeGenerator::default(),
        };

        compiler.expression(compiler.syntax_tree.root())?;

        let module = Module::new(compiler.bytecode.generate());

        Ok(module)
    }

    pub fn compile_script(input: &str) -> Result<Script, CompilerError> {
        let syntax_tree = Parser::new(input).parse()?;
        let mut compiler = Self {
            input: String::from(input),
            syntax_tree,
            bytecode: BytecodeGenerator::default(),
        };

        compiler.expression(compiler.syntax_tree.root())?;

        let script = Script::new(compiler.bytecode.generate());

        Ok(script)
    }
}
