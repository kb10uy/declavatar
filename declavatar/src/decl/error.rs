use std::result::Result as StdResult;

use miette::{Diagnostic, Error as MietteError, SourceSpan};
use semver::{Error as SemverError, Version, VersionReq};
use thiserror::Error as ThisError;

/// Result type for decl module.
pub type Result<T> = StdResult<T, DeclError>;

#[derive(Debug, ThisError, Diagnostic)]
#[error("{error_kind}")]
#[diagnostic()]
pub struct DeclError {
    /// Error position for diagnostics.
    #[label("{}", "here")]
    span: SourceSpan,

    // Error kind.
    error_kind: DeclErrorKind,
}

/// Describes errors in parsing declaration.
#[derive(Debug, ThisError, Diagnostic)]
pub enum DeclErrorKind {
    /// Incorrect node name detected (internal only).
    #[error("node name is incorrect: expected '{0}'")]
    IncorrectNodeName(&'static str),

    /// Too short arguments for node.
    #[error("node arguments are insufficient: <{0}> needed")]
    InsufficientArguments(&'static str),

    /// Too short properties for node.
    #[error("node properties are insufficient: <{0}> needed")]
    InsufficientProperties(&'static str),

    /// Wrong type.
    #[error("entry value has incorrect type: expected {0}")]
    IncorrectType(&'static str),

    /// This node must have children block.
    #[error("this node must have children")]
    MustHaveChildren,

    /// This node must have children block.
    #[error("this node must not have children")]
    MustNotHaveChildren,

    /// This value must be typed via `(type)` format.
    #[error("this value must be annoatated")]
    UnannotatedValue,

    /// Invalid type specified.
    #[error("specified annotation is invalid")]
    InvalidAnnotation,

    /// Invalid name node detected.
    #[error("the node is invalid")]
    InvalidNodeDetected,

    /// Mandatory node not found.
    #[error("must have node '{0}'")]
    NodeNotFound(&'static str),

    #[error("this node is duplicate")]
    DuplicateNodeFound,

    #[error("feature '{feature}' not defined in {current} (required {requirement})")]
    VersionDoesNotMeet {
        feature: String,
        current: Version,
        requirement: VersionReq,
    },

    #[error("version definition error: {0}")]
    VersionError(#[from] SemverError),
}

impl DeclError {
    pub fn new(span: &SourceSpan, kind: DeclErrorKind) -> DeclError {
        DeclError {
            span: *span,
            error_kind: kind,
        }
    }

    pub fn with_source(self, source: &str) -> MietteError {
        MietteError::new(self).with_source_code(source.to_string())
    }
}
