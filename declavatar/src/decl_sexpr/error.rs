use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum DeclError {
    #[error("unexpected value passed: {1} expected, {0} found")]
    UnexpectedTypeValue(String, String),

    #[error("keyword argument expected: {0}")]
    KeywordExpected(String),

    #[error("scope must be specified")]
    MustBeScope,

    #[error("invalid scope name: {0}")]
    InvalidScope(String),
}
