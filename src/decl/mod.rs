pub mod animations;
pub mod document;
pub mod drivers;
pub mod menu;
pub mod parameters;

use std::{collections::HashMap, result::Result as StdResult};

use kdl::{KdlEntry, KdlNode, KdlValue};
use miette::{Diagnostic, SourceSpan};
use semver::{Error as SemverError, Version, VersionReq};
use thiserror::Error as ThisError;

/// Result type for decl module.
pub type Result<T> = StdResult<T, DeclError>;

#[derive(Debug, ThisError, Diagnostic)]
#[error("{error_kind}")]
pub struct DeclError {
    /// Source text for diagnostics.
    #[source_code]
    input: String,

    /// Error position for diagnostics.
    #[label("{}", "here")]
    span: SourceSpan,

    // Error kind.
    error_kind: DeclErrorKind,
}

/// Describes errors in parsing declaration.
#[allow(dead_code)]
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
    pub fn new(source: &str, span: &SourceSpan, kind: DeclErrorKind) -> DeclError {
        DeclError {
            input: source.into(),
            span: span.clone(),
            error_kind: kind,
        }
    }
}

pub fn deconstruct_node<'a>(
    source: &'a str,
    node: &'a KdlNode,
    name: Option<&'static str>,
    children_existence: Option<bool>,
) -> Result<(&'a str, NodeEntries<'a>, &'a [KdlNode])> {
    let node_name = node.name().value();
    let node_span = node.name().span();
    let entries = NodeEntries::split_entries(node, source);
    let children = node.children();

    if let Some(expected_name) = name {
        if node_name != expected_name {
            return Err(DeclError::new(
                source,
                node_span,
                DeclErrorKind::IncorrectNodeName(expected_name),
            ));
        }
    }
    let children = match (children_existence, children) {
        (Some(true), Some(children_doc)) => children_doc.nodes(),
        (Some(true), None) => {
            return Err(DeclError::new(
                source,
                node_span,
                DeclErrorKind::MustHaveChildren,
            ));
        }
        (Some(false), Some(_)) => {
            return Err(DeclError::new(
                source,
                node_span,
                DeclErrorKind::MustNotHaveChildren,
            ));
        }
        (Some(false), None) => &[],
        (None, Some(children_doc)) => children_doc.nodes(),
        (None, None) => &[],
    };

    Ok((node_name, entries, children))
}

pub struct NodeEntries<'a> {
    source: &'a str,
    node_span: &'a SourceSpan,
    arguments: Vec<&'a KdlEntry>,
    properties: HashMap<&'a str, &'a KdlEntry>,
}

#[allow(dead_code)]
impl<'a> NodeEntries<'a> {
    fn split_entries(node: &'a KdlNode, source: &'a str) -> NodeEntries<'a> {
        let mut arguments = Vec::new();
        let mut properties = HashMap::new();

        for entry in node.entries() {
            if let Some(name) = entry.name() {
                properties.insert(name.value(), entry);
            } else {
                arguments.push(entry);
            }
        }

        NodeEntries {
            source,
            node_span: node.name().span(),
            arguments,
            properties,
        }
    }

    pub fn get_argument<T: FromKdlEntry<'a>>(&self, index: usize, name: &'static str) -> Result<T> {
        let entry = self.arguments.get(index).ok_or_else(|| {
            DeclError::new(
                self.source,
                self.node_span,
                DeclErrorKind::InsufficientArguments(name),
            )
        })?;
        T::from_kdl_entry(entry, self.source)
    }

    pub fn try_get_argument<T: FromKdlEntry<'a>>(&self, index: usize) -> Result<Option<T>> {
        self.arguments
            .get(index)
            .map(|e| T::from_kdl_entry(e, self.source))
            .transpose()
    }

    pub fn get_property<T: FromKdlEntry<'a>>(&self, name: &'static str) -> Result<T> {
        let entry = self.properties.get(name).ok_or_else(|| {
            DeclError::new(
                self.source,
                self.node_span,
                DeclErrorKind::InsufficientProperties(name),
            )
        })?;
        T::from_kdl_entry(entry, self.source)
    }

    pub fn try_get_property<T: FromKdlEntry<'a>>(&self, name: &'static str) -> Result<Option<T>> {
        self.properties
            .get(name)
            .map(|e| T::from_kdl_entry(e, self.source))
            .transpose()
    }
}

/// Parses into a value from KDL entry.
pub trait FromKdlEntry<'a>: Sized {
    /// Parses the node.
    fn from_kdl_entry(entry: &'a KdlEntry, source: &'a str) -> Result<Self>;
}

impl<'a> FromKdlEntry<'a> for &'a KdlValue {
    fn from_kdl_entry(entry: &'a KdlEntry, _source: &'a str) -> Result<&'a KdlValue> {
        Ok(entry.value())
    }
}

impl<'a> FromKdlEntry<'a> for String {
    fn from_kdl_entry(entry: &'a KdlEntry, source: &'a str) -> Result<String> {
        entry
            .value()
            .as_string()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                DeclError::new(source, entry.span(), DeclErrorKind::IncorrectType("string"))
            })
    }
}

impl<'a> FromKdlEntry<'a> for &'a str {
    fn from_kdl_entry(entry: &'a KdlEntry, source: &'a str) -> Result<&'a str> {
        entry.value().as_string().ok_or_else(|| {
            DeclError::new(source, entry.span(), DeclErrorKind::IncorrectType("string"))
        })
    }
}

impl<'a> FromKdlEntry<'a> for i64 {
    fn from_kdl_entry(entry: &'a KdlEntry, source: &'a str) -> Result<i64> {
        entry.value().as_i64().ok_or_else(|| {
            DeclError::new(
                source,
                entry.span(),
                DeclErrorKind::IncorrectType("integer"),
            )
        })
    }
}

impl<'a> FromKdlEntry<'a> for f64 {
    fn from_kdl_entry(entry: &'a KdlEntry, source: &'a str) -> Result<f64> {
        entry.value().as_f64().ok_or_else(|| {
            DeclError::new(source, entry.span(), DeclErrorKind::IncorrectType("float"))
        })
    }
}

impl<'a> FromKdlEntry<'a> for bool {
    fn from_kdl_entry(entry: &'a KdlEntry, source: &'a str) -> Result<bool> {
        entry.value().as_bool().ok_or_else(|| {
            DeclError::new(
                source,
                entry.span(),
                DeclErrorKind::IncorrectType("boolean"),
            )
        })
    }
}
