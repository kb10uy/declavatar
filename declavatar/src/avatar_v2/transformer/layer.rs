use crate::{
    avatar_v2::{
        data::{
            asset::AssetType,
            layer::{
                Layer, LayerAnimation, LayerContent, LayerGroupOption, LayerPuppetKeyframe, LayerRawAnimationKind,
                LayerRawBlendTreeType, LayerRawCondition, LayerRawField, LayerRawState, LayerRawTransition, Target,
            },
            parameter::ParameterType,
        },
        log::Log,
        transformer::{
            driver::{compile_parameter_drive, compile_tracking_control},
            failure, success, Compiled, DeclaredLayer, DeclaredLayerType, FirstPassData, UnsetValue,
        },
    },
    decl_v2::data::layer::{
        DeclGroupCopyMode, DeclGroupLayer, DeclGroupOption, DeclGroupOptionTarget, DeclPuppetLayer, DeclRawLayer,
        DeclRawLayerAnimation, DeclRawLayerAnimationKind, DeclRawLayerBlendTreeType, DeclRawLayerTransition,
        DeclRawLayerTransitionCondition, DeclRawLayerTransitionOrdering, DeclSwitchLayer,
    },
    log::Logger,
};

use std::{
    collections::BTreeMap,
    iter::{once, Once},
    vec::IntoIter as VecIntoIter,
};

use either::{Either, Left, Right};

pub fn first_pass_group_layer(_logger: &Logger<Log>, decl_group_layer: &DeclGroupLayer) -> Compiled<DeclaredLayer> {
    // if it compiles, order will be preserved
    let option_names = decl_group_layer
        .options
        .iter()
        .enumerate()
        .flat_map(|(di, o)| o.kind.as_selection().map(|(n, i)| (n, i.unwrap_or(di + 1))))
        .collect();
    success(DeclaredLayer {
        name: decl_group_layer.name.clone(),
        layer_type: DeclaredLayerType::Group(decl_group_layer.driven_by.clone().into(), option_names),
    })
}

pub fn first_pass_switch_layer(_logger: &Logger<Log>, decl_switch_layer: &DeclSwitchLayer) -> Compiled<DeclaredLayer> {
    match (&decl_switch_layer.driven_by, &decl_switch_layer.with_gate) {
        (Some(db), None) => success(DeclaredLayer {
            name: decl_switch_layer.name.clone(),
            layer_type: DeclaredLayerType::Switch(db.clone().into()),
        }),
        (None, Some(wg)) => success(DeclaredLayer {
            name: decl_switch_layer.name.clone(),
            layer_type: DeclaredLayerType::SwitchGate(wg.clone()),
        }),
        _ => failure(),
    }
}

pub fn first_pass_puppet_layer(_logger: &Logger<Log>, decl_puppet_layer: &DeclPuppetLayer) -> Compiled<DeclaredLayer> {
    success(DeclaredLayer {
        name: decl_puppet_layer.name.clone(),
        layer_type: DeclaredLayerType::Puppet(decl_puppet_layer.driven_by.clone().into()),
    })
}

pub fn first_pass_raw_layer(_logger: &Logger<Log>, decl_raw_layer: &DeclRawLayer) -> Compiled<DeclaredLayer> {
    // if it compiles, order will be preserved
    let state_names = decl_raw_layer.states.iter().map(|s| s.name.clone()).collect();

    success(DeclaredLayer {
        name: decl_raw_layer.name.clone(),
        layer_type: DeclaredLayerType::Raw(state_names),
    })
}

