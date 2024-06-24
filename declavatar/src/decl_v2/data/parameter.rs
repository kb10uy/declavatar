use crate::static_type_name_impl;

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
    Provided(DeclProvidedParameterKind),
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
