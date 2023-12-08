use std::error::Error as StdError;

use thiserror::Error as ThisError;

use crate::decl_v2::data::layer::DeclGroupOptionKind;

#[derive(Debug, ThisError)]
pub enum DeclError {
    #[error("specified format is currently unsupported")]
    UnsupportedFormat,

    #[error("internal error: {0}")]
    InternalError(Box<dyn StdError>),

    #[error("cannot fetch declaration object ({0:?})")]
    DelclarationNotReturned(Option<Box<dyn StdError>>),
}

#[derive(Debug, ThisError)]
pub enum DeclSexprError {
    #[error("unexpected value passed: {1} expected, {0} found")]
    UnexpectedTypeValue(String, String),

    #[error("keyword argument expected: {0}")]
    KeywordExpected(String),

    #[error("scope must be specified")]
    MustBeScope,

    #[error("invalid scope name: {0}")]
    InvalidScope(String),

    #[error("duplicate named option block")]
    DuplicateNamedOption,

    #[error("invalid option kind: {0:?}")]
    InvalidGroupOption(DeclGroupOptionKind),
}
