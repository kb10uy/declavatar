use crate::{
    avatar::{
        compiler::AvatarCompiler,
        data::{
            AnimationGroup, AnimationGroupContent, GroupOption, ObjectTarget, Parameter,
            ParameterType, Preventions, PuppetKeyframe, ShapeTarget, Target,
        },
        error::Result,
    },
    compiler::{Compile, Compiler},
    decl::data::{
        AnimationElement as DeclAnimationElement, AnimationGroup as DeclAnimationGroup,
        AnimationSwitch as DeclAnimationSwitch, Animations as DeclAnimations,
        DriveTarget as DeclDriveTarget, GroupBlock as DeclGroupBlock, Puppet as DeclPuppet,
        Target as DeclTarget,
    },
};

use std::collections::HashSet;

impl Compile<(Vec<DeclAnimations>, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Vec<AnimationGroup>;

    fn compile(
        &mut self,
        (animations_blocks, parameters): (Vec<DeclAnimations>, &Vec<Parameter>),
    ) -> Result<Vec<AnimationGroup>> {
        let mut animation_groups = vec![];

        let mut used_group_names: HashSet<String> = HashSet::new();
        let mut used_parameters: HashSet<String> = HashSet::new();
        let decl_animations = animations_blocks.into_iter().flat_map(|ab| ab.elements);
        for decl_animation in decl_animations {
            let Some(animation_group) = (match decl_animation {
                DeclAnimationElement::Group(group) => {
                    self.compile((group, parameters))?
                }
                DeclAnimationElement::Switch(switch) => {
                    self.compile((switch, parameters))?
                }
                DeclAnimationElement::Puppet(puppet) => {
                    self.compile((puppet, parameters))?
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

impl Compile<(DeclAnimationGroup, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (group, parameters): (DeclAnimationGroup, &Vec<Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &group.parameter, &ParameterType::INT_TYPE, true))? {
            return Ok(None);
        };

        let default_mesh = group.default_mesh.as_deref();
        let mut options = vec![];
        let mut default_targets: Vec<_> = match group.default_block {
            Some(db) => self
                .compile((db, default_mesh, false))?
                .map(|b| b.targets)
                .unwrap_or_default(),
            None => vec![],
        };
        let mut default_shape_indices: HashSet<_> =
            default_targets.iter().map(|t| t.index()).collect();

        for decl_option in group.options {
            let Some(option) = self.compile((decl_option, default_mesh, true))? else {
                continue;
            };

            for target in &option.targets {
                let shape_index = target.index();
                if default_shape_indices.contains(&shape_index) {
                    continue;
                }
                default_targets.push(target.clone_as_disabled());
                default_shape_indices.insert(shape_index);
            }

            options.push(option);
        }

        Ok(Some(AnimationGroup {
            name: group.name,
            parameter: group.parameter,
            content: AnimationGroupContent::Group {
                preventions: Preventions {
                    mouth: group.preventions.mouth.unwrap_or(false),
                    eyelids: group.preventions.eyelids.unwrap_or(false),
                },
                default_targets,
                options,
            },
        }))
    }
}

impl Compile<(DeclGroupBlock, Option<&str>, bool)> for AvatarCompiler {
    type Output = Option<GroupOption>;

    fn compile(
        &mut self,
        (group_block, default_mesh, default_to_max): (DeclGroupBlock, Option<&str>, bool),
    ) -> Result<Option<GroupOption>> {
        let name = group_block.name.unwrap_or_default();
        let default_shape_value = if default_to_max { 1.0 } else { 0.0 };

        let targets = if group_block.indeterminate {
            let block_targets = group_block.targets;
            let target = block_targets.into_iter().next();
            let Some(DeclTarget::Indeterminate {
                label,
                object,
                mesh,
                shape,
                value,
            }) = target else {
                unreachable!("must be indeterminate");
            };

            let single_target = match (mesh, shape, object, value) {
                // shape 1
                (
                    Some(mesh),
                    Some(shape),
                    None,
                    Some(DeclDriveTarget::FloatParameter { value, .. }),
                ) => Target::Shape(ShapeTarget {
                    mesh,
                    name: shape,
                    value,
                }),
                (Some(mesh), Some(shape), None, None) => Target::Shape(ShapeTarget {
                    mesh,
                    name: shape,
                    value: default_shape_value,
                }),
                // shape 2
                (Some(mesh), None, None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
                    Target::Shape(ShapeTarget {
                        mesh,
                        name: label,
                        value,
                    })
                }
                (Some(mesh), None, None, None) => Target::Shape(ShapeTarget {
                    mesh,
                    name: label,
                    value: default_shape_value,
                }),
                // shape 3
                (None, Some(shape), None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
                    let Some(mesh_name) = default_mesh else {
                        self.error(format!(
                            "shape '{}' in group '{}' has no specified mesh",
                            shape, name,
                        ));
                        return Ok(None);
                    };
                    Target::Shape(ShapeTarget {
                        mesh: mesh_name.to_string(),
                        name: shape,
                        value,
                    })
                }
                (None, Some(shape), None, None) => {
                    let Some(mesh_name) = default_mesh else {
                        self.error(format!(
                            "shape '{}' in group '{}' has no specified mesh",
                            shape, name,
                        ));
                        return Ok(None);
                    };
                    Target::Shape(ShapeTarget {
                        mesh: mesh_name.to_string(),
                        name: shape,
                        value: default_shape_value,
                    })
                }
                // object
                (None, None, Some(object), Some(DeclDriveTarget::BoolParameter { value, .. })) => {
                    Target::Object(ObjectTarget {
                        name: object,
                        enabled: value,
                    })
                }
                (None, None, Some(object), None) => Target::Object(ObjectTarget {
                    name: object,
                    enabled: default_to_max,
                }),
                // dependent
                (None, None, None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
                    let Some(mesh_name) = default_mesh else {
                        self.error(format!(
                            "shape '{}' in group '{}' has no specified mesh",
                            label, name,
                        ));
                        return Ok(None);
                    };
                    Target::Shape(ShapeTarget {
                        mesh: mesh_name.to_string(),
                        name: label,
                        value,
                    })
                }
                (None, None, None, Some(DeclDriveTarget::BoolParameter { value, .. })) => {
                    Target::Object(ObjectTarget {
                        name: label,
                        enabled: value,
                    })
                }
                (None, None, None, None) => {
                    if let Some(mesh_name) = default_mesh {
                        Target::Shape(ShapeTarget {
                            mesh: mesh_name.to_string(),
                            name: label,
                            value: default_shape_value,
                        })
                    } else {
                        Target::Object(ObjectTarget {
                            name: label,
                            enabled: default_to_max,
                        })
                    }
                }
                // indeterminate
                _ => {
                    self.error(format!(
                        "indeterminate option definition in {}: {}",
                        name, label,
                    ));
                    return Ok(None);
                }
            };
            vec![single_target]
        } else {
            let mut targets = vec![];
            for decl_target in group_block.targets {
                let target = match decl_target {
                    DeclTarget::Shape { shape, mesh, value } => {
                        let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                            self.error(format!(
                                "shape '{}' in group '{}' has no specified mesh",
                                shape, name,
                            ));
                            continue;
                        };
                        Target::Shape(ShapeTarget {
                            mesh: mesh_name.to_string(),
                            name: shape,
                            value: value.unwrap_or(default_shape_value),
                        })
                    }
                    DeclTarget::Object { object, value } => Target::Object(ObjectTarget {
                        name: object,
                        enabled: value.unwrap_or(default_to_max),
                    }),
                    _ => unreachable!("must be determinate"),
                };
                targets.push(target);
            }
            targets
        };

        Ok(Some(GroupOption {
            name,
            order: group_block.declared_order,
            targets,
        }))
    }
}

impl Compile<(DeclAnimationSwitch, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (switch, parameters): (DeclAnimationSwitch, &Vec<Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((
            parameters,
            &switch.parameter,
            &ParameterType::BOOL_TYPE,
            true,
        ))? {
            return Ok(None);
        };

        let mut disabled = vec![];
        let mut enabled = vec![];
        let default_mesh = switch.default_mesh.as_deref();
        for target in switch.enabled {
            match target {
                DeclTarget::Shape { shape, mesh, value } => {
                    let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                        self.error(format!(
                            "shape '{}' in group '{}' has no specified mesh",
                            shape, switch.name,
                        ));
                        continue;
                    };
                    enabled.push(Target::Shape(ShapeTarget {
                        mesh: mesh_name.to_string(),
                        name: shape,
                        value: value.unwrap_or(1.0),
                    }));
                }
                DeclTarget::Object { object, value } => {
                    enabled.push(Target::Object(ObjectTarget {
                        name: object,
                        enabled: value.unwrap_or(true),
                    }));
                }
                _ => unreachable!("must be determinate"),
            }
        }
        for target in switch.disabled {
            match target {
                DeclTarget::Shape { shape, mesh, value } => {
                    let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                        self.error(format!(
                            "shape '{}' in group '{}' has no specified mesh",
                            shape, switch.name,
                        ));
                        continue;
                    };
                    disabled.push(Target::Shape(ShapeTarget {
                        mesh: mesh_name.to_string(),
                        name: shape,
                        value: value.unwrap_or(0.0),
                    }));
                }
                DeclTarget::Object { object, value } => {
                    disabled.push(Target::Object(ObjectTarget {
                        name: object,
                        enabled: value.unwrap_or(false),
                    }));
                }
                _ => unreachable!("must be determinate"),
            }
        }

        Ok(Some(AnimationGroup {
            name: switch.name,
            parameter: switch.parameter,
            content: AnimationGroupContent::Switch {
                preventions: Preventions {
                    mouth: switch.preventions.mouth.unwrap_or(false),
                    eyelids: switch.preventions.eyelids.unwrap_or(false),
                },
                disabled,
                enabled,
            },
        }))
    }
}

impl Compile<(DeclPuppet, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (puppet, parameters): (DeclPuppet, &Vec<Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((
            parameters,
            &puppet.parameter,
            &ParameterType::FLOAT_TYPE,
            true,
        ))? {
            return Ok(None);
        };

        let default_mesh = puppet.mesh.as_deref();

        let mut keyframes = vec![];
        for decl_keyframe in puppet.keyframes {
            let mut targets = vec![];
            for decl_target in decl_keyframe.targets {
                let target = match decl_target {
                    DeclTarget::Shape { shape, mesh, value } => {
                        let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                            self.error(format!(
                                "shape '{}' in puppet '{}' has no specified mesh",
                                shape, puppet.name,
                            ));
                            continue;
                        };
                        Target::Shape(ShapeTarget {
                            mesh: mesh_name.to_string(),
                            name: shape,
                            value: value.unwrap_or(1.0),
                        })
                    }
                    DeclTarget::Object { object, value } => Target::Object(ObjectTarget {
                        name: object,
                        enabled: value.unwrap_or(true),
                    }),
                    _ => unreachable!("must be determinate"),
                };
                targets.push(target);
            }

            keyframes.push(PuppetKeyframe {
                position: decl_keyframe.position,
                targets,
            });
        }

        Ok(Some(AnimationGroup {
            name: puppet.name,
            parameter: puppet.parameter,
            content: AnimationGroupContent::Puppet { keyframes },
        }))
    }
}
