use crate::{runtime::core::Symbol, syntax::SyntaxError};

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error(transparent)]
    SyntaxError(#[from] SyntaxError),
    #[error("Encountered undeclared variable {0}.")]
    UndeclaredVariable(Symbol),
    #[error("Cannot assign to immutable variable {0}.")]
    ImmutableVariable(Symbol),
    #[error("Invalid assignment target.")]
    InvalidAssignmentTarget,

    #[error("Internal Compiler Error: {0}")]
    InternalCompilerError(String),
}
