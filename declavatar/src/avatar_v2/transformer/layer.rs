use crate::{
    avatar_v2::{
        data::{
            asset::AssetType,
            driver::ParameterDrive,
            layer::{Layer, LayerContent, LayerGroupOption, Target},
            parameter::{ParameterScope, ParameterType},
        },
        logger::{Log, Logger, LoggerContext},
        transformer::{
            failure, success, Compiled, DeclaredLayer, DeclaredLayerType, FirstPassData,
        },
    },
    decl_v2::data::{
        driver::DeclParameterDrive,
        layer::{
            DeclGroupLayer, DeclGroupOption, DeclGroupOptionKind, DeclGroupOptionTarget,
            DeclPuppetLayer, DeclRawLayer, DeclSwitchLayer,
        },
    },
};

pub fn first_pass_group_layer(
    _logger: &Logger,
    decl_group_layer: &DeclGroupLayer,
) -> Compiled<DeclaredLayer> {
    // if it compiles, order will be preserved
    let option_names = decl_group_layer
        .options
        .iter()
        .enumerate()
        .flat_map(|(di, o)| o.kind.as_selection().map(|(n, i)| (n, i.unwrap_or(di))))
        .collect();
    success(DeclaredLayer {
        name: decl_group_layer.name.clone(),
        layer_type: DeclaredLayerType::Group(decl_group_layer.driven_by.clone(), option_names),
    })
}

pub fn compile_group_layer(
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_group_layer: DeclGroupLayer,
) -> Compiled<Layer> {
    #[derive(Debug)]
    pub struct Context(String);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("group layer '{}' > {}", self.0, inner)
        }
    }

    let logger = logger.with_context(Context(decl_group_layer.name.clone()));

    let bound_parameter = first_pass.find_parameter(
        &logger,
        &decl_group_layer.driven_by,
        ParameterType::INT_TYPE,
        ParameterScope::MAYBE_INTERNAL,
    )?;

    for decl_option in decl_group_layer.options {}

    success(Layer {
        name: decl_group_layer.name,
        content: LayerContent::Group {
            parameter: bound_parameter.name.to_string(),
            default: todo!(),
            options: todo!(),
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
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_switch_layer: DeclSwitchLayer,
) -> Compiled<Layer> {
    #[derive(Debug)]
    pub struct Context(String);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("switch layer '{}' > {}", self.0, inner)
        }
    }

    let logger = logger.with_context(Context(decl_switch_layer.name.clone()));
    todo!();
}

pub fn first_pass_switch_layer(
    _logger: &Logger,
    decl_switch_layer: &DeclSwitchLayer,
) -> Compiled<DeclaredLayer> {
    success(DeclaredLayer {
        name: decl_switch_layer.name.clone(),
        layer_type: DeclaredLayerType::Switch(decl_switch_layer.driven_by.clone()),
    })
}

pub fn compile_puppet_layer(
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_puppet_layer: DeclPuppetLayer,
) -> Compiled<Layer> {
    #[derive(Debug)]
    pub struct Context(String);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("puppet layer '{}' > {}", self.0, inner)
        }
    }

    let logger = logger.with_context(Context(decl_puppet_layer.name.clone()));
    todo!();
}

pub fn first_pass_puppet_layer(
    _logger: &Logger,
    decl_puppet_layer: &DeclPuppetLayer,
) -> Compiled<DeclaredLayer> {
    success(DeclaredLayer {
        name: decl_puppet_layer.name.clone(),
        layer_type: DeclaredLayerType::Puppet(decl_puppet_layer.driven_by.clone()),
    })
}

pub fn compile_raw_layer(
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_raw_layer: DeclRawLayer,
) -> Compiled<Layer> {
    #[derive(Debug)]
    pub struct Context(String);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("raw layer '{}' > {}", self.0, inner)
        }
    }

    let logger = logger.with_context(Context(decl_raw_layer.name.clone()));
    todo!();
}

pub fn first_pass_raw_layer(_logger: &Logger, decl_raw_layer: &DeclRawLayer) -> Compiled<DeclaredLayer> {
    success(DeclaredLayer {
        name: decl_raw_layer.name.clone(),
        layer_type: DeclaredLayerType::Raw,
    })
}

