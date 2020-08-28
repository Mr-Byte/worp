use crate::{runtime::core::Symbol, syntax::SyntaxError};

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error(transparent)]
    SyntaxError(#[from] SyntaxError),
    #[error("Encountered undeclared variable {0}.")]
    UndeclaredVariable(Symbol),
}
