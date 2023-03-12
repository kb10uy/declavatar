use crate::{
    avatar::{
        compiler::AvatarCompiler,
        data::{
            AnimationGroup, AnimationGroupContent, ObjectGroupOption, ObjectTarget, Parameter,
            ParameterType, PuppetKeyframe, ShapeGroupOption, ShapeTarget,
        },
        error::Result,
    },
    compiler::{Compile, Compiler},
    decl::data::{
        AnimationElement as DeclAnimationElement, Animations as DeclAnimations,
        ObjectGroup as DeclObjectGroup, ObjectSwitch as DeclObjectSwitch, Puppet as DeclPuppet,
        ShapeGroup as DeclShapeGroup, ShapeGroupBlock as DeclShapeGroupBlock,
        ShapeSwitch as DeclShapeSwitch,
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

impl Compile<(DeclShapeGroup, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (sg, parameters): (DeclShapeGroup, &Vec<Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &sg.parameter, &ParameterType::INT_TYPE))? {
            return Ok(None);
        };

        let mut options = vec![];
        let default_mesh = sg.mesh.as_deref();
        let mut default_targets: Vec<_> = match sg.default_block {
            Some(db) => self.compile((db, default_mesh, 0.0))?.shapes,
            None => vec![],
        };
        let mut default_shapes: HashSet<_> = default_targets
            .iter()
            .map(|s| (s.mesh.clone(), s.name.clone()))
            .collect();

        for decl_option in sg.options {
            let option = self.compile((decl_option, default_mesh, 1.0))?;

            for target in &option.shapes {
                let shape_index = (target.mesh.clone(), target.name.clone());
                if default_shapes.contains(&shape_index) {
                    continue;
                }
                default_targets.push(ShapeTarget {
                    mesh: target.mesh.clone(),
                    name: target.name.clone(),
                    value: 0.0,
                });
                default_shapes.insert(shape_index);
            }

            options.push(option);
        }

        Ok(Some(AnimationGroup {
            name: sg.name,
            parameter: sg.parameter,
            content: AnimationGroupContent::ShapeGroup {
                prevent_mouth: sg.prevent_mouth.unwrap_or(false),
                prevent_eyelids: sg.prevent_eyelids.unwrap_or(false),
                default_targets,
                options,
            },
        }))
    }
}

impl Compile<(DeclShapeGroupBlock, Option<&str>, f64)> for AvatarCompiler {
    type Output = ShapeGroupOption;

    fn compile(
        &mut self,
        (sgb, default_mesh, default_value): (DeclShapeGroupBlock, Option<&str>, f64),
    ) -> Result<ShapeGroupOption> {
        let name = sgb.name.unwrap_or_default();
        let mut shapes = vec![];

        for target in sgb.shapes {
            let Some(mesh_name) = target.mesh.as_deref().or(default_mesh) else {
                self.error(format!(
                    "shape '{}' in group '{}' has no specified mesh",
                    target.shape, name,
                ));
                continue;
            };
            shapes.push(ShapeTarget {
                mesh: mesh_name.to_string(),
                name: target.shape,
                value: target.value.unwrap_or(default_value),
            });
        }

        Ok(ShapeGroupOption {
            name,
            order: sgb.declared_order,
            shapes,
        })
    }
}

