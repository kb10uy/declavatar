use crate::{
    avatar_v2::data::driver::{ParameterDrive, TrackingControl},
    decl_v2::data::layer::DeclMaterialValue,
};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Layer {
    pub name: String,
    pub content: LayerContent,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum LayerContent {
    Group {
        parameter: String,
        default: LayerGroupOption,
        options: Vec<LayerGroupOption>,
    },
    Switch {
        parameter: String,
        disabled: LayerAnimation,
        enabled: LayerAnimation,
    },
    Puppet {
        parameter: String,
        animation: LayerAnimation,
    },
    SwitchGate {
        gate: String,
        disabled: LayerAnimation,
        enabled: LayerAnimation,
    },
    Raw {
        default_index: usize,
        states: Vec<LayerRawState>,
        transitions: Vec<LayerRawTransition>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerGroupOption {
    pub name: String,
    pub value: usize,
    pub animation: LayerAnimation,
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerPuppetKeyframe {
    pub value: f64,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum Target {
    Shape {
        mesh: String,
        shape: String,
        value: f64,
    },
    Object {
        object: String,
        value: bool,
    },
    Material {
        mesh: String,
        index: usize,
        asset: String,
    },
    MaterialProperty {
        mesh: String,
        property: String,
        value: MaterialValue,
    },
    ParameterDrive(ParameterDrive),
    TrackingControl(TrackingControl),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum MaterialValue {
    Float(f64),
    VectorRgba([f64; 4]),
    VectorXyzw([f64; 4]),
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerRawState {
    pub name: String,
    pub animation: LayerRawAnimationKind,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum LayerRawAnimationKind {
    Clip {
        animation: LayerAnimation,
        speed: Option<f64>,
        speed_by: Option<String>,
        time_by: Option<String>,
    },
    BlendTree {
        blend_type: LayerRawBlendTreeType,
        params: Vec<String>,
        fields: Vec<LayerRawField>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum LayerRawBlendTreeType {
    Linear,
    Simple2D,
    Freeform2D,
    Cartesian2D,
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerRawField {
    pub animation: LayerAnimation,
    pub position: [f64; 2],
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerRawTransition {
    pub from_index: usize,
    pub target_index: usize,
    pub duration: f64,
    pub conditions: Vec<LayerRawCondition>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum LayerRawCondition {
    Be(String),
    Not(String),
    EqInt(String, i64),
    NeqInt(String, i64),
    GtInt(String, i64),
    LeInt(String, i64),
    GtFloat(String, f64),
    LeFloat(String, f64),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum LayerAnimation {
    Inline(Vec<Target>),
    KeyedInline(Vec<LayerPuppetKeyframe>),
    External(String),
}

impl Target {
    pub fn clone_as_zeroed(&self) -> Option<Target> {
        match self {
            Target::Shape { mesh, shape, .. } => Some(Target::Shape {
                mesh: mesh.clone(),
                shape: shape.clone(),
                value: 0.0,
            }),
            Target::Object { object, .. } => Some(Target::Object {
                object: object.clone(),
                value: false,
            }),
            _ => None,
        }
    }

    pub fn driving_key(&self) -> String {
        match self {
            Target::Shape { mesh, shape, .. } => format!("shape://{mesh}/{shape}"),
            Target::Object { object, .. } => format!("object://{object}"),
            Target::Material { mesh, index, .. } => format!("material://{mesh}/{index}"),
            Target::MaterialProperty { mesh, property, .. } => {
                format!("material+prop://{mesh}/{property}")
            }
            Target::ParameterDrive(pd) => format!("parameter://{}", pd.target_parameter()),
            Target::TrackingControl(tc) => format!("tracking://{:?}", tc.target),
        }
    }
}

impl From<DeclMaterialValue> for MaterialValue {
    fn from(value: DeclMaterialValue) -> Self {
        match value {
            DeclMaterialValue::Float(v) => MaterialValue::Float(v),
            DeclMaterialValue::Color(v) => MaterialValue::VectorRgba(v),
            DeclMaterialValue::ColorHdr(v) => MaterialValue::VectorXyzw(v),
            DeclMaterialValue::Vector(v) => MaterialValue::VectorXyzw(v),
        }
    }
}

impl Default for LayerAnimation {
    fn default() -> Self {
        LayerAnimation::Inline(vec![])
    }
}
