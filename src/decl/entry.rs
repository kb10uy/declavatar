use crate::decl::DeclError;

use std::collections::HashMap;

use kdl::{KdlEntry, KdlValue};

/// Parses into a value from KDL entry.
pub trait FromValue: Sized {
    /// Parses the node.
    fn from_value(value: &KdlValue) -> Result<Self, DeclError>;
}

impl FromValue for String {
    fn from_value(value: &KdlValue) -> Result<String, DeclError> {
        value
            .as_string()
            .map(|s| s.to_string())
            .ok_or(DeclError::IncorrectType("string"))
    }
}

impl FromValue for i64 {
    fn from_value(value: &KdlValue) -> Result<i64, DeclError> {
        value.as_i64().ok_or(DeclError::IncorrectType("integer"))
    }
}

impl FromValue for f64 {
    fn from_value(value: &KdlValue) -> Result<f64, DeclError> {
        value.as_f64().ok_or(DeclError::IncorrectType("float"))
    }
}

impl FromValue for bool {
    fn from_value(value: &KdlValue) -> Result<bool, DeclError> {
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
) -> Result<T, DeclError> {
    let value = arguments
        .get(index)
        .ok_or(DeclError::InsufficientArguments(0, name))?;
    T::from_value(value)
}

/// Gets a property value from properties list.
pub fn get_property<T: FromValue>(
    properties: &HashMap<&str, &KdlValue>,
    name: &'static str,
) -> Result<T, DeclError> {
    let value = properties
        .get(name)
        .ok_or(DeclError::InsufficientArguments(0, name))?;
    T::from_value(value)
}
