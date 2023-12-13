use crate::static_type_name_impl;

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclAssets {
    pub assets: Vec<DeclAsset>,
}
static_type_name_impl!(DeclAssets);

#[derive(Debug, Clone, PartialEq, Eq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclAsset {
    Material(String),
    Animation(String),
}
static_type_name_impl!(DeclAsset);
