use super::{object::ObjectKey, symbol::Symbol};
use crate::expression::ParseError;

#[derive(thiserror::Error, Debug)]
#[error("Evaluation failed.")]
pub enum RuntimeError {
    #[error("Runtime Error: Execution unexpectedly aborted.")]
    Aborted,
    #[error("Runtime Error: The target type {0} is not an object.")]
    NotAnObject(Symbol),
    #[error("Runtime Error: The target type {0} is not a function.")]
    NotAFunction(Symbol),
    #[error("Runtime Error: Missing field {0}.")]
    MissingField(ObjectKey),
    #[error("Runtime Error: Invalid number of parameters passed to function. Expected: {0}, Found: {1}.")]
    InvalidFunctionArgs(usize, usize),
    #[error("Runtime Error: Invalid type. Expected: {0}, Found: {1}.")]
    InvalidType(Symbol, Symbol),
    #[error("Runtime Error: {0}")]
    ParseError(#[from] ParseError),
}