pub fn compile_group_layer(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_group_layer: DeclGroupLayer,
) -> Compiled<Layer> {
    let logger = logger.with_context(format!("group layer '{}'", decl_group_layer.name));

    let qualified =
        first_pass.find_read_parameter(&logger, &decl_group_layer.driven_by.into(), ParameterType::INT_TYPE)?;
    let default_mesh = decl_group_layer.default_mesh.as_deref();

    let mut default = match decl_group_layer
        .default
        .map(|d| compile_group_option(&logger, first_pass, d, default_mesh, UnsetValue::Inactive))
    {
        Some(Some((None, None, animation))) => LayerGroupOption {
            name: "<default>".to_string(),
            value: 0,
            animation,
        },
        _ => LayerGroupOption {
            name: "<default>".to_string(),
            value: 0,
            animation: LayerAnimation::default(),
        },
    };

    let mut options = vec![];
    for (index, decl_option) in decl_group_layer.options.into_iter().enumerate() {
        let Some((Some(name), explicit_index, animation)) =
            compile_group_option(&logger, first_pass, decl_option, default_mesh, UnsetValue::Active)
        else {
            continue;
        };
        options.push(LayerGroupOption {
            name,
            value: explicit_index.unwrap_or(index + 1),
            animation,
        });
    }

    match decl_group_layer.copy_mode {
        Some(DeclGroupCopyMode::ToDefaultZeroed) => {
            let LayerAnimation::Inline(default_targets) = default.animation else {
                logger.log(Log::LayerGroupInvalidCopy);
                return failure();
            };

            let mut zeroed_option_targets = BTreeMap::new();
            for option in &options {
                let LayerAnimation::Inline(targets) = &option.animation else {
                    continue;
                };
                zeroed_option_targets.extend(
                    targets
                        .iter()
                        .flat_map(|t| t.clone_as_zeroed())
                        .map(|t| (t.driving_key(), t)),
                );
            }

            zeroed_option_targets.extend(default_targets.into_iter().map(|dt| (dt.driving_key(), dt)));
            default.animation = LayerAnimation::Inline(zeroed_option_targets.into_values().collect());
        }
        Some(DeclGroupCopyMode::ToOption) => {
            let LayerAnimation::Inline(default_targets) = &default.animation else {
                logger.log(Log::LayerGroupInvalidCopy);
                return failure();
            };

            let mut copied_options = vec![];
            for mut option in options {
                let mut new_targets: BTreeMap<_, _> = default_targets
                    .iter()
                    .map(|dt| (dt.driving_key(), dt.clone()))
                    .collect();
                let LayerAnimation::Inline(option_targets) = option.animation else {
                    return failure();
                };
                new_targets.extend(option_targets.into_iter().map(|t| (t.driving_key(), t)));
                option.animation = LayerAnimation::Inline(new_targets.into_values().collect());

                copied_options.push(option);
            }
            options = copied_options;
        }
        Some(DeclGroupCopyMode::MutualZeroed) => {
            // TODO: implement this mode
            logger.log(Log::LayerGroupInvalidCopy);
            return failure();
        }
        None => (),
    }

    success(Layer {
        name: decl_group_layer.name,
        content: LayerContent::Group {
            parameter: qualified.name,
            default,
            options,
        },
    })
}

pub fn compile_switch_layer(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_switch_layer: DeclSwitchLayer,
) -> Compiled<Layer> {
    let logger = logger.with_context(format!("switch layer '{}'", decl_switch_layer.name));

    let default_mesh = decl_switch_layer.default_mesh.as_deref();

    let disabled = compile_switch_option(
        &logger,
        first_pass,
        decl_switch_layer.disabled,
        default_mesh,
        UnsetValue::Inactive,
    )
    .map(|(_, t)| t)
    .unwrap_or_default();

    let enabled = compile_switch_option(
        &logger,
        first_pass,
        decl_switch_layer.enabled,
        default_mesh,
        UnsetValue::Active,
    )
    .map(|(_, t)| t)
    .unwrap_or_default();

    match (decl_switch_layer.driven_by, decl_switch_layer.with_gate) {
        (Some(db), None) => {
            let qualified = first_pass.find_read_parameter(&logger, &db.into(), ParameterType::BOOL_TYPE)?;
            success(Layer {
                name: decl_switch_layer.name,
                content: LayerContent::Switch {
                    parameter: qualified.name,
                    disabled,
                    enabled,
                },
            })
        }
        (None, Some(wg)) => {
            let bound_gate = first_pass.find_gate(&logger, &wg)?;
            success(Layer {
                name: decl_switch_layer.name,
                content: LayerContent::SwitchGate {
                    gate: bound_gate.to_string(),
                    disabled,
                    enabled,
                },
            })
        }
        _ => {
            logger.log(Log::LayerSwitchIndeterminateSource);
            failure()
        }
    }
}

