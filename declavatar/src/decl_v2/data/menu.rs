use crate::{
    decl_v2::data::driver::{DeclDrivePuppet, DeclParameterDrive},
    static_type_name_impl,
};

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclMenuElement {
    SubMenu(DeclSubMenu),
    Boolean(DeclBooleanControl),
    Puppet(DeclPuppetControl),
}
static_type_name_impl!(DeclMenuElement);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclSubMenu {
    pub name: String,
    pub elements: Vec<DeclMenuElement>,
}
static_type_name_impl!(DeclSubMenu);

#[derive(Debug, Clone, PartialEq)]
pub struct DeclBooleanControl {
    pub name: String,
    pub hold: bool,
    pub parameter_drive: DeclParameterDrive,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclPuppetControl {
    pub name: String,
    pub puppet_type: Box<DeclPuppetType>,
}

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclPuppetAxis {
    pub target: DeclPuppetTarget,
    pub label_positive: Option<String>,
    pub label_negative: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclPuppetType {
    Radial(DeclPuppetAxis),
    TwoAxis {
        horizontal: DeclPuppetAxis,
        vertical: DeclPuppetAxis,
    },
    FourAxis {
        up: DeclPuppetAxis,
        down: DeclPuppetAxis,
        left: DeclPuppetAxis,
        right: DeclPuppetAxis,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclPuppetTarget {
    Puppet(DeclDrivePuppet),
    Parameter(String),
}
