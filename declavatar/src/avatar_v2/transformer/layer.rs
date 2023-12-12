use crate::{
    avatar_v2::{
        data::{
            layer::{Layer, LayerContent, LayerGroupOption, Target},
            parameter::{ParameterScope, ParameterType},
        },
        logger::{LogKind, Logger},
        transformer::{success, Compiled, CompiledSources},
    },
    decl_v2::data::layer::{
        DeclGroupLayer, DeclGroupOption, DeclGroupOptionKind, DeclPuppetLayer, DeclRawLayer,
        DeclSwitchLayer,
    },
};

pub fn compile_group_layer(
    ctx: &mut Logger,
    sources: &CompiledSources,
    decl_group_layer: DeclGroupLayer,
) -> Compiled<Layer> {
    let bound_parameter = sources.find_parameter(
        ctx,
        &decl_group_layer.driven_by,
        ParameterType::INT_TYPE,
        ParameterScope::MAYBE_INTERNAL,
    )?;

    for decl_option in decl_group_layer.options {}

    success(Layer {
        name: decl_group_layer.name,
        content: LayerContent::Group {
            parameter: bound_parameter.name.to_string(),
            default: (),
            options: (),
        },
    })

    /*


    let default_mesh = decl_group_layer.default_mesh.as_deref();
    let mut options = vec![];
    let mut default_targets: Vec<_> = match decl_group_layer.default {
        Some(db) => compile_group_option(ctx, sources, default_mesh, false, db)
            .map(|b| b.targets)
            .unwrap_or_default(),
        None => vec![],
    };
    let mut default_shape_indices: HashSet<_> = default_targets.iter().map(|t| t.index()).collect();

    let canceled_defaults: Vec<_> = default_targets
        .iter()
        .flat_map(|dt| dt.clone_as_canceled())
        .collect();

    for decl_option in decl_group.options {
        let cancel_default = decl_option.cancel_default.unwrap_or(false);
        let Some(mut option) = compile_group_option(ctx, sources, default_mesh, true, decl_option)
        else {
            continue;
        };

        for target in &option.targets {
            let shape_index = target.index();
            if default_shape_indices.contains(&shape_index) {
                continue;
            }
            let Some(disabled_target) = target.clone_as_disabled() else {
                ctx.log_error(LogKind::AnimationGroupDisabledTargetFailure(
                    decl_group.name,
                    format!("{target:?}"),
                ));
                return failure();
            };
            default_targets.push(disabled_target);
            default_shape_indices.insert(shape_index);
        }

        if cancel_default {
            option.targets.extend_from_slice(&canceled_defaults);
        }

        options.push(option);
    }

    success(AnimationGroup {
        name: decl_group.name,
        content: AnimationGroupContent::Group {
            parameter: decl_group.parameter,
            preventions: Preventions {
                mouth: decl_group.preventions.mouth.unwrap_or(false),
                eyelids: decl_group.preventions.eyelids.unwrap_or(false),
            },
            default_targets,
            options,
        },
    })
    */
}

pub fn compile_switch_layer(
    ctx: &mut Logger,
    sources: &CompiledSources,
    decl_switch_layer: DeclSwitchLayer,
) -> Compiled<Layer> {
    todo!();
}

pub fn compile_puppet_layer(
    ctx: &mut Logger,
    sources: &CompiledSources,
    decl_puppet_layer: DeclPuppetLayer,
) -> Compiled<Layer> {
    todo!();
}

pub fn compile_raw_layer(
    ctx: &mut Logger,
    sources: &CompiledSources,
    decl_raw_layer: DeclRawLayer,
) -> Compiled<Layer> {
    todo!();
}

