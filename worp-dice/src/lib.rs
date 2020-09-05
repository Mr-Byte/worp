use compiler::{CompilationKind, CompilationUnit, Compiler};
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
        if let CompilationUnit::Script(script) = Compiler::try_from_str(input, CompilationKind::Script)?.compile()? {
            self.runtime.run_script(script).map_err(From::from)
        } else {
            unreachable!()
        }
    }

    pub fn disassemble_script(&self, input: &str) -> Result<String, DiceError> {
        if let CompilationUnit::Script(script) = Compiler::try_from_str(input, CompilationKind::Script)?.compile()? {
            Ok(script.to_string())
        } else {
            unreachable!()
        }
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
