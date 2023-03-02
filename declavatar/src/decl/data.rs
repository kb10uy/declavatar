#[derive(Debug, Clone)]
pub struct Animations {
    pub elements: Vec<AnimationElement>,
}

#[derive(Debug, Clone)]
pub enum AnimationElement {
    ShapeGroup(ShapeGroup),
    ShapeSwitch(ShapeSwitch),
    ObjectGroup(ObjectGroup),
    ObjectSwitch(ObjectSwitch),
}

#[derive(Debug, Clone)]
pub struct ShapeGroup {
    pub name: String,
    pub mesh: String,
    pub parameter: String,
    pub prevent_mouth: Option<bool>,
    pub prevent_eyelids: Option<bool>,
    pub default_block: Option<ShapeGroupBlock>,
    pub options: Vec<ShapeGroupBlock>,
}

#[derive(Debug, Clone)]
pub struct ShapeGroupBlock {
    pub name: Option<String>,
    pub declared_order: usize,
    pub shapes: Vec<(String, Option<f64>)>,
}

#[derive(Debug, Clone)]
pub struct ShapeSwitch {
    pub name: String,
    pub mesh: String,
    pub parameter: String,
    pub prevent_mouth: Option<bool>,
    pub prevent_eyelids: Option<bool>,
    pub shapes: Vec<ShapeSwitchPair>,
}

#[derive(Debug, Clone)]
pub struct ShapeSwitchPair {
    pub shape: String,
    pub enabled: Option<f64>,
    pub disabled: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ObjectGroup {
    pub name: String,
    pub parameter: String,
    pub default_block: Option<ObjectGroupBlock>,
    pub options: Vec<ObjectGroupBlock>,
}

#[derive(Debug, Clone)]
pub struct ObjectGroupBlock {
    pub name: Option<String>,
    pub declared_order: usize,
    pub objects: Vec<(String, Option<bool>)>,
}

#[derive(Debug, Clone)]
pub struct ObjectSwitch {
    pub name: String,
    pub parameter: String,
    pub objects: Vec<ObjectSwitchPair>,
}

#[derive(Debug, Clone)]
pub struct ObjectSwitchPair {
    pub object: String,
    pub disabled: Option<bool>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Menu {
    elements: Vec<MenuElement>,
}

#[derive(Debug, Clone)]
pub enum MenuElement {
    SubMenu(SubMenu),
    Boolean(BooleanControl),
    Puppet(Puppet),
}

#[derive(Debug, Clone)]
pub struct SubMenu {
    name: String,
    elements: Vec<MenuElement>,
}

#[derive(Debug, Clone)]
pub struct BooleanControl {
    name: String,
    toggle: bool,
    target: BooleanControlTarget,
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
pub struct Puppet {
    name: String,
    axes: PuppetAxes,
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
