use crate::static_type_name_impl;

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclAttachments {
    pub targets: DeclAttachmentTarget,
}
static_type_name_impl!(DeclAttachments);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclAttachmentTarget {
    pub name: String,
    // pub attachments: DeclAttachment,
}
static_type_name_impl!(DeclAttachmentTarget);

pub struct DeclAttachmentProperty {
    pub name: String,
    pub parameters: Vec<DeclAttachmentValue>,
}

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclAttachmentValue {
    Null,
    List(Vec<DeclAttachmentValue>),
    Tuple(Vec<DeclAttachmentValue>),
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
static_type_name_impl!(DeclAttachmentValue);
