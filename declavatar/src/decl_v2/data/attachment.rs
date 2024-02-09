use crate::static_type_name_impl;

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclAttachments {
    pub attachments: Vec<DeclAttachment>,
}
static_type_name_impl!(DeclAttachments);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclAttachment {
    pub name: String,
    pub properties: Vec<DeclAttachmentProperty>,
}
static_type_name_impl!(DeclAttachment);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclAttachmentProperty {
    pub name: String,
    pub parameters: Vec<DeclAttachmentValue>,
}
static_type_name_impl!(DeclAttachmentProperty);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclAttachmentValue {
    Null,
    UntypedList(Vec<DeclAttachmentValue>),
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
