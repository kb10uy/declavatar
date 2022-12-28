pub mod animations;
pub mod document;
pub mod drivers;
pub mod menu;
pub mod parameters;

use std::{collections::HashMap, error::Error, result::Result as StdResult};

use kdl::{KdlEntry, KdlNode, KdlValue};
use semver::{Version, VersionReq};
use thiserror::Error as ThisError;

/// Result type for decl module.
pub type Result<T> = StdResult<T, DeclError>;

/// Describes errors in parsing declaration.
#[derive(Debug, Clone, ThisError, PartialEq, Eq)]
pub enum DeclError {
    /// Incorrect node name detected (internal only).
    #[error("node name is incorrect: expected '{0}', found '{1}'")]
    IncorrectNodeName(&'static str, String),

    /// Too short arguments for node.
    #[error("node arguments are insufficient: '{1}' (#{0}) needed")]
    InsufficientArguments(usize, &'static str),

    /// Wrong type.
    #[error("entry value has incorrect type: expected {0}")]
    IncorrectType(&'static str),

    /// This node must have children block.
    #[error("node '{0}' must have children")]
    MustHaveChildren(String),

    /// Invalid name node detected.
    #[error("node '{0}' is invalid node name")]
    InvalidNodeDetected(String),

    /// Mandatory node not found.
    #[error("must have node '{0}' in '{1}'")]
    NodeNotFound(&'static str, &'static str),

    #[error("node '{0}' has duplicate")]
    DuplicateNodeFound(&'static str),

    #[error("feature '{feature}' not defined in {current} (required {requirement})")]
    VersionDoesNotMeet {
        current: Version,
        requirement: VersionReq,
        feature: String,
    },
}

/// Indicates that this struct can be constructed from KDL node.
pub trait DeclNode: Sized {
    /// Node name for this struct.
    const NODE_NAME: &'static str;

    /// Version requirement for this node struct.
    const REQUIRED_VERSION: VersionReq;

    /// Parses the node.
    fn parse(node: &KdlNode) -> Result<Self>;
}

/// Parses into a value from KDL node.
pub trait DeclNodeExt {
    fn parse<T: DeclNode>(&self, version: &Version) -> Result<T>;
}

impl DeclNodeExt for KdlNode {
    fn parse<T: DeclNode>(&self, version: &Version) -> Result<T> {
        if !T::REQUIRED_VERSION.matches(version) {
            return Err(DeclError::VersionDoesNotMeet {
                current: version.clone(),
                requirement: T::REQUIRED_VERSION.clone(),
                feature: format!("{} node", T::NODE_NAME),
            });
        }

        let self_name = self.name().value();
        if self_name != T::NODE_NAME {
            return Err(DeclError::IncorrectNodeName(self_name, T::NODE_NAME.into()));
        }

        T::parse(self)
    }
}

/// Parses into a value from KDL entry.
pub trait FromValue: Sized {
    /// Parses the node.
    fn from_value(value: &KdlValue) -> Result<Self>;
}

impl FromValue for String {
    fn from_value(value: &KdlValue) -> Result<String> {
        value
            .as_string()
            .map(|s| s.to_string())
            .ok_or(DeclError::IncorrectType("string"))
    }
}

impl FromValue for i64 {
    fn from_value(value: &KdlValue) -> Result<i64> {
        value.as_i64().ok_or(DeclError::IncorrectType("integer"))
    }
}

impl FromValue for f64 {
    fn from_value(value: &KdlValue) -> Result<f64> {
        value.as_f64().ok_or(DeclError::IncorrectType("float"))
    }
}

impl FromValue for bool {
    fn from_value(value: &KdlValue) -> Result<bool> {
        value.as_bool().ok_or(DeclError::IncorrectType("boolean"))
    }
}

/// Splits node entries into arguments list and properties map.
pub fn split_entries(entries: &[KdlEntry]) -> (Vec<&KdlValue>, HashMap<&str, &KdlValue>) {
    let mut arguments = Vec::new();
    let mut properties = HashMap::new();

    for entry in entries {
        if let Some(name) = entry.name() {
            properties.insert(name.value(), entry.value());
        } else {
            arguments.push(entry.value());
        }
    }

    (arguments, properties)
}

/// Gets an argument value from arguments list.
pub fn get_argument<T: FromValue>(
    arguments: &[&KdlValue],
    index: usize,
    name: &'static str,
) -> Result<T> {
    let value = arguments
        .get(index)
        .ok_or(DeclError::InsufficientArguments(0, name))?;
    T::from_value(value)
}

/// Gets a property value from properties list.
pub fn get_property<T: FromValue>(
    properties: &HashMap<&str, &KdlValue>,
    name: &'static str,
) -> Result<T> {
    let value = properties
        .get(name)
        .ok_or(DeclError::InsufficientArguments(0, name))?;
    T::from_value(value)
}