fn compile_group_option(
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    default_to_one: bool,
) -> Compiled<(Option<String>, Option<usize>, Vec<Target>)> {
    #[derive(Debug)]
    pub struct Context(Option<String>);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            if let Some(name) = &self.0 {
                format!("option '{}' > {}", name, inner)
            } else {
                format!("default option > {}", inner)
            }
        }
    }

    let DeclGroupOption {
        kind: DeclGroupOptionKind::Selection(name, value),
        targets,
    } = decl_group_option
    else {
        unreachable!("group option kind must be selection");
    };
    let logger = logger.with_context(Context(name.clone()));

    let compiled_targets = vec![];
    success((name, value, compiled_targets))
}

fn compile_switch_option(
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    default_to_one: bool,
) -> Compiled<(bool, Vec<Target>)> {
    #[derive(Debug)]
    pub struct Context(bool);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            let name = if self.0 { "enabled" } else { "disabled" };
            format!("{} option > {}", name, inner)
        }
    }

    let DeclGroupOption {
        kind: DeclGroupOptionKind::Boolean(value),
        targets,
    } = decl_group_option
    else {
        unreachable!("switch option kind must be boolean");
    };
    let logger = logger.with_context(Context(value));

    let compiled_targets = vec![];
    success((value, compiled_targets))
}

fn compile_puppet_option(
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    default_to_one: bool,
) -> Compiled<(f64, Vec<Target>)> {
    #[derive(Debug)]
    pub struct Context(f64);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("keyframe {} > {}", self.0, inner)
        }
    }

    let DeclGroupOption {
        kind: DeclGroupOptionKind::Keyframe(value),
        targets,
    } = decl_group_option
    else {
        unreachable!("switch option kind must be keyframe");
    };
    let logger = logger.with_context(Context(value));

    let compiled_targets = vec![];

    success((value, compiled_targets))
}

fn compile_target(
    logger: &Logger,
    first_pass: &FirstPassData,
    default_mesh: Option<&str>,
    unset_value: UnsetValue,
    decl_target: DeclGroupOptionTarget,
) -> Compiled<Target> {
    let target = match decl_target {
        DeclGroupOptionTarget::Shape(shape_target) => {
            let Some(mesh) = shape_target.mesh.as_deref().or(default_mesh) else {
                logger.log(Log::LayerIndeterminateShapeChange(shape_target.shape));
                return failure();
            };
            Target::Shape {
                mesh: mesh.to_string(),
                shape: shape_target.shape,
                value: unset_value.replace_f64(shape_target.value),
            }
        }
        DeclGroupOptionTarget::Object(object_target) => Target::Object {
            object: object_target.object,
            value: unset_value.replace_bool(object_target.value),
        },
        DeclGroupOptionTarget::Material(material_target) => {
            let Some(mesh) = material_target.mesh.as_deref().or(default_mesh) else {
                logger.log(Log::LayerIndeterminateMaterialChange(material_target.index));
                return failure();
            };
            first_pass.find_asset(logger, &material_target.value, AssetType::Material)?;
            Target::Material {
                mesh: mesh.to_string(),
                index: material_target.index,
                asset: material_target.value,
            }
        }
        DeclGroupOptionTarget::ParameterDrive(parameter_drive) => {
            Target::ParameterDrive(compile_parameter_drive(logger, first_pass, parameter_drive)?)
        }
    };
    success(target)
}

fn compile_parameter_drive(
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_parameter_drive: DeclParameterDrive,
) -> Compiled<ParameterDrive> {
    match decl_parameter_drive {
        DeclParameterDrive::Group(dg) => todo!(),
        DeclParameterDrive::Switch(ds) => todo!(),
        DeclParameterDrive::Puppet(dp) => todo!(),
    }
}

/*
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
*/
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

#[derive(Debug, Clone, Copy)]
enum UnsetValue {
    Active,
    Inactive,
}

impl UnsetValue {
    pub const fn as_bool(self) -> bool {
        match self {
            UnsetValue::Active => true,
            UnsetValue::Inactive => false,
        }
    }

    pub const fn as_f64(self) -> f64 {
        match self {
            UnsetValue::Active => 1.0,
            UnsetValue::Inactive => 0.0,
        }
    }

    pub fn replace_f64(self, base: Option<f64>) -> f64 {
        base.unwrap_or(self.as_f64())
    }

    pub fn replace_bool(self, base: Option<bool>) -> bool {
        base.unwrap_or(self.as_bool())
    }
}
