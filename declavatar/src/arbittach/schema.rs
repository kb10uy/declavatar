#[derive(Debug, Clone, PartialEq)]
pub struct Attachment {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub keywords: Vec<Keyword>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Keyword {
    pub name: String,
    pub required: bool,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Any,
    List(Box<ValueType>),
    Map(Box<ValueType>, Box<ValueType>),
    Boolean,
    Integer,
    Float,
    String,
    Vector(usize),
    GameObject,
    Material,
    AnimationClip,
}
