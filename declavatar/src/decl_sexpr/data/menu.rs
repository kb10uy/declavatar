use crate::static_type_name_impl;

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

use super::driver::{DeclDriveGroup, DeclDrivePuppet, DeclDriveSwitch};

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
    pub boolean_type: DeclBooleanTarget,
}

#[derive(Debug, Clone)]
pub enum DeclBooleanTarget {
    Group(DeclDriveGroup),
    Switch(DeclDriveSwitch),
    Puppet(DeclDrivePuppet),
}

#[derive(Debug, Clone)]
pub struct DeclPuppetControl {
    pub name: String,
    pub puppet_type: DeclPuppetType,
}

#[derive(Debug, Clone)]
pub enum DeclPuppetTarget {
    Puppet(DeclDrivePuppet),
}

#[derive(Debug, Clone)]
pub enum DeclPuppetType {
    Radial(DeclPuppetTarget),
    TwoAxis {
        horizontal: DeclPuppetTarget,
        vertical: DeclPuppetTarget,
    },
    FourAxis {
        up: DeclPuppetTarget,
        down: DeclPuppetTarget,
        left: DeclPuppetTarget,
        right: DeclPuppetTarget,
    },
}
