use crate::static_type_name_impl;

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclMenuElement {
    SubMenu(DeclSubMenu),
    Boolean(DeclBooleanControl),
    Puppet(DeclPuppetControl),
}
static_type_name_impl!(DeclMenuElement);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclSubMenu {
    pub name: String,
    pub elements: Vec<DeclMenuElement>,
}
static_type_name_impl!(DeclSubMenu);

#[derive(Debug, Clone)]
pub struct DeclBooleanControl {
    pub name: String,
    pub hold: bool,
}

#[derive(Debug, Clone)]
pub struct DeclPuppetControl {
    pub name: String,
    pub puppet_type: DeclPuppetType,
}

#[derive(Debug, Clone)]
pub enum DeclPuppetType {
    Radial(),
    TwoAxis(),
    FourAxis(),
}
