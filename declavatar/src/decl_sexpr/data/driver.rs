use crate::static_type_name_impl;

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclParameterDrive {
    Group(DeclDriveGroup),
    Switch(DeclDriveSwitch),
    Puppet(DeclDrivePuppet),
}

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclDriveGroup {
    pub group: String,
    pub option: String,
}
static_type_name_impl!(DeclDriveGroup);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclDriveSwitch {
    pub switch: String,
    pub value: Option<bool>,
}
static_type_name_impl!(DeclDriveSwitch);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclDrivePuppet {
    pub puppet: String,
    pub value: Option<f64>,
}
static_type_name_impl!(DeclDrivePuppet);
