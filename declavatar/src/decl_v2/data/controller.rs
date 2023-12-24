use crate::{decl_v2::data::layer::DeclControllerLayer, static_type_name_impl};

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclFxController {
    pub layers: Vec<DeclControllerLayer>,
}
static_type_name_impl!(DeclFxController);
