use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Avatar {
    pub name: String,
    pub parameters: HashMap<String, Parameter>,
    pub animation_groups: Vec<AnimationGroup>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub value_type: ParameterType,
    pub sync_type: ParameterSync,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterSync {
    Local,
    Synced(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnimationOption {
    Default,
    Option(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapeTarget(pub String, pub f64);

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectTarget(pub String, pub bool);

#[derive(Debug, Clone)]
pub struct AnimationGroup {
    pub name: String,
    pub parameter: String,
    pub content: AnimationGroupContent,
}

#[derive(Debug, Clone)]
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
