use crate::{
    decl_sexpr::data::{asset::DeclAssets, menu::DeclSubMenu, parameter::DeclParameters},
    static_type_name_impl,
};

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclAvatar {
    pub name: String,
    pub parameters_blocks: Vec<DeclParameters>,
    pub assets_blocks: Vec<DeclAssets>,
    pub menu_blocks: Vec<DeclSubMenu>,
}
static_type_name_impl!(DeclAvatar);