pub fn compile_puppet_layer(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_puppet_layer: DeclPuppetLayer,
) -> Compiled<Layer> {
    let logger = logger.with_context(format!("puppet layer '{}'", decl_puppet_layer.name));

    let qualified =
        first_pass.find_read_parameter(&logger, &decl_puppet_layer.driven_by.into(), ParameterType::FLOAT_TYPE)?;
    let default_mesh = decl_puppet_layer.default_mesh.as_deref();

    let animation = if let Some(animation_asset) = decl_puppet_layer.animation_asset {
        if !decl_puppet_layer.keyframes.is_empty() {
            logger.log(Log::LayerOptionMustBeExclusive);
            return failure();
        }
        first_pass.find_asset(&logger, &animation_asset, AssetType::Animation)?;
        LayerAnimation::External(animation_asset)
    } else {
        let mut keyframes = vec![];
        for decl_option in decl_puppet_layer.keyframes {
            let Some((value, targets)) =
                compile_puppet_option(&logger, first_pass, decl_option, default_mesh, UnsetValue::Active)
            else {
                continue;
            };
            if !(0.0..=1.0).contains(&value) {
                logger.log(Log::LayerKeyframeOutOfRange(value));
                continue;
            }

            let keyframe = LayerPuppetKeyframe { value, targets };
            keyframes.push(keyframe);
        }
        keyframes.sort_by(|lhs, rhs| lhs.value.total_cmp(&rhs.value));
        LayerAnimation::KeyedInline(keyframes)
    };

    success(Layer {
        name: decl_puppet_layer.name,
        content: LayerContent::Puppet {
            parameter: qualified.name,
            animation,
        },
    })
}

pub fn compile_raw_layer(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_raw_layer: DeclRawLayer,
) -> Compiled<Layer> {
    let logger = logger.with_context(format!("raw layer '{}'", decl_raw_layer.name));
    let layer_names = first_pass.find_raw(&logger, &decl_raw_layer.name)?;

    let mut states = vec![];
    let mut transitions = vec![];
    for (index, decl_state) in decl_raw_layer.states.into_iter().enumerate() {
        // if it compiles, order will be preserved
        let Some(animation) = compile_raw_animation_kind(&logger, first_pass, decl_state.kind) else {
            continue;
        };
        let state = LayerRawState {
            name: decl_state.name,
            animation,
        };
        states.push(state);

        for decl_transition in decl_state.transitions {
            let Some(transition) = compile_raw_transition(&logger, first_pass, decl_transition, index, layer_names)
            else {
                continue;
            };
            transitions.push(transition);
        }
    }
    let default_index = match decl_raw_layer.default {
        Some(dsn) => match states.iter().position(|s| s.name == dsn) {
            Some(i) => i,
            None => {
                logger.log(Log::LayerStateNotFound(dsn.clone()));
                return failure();
            }
        },
        None => 0,
    };

    success(Layer {
        name: decl_raw_layer.name,
        content: LayerContent::Raw {
            default_index,
            states,
            transitions,
        },
    })
}

