use crate::{
    avatar_v2::{
        data::driver::ParameterDrive,
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
            let (parameter, options) = first_pass.find_group(logger, &dg.group)?;
            let Some((_, value)) = options.iter().find(|(name, _)| name == &dg.option) else {
                logger.log(Log::LayerOptionNotFound(dg.option));
                return failure();
            };
            ParameterDrive::SetInt(parameter.to_string(), *value as u8)
        }
        DeclParameterDrive::Switch(ds) => {
            let parameter = first_pass.find_switch(logger, &ds.switch)?;
            ParameterDrive::SetBool(parameter.to_string(), unset_value.replace_bool(ds.value))
        }
        DeclParameterDrive::Puppet(dp) => {
            let parameter = first_pass.find_puppet(logger, &dp.puppet)?;
            ParameterDrive::SetFloat(parameter.to_string(), unset_value.replace_f64(dp.value))
        }
    };
    success(parameter_drive)
}
