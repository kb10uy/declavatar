use crate::avatar_v2::data::driver::{ParameterDrive, TrackingControl};

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
        keyframes: Vec<LayerPuppetKeyframe>,
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
    ParameterDrive(ParameterDrive),
    TrackingControl(TrackingControl),
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerRawState {
    pub name: String,
    pub animation: LayerRawAnimation,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum LayerRawAnimation {
    Clip {
        name: LayerAnimation,
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
    External(String),
}

impl Target {
    pub fn driving_key(&self) -> String {
        match self {
            Target::Shape { mesh, shape, .. } => format!("shape://{mesh}/{shape}"),
            Target::Object { object, .. } => format!("object://{object}"),
            Target::Material { mesh, index, .. } => format!("material://{mesh}/{index}"),
            Target::ParameterDrive(pd) => format!("parameter://{}", pd.target_parameter()),
            Target::TrackingControl(tc) => format!("tracking://{:?}", tc.target),
        }
    }
}
