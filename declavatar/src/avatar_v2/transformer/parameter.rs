use crate::{
    avatar_v2::{
        data::parameter::{Parameter, ParameterDescription, ParameterScope, ParameterType},
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
            let compiled_parameter = match parameter {
                DeclParameter::Primitive(primitive_parameter) => {
                    compile_primitive_parameter(&logger, primitive_parameter, &parameters)
                }
                DeclParameter::PhysBone(pb_desc) => compile_physbone_parameters(pb_desc.prefix),
                DeclParameter::Provided(vrc_desc) => compile_vrc_parameters(vrc_desc),
            };
            let Some(compiled_parameter) = compiled_parameter else {
                continue;
            };
            parameters.extend(compiled_parameter);
        }
    }

    success(parameters)
}

fn compile_primitive_parameter(
    logger: &Logger<Log>,
    decl_parameter: DeclPrimitiveParameter,
    declared: &[Parameter],
) -> Compiled<Vec<Parameter>> {
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

    success(vec![Parameter {
        name,
        value_type,
        description,
    }])
}

fn compile_vrc_parameters(kinds: Vec<DeclProvidedParameterKind>) -> Compiled<Vec<Parameter>> {
    let parameters = kinds
        .into_iter()
        .map(|k| {
            let name = "".into();
            let value_type = ParameterType::INT_TYPE;
            Parameter {
                name,
                value_type,
                description: ParameterDescription::Provided,
            }
        })
        .collect();
    success(parameters)
}

fn compile_physbone_parameters(prefix: String) -> Compiled<Vec<Parameter>> {
    success(vec![
        Parameter {
            name: format!("{prefix}_IsGrabbed"),
            value_type: ParameterType::BOOL_TYPE,
            description: ParameterDescription::Provided,
        },
        Parameter {
            name: format!("{prefix}_IsPosed"),
            value_type: ParameterType::BOOL_TYPE,
            description: ParameterDescription::Provided,
        },
        Parameter {
            name: format!("{prefix}_Angle"),
            value_type: ParameterType::FLOAT_TYPE,
            description: ParameterDescription::Provided,
        },
        Parameter {
            name: format!("{prefix}_Stretch"),
            value_type: ParameterType::FLOAT_TYPE,
            description: ParameterDescription::Provided,
        },
        Parameter {
            name: format!("{prefix}_Squish"),
            value_type: ParameterType::FLOAT_TYPE,
            description: ParameterDescription::Provided,
        },
    ])
}

fn to_declared_parameter(kind: DeclProvidedParameterKind) -> (&'static str, ParameterType) {
    // TODO: maybe inoptimal way
    match kind {
        DeclProvidedParameterKind::IsLocal => ("IsLocal", ParameterType::BOOL_TYPE),
        DeclProvidedParameterKind::Viseme => ("Viseme", ParameterType::INT_TYPE),
        DeclProvidedParameterKind::Voice => ("Voice", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::GestureLeft => ("GestureLeft", ParameterType::INT_TYPE),
        DeclProvidedParameterKind::GestureRight => ("GestureRight", ParameterType::INT_TYPE),
        DeclProvidedParameterKind::GestureLeftWeight => ("GestureLeftWeight", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::GestureRightWeight => ("GestureRightWeight", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::AngularY => ("AngularY", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::VelocityX => ("VelocityX", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::VelocityY => ("VelocityY", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::VelocityZ => ("VelocityZ", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::VelocityMagnitude => ("VelocityMagnitude", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::Upright => ("Upright", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::Grounded => ("Grounded", ParameterType::BOOL_TYPE),
        DeclProvidedParameterKind::Seated => ("Seated", ParameterType::BOOL_TYPE),
        DeclProvidedParameterKind::Afk => ("AFK", ParameterType::BOOL_TYPE),
        DeclProvidedParameterKind::TrackingType => ("TrackingType", ParameterType::INT_TYPE),
        DeclProvidedParameterKind::VrMode => ("VRMode", ParameterType::INT_TYPE),
        DeclProvidedParameterKind::MuteSelf => ("MuteSelf", ParameterType::BOOL_TYPE),
        DeclProvidedParameterKind::InStation => ("InStation", ParameterType::BOOL_TYPE),
        DeclProvidedParameterKind::Earmuffs => ("Earmuffs", ParameterType::BOOL_TYPE),
        DeclProvidedParameterKind::IsOnFriendsList => ("IsOnFriendsList", ParameterType::BOOL_TYPE),
        DeclProvidedParameterKind::AvatarVersion => ("AvatarVersion", ParameterType::INT_TYPE),
        DeclProvidedParameterKind::ScaleModified => ("ScaleModified", ParameterType::BOOL_TYPE),
        DeclProvidedParameterKind::ScaleFactor => ("ScaleFactor", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::ScaleFactorInverse => ("ScaleFactorInverse", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::EyeHeightAsMeters => ("EyeHeightAsMeters", ParameterType::FLOAT_TYPE),
        DeclProvidedParameterKind::EyeHeightAsPercent => ("EyeHeightAsPercent", ParameterType::FLOAT_TYPE),
    }
}