fn compile_group_option(
    ctx: &mut Logger,
    sources: &CompiledSources,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    default_to_one: bool,
) -> Compiled<(Option<String>, Option<usize>, Vec<Target>)> {
    let DeclGroupOption {
        kind: DeclGroupOptionKind::Selection(name, value),
        targets,
    } = decl_group_option
    else {
        unreachable!("group option kind must be selection");
    };

    let targets = targets
        .into_iter()
        .filter_map(|ds| compile_target(ctx, sources, &name, default_mesh, true, ds))
        .collect();

    success((name, value, targets))
}

fn compile_animation_target(
    ctx: &mut Logger,
    sources: &CompiledSources,
    group_name: &str,
    default_mesh: Option<&str>,
    default_to_one: bool,
    decl_target: DeclTarget,
) -> Compiled<Target> {
    let default_shape_value = if default_to_one { 1.0 } else { 0.0 };

    match decl_target {
        DeclTarget::Shape {
            shape,
            mesh,
            value,
            cancel_to,
        } => {
            let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                ctx.log_error(LogKind::AnimationGroupIndeterminateShapeChange(
                    group_name.to_string(),
                    shape,
                ));
                return failure();
            };
            success(Target::Shape(ShapeTarget {
                mesh: mesh_name.to_string(),
                name: shape,
                value: value.unwrap_or(default_shape_value),
                cancel_to,
            }))
        }
        DeclTarget::Object {
            object,
            value,
            cancel_to,
        } => success(Target::Object(ObjectTarget {
            name: object,
            enabled: value.unwrap_or(default_to_one),
            cancel_to,
        })),
        DeclTarget::Material {
            slot,
            value,
            mesh,
            cancel_to,
        } => {
            let Some(asset_key) = value else {
                ctx.log_error(LogKind::AnimationGroupMaterialFailure(slot));
                return failure();
            };
            sources.find_asset(ctx, &asset_key.key, AssetType::Material)?;

            if let Some(cancel_asset_key) = &cancel_to {
                sources.find_asset(ctx, &cancel_asset_key.key, AssetType::Material)?;
            }

            let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                ctx.log_error(LogKind::AnimationGroupIndeterminateMaterialChange(
                    group_name.to_string(),
                    slot,
                ));
                return failure();
            };

            success(Target::Material(MaterialTarget {
                mesh: mesh_name.to_string(),
                slot,
                asset_key: asset_key.key,
                cancel_to: cancel_to.map(|c| c.key),
            }))
        }
        DeclTarget::Indeterminate { .. } => unreachable!("must be determinate"),
    }
}

