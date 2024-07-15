use crate::{
    avatar_v2::{
        data::{
            driver::{ParameterDrive, TrackingControl},
            parameter::ParameterType,
        },
        log::Log,
        transformer::{failure, success, Compiled, FirstPassData, UnsetValue},
    },
    decl_v2::data::driver::{DeclParameterDrive, DeclTrackingControl},
    log::Logger,
};

pub fn compile_parameter_drive(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    unset_value: UnsetValue,
    decl_parameter_drive: DeclParameterDrive,
) -> Compiled<ParameterDrive> {
    let parameter_drive = match decl_parameter_drive {
        DeclParameterDrive::Group(dg) => {
            let (parameter, options) = first_pass.find_group(logger, &dg.group)?;
            let qualified = first_pass.find_writable_parameter(logger, parameter, ParameterType::INT_TYPE)?;
            let Some((_, value)) = options.iter().find(|(name, _)| name == &dg.option) else {
                logger.log(Log::LayerOptionNotFound(dg.option));
                return failure();
            };
            ParameterDrive::SetInt(qualified.name, *value as u8)
        }
        DeclParameterDrive::Switch(ds) => {
            let parameter = first_pass.find_switch(logger, &ds.switch)?;
            let qualified = first_pass.find_writable_parameter(logger, parameter, ParameterType::BOOL_TYPE)?;
            ParameterDrive::SetBool(qualified.name, unset_value.replace_bool(ds.value))
        }
        DeclParameterDrive::Puppet(dp) => {
            let parameter = first_pass.find_puppet(logger, &dp.puppet)?;
            let qualified = first_pass.find_writable_parameter(logger, parameter, ParameterType::FLOAT_TYPE)?;
            ParameterDrive::SetFloat(qualified.name, unset_value.replace_f64(dp.value))
        }
        DeclParameterDrive::SetInt { parameter, value } => {
            let qualified = first_pass.find_writable_parameter(logger, &parameter.into(), ParameterType::INT_TYPE)?;
            ParameterDrive::SetInt(qualified.name, value as u8)
        }
        DeclParameterDrive::SetBool { parameter, value } => {
            let qualified = first_pass.find_writable_parameter(logger, &parameter.into(), ParameterType::BOOL_TYPE)?;
            ParameterDrive::SetBool(qualified.name, unset_value.replace_bool(value))
        }
        DeclParameterDrive::SetFloat { parameter, value } => {
            let qualified = first_pass.find_writable_parameter(logger, &parameter.into(), ParameterType::FLOAT_TYPE)?;
            ParameterDrive::SetFloat(qualified.name, unset_value.replace_f64(value))
        }
        DeclParameterDrive::AddInt { parameter, value } => {
            let qualified = first_pass.find_writable_parameter(logger, &parameter.into(), ParameterType::INT_TYPE)?;
            ParameterDrive::AddInt(qualified.name, value as u8)
        }
        DeclParameterDrive::AddFloat { parameter, value } => {
            let qualified = first_pass.find_writable_parameter(logger, &parameter.into(), ParameterType::FLOAT_TYPE)?;
            ParameterDrive::AddFloat(qualified.name, value)
        }
        DeclParameterDrive::RandomInt { parameter, range } => {
            let qualified = first_pass.find_writable_parameter(logger, &parameter.into(), ParameterType::INT_TYPE)?;
            ParameterDrive::RandomInt(qualified.name, range)
        }
        DeclParameterDrive::RandomBool { parameter, value } => {
            let qualified = first_pass.find_writable_parameter(logger, &parameter.into(), ParameterType::BOOL_TYPE)?;
            ParameterDrive::RandomBool(qualified.name, value)
        }
        DeclParameterDrive::RandomFloat { parameter, range } => {
            let qualified = first_pass.find_writable_parameter(logger, &parameter.into(), ParameterType::FLOAT_TYPE)?;
            ParameterDrive::RandomFloat(qualified.name, range)
        }
        DeclParameterDrive::Copy { from, to, range } => {
            let qualified_from = first_pass.find_writable_parameter(logger, &from.into(), ParameterType::FLOAT_TYPE)?;
            let qualified_to = first_pass.find_writable_parameter(logger, &to.into(), ParameterType::FLOAT_TYPE)?;
            if let Some(range) = range {
                ParameterDrive::RangedCopy(qualified_from.name, qualified_to.name, range.0, range.1)
            } else {
                ParameterDrive::Copy(qualified_from.name, qualified_to.name)
            }
        }
    };
    success(parameter_drive)
}

pub fn compile_tracking_control(
    _logger: &Logger<Log>,
    _first_pass: &FirstPassData,
    decl_tracking_control: DeclTrackingControl,
) -> Compiled<impl Iterator<Item = TrackingControl>> {
    let tracking_controls = decl_tracking_control.targets.into_iter().map(move |t| TrackingControl {
        animation_desired: decl_tracking_control.animation_desired,
        target: t.into(),
    });

    success(tracking_controls)
}
