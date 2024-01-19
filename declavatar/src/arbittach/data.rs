use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub parameters: Vec<Value>,
    pub keywords: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    List(Vec<Value>),
    Tuple(Vec<Value>),
    // Map(HashMap<Value, Value>),
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Vector(Vec<f64>),
    GameObject(String),
    Material(String),
    AnimationClip(String),
}
