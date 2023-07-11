use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Avatar {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub assets: Vec<Asset>,
    pub animation_groups: Vec<AnimationGroup>,
    pub driver_groups: Vec<DriverGroup>,
    pub top_menu_group: MenuGroup,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Parameter {
    pub name: String,
    pub value_type: ParameterType,
    pub scope: ParameterScope,
}

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

    pub const fn type_name(&self) -> &'static str {
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Asset {
    pub asset_type: AssetType,
    pub key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum AssetType {
    Material,
    Animation,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnimationGroup {
    pub name: String,
    pub content: AnimationGroupContent,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum AnimationGroupContent {
    Group {
        parameter: String,
        preventions: Preventions,
        default_targets: Vec<Target>,
        options: Vec<GroupOption>,
    },
    Switch {
        parameter: String,
        preventions: Preventions,
        disabled: Vec<Target>,
        enabled: Vec<Target>,
    },
    Puppet {
        parameter: String,
        keyframes: Vec<PuppetKeyframe>,
    },
    Layer {
        default_index: usize,
        states: Vec<LayerState>,
    },
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Preventions {
    pub mouth: bool,
    pub eyelids: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupOption {
    pub name: String,
    pub order: usize,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum Target {
    Shape(ShapeTarget),
    Object(ObjectTarget),
    Material(MaterialTarget),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShapeTarget {
    pub mesh: String,
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ObjectTarget {
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MaterialTarget {
    pub mesh: String,
    pub slot: usize,
    pub asset_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PuppetKeyframe {
    pub position: f64,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LayerState {
    pub name: String,
    pub animation: LayerAnimation,
    pub speed: (Option<f64>, Option<String>),
    pub time: Option<String>,
    pub transitions: Vec<LayerTransition>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LayerAnimation {
    Clip(String),
    BlendTree(LayerBlendTreeType, Vec<LayerBlendTreeField>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum LayerBlendTreeType {
    Linear,
    Simple2D,
    Freeform2D,
    Cartesian2D,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LayerBlendTreeField {
    pub clip: String,
    pub position: [f64; 2],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LayerTransition {
    pub conditions: Vec<LayerCondition>,
    pub duration: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LayerCondition {
    Be(String),
    Not(String),
    EqInt(String, i64),
    NeqInt(String, i64),
    GtInt(String, i64),
    LeInt(String, i64),
    GtFloat(String, f64),
    LeFloat(String, f64),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DriverGroup {
    pub name: String,
    pub local: bool,
    pub drivers: Vec<Driver>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum Driver {
    SetInt(String, u8),
    SetFloat(String, f64),
    SetBool(String, bool),
    AddInt(String, u8),
    AddFloat(String, f64),
    RandomInt(String, (u8, u8)),
    RandomFloat(String, (f64, f64)),
    RandomBool(String, f64),
    Copy(String, String),
    RangedCopy(String, String, (f64, f64), (f64, f64)),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum MenuItem {
    SubMenu(MenuGroup),
    Button(MenuBoolean),
    Toggle(MenuBoolean),
    Radial(MenuRadial),
    TwoAxis(MenuTwoAxis),
    FourAxis(MenuFourAxis),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuGroup {
    pub name: String,
    pub id: usize,
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuBoolean {
    pub name: String,
    pub parameter: String,
    pub value: ParameterType,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuRadial {
    pub name: String,
    pub parameter: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuTwoAxis {
    pub name: String,
    pub horizontal_axis: BiAxis,
    pub vertical_axis: BiAxis,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MenuFourAxis {
    pub name: String,
    pub left_axis: UniAxis,
    pub right_axis: UniAxis,
    pub up_axis: UniAxis,
    pub down_axis: UniAxis,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BiAxis {
    pub parameter: String,
    pub label_negative: String,
    pub label_positive: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UniAxis {
    pub parameter: String,
    pub label: String,
}

impl Target {
    pub fn index(&self) -> String {
        match self {
            Target::Shape(ShapeTarget { mesh, name, .. }) => format!("s-{mesh}-{name}"),
            Target::Object(ObjectTarget { name, .. }) => format!("o-{name}"),
            Target::Material(MaterialTarget { mesh, slot, .. }) => format!("m-{mesh}-{slot}"),
        }
    }

    pub fn clone_as_disabled(&self) -> Option<Target> {
        match self {
            Target::Shape(ShapeTarget { mesh, name, .. }) => Some(Target::Shape(ShapeTarget {
                mesh: mesh.clone(),
                name: name.clone(),
                value: 0.0,
            })),
            Target::Object(ObjectTarget { name, .. }) => Some(Target::Object(ObjectTarget {
                name: name.clone(),
                enabled: false,
            })),
            Target::Material(_) => None,
        }
    }
}
