use crate::{
    avatar::{
        compiler::AvatarCompiler,
        data::{Parameter, ParameterSync, ParameterType},
        error::Result,
    },
    compiler::{Compile, Compiler},
    decl::data::{ParameterType as DeclParameterType, Parameters as DeclParameters},
};

use std::collections::HashMap;

impl Compile<Vec<DeclParameters>> for AvatarCompiler {
    type Output = HashMap<String, Parameter>;

    fn compile(
        &mut self,
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
                    self.error(format!(
                        "local parameter '{}' cannot be saved",
                        decl_parameter.name
                    ));
                    continue;
                }
            };

            match parameters.entry(decl_parameter.name.clone()) {
                Entry::Occupied(p) => {
                    let defined: &Parameter = p.get();
                    if defined.value_type != value_type || defined.sync_type != sync_type {
                        self.error(format!(
                            "parameter '{}' have incompatible declarations",
                            decl_parameter.name
                        ));
                        continue;
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
}
