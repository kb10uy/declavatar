use crate::decl_v2::data::parameter::{DeclParameterReference, DeclPhysBoneParameterKind, DeclProvidedParameterKind};

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(tag = "type", content = "default")]
pub enum ParameterType {
    Int(u8),
    Float(f64),
    Bool(bool),
}

impl ParameterType {
    pub const INT_TYPE: ParameterType = ParameterType::Int(0);
    pub const FLOAT_TYPE: ParameterType = ParameterType::Float(0.0);
    pub const BOOL_TYPE: ParameterType = ParameterType::Bool(false);

    pub fn matches(self, requirement: ParameterType) -> bool {
        matches!(
            (self, requirement),
            (ParameterType::Int(_), ParameterType::Int(_))
                | (ParameterType::Float(_), ParameterType::Float(_))
                | (ParameterType::Bool(_), ParameterType::Bool(_))
        )
    }

    pub const fn type_name(self) -> &'static str {
        match self {
            ParameterType::Int(_) => "int",
            ParameterType::Float(_) => "float",
            ParameterType::Bool(_) => "bool",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "type", content = "save")]
pub enum ParameterScope {
    Internal,
    Local(bool),
    Synced(bool),
}

impl ParameterScope {
    pub const MAYBE_INTERNAL: ParameterScope = ParameterScope::Internal;
    pub const MUST_EXPOSE: ParameterScope = ParameterScope::Local(false);
    pub const MUST_SYNC: ParameterScope = ParameterScope::Synced(false);

    pub const fn suitable_for(self, requirement: ParameterScope) -> bool {
        matches!(
            (requirement, self),
            (ParameterScope::Internal, _)
                | (
                    ParameterScope::Local(_),
                    ParameterScope::Local(_) | ParameterScope::Synced(_)
                )
                | (ParameterScope::Synced(_), ParameterScope::Synced(_))
        )
    }

