use super::{core::key::ValueKey, symbol::Symbol};
use crate::syntax::ParseError;
use std::{error::Error, num::ParseIntError};

#[derive(thiserror::Error, Debug)]
#[error("Evaluation failed.")]
pub enum RuntimeError {
    #[error("Runtime Error: Execution unexpectedly aborted.")]
    Aborted(#[from] Box<dyn Error>),
    #[error("Runtime Error: The target type {0} is not an object.")]
    NotAnObject(Symbol),
    #[error("Runtime Error: The target type {0} is not a function.")]
    NotAFunction(Symbol),
    #[error("Runtime Error: Functions require a self parameter to be called as a method.")]
    NoSelfParameterProvided,
    #[error("Runtime Error: {0} has no instructor.")]
    NoConstructor(Symbol),
    #[error("Runtime Error: Missing field {0}.")]
    MissingField(ValueKey),
    #[error("Runtime Error: Invalid number of parameters passed to function. Expected: {0}, Found: {1}.")]
    InvalidFunctionArgs(usize, usize),
    #[error("Runtime Error: Invalid type. Expected: {0}, Found: {1}.")]
    InvalidType(Symbol, Symbol),
    #[error("Runtime Error: Keys must be either Int or String. Found: {0}.")]
    InvalidKeyType(Symbol),
    #[error("Runtime Error: {0}")]
    ParseError(#[from] ParseError),
    #[error("Runtime Error: Variable {0} not found.")]
    VariableNotFound(Symbol),
    #[error("Runtime Error: Type {0} not found.")]
    TypeNotFound(Symbol),
    #[error("Runtime Error: Index out of bounds. Length: {0}, Index: {1}.")]
    IndexOutOfBounds(usize, i64),
    #[error("Runtime Error: Unable to parse value to Int.")]
    ParseIntError(#[from] ParseIntError),
}
