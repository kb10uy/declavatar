use crate::{
    avatar::{
        data::{
            AnimationGroup, AnimationGroupContent, Avatar, Driver, DriverGroup, ObjectTarget,
            Parameter, ParameterSync, ParameterType, ShapeTarget,
        },
        error::{AvatarError, Result},
    },
    compiler::{Compile, Compiler, ErrorStackCompiler, Validate},
    decl::{
        animations::{
            AnimationElement as DeclAnimationElement, Animations as DeclAnimations,
            ObjectGroup as DeclObjectGroup, ObjectSwitch as DeclObjectSwitch,
            ShapeGroup as DeclShapeGroup, ShapeSwitch as DeclShapeSwitch,
        },
        document::Avatar as DeclAvatar,
        drivers::{Drive as DeclDrive, DriveTarget as DeclDriveTarget, Drivers as DeclDrivers},
        parameters::{ParameterType as DeclParameterType, Parameters as DeclParameters},
    },
};

use std::{
    collections::{HashMap, HashSet},
    result::Result as StdResult,
};

pub type AvatarCompiler = ErrorStackCompiler<AvatarError>;

pub fn compile_avatar(avatar: DeclAvatar) -> Result<StdResult<Avatar, Vec<String>>> {
    let mut compiler = AvatarCompiler::new();
    let compiled_avatar = compiler.parse(avatar)?;

    if compiler.errornous() {
        Ok(Err(compiler
            .messages()
            .into_iter()
            .map(|(_, m)| m)
            .collect()))
    } else if let Some(a) = compiled_avatar {
        Ok(Ok(a))
    } else {
        Err(AvatarError::CompilerError(
            "neither functional avatar nor error list has been generated".into(),
        ))
    }
}

impl Compile<DeclAvatar> for AvatarCompiler {
    type Output = Option<Avatar>;

    fn compile(&mut self, avatar: DeclAvatar) -> Result<Option<Avatar>> {
        let name = {
            let decl_name = avatar.name.trim();
            if decl_name == "" {
                self.error(format!("invalid avatar name"));
                return Ok(None);
            }
            decl_name.to_string()
        };

        let parameters = self.parse(avatar.parameters_blocks)?;
        let animation_groups = self.parse((avatar.animations_blocks, &parameters))?;
        let driver_groups = self.parse((avatar.drivers_blocks, &parameters, &animation_groups))?;
        Ok(Some(Avatar {
            name,
            parameters,
            animation_groups,
            driver_groups,
        }))
    }
}

impl Compile<Vec<DeclParameters>> for AvatarCompiler {
    type Output = HashMap<String, Parameter>;

    fn compile(
        &mut self,
        parameters_blocks: Vec<DeclParameters>,
    ) -> Result<HashMap<String, Parameter>> {
        use std::collections::hash_map::Entry;

        let mut parameters = HashMap::new();

        let decl_parameters = parameters_blocks
            .into_iter()
            .map(|pb| pb.parameters)
            .flatten();
        for decl_parameter in decl_parameters {
            let name = decl_parameter.name.clone();
            let value_type = match decl_parameter.ty {
                DeclParameterType::Int(dv) => ParameterType::Int(dv.unwrap_or(0)),
                DeclParameterType::Float(dv) => ParameterType::Float(dv.unwrap_or(0.0)),
                DeclParameterType::Bool(dv) => ParameterType::Bool(dv.unwrap_or(false)),
            };
            let sync_type = match (decl_parameter.local, decl_parameter.save) {
                (Some(true), None | Some(false)) => ParameterSync::Local,
                (None | Some(false), None) => ParameterSync::Synced(false),
                (None | Some(false), Some(save)) => ParameterSync::Synced(save),
                (Some(true), Some(true)) => {
                    self.error(format!(
                        "local parameter '{}' cannot be saved",
                        decl_parameter.name
                    ));
                    continue;
                }
            };

            match parameters.entry(decl_parameter.name.clone()) {
                Entry::Occupied(p) => {
                    let defined: &Parameter = p.get();
                    if defined.value_type != value_type || defined.sync_type != sync_type {
                        self.error(format!(
                            "parameter '{}' have incompatible declarations",
                            decl_parameter.name
                        ));
                        continue;
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(Parameter {
                        name,
                        value_type,
                        sync_type,
                    });
                }
            }
        }

        Ok(parameters)
    }
}

impl Compile<(Vec<DeclAnimations>, &HashMap<String, Parameter>)> for AvatarCompiler {
    type Output = Vec<AnimationGroup>;

