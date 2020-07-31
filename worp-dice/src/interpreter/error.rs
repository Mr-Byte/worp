use super::object::ObjectType;
use crate::expression::ObjectKey;

#[derive(thiserror::Error, Debug)]
#[error("Evaluation failed.")]
pub enum RuntimeError {
    #[error("Runtime Error: Execution unexpectedly aborted.")]
    Aborted,
    #[error("Runtime Error: The target type {0:?} is not an object.")]
    NotAnObject(ObjectType),
    #[error("Runtime Error: The target type {0:?} is not a function.")]
    NotAFunction(ObjectType),
    #[error("Runtime Error: Missing field {0}.")]
    MissingField(ObjectKey),
    #[error("Runtime Error: Invalid number of parameters passed to function. Expected: {0}, Found: {1}.")]
    InvalidFunctionArgs(usize, usize),
    #[error("Runtime Error: Invalid type. Expected: {0:?}, Found: {1:?}.")]
    InvalidType(ObjectType, ObjectType),
}
