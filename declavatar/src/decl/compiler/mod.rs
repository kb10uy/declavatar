mod animations;
mod document;
mod drivers;
mod menu;
mod parameters;

use crate::{
    compiler::ErrorStackCompiler,
    decl::error::{DeclError, DeclErrorKind, Result},
};

use std::collections::HashMap;

use kdl::{KdlEntry, KdlNode, KdlValue};
use miette::SourceSpan;

pub type DeclCompiler = ErrorStackCompiler<DeclError>;

pub fn deconstruct_node<'a>(
    node: &'a KdlNode,
    name: Option<&'static str>,
    children_existence: Option<bool>,
) -> Result<(&'a str, NodeEntries<'a>, &'a [KdlNode])> {
    let node_name = node.name().value();
    let node_span = node.name().span();
    let entries = NodeEntries::split_entries(node);
    let children = node.children();

    if let Some(expected_name) = name {
        if node_name != expected_name {
            return Err(DeclError::new(
                node_span,
                DeclErrorKind::IncorrectNodeName(expected_name),
            ));
        }
    }
    let children = match (children_existence, children) {
        (Some(true), Some(children_doc)) => children_doc.nodes(),
        (Some(true), None) => {
            return Err(DeclError::new(node_span, DeclErrorKind::MustHaveChildren));
        }
        (Some(false), Some(_)) => {
            return Err(DeclError::new(
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
    node_span: &'a SourceSpan,
    arguments: Vec<&'a KdlEntry>,
    properties: HashMap<&'a str, &'a KdlEntry>,
}

#[allow(dead_code)]
impl<'a> NodeEntries<'a> {
    fn split_entries(node: &'a KdlNode) -> NodeEntries<'a> {
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
            node_span: node.name().span(),
            arguments,
            properties,
        }
    }

    pub fn get_argument<T: FromKdlEntry<'a>>(&self, index: usize, name: &'static str) -> Result<T> {
        let entry = self.arguments.get(index).ok_or_else(|| {
            DeclError::new(self.node_span, DeclErrorKind::InsufficientArguments(name))
        })?;
        T::from_kdl_entry(entry)
    }

    pub fn try_get_argument<T: FromKdlEntry<'a>>(&self, index: usize) -> Result<Option<T>> {
        self.arguments
            .get(index)
            .map(|e| T::from_kdl_entry(e))
            .transpose()
    }

    pub fn get_property<T: FromKdlEntry<'a>>(&self, name: &'static str) -> Result<T> {
        let entry = self.properties.get(name).ok_or_else(|| {
            DeclError::new(self.node_span, DeclErrorKind::InsufficientProperties(name))
        })?;
        T::from_kdl_entry(entry)
    }

    pub fn try_get_property<T: FromKdlEntry<'a>>(&self, name: &'static str) -> Result<Option<T>> {
        self.properties
            .get(name)
            .map(|e| T::from_kdl_entry(e))
            .transpose()
    }
}

/// Parses into a value from KDL entry.
pub trait FromKdlEntry<'a>: Sized {
    /// Parses the node.
    fn from_kdl_entry(entry: &'a KdlEntry) -> Result<Self>;
}

impl<'a> FromKdlEntry<'a> for &'a KdlValue {
    fn from_kdl_entry(entry: &'a KdlEntry) -> Result<&'a KdlValue> {
        Ok(entry.value())
    }
}

impl<'a> FromKdlEntry<'a> for String {
    fn from_kdl_entry(entry: &'a KdlEntry) -> Result<String> {
        entry
            .value()
            .as_string()
            .map(|s| s.to_string())
            .ok_or_else(|| DeclError::new(entry.span(), DeclErrorKind::IncorrectType("string")))
    }
}

impl<'a> FromKdlEntry<'a> for &'a str {
    fn from_kdl_entry(entry: &'a KdlEntry) -> Result<&'a str> {
        entry
            .value()
            .as_string()
            .ok_or_else(|| DeclError::new(entry.span(), DeclErrorKind::IncorrectType("string")))
    }
}

impl<'a> FromKdlEntry<'a> for i64 {
    fn from_kdl_entry(entry: &'a KdlEntry) -> Result<i64> {
        entry
            .value()
            .as_i64()
            .ok_or_else(|| DeclError::new(entry.span(), DeclErrorKind::IncorrectType("integer")))
    }
}

impl<'a> FromKdlEntry<'a> for f64 {
    fn from_kdl_entry(entry: &'a KdlEntry) -> Result<f64> {
        entry
            .value()
            .as_f64()
            .ok_or_else(|| DeclError::new(entry.span(), DeclErrorKind::IncorrectType("float")))
    }
}

impl<'a> FromKdlEntry<'a> for bool {
    fn from_kdl_entry(entry: &'a KdlEntry) -> Result<bool> {
        entry
            .value()
            .as_bool()
            .ok_or_else(|| DeclError::new(entry.span(), DeclErrorKind::IncorrectType("boolean")))
    }
}
