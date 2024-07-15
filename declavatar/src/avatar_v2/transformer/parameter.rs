use crate::{
    avatar_v2::{
        data::parameter::{DeclaredParameter, Parameter, ParameterScope, ParameterType, ProvidedParameter},
        log::Log,
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::parameter::{
        DeclParameter, DeclParameters, DeclPrimitiveParameter, DeclPrimitiveParameterScope, DeclPrimitiveParameterType,
        DeclProvidedParameterKind,
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
            match parameter {
                DeclParameter::Primitive(decl_primitive) => {
                    let Some(parameter) = compile_primitive_parameter(&logger, decl_primitive, &parameters) else {
                        continue;
                    };
                    parameters.push(parameter);
                }
                DeclParameter::Provided(vrc_kinds) => {
                    let Some(vrc_parameters) = compile_vrc_parameters(vrc_kinds) else {
                        unreachable!("VRChat parameters must compile");
                    };
                    parameters.extend(vrc_parameters);
                }
                DeclParameter::PhysBone(pb_prefix) => {
                    let Some(pb_parameter) = compile_physbone_parameters(pb_prefix.prefix) else {
                        unreachable!("PhysBone parameter must compile");
                    };
                    parameters.push(pb_parameter);
                }
            }
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
        DeclPrimitiveParameterType::Float(dv) => (ParameterType::Float(dv.unwrap_or(0.0)), dv.is_some()),
        DeclPrimitiveParameterType::Bool(dv) => (ParameterType::Bool(dv.unwrap_or(false)), dv.is_some()),
    };
    let scope = match (decl_parameter.scope, decl_parameter.save) {
        (Some(DeclPrimitiveParameterScope::Internal), None | Some(false)) => ParameterScope::Internal,
        (Some(DeclPrimitiveParameterScope::Local), None) => ParameterScope::Local(false),
        (Some(DeclPrimitiveParameterScope::Local), Some(saved)) => ParameterScope::Local(saved),
        (None | Some(DeclPrimitiveParameterScope::Synced), None) => ParameterScope::Synced(false),
        (None | Some(DeclPrimitiveParameterScope::Synced), Some(saved)) => ParameterScope::Synced(saved),

        (Some(DeclPrimitiveParameterScope::Internal), Some(true)) => {
            logger.log(Log::InternalMustBeTransient(decl_parameter.name));
            return failure();
        }
    };

    if declared.iter().any(|p| name == p.basename()) {
        logger.log(Log::IncompatibleParameterDeclaration(decl_parameter.name));
        return failure();
    }

    success(Parameter::Declared(DeclaredParameter {
        name,
        value_type,
        scope,
        unique: decl_parameter.unique.unwrap_or(false),
        explicit_default,
    }))
}

fn compile_vrc_parameters(kinds: Vec<DeclProvidedParameterKind>) -> Compiled<Vec<Parameter>> {
    let parameters = kinds
        .into_iter()
        .map(|k| Parameter::Provided(ProvidedParameter::Vrchat(k.into())))
        .collect();
    success(parameters)
}

fn compile_physbone_parameters(prefix: String) -> Compiled<Parameter> {
    success(Parameter::Provided(ProvidedParameter::PhysBone(prefix)))
}
