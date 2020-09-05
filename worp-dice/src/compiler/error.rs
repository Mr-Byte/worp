use crate::runtime::core::Symbol;

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error("Encountered undeclared variable {0}.")]
    UndeclaredVariable(Symbol),
    #[error("Cannot assign to immutable variable {0}.")]
    ImmutableVariable(Symbol),
    #[error("Invalid assignment target.")]
    InvalidAssignmentTarget,

    #[error("The break keyword can only be used inside loops.")]
    InvalidBreak,
    #[error("The continue keyword can only be used inside loops.")]
    InvalidContinue,
    #[error("Loops cannot end with an expression. Try adding ; to the last statement.")]
    InvalidLoopEnding,

    #[error("Internal Compiler Error: {0}")]
    InternalCompilerError(String),
}
