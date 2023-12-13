use crate::{
    avatar_v2::{
        data::{
            driver::ParameterDrive,
            parameter::{ParameterScope, ParameterType},
        },
        logger::{Log, Logger},
        transformer::{failure, success, Compiled, FirstPassData, UnsetValue},
    },
    decl_v2::data::driver::DeclParameterDrive,
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
        DeclParameterDrive::IntParameter(di) => {
            first_pass.find_parameter(
                logger,
                &di.parameter,
                ParameterType::INT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::SetInt(di.parameter, di.value as u8)
        }
        DeclParameterDrive::BoolParameter(db) => {
            first_pass.find_parameter(
                logger,
                &db.parameter,
                ParameterType::BOOL_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::SetBool(db.parameter, unset_value.replace_bool(db.value))
        }
        DeclParameterDrive::FloatParameter(df) => {
            first_pass.find_parameter(
                logger,
                &df.parameter,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            ParameterDrive::SetFloat(df.parameter, unset_value.replace_f64(df.value))
        }
    };
    success(parameter_drive)
}
