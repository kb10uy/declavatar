use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Layer {
    pub name: String,
    pub content: LayerContent,
}

#[derive(Debug, Clone, Serialize)]
pub enum LayerContent {
    Group {
        name: String,
        parameter: String,
        default: LayerGroupOption,
        options: Vec<LayerGroupOption>,
    },
    Switch {
        name: String,
        parameter: String,
    },
    Puppet {
        name: String,
        parameter: String,
    },
    Raw {
        name: String,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerGroupOption {
    pub name: String,
    pub value: usize,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Target {}
