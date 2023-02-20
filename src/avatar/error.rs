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
    #[error("avatar name '{0} is invalid'")]
    InvalidAvatarName(String),

    #[error("parameter '{0}' is defined incompatibly")]
    IncompatibleParameterDefinition(String),

    #[error("parameter '{0}' is local, cannot be saved")]
    CannotSaveLocalParameter(String),

    #[error("parameter '{name}' (used by '{used_by}') not found")]
    ParameterNotFound { name: String, used_by: String },

    #[error("parameter '{name}' (used by '{used_by}') has wrong type, {expected} expected")]
    WrongParameterType {
        name: String,
        used_by: String,
        expected: &'static str,
    },
}
