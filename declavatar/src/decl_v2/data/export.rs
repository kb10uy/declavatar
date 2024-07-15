use crate::{decl_v2::data::parameter::DeclParameterReference, static_type_name_impl};

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclExports {
    pub exports: Vec<DeclExport>,
}
static_type_name_impl!(DeclExports);

#[derive(Debug, Clone, PartialEq, Eq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclExport {
    Gate(String),
    Guard(String, DeclParameterReference),
}
static_type_name_impl!(DeclExport);
