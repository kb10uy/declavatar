use crate::{
    avatar::{
        data::{Driver, DriverGroup, ParameterScope, ParameterType},
        transformer::{
            dependencies::{AnimationGroupFilterExt, CompiledAnimations},
            failure, success, Compiled, Context, LogKind,
        },
    },
    decl::data::{Drive as DeclDrive, DriveTarget as DeclDriveTarget, Drivers as DeclDrivers},
};

pub fn compile_drivers_blocks(
    ctx: &mut Context,
    animations: &CompiledAnimations,
    decl_drivers_blocks: Vec<DeclDrivers>,
) -> Compiled<Vec<DriverGroup>> {
    let mut driver_groups = vec![];

    let decl_drivers = decl_drivers_blocks.into_iter().flat_map(|db| db.groups);
    for decl_driver in decl_drivers {
        let mut drivers = vec![];

        for drive in decl_driver.drives {
            let Some(driver) = compile_driver(ctx, animations, &decl_driver.name, drive) else {
                continue;
            };
            drivers.push(driver);
        }

        driver_groups.push(DriverGroup {
            name: decl_driver.name,
            local: decl_driver.local.unwrap_or(true),
            drivers,
        });
    }

    success(driver_groups)
}

pub fn compile_driver(
    ctx: &mut Context,
    animations: &CompiledAnimations,
    driver_name: &str,
    decl_drive: DeclDrive,
) -> Compiled<Driver> {
    let sources = animations.sources();

    match decl_drive {
        DeclDrive::Set(dt) => match dt {
            DeclDriveTarget::Group {
                name: group_name,
                option,
            } => {
                let Some(option_name) = option else {
                    ctx.log_error(LogKind::DriverOptionNotSpecified(driver_name.to_string()));
                    return failure();
                };
                let group = animations.find_animation_group(ctx, &group_name)?;
                let (parameter, options) = group.ensure_group(ctx)?;
                sources.find_parameter(
                    ctx,
                    parameter,
                    ParameterType::INT_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
                let Some(option) = options.iter().find(|o| o.name == option_name) else {
                    ctx.log_error(LogKind::AnimationGroupOptionNotFound(
                        group_name.to_string(),
                        option_name,
                    ));
                    return failure();
                };

                success(Driver::SetInt(parameter.to_string(), option.order as u8))
            }
            DeclDriveTarget::IntParameter { name, value } => {
                sources.find_parameter(
                    ctx,
                    &name,
                    ParameterType::INT_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
                success(Driver::SetInt(name, value))
            }
            DeclDriveTarget::FloatParameter { name, value } => {
                sources.find_parameter(
                    ctx,
                    &name,
                    ParameterType::FLOAT_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
                success(Driver::SetFloat(name, value))
            }
            DeclDriveTarget::BoolParameter { name, value } => {
                sources.find_parameter(
                    ctx,
                    &name,
                    ParameterType::BOOL_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
                success(Driver::SetBool(name, value))
            }
        },
        DeclDrive::Add(dt) => match dt {
            DeclDriveTarget::IntParameter { name, value } => {
                sources.find_parameter(
                    ctx,
                    &name,
                    ParameterType::INT_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
                success(Driver::AddInt(name, value))
            }
            DeclDriveTarget::FloatParameter { name, value } => {
                sources.find_parameter(
                    ctx,
                    &name,
                    ParameterType::FLOAT_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
                success(Driver::AddFloat(name, value))
            }
            _ => {
                ctx.log_error(LogKind::DriverInvalidAddTarget(driver_name.to_string()));
                failure()
            }
        },
        DeclDrive::Random {
            group,
            parameter,
            chance,
            range,
        } => match (group, parameter, chance, range) {
            (Some(group_name), None, None, (None, None)) => {
                let group = animations.find_animation_group(ctx, &group_name)?;
                let (parameter, options) = group.ensure_group(ctx)?;
                sources.find_parameter(
                    ctx,
                    parameter,
                    ParameterType::INT_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
                let max_index = options.iter().map(|o| o.order).max().unwrap_or(1);
                success(Driver::RandomInt(
                    parameter.to_string(),
                    (1, max_index as u8),
                ))
            }
            (None, Some(param), Some(chance), (None, None)) => {
                sources.find_parameter(
                    ctx,
                    &param,
                    ParameterType::BOOL_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
                success(Driver::RandomBool(param, chance))
            }
            (None, Some(param), None, (Some(min), Some(max))) => {
                sources.find_parameter(
                    ctx,
                    &param,
                    ParameterType::FLOAT_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
                success(Driver::RandomFloat(param, (min, max)))
            }

            _ => {
                ctx.log_error(LogKind::DriverInvalidRandomSpecification(
                    driver_name.to_string(),
                ));
                failure()
            }
        },
        DeclDrive::Copy {
            from,
            to,
            from_range,
            to_range,
        } => {
            let from_parameter =
                sources.find_parameter_untyped(ctx, &from, ParameterScope::MAYBE_INTERNAL)?;
            let to_parameter =
                sources.find_parameter_untyped(ctx, &to, ParameterScope::MAYBE_INTERNAL)?;
            if !from_parameter.value_type.matches(to_parameter.value_type) {
                ctx.log_error(LogKind::DriverCopyMismatch(
                    driver_name.to_string(),
                    (from, to),
                ));
                return failure();
            }

            match (from_range, to_range) {
                ((Some(from_min), Some(from_max)), (Some(to_min), Some(to_max))) => success(
                    Driver::RangedCopy(from, to, (from_min, from_max), (to_min, to_max)),
                ),
                ((None, None), (None, None)) => success(Driver::Copy(from, to)),
                _ => {
                    ctx.log_error(LogKind::DriverInvalidCopyTarget(driver_name.to_string()));
                    failure()
                }
            }
        }
    }
}
