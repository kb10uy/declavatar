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

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationGroup {
    ShapeGroup(),
    ShapeSwitch(),
    ObjectGroup(),
    ObjectSwitch(),
}
