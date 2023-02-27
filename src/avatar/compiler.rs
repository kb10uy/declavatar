use crate::{
    avatar::{
        data::{
            AnimationGroup, AnimationGroupContent, AnimationOption, Avatar, ObjectTarget,
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

    let result = match compiler.parse(avatar)? {
        Some(a) => Ok(a),
        None => Err(compiler.messages().into_iter().map(|(_, m)| m).collect()),
    };
    Ok(result)
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
        Ok(Some(Avatar {
            name,
            parameters,
            animation_groups,
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

            options.insert(AnimationOption::Option(name), option);
        }

        options.insert(AnimationOption::Default, default_shapes);

        Ok(Some(AnimationGroup {
            name: sg.name,
            parameter: sg.parameter,
            content: AnimationGroupContent::ShapeGroup {
                mesh: sg.mesh,
                prevent_mouth: sg.prevent_mouth.unwrap_or(false),
                prevent_eyelids: sg.prevent_eyelids.unwrap_or(false),
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

            options.insert(AnimationOption::Option(name), option);
        }

        options.insert(AnimationOption::Default, default_objects);

        Ok(Some(AnimationGroup {
            name: og.name,
            parameter: og.parameter,
            content: AnimationGroupContent::ObjectGroup { options },
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