impl Compile<(DeclShapeSwitch, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (ss, parameters): (DeclShapeSwitch, &Vec<Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &ss.parameter, &ParameterType::BOOL_TYPE))? {
            return Ok(None);
        };

        let mut disabled = vec![];
        let mut enabled = vec![];
        let default_mesh = ss.mesh.as_deref();
        for shape in ss.shapes {
            let Some(mesh_name) = shape.mesh.as_deref().or(default_mesh) else {
                self.error(format!(
                    "shape '{}' in group '{}' has no specified mesh",
                    shape.shape, ss.name,
                ));
                continue;
            };
            disabled.push(ShapeTarget {
                mesh: mesh_name.to_string(),
                name: shape.shape.clone(),
                value: shape.disabled.unwrap_or(0.0),
            });
            enabled.push(ShapeTarget {
                mesh: mesh_name.to_string(),
                name: shape.shape.clone(),
                value: shape.enabled.unwrap_or(1.0),
            });
        }

        Ok(Some(AnimationGroup {
            name: ss.name,
            parameter: ss.parameter,
            content: AnimationGroupContent::ShapeSwitch {
                prevent_mouth: ss.prevent_mouth.unwrap_or(false),
                prevent_eyelids: ss.prevent_eyelids.unwrap_or(false),
                disabled,
                enabled,
            },
        }))
    }
}

impl Compile<(DeclObjectGroup, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (og, parameters): (DeclObjectGroup, &Vec<Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &og.parameter, &ParameterType::INT_TYPE))? {
            return Ok(None);
        };

        let mut options = vec![];
        let mut default_objects: Vec<_> = og
            .default_block
            .map(|b| b.objects)
            .unwrap_or_default()
            .into_iter()
            .map(|ds| ObjectTarget {
                name: ds.object,
                enabled: ds.value.unwrap_or(false),
            })
            .collect();
        let mut default_object_names: HashSet<_> =
            default_objects.iter().map(|s| s.name.clone()).collect();

        for decl_option in og.options {
            let name = decl_option.name.expect("option block must have name");
            let objects: Vec<_> = decl_option
                .objects
                .into_iter()
                .map(|ds| ObjectTarget {
                    name: ds.object,
                    enabled: ds.value.unwrap_or(true),
                })
                .collect();

            for target in &objects {
                if default_object_names.contains(&target.name) {
                    continue;
                }
                default_objects.push(ObjectTarget {
                    name: target.name.clone(),
                    enabled: false,
                });
                default_object_names.insert(target.name.clone());
            }

            options.push(ObjectGroupOption {
                name,
                order: decl_option.declared_order,
                objects,
            });
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

impl Compile<(DeclObjectSwitch, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (os, parameters): (DeclObjectSwitch, &Vec<Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &os.parameter, &ParameterType::BOOL_TYPE))? {
            return Ok(None);
        };

        let mut disabled = vec![];
        let mut enabled = vec![];
        for object in os.objects {
            disabled.push(ObjectTarget {
                name: object.object.clone(),
                enabled: object.disabled.unwrap_or(false),
            });
            enabled.push(ObjectTarget {
                name: object.object.clone(),
                enabled: object.enabled.unwrap_or(true),
            });
        }

        Ok(Some(AnimationGroup {
            name: os.name,
            parameter: os.parameter,
            content: AnimationGroupContent::ObjectSwitch { disabled, enabled },
        }))
    }
}

impl Compile<(DeclPuppet, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (puppet, parameters): (DeclPuppet, &Vec<Parameter>),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((parameters, &puppet.parameter, &ParameterType::FLOAT_TYPE))? {
            return Ok(None);
        };

        let default_mesh = puppet.mesh.as_deref();

        let mut keyframes = vec![];
        for decl_keyframe in puppet.keyframes {
            let mut shapes = vec![];
            for shape in decl_keyframe.shapes {
                let Some(mesh_name) = shape.mesh.as_deref().or(default_mesh) else {
                    self.error(format!(
                        "shape '{}' in group '{}' has no specified mesh",
                        shape.shape, puppet.name,
                    ));
                    continue;
                };
                shapes.push(ShapeTarget {
                    mesh: mesh_name.to_string(),
                    name: shape.shape,
                    value: shape.value.unwrap_or(1.0),
                });
            }

            keyframes.push(PuppetKeyframe {
                position: decl_keyframe.position,
                shapes,
            });
        }

        Ok(Some(AnimationGroup {
            name: puppet.name,
            parameter: puppet.parameter,
            content: AnimationGroupContent::Puppet { keyframes },
        }))
    }
}
