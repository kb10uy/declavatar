use crate::{decl_sexpr::data::layer::DeclControllerLayer, static_type_name_impl};

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclFxController {
    pub layers: Vec<DeclControllerLayer>,
}
static_type_name_impl!(DeclFxController);
