use crate::static_type_name_impl;

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclParameters {
    pub parameters: Vec<DeclParameter>,
}
static_type_name_impl!(DeclParameters);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclParameter {
    pub ty: DeclParameterType,
    pub name: String,
    pub scope: Option<DeclParameterScope>,
    pub save: Option<bool>,
    pub unique: Option<bool>,
}
static_type_name_impl!(DeclParameter);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeclParameterType {
    Int(Option<u8>),
    Float(Option<f64>),
    Bool(Option<bool>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclParameterScope {
    Internal,
    Local,
    Synced,
}
