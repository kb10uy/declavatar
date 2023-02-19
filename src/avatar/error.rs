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
    #[error("parameter '{0}' is defined incompatibly")]
    IncompatibleParameterDefinition(String),

    #[error("parameter '{0}' is local, cannot be saved")]
    CannotSaveLocalParameter(String),
}
