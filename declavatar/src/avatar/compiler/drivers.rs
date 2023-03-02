use crate::{
    avatar::{
        compiler::AvatarCompiler,
        data::{
            AnimationGroup, AnimationGroupContent, Driver, DriverGroup, Parameter, ParameterType,
        },
        error::Result,
    },
    compiler::{Compile, Compiler},
    decl::data::{Drive as DeclDrive, DriveTarget as DeclDriveTarget, Drivers as DeclDrivers},
};

use std::collections::HashMap;

impl
    Compile<(
        Vec<DeclDrivers>,
        &HashMap<String, Parameter>,
        &Vec<AnimationGroup>,
    )> for AvatarCompiler
{
    type Output = Vec<DriverGroup>;

    fn compile(
        &mut self,
        (drivers_blocks, parameters, animation_groups): (
            Vec<DeclDrivers>,
            &HashMap<String, Parameter>,
            &Vec<AnimationGroup>,
        ),
    ) -> Result<Vec<DriverGroup>> {
        let mut driver_groups = vec![];

        let decl_drivers = drivers_blocks.into_iter().map(|db| db.groups).flatten();
        for decl_driver in decl_drivers {
            let mut drivers = vec![];

            for drive in decl_driver.drives {
                let Some(driver) = self.compile((drive, parameters, animation_groups))? else {
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

        Ok(driver_groups)
    }
}

impl Compile<(DeclDrive, &HashMap<String, Parameter>, &Vec<AnimationGroup>)> for AvatarCompiler {
    type Output = Option<Driver>;

    fn compile(
        &mut self,
        (decl_drive, parameters, animation_groups): (
            DeclDrive,
            &HashMap<String, Parameter>,
            &Vec<AnimationGroup>,
        ),
    ) -> Result<Option<Driver>> {
        let driver = match decl_drive {
            DeclDrive::Set(dt) => match dt {
                DeclDriveTarget::Group {
                    name: group_name,
                    option,
                } => {
                    let Some(option_name) = option else {
                        self.error(format!("option must be specified"));
                        return Ok(None);
                    };
                    let Some(group) = animation_groups.iter().find(|ag| ag.name == group_name) else {
                        self.error(format!("animation group '{group_name}' not found"));
                        return Ok(None);
                    };
                    if !self.ensure((parameters, &group.parameter, &ParameterType::INT_TYPE))? {
                        self.error(format!(
                            "animation group '{group_name}' should refer int parameter"
                        ));
                        return Ok(None);
                    };
                    let option_index = match &group.content {
                        AnimationGroupContent::ShapeGroup { options, .. } => {
                            let Some((option_order, _)) = options.get(&option_name) else {
                                self.error(format!("option '{option_name}' not found in '{group_name}'"));
                                return Ok(None);
                            };
                            *option_order
                        }
                        AnimationGroupContent::ObjectGroup { options, .. } => {
                            let Some((option_order, _)) = options.get(&option_name) else {
                                self.error(format!("option '{option_name}' not found in '{group_name}'"));
                                return Ok(None);
                            };
                            *option_order
                        }
                        _ => {
                            self.error(format!(
                                "parameter driver with group is valid only for groups but not switches"
                            ));
                            return Ok(None);
                        }
                    };

                    Driver::SetInt(group.parameter.clone(), option_index as u8)
                }
                DeclDriveTarget::IntParameter { name, value } => {
                    if !self.ensure((parameters, &name, &ParameterType::INT_TYPE))? {
                        return Ok(None);
                    };
                    Driver::SetInt(name, value)
                }
                DeclDriveTarget::FloatParameter { name, value } => {
                    if !self.ensure((parameters, &name, &ParameterType::FLOAT_TYPE))? {
                        return Ok(None);
                    };
                    Driver::SetFloat(name, value)
                }
                DeclDriveTarget::BoolParameter { name, value } => {
                    if !self.ensure((parameters, &name, &ParameterType::BOOL_TYPE))? {
                        return Ok(None);
                    };
                    Driver::SetBool(name, value)
                }
            },
            DeclDrive::Add(dt) => match dt {
                DeclDriveTarget::IntParameter { name, value } => {
                    if !self.ensure((parameters, &name, &ParameterType::INT_TYPE))? {
                        return Ok(None);
                    };
                    Driver::AddInt(name, value)
                }
                DeclDriveTarget::FloatParameter { name, value } => {
                    if !self.ensure((parameters, &name, &ParameterType::FLOAT_TYPE))? {
                        return Ok(None);
                    };
                    Driver::AddFloat(name, value)
                }
                _ => {
                    self.error(format!(
                        "parameter driver of add is valid only for int and float"
                    ));
                    return Ok(None);
                }
            },
            DeclDrive::Random {
                group,
                parameter,
                chance,
                range,
            } => match (group, parameter, chance, range) {
                (Some(group_name), None, None, (None, None)) => {
                    let Some(group) = animation_groups.iter().find(|ag| ag.name == group_name) else {
                        self.error(format!("animation group '{group_name}' not found"));
                        return Ok(None);
                    };
                    if !self.ensure((parameters, &group.parameter, &ParameterType::INT_TYPE))? {
                        self.error(format!(
                            "animation group '{group_name}' should refer int parameter"
                        ));
                        return Ok(None);
                    };
                    let max_index = match &group.content {
                        AnimationGroupContent::ShapeGroup { options, .. } => {
                            options.iter().map(|o| o.1 .0).max().unwrap_or(1)
                        }
                        AnimationGroupContent::ObjectGroup { options, .. } => {
                            options.iter().map(|o| o.1 .0).max().unwrap_or(1)
                        }
                        _ => {
                            self.error(format!(
                                "parameter driver with group is valid only for groups but not switches"
                            ));
                            return Ok(None);
                        }
                    };
                    Driver::RandomInt(group.parameter.clone(), (1, max_index as u8))
                }
                (None, Some(param), Some(chance), (None, None)) => {
                    Driver::RandomBool(param, chance)
                }
                (None, Some(param), None, (Some(min), Some(max))) => {
                    Driver::RandomFloat(param, (min, max))
                }

                (Some(_), Some(_), _, _) => {
                    // random group="x" parameter="y" ...
                    self.error(format!("ambiguous random group"));
                    return Ok(None);
                }
                (Some(_), None, _, _) => {
                    // random group="x" chance=0.5 ...
                    self.error(format!("redundant parameters specified for random driver"));
                    return Ok(None);
                }
                (None, Some(_), Some(_), _) => {
                    // random parameter="x" chance=0.5 min=0.0 ...
                    self.error(format!("ambiguous random chance"));
                    return Ok(None);
                }
                (None, Some(_), None, _) => {
                    // random parameter="x" max=0.5
                    self.error(format!("insufficient random chance"));
                    return Ok(None);
                }
                _ => {
                    // random
                    self.error(format!("invalid random driver"));
                    return Ok(None);
                }
            },
            DeclDrive::Copy {
                from,
                to,
                from_range,
                to_range,
            } => match (from_range, to_range) {
                ((Some(from_min), Some(from_max)), (Some(to_min), Some(to_max))) => {
                    Driver::RangedCopy(from, to, (from_min, from_max), (to_min, to_max))
                }
                ((None, None), (None, None)) => Driver::Copy(from, to),
                _ => {
                    self.error(format!("insufficient copy range"));
                    return Ok(None);
                }
            },
        };

        Ok(Some(driver))
    }
}