fn compile_group_option(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    unset_value: UnsetValue,
) -> Compiled<(Option<String>, Option<usize>, LayerAnimation)> {
    let (name, value) = decl_group_option
        .kind
        .as_all_selection()
        .expect("group option kind must be selection");
    let logger = logger.with_context(if let Some(n) = &name {
        format!("option '{}'", n)
    } else {
        "default option".to_string()
    });

    let animation = if let Some(animation_asset) = decl_group_option.animation_asset {
        if !decl_group_option.targets.is_empty() {
            logger.log(Log::LayerOptionMustBeExclusive);
            return failure();
        }
        first_pass.find_asset(&logger, &animation_asset, AssetType::Animation)?;
        LayerAnimation::External(animation_asset)
    } else {
        let mut compiled_targets = BTreeMap::new();
        for decl_target in decl_group_option.targets {
            let Some(targets) = compile_target(&logger, first_pass, default_mesh, unset_value, decl_target) else {
                continue;
            };
            for target in targets.into_iter() {
                compiled_targets.insert(target.driving_key(), target);
            }
        }
        LayerAnimation::Inline(compiled_targets.into_values().collect())
    };

    success((name, value, animation))
}

fn compile_switch_option(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    unset_value: UnsetValue,
) -> Compiled<(bool, LayerAnimation)> {
    let value = decl_group_option
        .kind
        .as_boolean()
        .expect("group option kind must be boolean");
    let logger = logger.with_context(if value { "enabled option" } else { "disabled option" });

    let animation = if let Some(animation_asset) = decl_group_option.animation_asset {
        if !decl_group_option.targets.is_empty() {
            logger.log(Log::LayerOptionMustBeExclusive);
            return failure();
        }
        first_pass.find_asset(&logger, &animation_asset, AssetType::Animation)?;
        LayerAnimation::External(animation_asset)
    } else {
        let mut compiled_targets = BTreeMap::new();
        for decl_target in decl_group_option.targets {
            let Some(targets) = compile_target(&logger, first_pass, default_mesh, unset_value, decl_target) else {
                continue;
            };
            for target in targets.into_iter() {
                compiled_targets.insert(target.driving_key(), target);
            }
        }
        LayerAnimation::Inline(compiled_targets.into_values().collect())
    };

    success((value, animation))
}

fn compile_puppet_option(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    unset_value: UnsetValue,
) -> Compiled<(f64, Vec<Target>)> {
    let value = decl_group_option
        .kind
        .as_keyframe()
        .expect("group option kind must be keyframe");
    let logger = logger.with_context(format!("keyframe {value}"));

    if decl_group_option.animation_asset.is_some() {
        logger.log(Log::LayerPuppetOptionMustBeInlined);
        return failure();
    }

    let mut compiled_targets = BTreeMap::new();
    for decl_target in decl_group_option.targets {
        let Some(targets) = compile_target(&logger, first_pass, default_mesh, unset_value, decl_target) else {
            continue;
        };
        for target in targets.into_iter() {
            if let Target::ParameterDrive(_) = target {
                logger.log(Log::LayerPuppetCannotDrive);
                continue;
            }
            compiled_targets.insert(target.driving_key(), target);
        }
    }

    success((value, compiled_targets.into_values().collect()))
}

