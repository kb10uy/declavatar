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
