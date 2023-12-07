use crate::{decl_sexpr::data::driver::DeclParameterDrive, static_type_name_impl};

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclControllerLayer {
    Group(DeclGroupLayer),
}
static_type_name_impl!(DeclControllerLayer);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupLayer {
    pub name: String,
    pub driven_by: String,
    pub default_mesh: Option<String>,
    pub options: Vec<DeclGroupOption>,
}
static_type_name_impl!(DeclGroupLayer);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupOption {
    pub kind: DeclGroupOptionKind,
    pub targets: Vec<DeclGroupOptionTarget>,
}
static_type_name_impl!(DeclGroupOption);

#[derive(Debug, Clone, PartialEq)]
pub enum DeclGroupOptionKind {
    Boolean(bool),
    Selection(Option<String>, Option<usize>),
    Keyframe(f64),
}

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclGroupOptionTarget {
    Shape(DeclGroupShapeTarget),
    Object(DeclGroupObjectTarget),
    Material(DeclGroupMaterialTarget),
    ParameterDrive(DeclParameterDrive),
}
static_type_name_impl!(DeclGroupOptionTarget);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupShapeTarget {
    pub shape: String,
    pub value: Option<f64>,
    pub mesh: Option<String>,
}
static_type_name_impl!(DeclGroupShapeTarget);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupObjectTarget {
    pub object: String,
    pub value: Option<bool>,
}
static_type_name_impl!(DeclGroupObjectTarget);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupMaterialTarget {
    pub index: usize,
    pub value: String,
    pub mesh: Option<String>,
}
static_type_name_impl!(DeclGroupMaterialTarget);
