use crate::{
    avatar::{
        data::{AnimationGroup, Avatar, Parameter, ParameterSync, ParameterType},
        error::{AvatarError, Result},
    },
    decl::{
        animations::{AnimationElement as DeclAnimationElement, Animations as DeclAnimations},
        document::Avatar as DeclAvatar,
        parameters::{ParameterType as DeclParameterType, Parameters as DeclParameters},
    },
};

use std::collections::HashMap;

pub fn compile_avatar(avatar: DeclAvatar) -> Result<Avatar> {
    let name = {
        let decl_name = avatar.name.trim();
        if decl_name == "" {
            return Err(AvatarError::InvalidAvatarName(avatar.name));
        }
        decl_name.to_string()
    };

    let parameters = compile_parameters(avatar.parameters_blocks)?;
    let animation_groups = compile_animations(avatar.animations_blocks, &parameters)?;
    Ok(Avatar {
        name,
        parameters,
        animation_groups,
    })
}

fn compile_parameters(
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
                return Err(AvatarError::CannotSaveLocalParameter(decl_parameter.name));
            }
        };

        match parameters.entry(decl_parameter.name) {
            Entry::Occupied(p) => {
                let defined: &Parameter = p.get();
                if defined.value_type != value_type || defined.sync_type != sync_type {
                    return Err(AvatarError::IncompatibleParameterDefinition(name));
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

fn compile_animations(
    animations_blocks: Vec<DeclAnimations>,
    parameters: &HashMap<String, Parameter>,
) -> Result<Vec<AnimationGroup>> {
    let mut animation_groups = vec![];

    let decl_animations = animations_blocks
        .into_iter()
        .map(|ab| ab.elements)
        .flatten();
    for decl_animation in decl_animations {
        // TODO: check name duplicates
        let animation_group = match decl_animation {
            DeclAnimationElement::ShapeGroup(sg) => {
                ensure_parameter(
                    parameters,
                    &sg.parameter,
                    &ParameterType::INT_TYPE,
                    &sg.name,
                )?;
                todo!();
            }
            DeclAnimationElement::ShapeSwitch(ss) => {
                ensure_parameter(
                    parameters,
                    &ss.parameter,
                    &ParameterType::BOOL_TYPE,
                    &ss.name,
                )?;
                todo!();
            }
            DeclAnimationElement::ObjectGroup(og) => {
                ensure_parameter(
                    parameters,
                    &og.parameter,
                    &ParameterType::INT_TYPE,
                    &og.name,
                )?;
                todo!();
            }
            DeclAnimationElement::ObjectSwitch(os) => {
                ensure_parameter(
                    parameters,
                    &os.parameter,
                    &ParameterType::BOOL_TYPE,
                    &os.name,
                )?;
                todo!();
            }
        };
        animation_groups.push(animation_group);
    }

    Ok(animation_groups)
}

fn ensure_parameter(
    parameters: &HashMap<String, Parameter>,
    name: &str,
    ty: &ParameterType,
    used_by: &str,
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
