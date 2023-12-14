use crate::{
    decl_v2::data::driver::{DeclParameterDrive, DeclTrackingControl},
    static_type_name_impl,
};

use ketos::{ForeignValue, FromValue, FromValueRef, IntoValue};

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclControllerLayer {
    Group(DeclGroupLayer),
    Switch(DeclSwitchLayer),
    Puppet(DeclPuppetLayer),
    Raw(DeclRawLayer),
}
static_type_name_impl!(DeclControllerLayer);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupLayer {
    pub name: String,
    pub driven_by: String,
    pub default_mesh: Option<String>,
    pub default: Option<DeclGroupOption>,
    pub options: Vec<DeclGroupOption>,
}
static_type_name_impl!(DeclGroupLayer);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupOption {
    pub kind: DeclGroupOptionKind,
    pub targets: Vec<DeclGroupOptionTarget>,
}
static_type_name_impl!(DeclGroupOption);

#[derive(Debug, Clone, PartialEq)]
pub enum DeclGroupOptionKind {
    Boolean(bool),
    Selection(Option<String>, Option<usize>),
    Keyframe(f64),
}

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclGroupOptionTarget {
    Shape(DeclGroupShapeTarget),
    Object(DeclGroupObjectTarget),
    Material(DeclGroupMaterialTarget),
    ParameterDrive(DeclParameterDrive),
    TrackingControl(DeclTrackingControl),
}
static_type_name_impl!(DeclGroupOptionTarget);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupShapeTarget {
    pub shape: String,
    pub value: Option<f64>,
    pub mesh: Option<String>,
}
static_type_name_impl!(DeclGroupShapeTarget);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupObjectTarget {
    pub object: String,
    pub value: Option<bool>,
}
static_type_name_impl!(DeclGroupObjectTarget);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclGroupMaterialTarget {
    pub index: usize,
    pub value: String,
    pub mesh: Option<String>,
}
static_type_name_impl!(DeclGroupMaterialTarget);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclSwitchLayer {
    pub name: String,
    pub driven_by: String,
    pub default_mesh: Option<String>,
    pub disabled: DeclGroupOption,
    pub enabled: DeclGroupOption,
}
static_type_name_impl!(DeclSwitchLayer);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclPuppetLayer {
    pub name: String,
    pub driven_by: String,
    pub default_mesh: Option<String>,
    pub keyframes: Vec<DeclGroupOption>,
}
static_type_name_impl!(DeclPuppetLayer);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclRawLayer {
    pub name: String,
    pub default: Option<String>,
    pub states: Vec<DeclRawLayerState>,
}
static_type_name_impl!(DeclRawLayer);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclRawLayerState {
    pub name: String,
    pub animation: DeclRawLayerAnimation,
    pub transitions: Vec<DeclRawLayerTransition>,
}
static_type_name_impl!(DeclRawLayerState);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub enum DeclRawLayerAnimation {
    Clip {
        name: String,
        speed: (Option<f64>, Option<String>),
        time: Option<String>,
    },
    BlendTree {
        tree_type: DeclRawLayerBlendTreeType,
        fields: Vec<DeclRawLayerBlendTreeField>,
    },
}
static_type_name_impl!(DeclRawLayerAnimation);

#[derive(Debug, Clone)]
pub enum DeclRawLayerBlendTreeType {
    Linear(String),
    Simple2D(String, String),
    Freeform2D(String, String),
    Cartesian2D(String, String),
}

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclRawLayerBlendTreeField {
    pub name: String,
    pub values: [f64; 2],
}
static_type_name_impl!(DeclRawLayerBlendTreeField);

#[derive(Debug, Clone, ForeignValue, FromValue, FromValueRef, IntoValue)]
pub struct DeclRawLayerTransition {
    pub target: String,
    pub duration: Option<f64>,
    pub and_terms: Vec<DeclRawLayerTransitionCondition>,
}
static_type_name_impl!(DeclRawLayerTransition);

#[derive(Debug, Clone)]
pub enum DeclRawLayerTransitionCondition {
    Bool(String, bool),
    Int(String, DeclRawLayerTransitionOrdering, i64),
    Float(String, DeclRawLayerTransitionOrdering, f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclRawLayerTransitionOrdering {
    Equal,
    NotEqual,
    Greater,
    Lesser,
}

impl DeclGroupOptionKind {
    pub fn as_all_selection(&self) -> Option<(Option<String>, Option<usize>)> {
        let DeclGroupOptionKind::Selection(name, value) = self else {
            return None;
        };
        Some((name.clone(), *value))
    }

    pub fn as_selection(&self) -> Option<(String, Option<usize>)> {
        let DeclGroupOptionKind::Selection(Some(name), value) = self else {
            return None;
        };
        Some((name.clone(), *value))
    }

    pub fn as_boolean(&self) -> Option<bool> {
        let DeclGroupOptionKind::Boolean(value) = self else {
            return None;
        };
        Some(*value)
    }

    pub fn as_keyframe(&self) -> Option<f64> {
        let DeclGroupOptionKind::Keyframe(value) = self else {
            return None;
        };
        Some(*value)
    }
}