fn compile_target(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    default_mesh: Option<&str>,
    unset_value: UnsetValue,
    decl_target: DeclGroupOptionTarget,
) -> Compiled<Either<Once<Target>, VecIntoIter<Target>>> {
    let target = match decl_target {
        DeclGroupOptionTarget::Shape(shape_target) => {
            let Some(mesh) = shape_target.mesh.as_deref().or(default_mesh) else {
                logger.log(Log::LayerIndeterminateShapeChange(shape_target.shape));
                return failure();
            };
            Left(once(Target::Shape {
                mesh: mesh.to_string(),
                shape: shape_target.shape,
                value: unset_value.replace_f64(shape_target.value),
            }))
        }
        DeclGroupOptionTarget::Object(object_target) => Left(once(Target::Object {
            object: object_target.object,
            value: unset_value.replace_bool(object_target.value),
        })),
        DeclGroupOptionTarget::Material(material_target) => {
            let Some(mesh) = material_target.mesh.as_deref().or(default_mesh) else {
                logger.log(Log::LayerIndeterminateMaterialChange(material_target.index));
                return failure();
            };
            first_pass.find_asset(logger, &material_target.value, AssetType::Material)?;
            Left(once(Target::Material {
                mesh: mesh.to_string(),
                index: material_target.index,
                asset: material_target.value,
            }))
        }
        DeclGroupOptionTarget::MaterialProperty(material_prop_target) => {
            let Some(mesh) = material_prop_target.mesh.as_deref().or(default_mesh) else {
                logger.log(Log::LayerIndeterminateShapeChange(material_prop_target.property));
                return failure();
            };
            Left(once(Target::MaterialProperty {
                mesh: mesh.to_string(),
                property: material_prop_target.property,
                value: material_prop_target.value.into(),
            }))
        }
        DeclGroupOptionTarget::ParameterDrive(parameter_drive) => Left(once(Target::ParameterDrive(
            compile_parameter_drive(logger, first_pass, unset_value, parameter_drive)?,
        ))),
        DeclGroupOptionTarget::TrackingControl(tracking_control) => {
            let tracking_controls: Vec<_> = compile_tracking_control(logger, first_pass, tracking_control)?
                .map(Target::TrackingControl)
                .collect();
            Right(tracking_controls.into_iter())
        }
    };
    success(target)
}

fn compile_raw_animation_kind(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_animation_kind: DeclRawLayerAnimationKind,
) -> Compiled<LayerRawAnimationKind> {
    let animation = match decl_animation_kind {
        DeclRawLayerAnimationKind::Clip {
            animation,
            speed: (speed, speed_by),
            time: time_by,
        } => {
            let animation = compile_raw_animation(logger, first_pass, animation)?;
            let speed_by = speed_by
                .and_then(|q| first_pass.find_read_parameter(logger, &q.into(), ParameterType::FLOAT_TYPE))
                .map(|qp| qp.name);
            let time_by = time_by
                .and_then(|q| first_pass.find_read_parameter(logger, &q.into(), ParameterType::FLOAT_TYPE))
                .map(|qp| qp.name);
            LayerRawAnimationKind::Clip {
                animation,
                speed,
                speed_by,
                time_by,
            }
        }
        DeclRawLayerAnimationKind::BlendTree {
            tree_type,
            fields: decl_fields,
        } => {
            let (blend_type, params) = match tree_type {
                DeclRawLayerBlendTreeType::Linear(p) => (LayerRawBlendTreeType::Linear, vec![p]),
                DeclRawLayerBlendTreeType::Simple2D(px, py) => (LayerRawBlendTreeType::Simple2D, vec![px, py]),
                DeclRawLayerBlendTreeType::Freeform2D(px, py) => (LayerRawBlendTreeType::Freeform2D, vec![px, py]),
                DeclRawLayerBlendTreeType::Cartesian2D(px, py) => (LayerRawBlendTreeType::Cartesian2D, vec![px, py]),
            };
            let params = params
                .into_iter()
                .flat_map(|pr| first_pass.find_read_parameter(logger, &pr.into(), ParameterType::FLOAT_TYPE))
                .map(|qp| qp.name)
                .collect();

            let mut fields = vec![];
            for decl_field in decl_fields {
                fields.push(LayerRawField {
                    animation: compile_raw_animation(logger, first_pass, decl_field.animation)?,
                    position: decl_field.values,
                });
            }

            LayerRawAnimationKind::BlendTree {
                blend_type,
                params,
                fields,
            }
        }
    };

    success(animation)
}

