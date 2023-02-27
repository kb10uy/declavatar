use crate::decl::{deconstruct_node, DeclError, DeclErrorKind, Result};

use kdl::KdlNode;

pub const NODE_NAME_PARAMETERS: &str = "parameters";
pub const NODE_NAME_INT: &str = "int";
pub const NODE_NAME_FLOAT: &str = "float";
pub const NODE_NAME_BOOL: &str = "bool";

#[derive(Debug, Clone)]
pub struct Parameters {
    pub parameters: Vec<Parameter>,
}

impl Parameters {
    pub fn parse(node: &KdlNode) -> Result<Self> {
        let (_, _, children) = deconstruct_node(node, Some(NODE_NAME_PARAMETERS), Some(true))?;

        let mut parameters = vec![];
        for child in children {
            let child_name = child.name().value();
            let parameter = match child_name {
                NODE_NAME_INT | NODE_NAME_FLOAT | NODE_NAME_BOOL => Parameter::parse(child)?,
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            };
            parameters.push(parameter);
        }

        Ok(Parameters { parameters })
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub ty: ParameterType,
    pub save: Option<bool>,
    pub local: Option<bool>,
    pub name: String,
}

impl Parameter {
    pub fn parse(node: &KdlNode) -> Result<Self> {
        let (name, entries, _) = deconstruct_node(node, None, Some(false))?;

        let parameter_name = entries.get_argument(0, "name")?;
        let save = entries.try_get_property("save")?;
        let local = entries.try_get_property("local")?;
        let ty = match name {
            NODE_NAME_INT => {
                let default = entries.try_get_property("default")?.map(|x: i64| x as u8);
                ParameterType::Int(default)
            }
            NODE_NAME_FLOAT => {
                let default = entries.try_get_property("default")?;
                ParameterType::Float(default)
            }
            NODE_NAME_BOOL => {
                let default = entries.try_get_property("default")?;
                ParameterType::Bool(default)
            }
            _ => unreachable!("parameter type already refined here"),
        };

        Ok(Parameter {
            ty,
            save,
            local,
            name: parameter_name,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ParameterType {
    Int(Option<u8>),
    Float(Option<f64>),
    Bool(Option<bool>),
}
