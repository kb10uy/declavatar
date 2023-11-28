use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum DeclError {
    #[error("unexpected value passed: type {0}")]
    UnexpectedTypeValue(String),

    #[error("scope must be specified")]
    MustBeScope,

    #[error("invalid scope name: {0}")]
    InvalidScope(String),
}
