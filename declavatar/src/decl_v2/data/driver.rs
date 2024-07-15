use crate::{decl_v2::data::parameter::DeclParameterReference, static_type_name_impl};

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclParameterDrive {
    Group(DeclDriveGroup),
    Switch(DeclDriveSwitch),
    Puppet(DeclDrivePuppet),
    SetInt {
        parameter: DeclParameterReference,
        value: i64,
    },
    SetBool {
        parameter: DeclParameterReference,
        value: Option<bool>,
    },
    SetFloat {
        parameter: DeclParameterReference,
        value: Option<f64>,
    },
    AddInt {
        parameter: DeclParameterReference,
        value: i64,
    },
    AddFloat {
        parameter: DeclParameterReference,
        value: f64,
    },
    RandomInt {
        parameter: DeclParameterReference,
        range: (u8, u8),
    },
    RandomBool {
        parameter: DeclParameterReference,
        value: f64,
    },
    RandomFloat {
        parameter: DeclParameterReference,
        range: (f64, f64),
    },
    Copy {
        from: DeclParameterReference,
        to: DeclParameterReference,
        range: Option<((f64, f64), (f64, f64))>,
    },
}
static_type_name_impl!(DeclParameterDrive);

#[derive(Debug, Clone, PartialEq, Eq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclDriveGroup {
    pub group: String,
    pub option: String,
}
static_type_name_impl!(DeclDriveGroup);

#[derive(Debug, Clone, PartialEq, Eq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclDriveSwitch {
    pub switch: String,
    pub value: Option<bool>,
}
static_type_name_impl!(DeclDriveSwitch);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclDrivePuppet {
    pub puppet: String,
    pub value: Option<f64>,
}
static_type_name_impl!(DeclDrivePuppet);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclTrackingControl {
    pub animation_desired: bool,
    pub targets: Vec<DeclTrackingTarget>,
}
static_type_name_impl!(DeclTrackingControl);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclTrackingTarget {
    Head,
    Hip,
    Eyes,
    Mouth,
    HandLeft,
    HandRight,
    FootLeft,
    FoorRight,
    FingersLeft,
    FingersRight,
}
