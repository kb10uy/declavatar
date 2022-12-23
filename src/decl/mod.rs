mod animation;
mod avatar;
pub mod document;
mod driver;
mod entry;
mod menu;

use std::error::Error;

use kdl::KdlNode;
use thiserror::Error as ThisError;

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
}

/// Parses into a value from KDL node.
pub trait FromNode: Sized {
    /// Corresponding error type.
    type Err: Error;

    /// Parses the node.
    fn from_node(node: &KdlNode) -> Result<Self, Self::Err>;
}

pub trait FromNodeExt {
    fn parse<T: FromNode>(&self) -> Result<T, T::Err>;
}

impl FromNodeExt for KdlNode {
    fn parse<T: FromNode>(&self) -> Result<T, T::Err> {
        T::from_node(self)
    }
}

/// Validates itself.
pub fn validate_self_node(node: &KdlNode, name: &'static str) -> Result<(), DeclError> {
    let node_name = node.name().value();
    if node_name != name {
        return Err(DeclError::IncorrectNodeName(name, node_name.into()));
    }
    Ok(())
}
