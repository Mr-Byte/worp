use super::object::ObjectKey;

#[derive(thiserror::Error, Debug)]
#[error("Evaluation failed.")]
pub enum RuntimeError {
    #[error("Runtime Error: Execution unexpectedly aborted.")]
    Aborted,
    #[error("Runtime Error: The target type is not an object.")]
    NotAnObject,
    #[error("Runtime Error: The target type is not a function.")]
    NotAFunction,
    #[error("Runtime Error: Missing field {0}.")]
    MissingField(ObjectKey),
    #[error("Runtime Error: Invalid number of parameters passed to function. Expected: {0}, Found: {1}.")]
    InvalidFunctionArgs(usize, usize),
    #[error("Runtime Error: Invalid type")]
    InvalidType,
}