    fn compile(
        &mut self,
        (animations_blocks, parameters): (Vec<DeclAnimations>, &HashMap<String, Parameter>),
    ) -> Result<Vec<AnimationGroup>> {
        let mut animation_groups = vec![];

        let mut used_group_names: HashSet<String> = HashSet::new();
        let mut used_parameters: HashSet<String> = HashSet::new();
        let decl_animations = animations_blocks
            .into_iter()
            .map(|ab| ab.elements)
            .flatten();
        for decl_animation in decl_animations {
            let Some(animation_group) = (match decl_animation {
                DeclAnimationElement::ShapeGroup(shape_group) => {
                    self.compile((shape_group, parameters))?
                }
                DeclAnimationElement::ShapeSwitch(shape_switch) => {
                    self.compile((shape_switch, parameters))?
                }
                DeclAnimationElement::ObjectGroup(object_group) => {
                    self.compile((object_group, parameters))?
                }
                DeclAnimationElement::ObjectSwitch(object_switch) => {
                    self.compile((object_switch, parameters))?
                }
            }) else {
                continue;
            };

            if used_group_names.contains(&animation_group.name) {
                self.warn(format!(
                    "group name '{}' is used multiple times",
                    animation_group.name
                ));
            } else {
                used_group_names.insert(animation_group.name.clone());
            }

            if used_parameters.contains(&animation_group.parameter) {
                self.warn(format!(
                    "parameter '{}' is used multiple times",
                    animation_group.parameter
                ));
            } else {
                used_parameters.insert(animation_group.parameter.clone());
            }

            animation_groups.push(animation_group);
        }

        Ok(animation_groups)
    }
}

impl Compile<(DeclShapeGroup, &HashMap<String, Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (sg, parameters): (DeclShapeGroup, &HashMap<String, Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &sg.parameter, &ParameterType::INT_TYPE))? {
            return Ok(None);
        };

        let mut options = HashMap::new();
        let mut default_shapes: Vec<_> = sg
            .default_block
            .map(|b| b.shapes)
            .unwrap_or_default()
            .into_iter()
            .map(|ds| ShapeTarget(ds.0, ds.1.unwrap_or(0.0)))
            .collect();
        let mut default_shape_names: HashSet<_> =
            default_shapes.iter().map(|s| s.0.clone()).collect();

        for decl_option in sg.options {
            let name = decl_option.name.expect("option block must have name");
            let option: Vec<_> = decl_option
                .shapes
                .into_iter()
                .map(|ds| ShapeTarget(ds.0, ds.1.unwrap_or(1.0)))
                .collect();

            for target in &option {
                if default_shape_names.contains(&target.0) {
                    continue;
                }
                default_shapes.push(ShapeTarget(target.0.clone(), 0.0));
                default_shape_names.insert(target.0.clone());
            }

            options.insert(name, (decl_option.declared_order, option));
        }

        Ok(Some(AnimationGroup {
            name: sg.name,
            parameter: sg.parameter,
            content: AnimationGroupContent::ShapeGroup {
                mesh: sg.mesh,
                prevent_mouth: sg.prevent_mouth.unwrap_or(false),
                prevent_eyelids: sg.prevent_eyelids.unwrap_or(false),
                default_targets: default_shapes,
                options,
            },
        }))
    }
}

