use crate::{
    avatar::error::{AvatarError, Result},
    decl::parameters::{ParameterType as DeclParameterType, Parameters as DeclParameters},
};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Avatar {
    name: String,
    parameters: HashMap<String, Parameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    name: String,
    value_type: ParameterType,
    sync_type: ParameterSync,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterType {
    Int(u8),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterSync {
    Local,
    Synced(bool),
}

pub fn compile_parameters(
    parameters_blocks: Vec<DeclParameters>,
) -> Result<HashMap<String, Parameter>> {
    use std::collections::hash_map::Entry;

    let mut parameters = HashMap::new();

    let decl_parameters = parameters_blocks
        .into_iter()
        .map(|pb| pb.parameters)
        .flatten();
    for decl_parameter in decl_parameters {
        let name = decl_parameter.name.clone();
        let value_type = match decl_parameter.ty {
            DeclParameterType::Int(dv) => ParameterType::Int(dv.unwrap_or(0)),
            DeclParameterType::Float(dv) => ParameterType::Float(dv.unwrap_or(0.0)),
            DeclParameterType::Bool(dv) => ParameterType::Bool(dv.unwrap_or(false)),
        };
        let sync_type = match (decl_parameter.local, decl_parameter.save) {
            (Some(true), None | Some(false)) => ParameterSync::Local,
            (None | Some(false), None) => ParameterSync::Synced(false),
            (None | Some(false), Some(save)) => ParameterSync::Synced(save),
            (Some(true), Some(true)) => {
                return Err(AvatarError::CannotSaveLocalParameter(decl_parameter.name));
            }
        };

        match parameters.entry(decl_parameter.name) {
            Entry::Occupied(p) => {
                let defined: &Parameter = p.get();
                if defined.value_type != value_type || defined.sync_type != sync_type {
                    return Err(AvatarError::IncompatibleParameterDefinition(name));
                }
            }
            Entry::Vacant(v) => {
                v.insert(Parameter {
                    name,
                    value_type,
                    sync_type,
                });
            }
        }
    }

    Ok(parameters)
}
