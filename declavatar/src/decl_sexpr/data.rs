use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclParameters {
    pub parameters: Vec<DeclParameter>,
}

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclParameter {
    pub ty: DeclParameterType,
    pub scope: Option<DeclParameterScope>,
    pub save: Option<bool>,
    pub name: String,
}

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

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclAssets {
    pub assets: Vec<DeclAsset>,
}

#[derive(Debug, Clone, PartialEq, Eq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclAsset {
    Material(String),
    Animation(String),
}
