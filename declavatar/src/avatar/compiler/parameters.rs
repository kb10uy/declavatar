use crate::{
    avatar::{
        compiler::AvatarCompiler,
        data::{Parameter, ParameterScope, ParameterType},
        error::Result,
    },
    compiler::{Compile, Compiler},
    decl::data::{
        ParameterScope as DeclParameterScope, ParameterType as DeclParameterType,
        Parameters as DeclParameters,
    },
};

impl Compile<Vec<DeclParameters>> for AvatarCompiler {
    type Output = Vec<Parameter>;

    fn compile(&mut self, parameters_blocks: Vec<DeclParameters>) -> Result<Vec<Parameter>> {
        let mut parameters: Vec<Parameter> = vec![];

        let decl_parameters = parameters_blocks.into_iter().flat_map(|pb| pb.parameters);
        for decl_parameter in decl_parameters {
            let name = decl_parameter.name.clone();
            let value_type = match decl_parameter.ty {
                DeclParameterType::Int(dv) => ParameterType::Int(dv.unwrap_or(0)),
                DeclParameterType::Float(dv) => ParameterType::Float(dv.unwrap_or(0.0)),
                DeclParameterType::Bool(dv) => ParameterType::Bool(dv.unwrap_or(false)),
            };
            let scope = match (decl_parameter.scope, decl_parameter.save) {
                (Some(DeclParameterScope::Internal), None | Some(false)) => {
                    ParameterScope::Internal
                }
                (Some(DeclParameterScope::Local), None) => ParameterScope::Local(false),
                (Some(DeclParameterScope::Local), Some(saved)) => ParameterScope::Local(saved),
                (None | Some(DeclParameterScope::Synced), None) => ParameterScope::Synced(false),
                (None | Some(DeclParameterScope::Synced), Some(saved)) => {
                    ParameterScope::Synced(saved)
                }

                (Some(DeclParameterScope::Internal), Some(true)) => {
                    self.error(format!(
                        "internal parameter '{}' cannot be saved",
                        decl_parameter.name
                    ));
                    continue;
                }
            };

            if let Some(defined) = parameters.iter().find(|p| p.name == decl_parameter.name) {
                if defined.value_type != value_type || defined.scope != scope {
                    self.error(format!(
                        "parameter '{}' have incompatible declarations",
                        decl_parameter.name
                    ));
                    continue;
                }
            }

            parameters.push(Parameter {
                name,
                value_type,
                scope,
            });
        }

        Ok(parameters)
    }
}
