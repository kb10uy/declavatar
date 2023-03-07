use crate::{
    avatar::{
        compiler::AvatarCompiler,
        data::{
            AnimationGroup, AnimationGroupContent, ObjectGroupOption, ObjectTarget, Parameter,
            ParameterType, ShapeGroupOption, ShapeTarget,
        },
        error::Result,
    },
    compiler::{Compile, Compiler},
    decl::data::{
        AnimationElement as DeclAnimationElement, Animations as DeclAnimations,
        ObjectGroup as DeclObjectGroup, ObjectSwitch as DeclObjectSwitch,
        ShapeGroup as DeclShapeGroup, ShapeSwitch as DeclShapeSwitch,
    },
};

use std::collections::{HashMap, HashSet};

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
        let mut default_shapes: Vec<_> = sg
            .default_block
            .map(|b| b.shapes)
            .unwrap_or_default()
            .into_iter()
            .map(|ds| ShapeTarget {
                name: ds.0,
                value: ds.1.unwrap_or(0.0),
            })
            .collect();
        let mut default_shape_names: HashSet<_> =
            default_shapes.iter().map(|s| s.name.clone()).collect();

        for decl_option in sg.options {
            let name = decl_option.name.expect("option block must have name");
            let shapes: Vec<_> = decl_option
                .shapes
                .into_iter()
                .map(|ds| ShapeTarget {
                    name: ds.0,
                    value: ds.1.unwrap_or(1.0),
                })
                .collect();

            for target in &shapes {
                if default_shape_names.contains(&target.name) {
                    continue;
                }
                default_shapes.push(ShapeTarget {
                    name: target.name.clone(),
                    value: 0.0,
                });
                default_shape_names.insert(target.name.clone());
            }

            options.push(ShapeGroupOption {
                name,
                order: decl_option.declared_order,
                shapes,
            });
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
        for shape in ss.shapes {
            disabled.push(ShapeTarget {
                name: shape.shape.clone(),
                value: shape.disabled.unwrap_or(0.0),
            });
            enabled.push(ShapeTarget {
                name: shape.shape.clone(),
                value: shape.enabled.unwrap_or(1.0),
            });
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
                name: ds.0,
                enabled: ds.1.unwrap_or(false),
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
                    name: ds.0,
                    enabled: ds.1.unwrap_or(true),
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
