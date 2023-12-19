use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum DeclError {
    #[error("specified format is currently unsupported")]
    UnsupportedFormat,

    #[error("internal error: {0}")]
    InternalError(String),

    #[error("cannot fetch declaration object ({0:?})")]
    DelclarationNotReturned(Option<String>),
}