fn compile_raw_animation(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_animation: DeclRawLayerAnimation,
) -> Compiled<LayerAnimation> {
    let animation = match decl_animation {
        DeclRawLayerAnimation::Inline(targets) => {
            let mut compiled_targets = BTreeMap::new();
            for decl_target in targets.targets {
                let Some(targets) = compile_target(logger, first_pass, None, UnsetValue::Active, decl_target) else {
                    continue;
                };
                for target in targets.into_iter() {
                    if let Target::ParameterDrive(_) = target {
                        logger.log(Log::LayerPuppetCannotDrive);
                        continue;
                    }
                    compiled_targets.insert(target.driving_key(), target);
                }
            }
            LayerAnimation::Inline(compiled_targets.into_values().collect())
        }
        DeclRawLayerAnimation::External(name) => {
            first_pass.find_asset(logger, &name, AssetType::Animation)?;
            LayerAnimation::External(name)
        }
    };

    success(animation)
}

fn compile_raw_transition(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_transition: DeclRawLayerTransition,
    from_index: usize,
    layer_names: &[String],
) -> Compiled<LayerRawTransition> {
    let target_index = layer_names.iter().position(|s| s == &decl_transition.target)?;

    let mut conditions = vec![];
    for decl_condition in decl_transition.conditions {
        let Some(condition) = compile_raw_condition(logger, first_pass, decl_condition) else {
            continue;
        };
        conditions.push(condition);
    }

    success(LayerRawTransition {
        from_index,
        target_index,
        duration: decl_transition.duration.unwrap_or(0.0),
        conditions,
    })
}

fn compile_raw_condition(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_condition: DeclRawLayerTransitionCondition,
) -> Compiled<LayerRawCondition> {
    let condition = match decl_condition {
        DeclRawLayerTransitionCondition::Bool(query, value) => {
            let qualified = first_pass.find_read_parameter(logger, &query.into(), ParameterType::BOOL_TYPE)?;
            if value {
                LayerRawCondition::Be(qualified.name)
            } else {
                LayerRawCondition::Not(qualified.name)
            }
        }
        DeclRawLayerTransitionCondition::Int(query, order, value) => {
            let qualified = first_pass.find_read_parameter(logger, &query.into(), ParameterType::INT_TYPE)?;
            match order {
                DeclRawLayerTransitionOrdering::Equal => LayerRawCondition::EqInt(qualified.name, value),
                DeclRawLayerTransitionOrdering::NotEqual => LayerRawCondition::NeqInt(qualified.name, value),
                DeclRawLayerTransitionOrdering::Greater => LayerRawCondition::GtInt(qualified.name, value),
                DeclRawLayerTransitionOrdering::Lesser => LayerRawCondition::LeInt(qualified.name, value),
            }
        }
        DeclRawLayerTransitionCondition::Float(query, order, value) => {
            let qualified = first_pass.find_read_parameter(logger, &query.into(), ParameterType::FLOAT_TYPE)?;
            match order {
                DeclRawLayerTransitionOrdering::Greater => LayerRawCondition::GtFloat(qualified.name, value),
                DeclRawLayerTransitionOrdering::Lesser => LayerRawCondition::LeFloat(qualified.name, value),
                _ => {
                    logger.log(Log::LayerInvalidCondition);
                    return failure();
                }
            }
        }
        DeclRawLayerTransitionCondition::Zero(query, not_zero) => {
            let qualified = first_pass.find_untyped_parameter(logger, &query.into())?;
            match qualified.value_type {
                ParameterType::Int(_) => {
                    if not_zero {
                        LayerRawCondition::NeqInt(qualified.name, 0)
                    } else {
                        LayerRawCondition::EqInt(qualified.name, 0)
                    }
                }
                ParameterType::Bool(_) => {
                    if not_zero {
                        LayerRawCondition::Be(qualified.name)
                    } else {
                        LayerRawCondition::Not(qualified.name)
                    }
                }
                _ => {
                    logger.log(Log::LayerInvalidCondition);
                    return failure();
                }
            }
        }
    };

    success(condition)
}
