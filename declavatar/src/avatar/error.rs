use std::result::Result as StdResult;

use miette::Diagnostic;
use thiserror::Error as ThisError;

pub type Result<T> = StdResult<T, AvatarError>;

/*
#[derive(Debug, Clone, ThisError, Diagnostic)]
#[error("{kind}")]
pub struct AvatarError {
    kind: AvatarErrorKind,
}
*/

#[derive(Debug, Clone, ThisError, Diagnostic)]
pub enum AvatarError {
    #[error("internal compiler error: {0}")]
    CompilerError(String),
}
