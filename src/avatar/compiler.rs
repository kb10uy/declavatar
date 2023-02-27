use crate::{
    avatar::{
        data::{
            AnimationGroup, AnimationGroupContent, AnimationOption, Avatar, ObjectTarget,
            Parameter, ParameterSync, ParameterType, ShapeTarget,
        },
        diagnostic::{CompilerInfo, Diagnostic, InstrumentKey},
        error::{AvatarError, Result},
    },
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

use std::collections::{HashMap, HashSet};

pub fn compile_avatar(avatar: DeclAvatar) -> Result<Avatar> {
    let mut ci = CompilerInfo::new();

    let name = {
        let decl_name = avatar.name.trim();
        if decl_name == "" {
            return Err(AvatarError::InvalidAvatarName(avatar.name));
        }
        decl_name.to_string()
    };

    let parameters = compile_parameters(
        avatar.parameters_blocks,
        ci.with::<Avatar>(InstrumentKey::Unkeyed),
    )?;
    let animation_groups = compile_animations(
        avatar.animations_blocks,
        &parameters,
        ci.with::<Avatar>(InstrumentKey::Unkeyed),
    )?;
    Ok(Avatar {
        name,
        parameters,
        animation_groups,
    })
}

fn compile_parameters<'a, D: Diagnostic<'a>>(
    parameters_blocks: Vec<DeclParameters>,
    mut ci: D,
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
                ci.error::<Parameter>(
                    InstrumentKey::Map(decl_parameter.name.into()),
                    format!("local parameter cannot be saved"),
                )
                .err();
                continue;
            }
        };

        match parameters.entry(decl_parameter.name.clone()) {
            Entry::Occupied(p) => {
                let defined: &Parameter = p.get();
                if defined.value_type != value_type || defined.sync_type != sync_type {
                    ci.error::<Parameter>(
                        InstrumentKey::Map(decl_parameter.name.into()),
                        format!("incompatible declaration detected"),
                    )
                    .err();
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

    ci.commit();
    Ok(parameters)
}

fn compile_animations<'a, D: Diagnostic<'a>>(
    animations_blocks: Vec<DeclAnimations>,
    parameters: &HashMap<String, Parameter>,
    mut ci: D,
) -> Result<Vec<AnimationGroup>> {
    let mut animation_groups = vec![];

    let mut used_group_names: HashSet<String> = HashSet::new();
    let mut used_parameters: HashSet<String> = HashSet::new();
    let decl_animations = animations_blocks
        .into_iter()
        .map(|ab| ab.elements)
        .flatten();
    for (i, decl_animation) in decl_animations.into_iter().enumerate() {
        let animation_group = match decl_animation {
            DeclAnimationElement::ShapeGroup(shape_group) => {
                compile_animation_shape_group(shape_group, parameters)?
            }
            DeclAnimationElement::ShapeSwitch(shape_switch) => {
                compile_animation_shape_switch(shape_switch, parameters)?
            }
            DeclAnimationElement::ObjectGroup(object_group) => {
                compile_animation_object_group(object_group, parameters)?
            }
            DeclAnimationElement::ObjectSwitch(object_switch) => {
                compile_animation_object_switch(object_switch, parameters)?
            }
        };

        if used_group_names.contains(&animation_group.name) {
            ci.warn::<AnimationGroup>(
                InstrumentKey::Array(i),
                format!(
                    "group name '{}' is used multiple times",
                    animation_group.name
                ),
            );
        } else {
            used_group_names.insert(animation_group.name.clone());
        }

        if used_parameters.contains(&animation_group.parameter) {
            ci.warn::<AnimationGroup>(
                InstrumentKey::Map(animation_group.name.clone()),
                format!(
                    "parameter '{}' is used multiple times",
                    animation_group.parameter
                ),
            );
        } else {
            used_parameters.insert(animation_group.parameter.clone());
        }

        animation_groups.push(animation_group);
    }

    Ok(animation_groups)
}

fn ensure_parameter(
    parameters: &HashMap<String, Parameter>,
    name: &str,
    ty: &ParameterType,
    used_by: &str,
    // ci: &mut D,
) -> Result<()> {
    let parameter = match parameters.get(name) {
        Some(p) => p,
        None => {
            return Err(AvatarError::ParameterNotFound {
                name: name.to_string(),
                used_by: used_by.to_string(),
            })
        }
    };
    match (&parameter.value_type, ty) {
        (ParameterType::Int(_), ParameterType::Int(_)) => Ok(()),
        (ParameterType::Float(_), ParameterType::Float(_)) => Ok(()),
        (ParameterType::Bool(_), ParameterType::Bool(_)) => Ok(()),
        _ => Err(AvatarError::WrongParameterType {
            name: name.to_string(),
            used_by: used_by.to_string(),
            expected: ty.type_name(),
        }),
    }
}

fn compile_animation_shape_group(
    sg: DeclShapeGroup,
    parameters: &HashMap<String, Parameter>,
) -> Result<AnimationGroup> {
    ensure_parameter(
        parameters,
        &sg.parameter,
        &ParameterType::INT_TYPE,
        &sg.name,
    )?;

    let mut options = HashMap::new();
    let mut default_shapes: Vec<_> = sg
        .default_block
        .map(|b| b.shapes)
        .unwrap_or_default()
        .into_iter()
        .map(|ds| ShapeTarget(ds.0, ds.1.unwrap_or(0.0)))
        .collect();
    let mut default_shape_names: HashSet<_> = default_shapes.iter().map(|s| s.0.clone()).collect();

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

    Ok(AnimationGroup {
        name: sg.name,
        parameter: sg.parameter,
        content: AnimationGroupContent::ShapeGroup {
            mesh: sg.mesh,
            prevent_mouth: sg.prevent_mouth.unwrap_or(false),
            prevent_eyelids: sg.prevent_eyelids.unwrap_or(false),
            options,
        },
    })
}

fn compile_animation_shape_switch(
    ss: DeclShapeSwitch,
    parameters: &HashMap<String, Parameter>,
) -> Result<AnimationGroup> {
    ensure_parameter(
        parameters,
        &ss.parameter,
        &ParameterType::BOOL_TYPE,
        &ss.name,
    )?;

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

    Ok(AnimationGroup {
        name: ss.name,
        parameter: ss.parameter,
        content: AnimationGroupContent::ShapeSwitch {
            mesh: ss.mesh,
            prevent_mouth: ss.prevent_mouth.unwrap_or(false),
            prevent_eyelids: ss.prevent_eyelids.unwrap_or(false),
            disabled,
            enabled,
        },
    })
}

fn compile_animation_object_group(
    og: DeclObjectGroup,
    parameters: &HashMap<String, Parameter>,
) -> Result<AnimationGroup> {
    ensure_parameter(
        parameters,
        &og.parameter,
        &ParameterType::INT_TYPE,
        &og.name,
    )?;

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

    Ok(AnimationGroup {
        name: og.name,
        parameter: og.parameter,
        content: AnimationGroupContent::ObjectGroup { options },
    })
}

fn compile_animation_object_switch(
    os: DeclObjectSwitch,
    parameters: &HashMap<String, Parameter>,
) -> Result<AnimationGroup> {
    ensure_parameter(
        parameters,
        &os.parameter,
        &ParameterType::BOOL_TYPE,
        &os.name,
    )?;

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

    Ok(AnimationGroup {
        name: os.name,
        parameter: os.parameter,
        content: AnimationGroupContent::ObjectSwitch { disabled, enabled },
    })
}
