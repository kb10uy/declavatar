use crate::decl_v2::data::layer::DeclGroupOptionKind;

use ketos::Error;
use thiserror::Error as ThisError;

pub type KetosResult<T> = Result<T, Error>;

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

    #[error("invalid copy mode: {0}")]
    InvalidCopyMode(String),

    #[error("duplicate named option block")]
    DuplicateNamedOption,

    #[error("invalid option kind: {0:?}")]
    InvalidGroupOption(DeclGroupOptionKind),

    #[error("invalid condition expression")]
    InvalidCondition,
}
