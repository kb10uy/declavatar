use crate::{
    avatar_v2::{
        data::{
            driver::{ParameterDrive, TrackingControl},
            parameter::{ParameterScope, ParameterType},
        },
        logger::{Log, Logger},
        transformer::{failure, success, Compiled, FirstPassData, UnsetValue},
    },
    decl_v2::data::driver::{DeclParameterDrive, DeclTrackingControl, DeclTrackingTarget},
};

pub fn compile_parameter_drive(
    logger: &Logger,
    first_pass: &FirstPassData,
    unset_value: UnsetValue,
    decl_parameter_drive: DeclParameterDrive,
) -> Compiled<ParameterDrive> {
    let parameter_drive = match decl_parameter_drive {
        DeclParameterDrive::Group(dg) => {
            let (parameter, options) =
                first_pass.find_group(logger, &dg.group, ParameterScope::MAYBE_INTERNAL)?;
            let Some((_, value)) = options.iter().find(|(name, _)| name == &dg.option) else {
                logger.log(Log::LayerOptionNotFound(dg.option));
                return failure();
            };
            ParameterDrive::SetInt(parameter.to_string(), *value as u8)
        }
        DeclParameterDrive::Switch(ds) => {
            let parameter =
                first_pass.find_switch(logger, &ds.switch, ParameterScope::MAYBE_INTERNAL)?;
            ParameterDrive::SetBool(parameter.to_string(), unset_value.replace_bool(ds.value))
        }
        DeclParameterDrive::Puppet(dp) => {
            let parameter =
                first_pass.find_puppet(logger, &dp.puppet, ParameterScope::MAYBE_INTERNAL)?;
            ParameterDrive::SetFloat(parameter.to_string(), unset_value.replace_f64(dp.value))
        }
        DeclParameterDrive::SetInt { parameter, value } => {
            first_pass.find_parameter(
                logger,
                &parameter,
                ParameterType::INT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::SetInt(parameter, value as u8)
        }
        DeclParameterDrive::SetBool { parameter, value } => {
            first_pass.find_parameter(
                logger,
                &parameter,
                ParameterType::BOOL_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::SetBool(parameter, unset_value.replace_bool(value))
        }
        DeclParameterDrive::SetFloat { parameter, value } => {
            first_pass.find_parameter(
                logger,
                &parameter,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::SetFloat(parameter, unset_value.replace_f64(value))
        }
        DeclParameterDrive::AddInt { parameter, value } => {
            first_pass.find_parameter(
                logger,
                &parameter,
                ParameterType::INT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::AddInt(parameter, value as u8)
        }
        DeclParameterDrive::AddFloat { parameter, value } => {
            first_pass.find_parameter(
                logger,
                &parameter,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::AddFloat(parameter, value)
        }
        DeclParameterDrive::RandomInt { parameter, range } => {
            first_pass.find_parameter(
                logger,
                &parameter,
                ParameterType::INT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::RandomInt(parameter, range)
        }
        DeclParameterDrive::RandomBool { parameter, value } => {
            first_pass.find_parameter(
                logger,
                &parameter,
                ParameterType::BOOL_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::RandomBool(parameter, value)
        }
        DeclParameterDrive::RandomFloat { parameter, range } => {
            first_pass.find_parameter(
                logger,
                &parameter,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::RandomFloat(parameter, range)
        }
        DeclParameterDrive::Copy { from, to, range } => {
            first_pass.find_parameter(
                logger,
                &from,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            first_pass.find_parameter(
                logger,
                &to,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            if let Some(range) = range {
                ParameterDrive::RangedCopy(from, to, range.0, range.1)
            } else {
                ParameterDrive::Copy(from, to)
            }
        }
    };
    success(parameter_drive)
}

pub fn compile_tracking_control(
    _logger: &Logger,
    _first_pass: &FirstPassData,
    decl_tracking_control: DeclTrackingControl,
) -> Compiled<TrackingControl> {
    success(TrackingControl {
        animation_desired: decl_tracking_control.animation_desired,
        targets: decl_tracking_control
            .targets
            .into_iter()
            .map(|t| match t {
                DeclTrackingTarget::Head => todo!(),
                DeclTrackingTarget::Hip => todo!(),
                DeclTrackingTarget::Eyes => todo!(),
                DeclTrackingTarget::Mouth => todo!(),
                DeclTrackingTarget::HandLeft => todo!(),
                DeclTrackingTarget::HandRight => todo!(),
                DeclTrackingTarget::FootLeft => todo!(),
                DeclTrackingTarget::FoorRight => todo!(),
                DeclTrackingTarget::FingersLeft => todo!(),
                DeclTrackingTarget::FingersRight => todo!(),
            })
            .collect(),
    })
}
