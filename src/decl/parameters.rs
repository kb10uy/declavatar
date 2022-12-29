use crate::decl::{
    get_argument, try_get_property, DeclError, DeclNode, DeclNodeExt, Result, VERSION_REQ_SINCE_1_0,
};

use std::collections::HashMap;

use kdl::{KdlNode, KdlValue};
use semver::{Version, VersionReq};

pub const NODE_NAME_PARAMETER: &str = "parameter";
pub const NODE_NAME_INT: &str = "int";
pub const NODE_NAME_FLOAT: &str = "float";
pub const NODE_NAME_BOOL: &str = "bool";

#[derive(Debug, Clone)]
pub struct Parameters {
    parameters: Vec<Parameter>,
}

impl Parameters {
    /// Returns sum of bits consumption in this block.
    pub fn packed_bits_in_block(&self) -> usize {
        self.parameters.iter().map(|p| p.ty.packed_bits()).sum()
    }
}

impl DeclNode for Parameters {
    const NODE_NAME: &'static str = NODE_NAME_PARAMETER;

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let mut parameters = vec![];
        for child in children {
            let child_name = child.name().value();
            let parameter = match child_name {
                NODE_NAME_INT | NODE_NAME_FLOAT | NODE_NAME_BOOL => child.parse_multi(version)?,
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.into())),
            };
            parameters.push(parameter);
        }

        Ok(Parameters { parameters })
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    ty: ParameterType,
    save: bool,
    name: String,
}

impl DeclNode for Parameter {
    const NODE_NAME: &'static str = "";

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(false);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let parameter_name = get_argument(args, 0, "name")?;
        let save = try_get_property(props, "save")?.unwrap_or(false);
        let ty = match name {
            NODE_NAME_INT => {
                let default = try_get_property(props, "default")?.map(|x: i64| x as u8);
                ParameterType::Int(default)
            }
            NODE_NAME_FLOAT => {
                let default = try_get_property(props, "default")?;
                ParameterType::Float(default)
            }
            NODE_NAME_BOOL => {
                let default = try_get_property(props, "default")?;
                ParameterType::Bool(default)
            }
            _ => unreachable!("parameter type already refined here"),
        };

        Ok(Parameter {
            ty,
            save,
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

impl ParameterType {
    pub const fn packed_bits(&self) -> usize {
        match self {
            ParameterType::Int(_) | ParameterType::Float(_) => 8,
            ParameterType::Bool(_) => 1,
        }
    }
}
