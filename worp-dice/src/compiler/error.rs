use crate::syntax::SyntaxError;

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error(transparent)]
    SyntaxError(#[from] SyntaxError),
}
