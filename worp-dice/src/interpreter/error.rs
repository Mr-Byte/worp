#[derive(thiserror::Error, Debug)]
#[error("Evaluation failed.")]
pub struct EvaluationError;
