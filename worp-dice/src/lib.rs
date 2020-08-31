use compiler::Compiler;
use runtime::interpreter::Runtime;

#[macro_use]
mod macros;
mod compiler;
mod runtime;
mod syntax;

pub use compiler::error::CompilerError;
pub use runtime::core::Symbol;
pub use runtime::core::Value;
pub use runtime::error::RuntimeError;
pub use syntax::SyntaxError;

#[derive(Default)]
pub struct Dice {
    runtime: Runtime,
}

impl Dice {
    pub fn run_script(&mut self, input: &str) -> Result<Value, DiceError> {
        let script = Compiler::compile_script(input)?;
        self.runtime.run_script(script).map_err(From::from)
    }

    pub fn disassemble_script(&self, input: &str) -> Result<String, DiceError> {
        let script = Compiler::compile_script(input)?;
        Ok(script.to_string())
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
