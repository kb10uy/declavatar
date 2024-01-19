use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub required: bool,
    pub parameters: Vec<Parameter>,
    pub keywords: Vec<Keyword>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyword {
    pub name: String,
    pub required: bool,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueType {
    Any,
    OneOf(Vec<ValueType>),
    List(Box<ValueType>),
    Tuple(Vec<ValueType>),
    Map(Box<ValueType>, Box<ValueType>),
    Null,
    Boolean,
    Integer,
    Float,
    String,
    Vector(usize),
    GameObject,
    Material,
    AnimationClip,
}

impl ValueType {
    pub const fn name(&self) -> &'static str {
        match self {
            ValueType::Any => "any",
            ValueType::OneOf(_) => "one-of",
            ValueType::List(_) => "list",
            ValueType::Tuple(_) => "tuple",
            ValueType::Map(_, _) => "map",
            ValueType::Null => "null",
            ValueType::Boolean => "boolean",
            ValueType::Integer => "integer",
            ValueType::Float => "float",
            ValueType::String => "string",
            ValueType::Vector(_) => "vector",
            ValueType::GameObject => "game object",
            ValueType::Material => "material",
            ValueType::AnimationClip => "animation clip",
        }
    }
}