    pub const fn name(self) -> &'static str {
        match self {
            ParameterScope::Internal => "internal",
            ParameterScope::Local(_) => "local",
            ParameterScope::Synced(_) => "synced",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeclaredParameter {
    pub name: String,
    pub value_type: ParameterType,
    pub scope: ParameterScope,
    pub unique: bool,
    pub explicit_default: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "value")]
pub enum ProvidedParameter {
    PhysBone(String),
    Vrchat(VrchatParameterKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum VrchatParameterKind {
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

impl VrchatParameterKind {
    pub fn value_type(&self) -> ParameterType {
        match self {
            VrchatParameterKind::IsLocal => ParameterType::BOOL_TYPE,
            VrchatParameterKind::Viseme => ParameterType::INT_TYPE,
            VrchatParameterKind::Voice => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::GestureLeft => ParameterType::INT_TYPE,
            VrchatParameterKind::GestureRight => ParameterType::INT_TYPE,
            VrchatParameterKind::GestureLeftWeight => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::GestureRightWeight => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::AngularY => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::VelocityX => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::VelocityY => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::VelocityZ => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::VelocityMagnitude => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::Upright => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::Grounded => ParameterType::BOOL_TYPE,
            VrchatParameterKind::Seated => ParameterType::BOOL_TYPE,
            VrchatParameterKind::Afk => ParameterType::BOOL_TYPE,
            VrchatParameterKind::TrackingType => ParameterType::INT_TYPE,
            VrchatParameterKind::VrMode => ParameterType::INT_TYPE,
            VrchatParameterKind::MuteSelf => ParameterType::BOOL_TYPE,
            VrchatParameterKind::InStation => ParameterType::BOOL_TYPE,
            VrchatParameterKind::Earmuffs => ParameterType::BOOL_TYPE,
            VrchatParameterKind::IsOnFriendsList => ParameterType::BOOL_TYPE,
            VrchatParameterKind::AvatarVersion => ParameterType::INT_TYPE,
            VrchatParameterKind::ScaleModified => ParameterType::BOOL_TYPE,
            VrchatParameterKind::ScaleFactor => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::ScaleFactorInverse => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::EyeHeightAsMeters => ParameterType::FLOAT_TYPE,
            VrchatParameterKind::EyeHeightAsPercent => ParameterType::FLOAT_TYPE,
        }
    }

    pub fn parameter_name(&self) -> &'static str {
        match self {
            VrchatParameterKind::IsLocal => "IsLocal",
            VrchatParameterKind::Viseme => "Viseme",
            VrchatParameterKind::Voice => "Voice",
            VrchatParameterKind::GestureLeft => "GestureLeft",
            VrchatParameterKind::GestureRight => "GestureRight",
            VrchatParameterKind::GestureLeftWeight => "GestureLeftWeight",
            VrchatParameterKind::GestureRightWeight => "GestureRightWeight",
            VrchatParameterKind::AngularY => "AngularY",
            VrchatParameterKind::VelocityX => "VelocityX",
            VrchatParameterKind::VelocityY => "VelocityY",
            VrchatParameterKind::VelocityZ => "VelocityZ",
            VrchatParameterKind::VelocityMagnitude => "VelocityMagnitude",
            VrchatParameterKind::Upright => "Upright",
            VrchatParameterKind::Grounded => "Grounded",
            VrchatParameterKind::Seated => "Seated",
            VrchatParameterKind::Afk => "AFK",
            VrchatParameterKind::TrackingType => "TrackingType",
            VrchatParameterKind::VrMode => "VRMode",
            VrchatParameterKind::MuteSelf => "MuteSelf",
            VrchatParameterKind::InStation => "InStation",
            VrchatParameterKind::Earmuffs => "Earmuffs",
            VrchatParameterKind::IsOnFriendsList => "IsOnFriendsList",
            VrchatParameterKind::AvatarVersion => "AvatarVersion",
            VrchatParameterKind::ScaleModified => "ScaleModified",
            VrchatParameterKind::ScaleFactor => "ScaleFactor",
            VrchatParameterKind::ScaleFactorInverse => "ScaleFactorInverse",
            VrchatParameterKind::EyeHeightAsMeters => "EyeHeightAsMeters",
            VrchatParameterKind::EyeHeightAsPercent => "EyeHeightAsPercent",
        }
    }
}

impl From<DeclProvidedParameterKind> for VrchatParameterKind {
    fn from(value: DeclProvidedParameterKind) -> Self {
        match value {
            DeclProvidedParameterKind::IsLocal => VrchatParameterKind::IsLocal,
            DeclProvidedParameterKind::Viseme => VrchatParameterKind::Viseme,
            DeclProvidedParameterKind::Voice => VrchatParameterKind::Voice,
            DeclProvidedParameterKind::GestureLeft => VrchatParameterKind::GestureLeft,
            DeclProvidedParameterKind::GestureRight => VrchatParameterKind::GestureRight,
            DeclProvidedParameterKind::GestureLeftWeight => VrchatParameterKind::GestureLeftWeight,
            DeclProvidedParameterKind::GestureRightWeight => VrchatParameterKind::GestureRightWeight,
            DeclProvidedParameterKind::AngularY => VrchatParameterKind::AngularY,
            DeclProvidedParameterKind::VelocityX => VrchatParameterKind::VelocityX,
            DeclProvidedParameterKind::VelocityY => VrchatParameterKind::VelocityY,
            DeclProvidedParameterKind::VelocityZ => VrchatParameterKind::VelocityZ,
            DeclProvidedParameterKind::VelocityMagnitude => VrchatParameterKind::VelocityMagnitude,
            DeclProvidedParameterKind::Upright => VrchatParameterKind::Upright,
            DeclProvidedParameterKind::Grounded => VrchatParameterKind::Grounded,
            DeclProvidedParameterKind::Seated => VrchatParameterKind::Seated,
            DeclProvidedParameterKind::Afk => VrchatParameterKind::Afk,
            DeclProvidedParameterKind::TrackingType => VrchatParameterKind::TrackingType,
            DeclProvidedParameterKind::VrMode => VrchatParameterKind::VrMode,
            DeclProvidedParameterKind::MuteSelf => VrchatParameterKind::MuteSelf,
            DeclProvidedParameterKind::InStation => VrchatParameterKind::InStation,
            DeclProvidedParameterKind::Earmuffs => VrchatParameterKind::Earmuffs,
            DeclProvidedParameterKind::IsOnFriendsList => VrchatParameterKind::IsOnFriendsList,
            DeclProvidedParameterKind::AvatarVersion => VrchatParameterKind::AvatarVersion,
            DeclProvidedParameterKind::ScaleModified => VrchatParameterKind::ScaleModified,
            DeclProvidedParameterKind::ScaleFactor => VrchatParameterKind::ScaleFactor,
            DeclProvidedParameterKind::ScaleFactorInverse => VrchatParameterKind::ScaleFactorInverse,
            DeclProvidedParameterKind::EyeHeightAsMeters => VrchatParameterKind::EyeHeightAsMeters,
            DeclProvidedParameterKind::EyeHeightAsPercent => VrchatParameterKind::EyeHeightAsPercent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysBoneParameterKind {
    IsGrabbed,
    IsPosed,
    Angle,
    Stretch,
    Squish,
}

impl PhysBoneParameterKind {
    pub fn value_type(&self) -> ParameterType {
        match self {
            PhysBoneParameterKind::IsGrabbed => ParameterType::BOOL_TYPE,
            PhysBoneParameterKind::IsPosed => ParameterType::BOOL_TYPE,
            PhysBoneParameterKind::Angle => ParameterType::FLOAT_TYPE,
            PhysBoneParameterKind::Stretch => ParameterType::FLOAT_TYPE,
            PhysBoneParameterKind::Squish => ParameterType::FLOAT_TYPE,
        }
    }

    pub fn parameter_suffix(&self) -> &'static str {
        match self {
            PhysBoneParameterKind::IsGrabbed => "_IsGrabbed",
            PhysBoneParameterKind::IsPosed => "_IsPosed",
            PhysBoneParameterKind::Angle => "_Angle",
            PhysBoneParameterKind::Stretch => "_Stretch",
            PhysBoneParameterKind::Squish => "_Squish",
        }
    }
}

impl From<DeclPhysBoneParameterKind> for PhysBoneParameterKind {
    fn from(value: DeclPhysBoneParameterKind) -> Self {
        match value {
            DeclPhysBoneParameterKind::IsGrabbed => PhysBoneParameterKind::IsGrabbed,
            DeclPhysBoneParameterKind::IsPosed => PhysBoneParameterKind::IsPosed,
            DeclPhysBoneParameterKind::Angle => PhysBoneParameterKind::Angle,
            DeclPhysBoneParameterKind::Stretch => PhysBoneParameterKind::Stretch,
            DeclPhysBoneParameterKind::Squish => PhysBoneParameterKind::Squish,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    Declared(DeclaredParameter),
    Provided(ProvidedParameter),
}

impl Parameter {
    pub fn basename(&self) -> &str {
        match self {
            Parameter::Declared(declared) => &declared.name,
            Parameter::Provided(ProvidedParameter::Vrchat(vrc_param)) => vrc_param.parameter_name(),
            Parameter::Provided(ProvidedParameter::PhysBone(prefix)) => prefix,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedParameter {
    pub name: String,
    pub value_type: ParameterType,
    pub unique: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterQuery {
    Declared(String),
    PhysBone(String, PhysBoneParameterKind),
    Vrchat(VrchatParameterKind),
}

impl ParameterQuery {
    pub fn qualify_match(&self, parameter: &Parameter) -> Option<QualifiedParameter> {
        match (self, parameter) {
            (ParameterQuery::Declared(qn), Parameter::Declared(pn)) if qn == &pn.name => Some(QualifiedParameter {
                name: pn.name.clone(),
                value_type: pn.value_type,
                unique: pn.unique,
            }),
            (ParameterQuery::Vrchat(qk), Parameter::Provided(ProvidedParameter::Vrchat(pk))) if qk == pk => {
                Some(QualifiedParameter {
                    name: pk.parameter_name().to_string(),
                    value_type: pk.value_type(),
                    unique: false,
                })
            }
            (ParameterQuery::PhysBone(qp, kind), Parameter::Provided(ProvidedParameter::PhysBone(pp))) if qp == pp => {
                Some(QualifiedParameter {
                    name: format!("{pp}{}", kind.parameter_suffix()),
                    value_type: kind.value_type(),
                    unique: false,
                })
            }
            _ => None,
        }
    }

    pub fn querying_name(&self) -> &str {
        match self {
            ParameterQuery::Declared(name) => name,
            ParameterQuery::Vrchat(kind) => kind.parameter_name(),
            ParameterQuery::PhysBone(prefix, _) => prefix,
        }
    }
}

impl From<DeclParameterReference> for ParameterQuery {
    fn from(value: DeclParameterReference) -> Self {
        match value {
            DeclParameterReference::Primitive(name) => ParameterQuery::Declared(name),
            DeclParameterReference::PhysBone(name, kind) => ParameterQuery::PhysBone(name, kind.into()),
            DeclParameterReference::Provided(kind) => ParameterQuery::Vrchat(kind.into()),
        }
    }
}
