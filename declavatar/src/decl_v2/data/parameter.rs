use crate::static_type_name_impl;

use std::str::FromStr;

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclParameters {
    pub parameters: Vec<DeclParameter>,
}
static_type_name_impl!(DeclParameters);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclParameter {
    Primitive(DeclPrimitiveParameter),
    PhysBone(DeclPhysBoneParameter),
    Provided(Vec<DeclProvidedParameterKind>),
}
static_type_name_impl!(DeclParameter);

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclPrimitiveParameter {
    pub ty: DeclPrimitiveParameterType,
    pub name: String,
    pub scope: Option<DeclPrimitiveParameterScope>,
    pub save: Option<bool>,
    pub unique: Option<bool>,
}
static_type_name_impl!(DeclPrimitiveParameter);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeclPrimitiveParameterType {
    Int(Option<u8>),
    Float(Option<f64>),
    Bool(Option<bool>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclPrimitiveParameterScope {
    Internal,
    Local,
    Synced,
}

#[derive(Debug, Clone, PartialEq, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclPhysBoneParameter {
    pub prefix: String,
}
static_type_name_impl!(DeclPhysBoneParameter);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclProvidedParameterKind {
    IsLocal,
    Viseme,
    Voice,
    GestureLeft,
    GestureRight,
    GestureLeftWeight,
    GestureRightWeight,
    AngularY,
    VelocityX,
    VelocityY,
    VelocityZ,
    VelocityMagnitude,
    Upright,
    Grounded,
    Seated,
    Afk,
    TrackingType,
    VrMode,
    MuteSelf,
    InStation,
    Earmuffs,
    IsOnFriendsList,
    AvatarVersion,
    ScaleModified,
    ScaleFactor,
    ScaleFactorInverse,
    EyeHeightAsMeters,
    EyeHeightAsPercent,
}

impl FromStr for DeclProvidedParameterKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "is-local" => DeclProvidedParameterKind::IsLocal,
            "viseme" => DeclProvidedParameterKind::Viseme,
            "voice" => DeclProvidedParameterKind::Voice,
            "gesture-left" => DeclProvidedParameterKind::GestureLeft,
            "gesture-right" => DeclProvidedParameterKind::GestureRight,
            "gesture-left-weight" => DeclProvidedParameterKind::GestureLeftWeight,
            "gesture-right-weight" => DeclProvidedParameterKind::GestureRightWeight,
            "angular-y" => DeclProvidedParameterKind::AngularY,
            "velocity-x" => DeclProvidedParameterKind::VelocityX,
            "velocity-y" => DeclProvidedParameterKind::VelocityY,
            "velocity-z" => DeclProvidedParameterKind::VelocityZ,
            "velocity-magnitude" => DeclProvidedParameterKind::VelocityMagnitude,
            "upright" => DeclProvidedParameterKind::Upright,
            "grounded" => DeclProvidedParameterKind::Grounded,
            "seated" => DeclProvidedParameterKind::Seated,
            "afk" => DeclProvidedParameterKind::Afk,
            "tracking-type" => DeclProvidedParameterKind::TrackingType,
            "vr-mode" => DeclProvidedParameterKind::VrMode,
            "mute-self" => DeclProvidedParameterKind::MuteSelf,
            "in-station" => DeclProvidedParameterKind::InStation,
            "earmuffs" => DeclProvidedParameterKind::Earmuffs,
            "is-on-friends-list" => DeclProvidedParameterKind::IsOnFriendsList,
            "avatar-version" => DeclProvidedParameterKind::AvatarVersion,
            "scale-modified" => DeclProvidedParameterKind::ScaleModified,
            "scale-factor" => DeclProvidedParameterKind::ScaleFactor,
            "scale-factor-inverse" => DeclProvidedParameterKind::ScaleFactorInverse,
            "eye-height-as-meters" => DeclProvidedParameterKind::EyeHeightAsMeters,
            "eye-height-as-percent" => DeclProvidedParameterKind::EyeHeightAsPercent,
            _ => return Err(s.to_string()),
        };
        Ok(kind)
    }
}