impl Compile<(DeclShapeSwitch, &HashMap<String, Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (ss, parameters): (DeclShapeSwitch, &HashMap<String, Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &ss.parameter, &ParameterType::BOOL_TYPE))? {
            return Ok(None);
        };

        let mut disabled = vec![];
        let mut enabled = vec![];
        for shape in ss.shapes {
            disabled.push(ShapeTarget(
                shape.shape.clone(),
                shape.disabled.unwrap_or(0.0),
            ));
            enabled.push(ShapeTarget(
                shape.shape.clone(),
                shape.enabled.unwrap_or(1.0),
            ));
        }

        Ok(Some(AnimationGroup {
            name: ss.name,
            parameter: ss.parameter,
            content: AnimationGroupContent::ShapeSwitch {
                mesh: ss.mesh,
                prevent_mouth: ss.prevent_mouth.unwrap_or(false),
                prevent_eyelids: ss.prevent_eyelids.unwrap_or(false),
                disabled,
                enabled,
            },
        }))
    }
}

impl Compile<(DeclObjectGroup, &HashMap<String, Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (og, parameters): (DeclObjectGroup, &HashMap<String, Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &og.parameter, &ParameterType::INT_TYPE))? {
            return Ok(None);
        };

        let mut options = HashMap::new();
        let mut default_objects: Vec<_> = og
            .default_block
            .map(|b| b.objects)
            .unwrap_or_default()
            .into_iter()
            .map(|ds| ObjectTarget(ds.0, ds.1.unwrap_or(false)))
            .collect();
        let mut default_object_names: HashSet<_> =
            default_objects.iter().map(|s| s.0.clone()).collect();

        for decl_option in og.options {
            let name = decl_option.name.expect("option block must have name");
            let option: Vec<_> = decl_option
                .objects
                .into_iter()
                .map(|ds| ObjectTarget(ds.0, ds.1.unwrap_or(true)))
                .collect();

            for target in &option {
                if default_object_names.contains(&target.0) {
                    continue;
                }
                default_objects.push(ObjectTarget(target.0.clone(), false));
                default_object_names.insert(target.0.clone());
            }

            options.insert(name, (decl_option.declared_order, option));
        }

        Ok(Some(AnimationGroup {
            name: og.name,
            parameter: og.parameter,
            content: AnimationGroupContent::ObjectGroup {
                default_targets: default_objects,
                options,
            },
        }))
    }
}

impl Compile<(DeclObjectSwitch, &HashMap<String, Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (os, parameters): (DeclObjectSwitch, &HashMap<String, Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &os.parameter, &ParameterType::BOOL_TYPE))? {
            return Ok(None);
        };

        let mut disabled = vec![];
        let mut enabled = vec![];
        for object in os.objects {
            disabled.push(ObjectTarget(
                object.object.clone(),
                object.disabled.unwrap_or(false),
            ));
            enabled.push(ObjectTarget(
                object.object.clone(),
                object.enabled.unwrap_or(true),
            ));
        }

        Ok(Some(AnimationGroup {
            name: os.name,
            parameter: os.parameter,
            content: AnimationGroupContent::ObjectSwitch { disabled, enabled },
        }))
    }
}

impl Validate<(&HashMap<String, Parameter>, &str, &ParameterType)> for AvatarCompiler {
    fn validate(
        &mut self,
        (parameters, name, ty): (&HashMap<String, Parameter>, &str, &ParameterType),
    ) -> Result<bool> {
        let parameter = match parameters.get(name) {
            Some(p) => p,
            None => {
                self.error(format!("parameter '{}' not found", name));
                return Ok(false);
            }
        };
        match (&parameter.value_type, ty) {
            (ParameterType::Int(_), ParameterType::Int(_)) => Ok(true),
            (ParameterType::Float(_), ParameterType::Float(_)) => Ok(true),
            (ParameterType::Bool(_), ParameterType::Bool(_)) => Ok(true),
            _ => {
                self.error(format!(
                    "parameter '{}' has wrong type; {} expected",
                    name,
                    ty.type_name()
                ));
                Ok(false)
            }
        }
    }
}

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
