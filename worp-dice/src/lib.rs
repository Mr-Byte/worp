use compiler::{CompilationKind, Compiler};
use runtime::interpreter::Runtime;

#[macro_use]
mod macros;
mod compiler;
mod runtime;
mod syntax;

pub use compiler::error::CompilerError;
pub use runtime::{
    core::{Symbol, Value},
    error::RuntimeError,
};
pub use syntax::SyntaxError;

#[derive(Default)]
pub struct Dice {
    runtime: Runtime,
}

impl Dice {
    pub fn run_script(&mut self, input: &str) -> Result<Value, DiceError> {
        let bytecode = Compiler::compile_str(input, CompilationKind::Script)?;
        self.runtime.run_script(bytecode).map_err(From::from)
    }

    pub fn disassemble_script(&self, input: &str) -> Result<String, DiceError> {
        let bytecode = Compiler::compile_str(input, CompilationKind::Script)?;
        Ok(bytecode.to_string())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DiceError {
    #[error(transparent)]
    RuntimeError(#[from] RuntimeError),
    #[error(transparent)]
    CompilerError(#[from] CompilerError),
    #[error(transparent)]
    SyntaxError(#[from] SyntaxError),
}
