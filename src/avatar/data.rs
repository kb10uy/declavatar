use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Avatar {
    pub name: String,
    pub parameters: HashMap<String, Parameter>,
    pub animation_groups: Vec<AnimationGroup>,
    pub driver_groups: Vec<DriverGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Parameter {
    pub name: String,
    pub value_type: ParameterType,
    pub sync_type: ParameterSync,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(tag = "type", content = "default")]
pub enum ParameterType {
    Int(u8),
    Float(f64),
    Bool(bool),
}

#[allow(dead_code)]
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

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum AnimationGroupContent {
    ShapeGroup {
        mesh: String,
        prevent_mouth: bool,
        prevent_eyelids: bool,
        default_targets: Vec<ShapeTarget>,
        options: HashMap<String, (usize, Vec<ShapeTarget>)>,
    },
    ShapeSwitch {
        mesh: String,
        prevent_mouth: bool,
        prevent_eyelids: bool,
        disabled: Vec<ShapeTarget>,
        enabled: Vec<ShapeTarget>,
    },
    ObjectGroup {
        default_targets: Vec<ObjectTarget>,
        options: HashMap<String, (usize, Vec<ObjectTarget>)>,
    },
    ObjectSwitch {
        disabled: Vec<ObjectTarget>,
        enabled: Vec<ObjectTarget>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShapeTarget(pub String, pub f64);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ObjectTarget(pub String, pub bool);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DriverGroup {
    pub name: String,
    pub local: bool,
    pub drivers: Vec<Driver>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Driver {
    SetInt(String, u8),
    SetFloat(String, f64),
    SetBool(String, bool),
    AddInt(String, u8),
    AddFloat(String, f64),
    RandomInt(String, (u8, u8)),
    RandomFloat(String, (f64, f64)),
    RandomBool(String, f64),
    Copy(String, String),
    RangedCopy(String, String, (f64, f64), (f64, f64)),
}
