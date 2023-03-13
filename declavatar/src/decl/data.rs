use semver::Version;

#[derive(Debug, Clone)]
pub struct Document {
    pub version: Version,
    pub avatar: Avatar,
}

#[derive(Debug, Clone)]
pub struct Avatar {
    pub name: String,
    pub parameters_blocks: Vec<Parameters>,
    pub animations_blocks: Vec<Animations>,
    pub drivers_blocks: Vec<Drivers>,
    pub menu_blocks: Vec<Menu>,
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub ty: ParameterType,
    pub save: Option<bool>,
    pub local: Option<bool>,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum ParameterType {
    Int(Option<u8>),
    Float(Option<f64>),
    Bool(Option<bool>),
}

#[derive(Debug, Clone)]
pub struct Animations {
    pub elements: Vec<AnimationElement>,
}

#[derive(Debug, Clone)]
pub enum AnimationElement {
    Group(AnimationGroup),
    Switch(AnimationSwitch),
    Puppet(Puppet),
}

#[derive(Debug, Clone)]
pub struct AnimationGroup {
    pub name: String,
    pub parameter: String,
    pub default_mesh: Option<String>,
    pub preventions: Preventions,
    pub default_block: Option<GroupBlock>,
    pub options: Vec<GroupBlock>,
}

#[derive(Debug, Clone)]
pub struct GroupBlock {
    pub name: Option<String>,
    pub declared_order: usize,
    pub indeterminate: bool,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone)]
pub struct AnimationSwitch {
    pub name: String,
    pub parameter: String,
    pub default_mesh: Option<String>,
    pub preventions: Preventions,
    pub disabled: Vec<Target>,
    pub enabled: Vec<Target>,
}

#[derive(Debug, Clone)]
pub struct Puppet {
    pub name: String,
    pub mesh: Option<String>,
    pub parameter: String,
    pub keyframes: Vec<PuppetKeyframe>,
}

#[derive(Debug, Clone, Default)]
pub struct Preventions {
    pub mouth: Option<bool>,
    pub eyelids: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct PuppetKeyframe {
    pub position: f64,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone)]
pub enum Target {
    Shape {
        shape: String,
        mesh: Option<String>,
        value: Option<f64>,
    },
    Object {
        object: String,
        value: Option<bool>,
    },
    Indeterminate {
        label: String,
        object: Option<String>,
        mesh: Option<String>,
        shape: Option<String>,
        value: Option<DriveTarget>,
    },
}

#[derive(Debug, Clone)]
pub struct Drivers {
    pub groups: Vec<DriverGroup>,
}

#[derive(Debug, Clone)]
pub struct DriverGroup {
    pub name: String,
    pub local: Option<bool>,
    pub drives: Vec<Drive>,
}

#[derive(Debug, Clone)]
pub enum Drive {
    Set(DriveTarget),
    Add(DriveTarget),
    Random {
        group: Option<String>,
        parameter: Option<String>,
        chance: Option<f64>,
        range: (Option<f64>, Option<f64>),
    },
    Copy {
        from: String,
        to: String,
        from_range: (Option<f64>, Option<f64>),
        to_range: (Option<f64>, Option<f64>),
    },
}

#[derive(Debug, Clone)]
pub enum DriveTarget {
    Group {
        name: String,
        option: Option<String>,
    },
    IntParameter {
        name: String,
        value: u8,
    },
    FloatParameter {
        name: String,
        value: f64,
    },
    BoolParameter {
        name: String,
        value: bool,
    },
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub elements: Vec<MenuElement>,
}

#[derive(Debug, Clone)]
pub enum MenuElement {
    SubMenu(SubMenu),
    Boolean(BooleanControl),
    Puppet(PuppetControl),
}

#[derive(Debug, Clone)]
pub struct SubMenu {
    pub name: String,
    pub elements: Vec<MenuElement>,
}

#[derive(Debug, Clone)]
pub struct BooleanControl {
    pub name: String,
    pub toggle: bool,
    pub target: BooleanControlTarget,
}

#[derive(Debug, Clone)]
pub enum BooleanControlTarget {
    Group {
        name: String,
        option: Option<String>,
    },
    IntParameter {
        name: String,
        value: u8,
    },
    BoolParameter {
        name: String,
        value: bool,
    },
}

#[derive(Debug, Clone)]
pub struct PuppetControl {
    pub name: String,
    pub axes: PuppetAxes,
}

#[derive(Debug, Clone)]
pub enum PuppetAxes {
    Radial(String),
    TwoAxis {
        horizontal: (String, (String, String)),
        vertical: (String, (String, String)),
    },
    FourAxis {
        left: (String, String),
        right: (String, String),
        up: (String, String),
        down: (String, String),
    },
}
