use crate::{
    decl_v2::data::{
        asset::DeclAssets, attachment::DeclAttachments, controller::DeclFxController,
        export::DeclExports, menu::DeclSubMenu, parameter::DeclParameters,
    },
    static_type_name_impl,
};

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclAvatar {
    pub name: String,
    pub exports_blocks: Vec<DeclExports>,
    pub parameters_blocks: Vec<DeclParameters>,
    pub assets_blocks: Vec<DeclAssets>,
    pub fx_controllers: Vec<DeclFxController>,
    pub menu_blocks: Vec<DeclSubMenu>,
    pub attachment_blocks: Vec<DeclAttachments>,
}
static_type_name_impl!(DeclAvatar);
