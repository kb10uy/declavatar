use crate::{
    avatar_v2::{
        data::parameter::{Parameter, ParameterScope, ParameterType},
        log::Log,
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::parameter::{
        DeclParameter, DeclParameterScope, DeclParameterType, DeclParameters,
    },
    log::Logger,
};

pub fn compile_parameters_blocks(
    logger: &Logger<Log>,
    parameters_blocks: Vec<DeclParameters>,
) -> Compiled<Vec<Parameter>> {
    let mut parameters = vec![];
    for (index, decl_parameters) in parameters_blocks.into_iter().enumerate() {
        let logger = logger.with_context(format!("parameters block {index}"));
        for parameter in decl_parameters.parameters {
            let Some(parameter) = compile_parameter(&logger, parameter, &parameters) else {
                continue;
            };
            parameters.push(parameter);
        }
    }

    success(parameters)
}

fn compile_parameter(
    logger: &Logger<Log>,
    decl_parameter: DeclParameter,
    declared: &[Parameter],
) -> Compiled<Parameter> {
    let name = decl_parameter.name.clone();
    let (value_type, explicit_default) = match decl_parameter.ty {
        DeclParameterType::Int(dv) => (ParameterType::Int(dv.unwrap_or(0)), dv.is_some()),
        DeclParameterType::Float(dv) => (ParameterType::Float(dv.unwrap_or(0.0)), dv.is_some()),
        DeclParameterType::Bool(dv) => (ParameterType::Bool(dv.unwrap_or(false)), dv.is_some()),
    };
    let scope = match (decl_parameter.scope, decl_parameter.save) {
        (Some(DeclParameterScope::Internal), None | Some(false)) => ParameterScope::Internal,
        (Some(DeclParameterScope::Local), None) => ParameterScope::Local(false),
        (Some(DeclParameterScope::Local), Some(saved)) => ParameterScope::Local(saved),
        (None | Some(DeclParameterScope::Synced), None) => ParameterScope::Synced(false),
        (None | Some(DeclParameterScope::Synced), Some(saved)) => ParameterScope::Synced(saved),

        (Some(DeclParameterScope::Internal), Some(true)) => {
            logger.log(Log::InternalMustBeTransient(decl_parameter.name));
            return failure();
        }
    };

    if let Some(defined) = declared.iter().find(|p| p.name == decl_parameter.name) {
        if defined.value_type != value_type || defined.scope != scope {
            logger.log(Log::IncompatibleParameterDeclaration(decl_parameter.name));
        }
        return failure();
    }

    success(Parameter {
        name,
        value_type,
        scope,
        unique: decl_parameter.unique.unwrap_or(false),
        explicit_default,
    })
}