/*
fn compile_switch(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_switch: DeclAnimationSwitch,
) -> Compiled<AnimationGroup> {
    sources.find_parameter(
        ctx,
        &decl_switch.parameter,
        ParameterType::BOOL_TYPE,
        ParameterScope::MAYBE_INTERNAL,
    )?;

    let default_mesh = decl_switch.default_mesh.as_deref();

    let disabled: Vec<_> = decl_switch
        .disabled
        .into_iter()
        .filter_map(|ds| {
            compile_animation_target(ctx, sources, &decl_switch.name, default_mesh, false, ds)
        })
        .collect();
    let enabled: Vec<_> = decl_switch
        .enabled
        .into_iter()
        .filter_map(|ds| {
            compile_animation_target(ctx, sources, &decl_switch.name, default_mesh, true, ds)
        })
        .collect();

    success(AnimationGroup {
        name: decl_switch.name,
        content: AnimationGroupContent::Switch {
            parameter: decl_switch.parameter,
            preventions: Preventions {
                mouth: decl_switch.preventions.mouth.unwrap_or(false),
                eyelids: decl_switch.preventions.eyelids.unwrap_or(false),
            },
            disabled,
            enabled,
        },
    })
}

fn compile_puppet(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_puppet: DeclPuppet,
) -> Compiled<AnimationGroup> {
    sources.find_parameter(
        ctx,
        &decl_puppet.parameter,
        ParameterType::FLOAT_TYPE,
        ParameterScope::MAYBE_INTERNAL,
    )?;

    let default_mesh = decl_puppet.mesh.as_deref();

    let mut keyframes = vec![];
    for decl_keyframe in decl_puppet.keyframes {
        let targets: Vec<_> = decl_keyframe
            .targets
            .into_iter()
            .filter_map(|ds| {
                compile_animation_target(ctx, sources, &decl_puppet.name, default_mesh, true, ds)
            })
            .collect();

        keyframes.push(PuppetKeyframe {
            position: decl_keyframe.position,
            targets,
        });
    }

    success(AnimationGroup {
        name: decl_puppet.name,
        content: AnimationGroupContent::Puppet {
            parameter: decl_puppet.parameter,
            keyframes,
        },
    })
}


fn compile_indeterminate_target(
    ctx: &mut Context,
    _sources: &CompiledSources,
    group_name: &str,
    default_mesh: Option<&str>,
    default_to_one: bool,
    (label, mesh, shape, object, value): (
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<DeclDriveTarget>,
    ),
) -> Compiled<Target> {
    let default_shape_value = if default_to_one { 1.0 } else { 0.0 };

    match (mesh, shape, object, value) {
        // shape 1
        (Some(mesh), Some(shape), None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
            success(Target::Shape(ShapeTarget {
                mesh,
                name: shape,
                value,
                cancel_to: None,
            }))
        }
        (Some(mesh), Some(shape), None, None) => success(Target::Shape(ShapeTarget {
            mesh,
            name: shape,
            value: default_shape_value,
            cancel_to: None,
        })),
        // shape 2
        (Some(mesh), None, None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
            success(Target::Shape(ShapeTarget {
                mesh,
                name: label,
                value,
                cancel_to: None,
            }))
        }
        (Some(mesh), None, None, None) => success(Target::Shape(ShapeTarget {
            mesh,
            name: label,
            value: default_shape_value,
            cancel_to: None,
        })),
        // shape 3
        (None, Some(shape), None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
            let Some(mesh_name) = default_mesh else {
                ctx.log_error(LogKind::AnimationGroupIndeterminateShapeChange(
                    group_name.to_string(),
                    shape,
                ));
                return failure();
            };
            success(Target::Shape(ShapeTarget {
                mesh: mesh_name.to_string(),
                name: shape,
                value,
                cancel_to: None,
            }))
        }
        (None, Some(shape), None, None) => {
            let Some(mesh_name) = default_mesh else {
                ctx.log_error(LogKind::AnimationGroupIndeterminateShapeChange(
                    group_name.to_string(),
                    shape,
                ));
                return failure();
            };
            success(Target::Shape(ShapeTarget {
                mesh: mesh_name.to_string(),
                name: shape,
                value: default_shape_value,
                cancel_to: None,
            }))
        }
        // object
        (None, None, Some(object), Some(DeclDriveTarget::BoolParameter { value, .. })) => {
            success(Target::Object(ObjectTarget {
                name: object,
                enabled: value,
                cancel_to: None,
            }))
        }
        (None, None, Some(object), None) => success(Target::Object(ObjectTarget {
            name: object,
            enabled: default_to_one,
            cancel_to: None,
        })),
        // dependent
        (None, None, None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
            let Some(mesh_name) = default_mesh else {
                ctx.log_error(LogKind::AnimationGroupIndeterminateShapeChange(
                    group_name.to_string(),
                    label,
                ));
                return failure();
            };
            success(Target::Shape(ShapeTarget {
                mesh: mesh_name.to_string(),
                name: label,
                value,
                cancel_to: None,
            }))
        }
        (None, None, None, Some(DeclDriveTarget::BoolParameter { value, .. })) => {
            success(Target::Object(ObjectTarget {
                name: label,
                enabled: value,
                cancel_to: None,
            }))
        }
        (None, None, None, None) => {
            if let Some(mesh_name) = default_mesh {
                success(Target::Shape(ShapeTarget {
                    mesh: mesh_name.to_string(),
                    name: label,
                    value: default_shape_value,
                    cancel_to: None,
                }))
            } else {
                success(Target::Object(ObjectTarget {
                    name: label,
                    enabled: default_to_one,
                    cancel_to: None,
                }))
            }
        }
        // indeterminate
        _ => {
            ctx.log_error(LogKind::AnimationGroupIndeterminateShapeChange(
                group_name.to_string(),
                label,
            ));
            failure()
        }
    }
}

fn compile_raw_layer(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_layer: DeclLayer,
) -> Compiled<AnimationGroup> {
    // if it compiles, states will be registered same order
    let state_names: Vec<_> = decl_layer.states.iter().map(|s| s.name.clone()).collect();

    let compiled_states: Vec<_> = decl_layer
        .states
        .into_iter()
        .filter_map(|ds| compile_raw_layer_state(ctx, sources, &decl_layer.name, &state_names, ds))
        .collect();

    let default_index = match decl_layer.default_state {
        Some(ds) => {
            let Some(i) = compiled_states.iter().position(|s| s.name == ds) else {
                ctx.log_error(LogKind::LayerStateNotFound(decl_layer.name, ds));
                return failure();
            };
            i
        }
        None => 0,
    };

    success(AnimationGroup {
        name: decl_layer.name,
        content: AnimationGroupContent::Layer {
            default_index,
            states: compiled_states,
        },
    })
}

fn compile_raw_layer_state(
    ctx: &mut Context,
    sources: &CompiledSources,
    layer_name: &str,
    state_names: &[String],
    decl_layer_state: DeclLayerState,
) -> Compiled<LayerState> {
    let animation = match decl_layer_state.animation {
        DeclLayerAnimation::Clip(anim) => {
            sources.find_asset(ctx, &anim.key, AssetType::Animation)?;
            LayerAnimation::Clip(anim.key)
        }
        DeclLayerAnimation::BlendTree(bt) => {
            let blend_type = match bt.ty {
                Some(DeclLayerBlendTreeType::Linear) => LayerBlendTreeType::Linear,
                Some(DeclLayerBlendTreeType::Simple2D) => LayerBlendTreeType::Simple2D,
                Some(DeclLayerBlendTreeType::Freeform2D) => LayerBlendTreeType::Freeform2D,
                Some(DeclLayerBlendTreeType::Cartesian2D) => LayerBlendTreeType::Cartesian2D,
                None => {
                    ctx.log_error(LogKind::LayerBlendTreeParameterNotFound(
                        layer_name.to_string(),
                        decl_layer_state.name,
                    ));
                    return failure();
                }
            };
            let params = match blend_type {
                LayerBlendTreeType::Linear => {
                    let Some(x) = bt.x else {
                        ctx.log_error(LogKind::LayerBlendTreeParameterNotFound(
                            layer_name.to_string(),
                            decl_layer_state.name,
                        ));
                        return failure();
                    };
                    vec![x]
                }
                LayerBlendTreeType::Simple2D
                | LayerBlendTreeType::Freeform2D
                | LayerBlendTreeType::Cartesian2D => {
                    let (Some(x), Some(y)) = (bt.x, bt.y) else {
                        ctx.log_error(LogKind::LayerBlendTreeParameterNotFound(
                            layer_name.to_string(),
                            decl_layer_state.name,
                        ));
                        return failure();
                    };
                    vec![x, y]
                }
            };
            for param_name in &params {
                sources.find_parameter(
                    ctx,
                    param_name,
                    ParameterType::FLOAT_TYPE,
                    ParameterScope::MAYBE_INTERNAL,
                )?;
            }

            let mut fields = vec![];
            for decl_field in bt.fields {
                sources.find_asset(ctx, &decl_field.clip.key, AssetType::Animation)?;
                fields.push(LayerBlendTreeField {
                    clip: decl_field.clip.key,
                    position: decl_field.position,
                });
            }
            LayerAnimation::BlendTree(LayerBlendTree {
                blend_type,
                params,
                fields,
            })
        }
    };

    let mut transitions = vec![];
    for decl_transition in decl_layer_state.transitions {
        let Some(target) = state_names
            .iter()
            .position(|n| &decl_transition.target == n)
        else {
            ctx.log_error(LogKind::LayerStateNotFound(
                layer_name.to_string(),
                decl_transition.target,
            ));
            return failure();
        };
        let duration = decl_transition.duration.unwrap_or(0.0);

        let mut conditions = vec![];
        for decl_condition in decl_transition.conditions {
            let condition = match decl_condition {
                DeclLayerCondition::Be(param) => {
                    sources.find_parameter(
                        ctx,
                        &param,
                        ParameterType::BOOL_TYPE,
                        ParameterScope::MAYBE_INTERNAL,
                    )?;
                    LayerCondition::Be(param)
                }
                DeclLayerCondition::Not(param) => {
                    sources.find_parameter(
                        ctx,
                        &param,
                        ParameterType::BOOL_TYPE,
                        ParameterScope::MAYBE_INTERNAL,
                    )?;
                    LayerCondition::Not(param)
                }
                DeclLayerCondition::EqInt(param, value) => {
                    sources.find_parameter(
                        ctx,
                        &param,
                        ParameterType::INT_TYPE,
                        ParameterScope::MAYBE_INTERNAL,
                    )?;
                    LayerCondition::EqInt(param, value)
                }
                DeclLayerCondition::NeqInt(param, value) => {
                    sources.find_parameter(
                        ctx,
                        &param,
                        ParameterType::INT_TYPE,
                        ParameterScope::MAYBE_INTERNAL,
                    )?;
                    LayerCondition::NeqInt(param, value)
                }
                DeclLayerCondition::GtInt(param, value) => {
                    sources.find_parameter(
                        ctx,
                        &param,
                        ParameterType::INT_TYPE,
                        ParameterScope::MAYBE_INTERNAL,
                    )?;
                    LayerCondition::GtInt(param, value)
                }
                DeclLayerCondition::LeInt(param, value) => {
                    sources.find_parameter(
                        ctx,
                        &param,
                        ParameterType::INT_TYPE,
                        ParameterScope::MAYBE_INTERNAL,
                    )?;
                    LayerCondition::LeInt(param, value)
                }
                DeclLayerCondition::GtFloat(param, value) => {
                    sources.find_parameter(
                        ctx,
                        &param,
                        ParameterType::FLOAT_TYPE,
                        ParameterScope::MAYBE_INTERNAL,
                    )?;
                    LayerCondition::GtFloat(param, value)
                }
                DeclLayerCondition::LeFloat(param, value) => {
                    sources.find_parameter(
                        ctx,
                        &param,
                        ParameterType::FLOAT_TYPE,
                        ParameterScope::MAYBE_INTERNAL,
                    )?;
                    LayerCondition::LeFloat(param, value)
                }
            };
            conditions.push(condition);
        }

        transitions.push(LayerTransition {
            target,
            duration,
            conditions,
        });
    }

    if let Some(speed_param) = decl_layer_state.speed.1.as_deref() {
        sources.find_parameter(
            ctx,
            speed_param,
            ParameterType::FLOAT_TYPE,
            ParameterScope::MAYBE_INTERNAL,
        )?;
    }
    if let Some(time_param) = decl_layer_state.time.as_deref() {
        sources.find_parameter(
            ctx,
            time_param,
            ParameterType::FLOAT_TYPE,
            ParameterScope::MAYBE_INTERNAL,
        )?;
    }

    success(LayerState {
        name: decl_layer_state.name,
        animation,
        speed: decl_layer_state.speed,
        time: decl_layer_state.time,
        transitions,
    })
}
*/
