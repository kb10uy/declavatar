use declavatar_derive::EnumLog;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, EnumLog)]
pub enum DeclError {
    #[log_error("decl.unsupported")]
    #[error("specified format is currently unsupported")]
    UnsupportedFormat,

    #[log_error("decl.internal")]
    #[error("internal error: {0}")]
    InternalError(String),

    #[log_error("decl.not_returned")]
    #[error("cannot fetch declaration object ({0:?})")]
    DelclarationNotReturned(String),
}
