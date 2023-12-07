use crate::static_type_name_impl;

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupLayer {}
static_type_name_impl!(DeclGroupLayer);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupOption {
    pub kind: DeclGroupOptionKind,
    pub targets: Vec<DeclGroupOptionTarget>,
}
static_type_name_impl!(DeclGroupOption);

#[derive(Debug, Clone, PartialEq)]
pub enum DeclGroupOptionKind {
    Boolean(bool),
    Selection(Option<String>),
    Keyframe(f64),
}

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclGroupOptionTarget {
    Shape,
    Object,
    Material,
}
static_type_name_impl!(DeclGroupOptionTarget);
