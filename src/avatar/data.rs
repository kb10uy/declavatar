use crate::avatar::diagnostic::Instrument;

use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Avatar {
    pub name: String,
    pub parameters: HashMap<String, Parameter>,
    pub animation_groups: Vec<AnimationGroup>,
}

impl Instrument for Avatar {
    const INSTRUMENT_NAME: &'static str = "avatar";
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Parameter {
    pub name: String,
    pub value_type: ParameterType,
    pub sync_type: ParameterSync,
}

impl Instrument for Parameter {
    const INSTRUMENT_NAME: &'static str = "parameter";
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(tag = "type", content = "default")]
pub enum ParameterType {
    Int(u8),
    Float(f64),
    Bool(bool),
}

impl ParameterType {
    pub const INT_TYPE: ParameterType = ParameterType::Int(0);
    pub const FLOAT_TYPE: ParameterType = ParameterType::Float(0.0);
    pub const BOOL_TYPE: ParameterType = ParameterType::Bool(false);

    pub const fn type_name(&self) -> &'static str {
        match self {
            ParameterType::Int(_) => "int",
            ParameterType::Float(_) => "float",
            ParameterType::Bool(_) => "bool",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "type", content = "save")]
pub enum ParameterSync {
    Local,
    Synced(bool),
}

#[derive(Debug, Clone, Serialize)]
pub struct AnimationGroup {
    pub name: String,
    pub parameter: String,
    pub content: AnimationGroupContent,
}

impl Instrument for AnimationGroup {
    const INSTRUMENT_NAME: &'static str = "animation group";
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum AnimationGroupContent {
    ShapeGroup {
        mesh: String,
        prevent_mouth: bool,
        prevent_eyelids: bool,
        options: HashMap<AnimationOption, Vec<ShapeTarget>>,
    },
    ShapeSwitch {
        mesh: String,
        prevent_mouth: bool,
        prevent_eyelids: bool,
        disabled: Vec<ShapeTarget>,
        enabled: Vec<ShapeTarget>,
    },
    ObjectGroup {
        options: HashMap<AnimationOption, Vec<ObjectTarget>>,
    },
    ObjectSwitch {
        disabled: Vec<ObjectTarget>,
        enabled: Vec<ObjectTarget>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "type", content = "name", into = "String")]
pub enum AnimationOption {
    Default,
    Option(String),
}

impl From<AnimationOption> for String {
    fn from(value: AnimationOption) -> Self {
        match value {
            AnimationOption::Default => "d".into(),
            AnimationOption::Option(s) => format!("o:{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShapeTarget(pub String, pub f64);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ObjectTarget(pub String, pub bool);
