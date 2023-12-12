use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Layer {
    pub name: String,
    pub content: LayerContent,
}

#[derive(Debug, Clone, Serialize)]
pub enum LayerContent {
    Group {
        parameter: String,
        default: LayerGroupOption,
        options: Vec<LayerGroupOption>,
    },
    Switch {
        parameter: String,
    },
    Puppet {
        parameter: String,
    },
    Raw {},
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerGroupOption {
    pub name: String,
    pub value: usize,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerPuppetOption {
    pub name: String,
    pub value: usize,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize)]
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
}
