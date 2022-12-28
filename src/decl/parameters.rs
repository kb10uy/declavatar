pub const NODE_NAME_PARAMETER: &str = "parameter";

#[derive(Debug, Clone)]
pub struct Parameter {
    elements: Vec<ParameterElement>,
}

#[derive(Debug, Clone)]
pub struct ParameterElement {
    ty: ParameterType,
    name: String,
}

#[derive(Debug, Clone)]
pub enum ParameterType {
    Int {
        name: String,
        save: bool,
        default: u8,
    },
}
