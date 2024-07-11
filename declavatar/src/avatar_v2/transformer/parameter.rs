use crate::{
    avatar_v2::{
        data::parameter::{Parameter, ParameterDescription, ParameterScope, ParameterType},
        log::Log,
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::parameter::{
        DeclParameter, DeclParameters, DeclPrimitiveParameter, DeclPrimitiveParameterScope,
        DeclPrimitiveParameterType,
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
            let compiled_parameter = match parameter {
                DeclParameter::Primitive(primitive_parameter) => {
                    compile_primitive_parameter(&logger, primitive_parameter, &parameters)
                }
                DeclParameter::PhysBone(_) => unimplemented!(),
                DeclParameter::Provided(_) => unimplemented!(),
            };
            let Some(compiled_parameter) = compiled_parameter else {
                continue;
            };
            parameters.push(compiled_parameter);
        }
    }

    success(parameters)
}

fn compile_primitive_parameter(
    logger: &Logger<Log>,
    decl_parameter: DeclPrimitiveParameter,
    declared: &[Parameter],
) -> Compiled<Parameter> {
    let name = decl_parameter.name.clone();
    let (value_type, explicit_default) = match decl_parameter.ty {
        DeclPrimitiveParameterType::Int(dv) => (ParameterType::Int(dv.unwrap_or(0)), dv.is_some()),
        DeclPrimitiveParameterType::Float(dv) => {
            (ParameterType::Float(dv.unwrap_or(0.0)), dv.is_some())
        }
        DeclPrimitiveParameterType::Bool(dv) => {
            (ParameterType::Bool(dv.unwrap_or(false)), dv.is_some())
        }
    };
    let scope = match (decl_parameter.scope, decl_parameter.save) {
        (Some(DeclPrimitiveParameterScope::Internal), None | Some(false)) => {
            ParameterScope::Internal
        }
        (Some(DeclPrimitiveParameterScope::Local), None) => ParameterScope::Local(false),
        (Some(DeclPrimitiveParameterScope::Local), Some(saved)) => ParameterScope::Local(saved),
        (None | Some(DeclPrimitiveParameterScope::Synced), None) => ParameterScope::Synced(false),
        (None | Some(DeclPrimitiveParameterScope::Synced), Some(saved)) => {
            ParameterScope::Synced(saved)
        }

        (Some(DeclPrimitiveParameterScope::Internal), Some(true)) => {
            logger.log(Log::InternalMustBeTransient(decl_parameter.name));
            return failure();
        }
    };

    let description = ParameterDescription::Declared {
        scope,
        unique: decl_parameter.unique.unwrap_or(false),
        explicit_default,
    };

    if let Some(defined) = declared.iter().find(|p| p.name == decl_parameter.name) {
        if defined.value_type != value_type || defined.description != description {
            logger.log(Log::IncompatibleParameterDeclaration(decl_parameter.name));
        }
        return failure();
    }

    success(Parameter {
        name,
        value_type,
        description,
    })
}
