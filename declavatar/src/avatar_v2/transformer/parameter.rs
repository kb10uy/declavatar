use crate::{
    avatar_v2::{
        data::parameter::{Parameter, ParameterScope, ParameterType},
        logger::{Log, Logger, LoggerContext},
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::parameter::{
        DeclParameter, DeclParameterScope, DeclParameterType, DeclParameters,
    },
};

pub fn compile_parameters_blocks(
    logger: &mut Logger,
    parameters_blocks: Vec<DeclParameters>,
) -> Compiled<Vec<Parameter>> {
    #[derive(Debug)]
    pub struct Context(usize);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("parameters block {} > {}", self.0, inner)
        }
    }

    let mut parameters = vec![];
    for (index, decl_parameters) in parameters_blocks.into_iter().enumerate() {
        logger.push_context(Context(index));
        for parameter in decl_parameters.parameters {
            let Some(parameter) = compile_parameter(logger, parameter, &parameters) else {
                continue;
            };
            parameters.push(parameter);
        }
        logger.pop_context();
    }

    success(parameters)
}

fn compile_parameter(
    logger: &mut Logger,
    decl_parameter: DeclParameter,
    declared: &[Parameter],
) -> Compiled<Parameter> {
    let name = decl_parameter.name.clone();
    let value_type = match decl_parameter.ty {
        DeclParameterType::Int(dv) => ParameterType::Int(dv.unwrap_or(0)),
        DeclParameterType::Float(dv) => ParameterType::Float(dv.unwrap_or(0.0)),
        DeclParameterType::Bool(dv) => ParameterType::Bool(dv.unwrap_or(false)),
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
    })
}
