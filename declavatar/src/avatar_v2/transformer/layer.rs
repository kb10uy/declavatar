use crate::{
    avatar_v2::{
        data::{
            asset::AssetType,
            layer::{
                Layer, LayerAnimation, LayerContent, LayerGroupOption, LayerPuppetKeyframe,
                LayerRawAnimationKind, LayerRawBlendTreeType, LayerRawCondition, LayerRawField,
                LayerRawState, LayerRawTransition, Target,
            },
            parameter::{ParameterScope, ParameterType},
        },
        logger::{Log, Logger, LoggerContext},
        transformer::{
            driver::{compile_parameter_drive, compile_tracking_control},
            failure, success, Compiled, DeclaredLayer, DeclaredLayerType, FirstPassData,
            UnsetValue,
        },
    },
    decl_v2::data::layer::{
        DeclGroupCopyMode, DeclGroupLayer, DeclGroupOption, DeclGroupOptionTarget, DeclPuppetLayer,
        DeclRawLayer, DeclRawLayerAnimation, DeclRawLayerAnimationKind, DeclRawLayerBlendTreeType,
        DeclRawLayerTransition, DeclRawLayerTransitionCondition, DeclRawLayerTransitionOrdering,
        DeclSwitchLayer,
    },
};

use std::{
    collections::BTreeMap,
    iter::{once, Once},
    vec::IntoIter as VecIntoIter,
};

use either::{Either, Left, Right};

pub fn first_pass_group_layer(
    _logger: &Logger,
    decl_group_layer: &DeclGroupLayer,
) -> Compiled<DeclaredLayer> {
    // if it compiles, order will be preserved
    let option_names = decl_group_layer
        .options
        .iter()
        .enumerate()
        .flat_map(|(di, o)| o.kind.as_selection().map(|(n, i)| (n, i.unwrap_or(di + 1))))
        .collect();
    success(DeclaredLayer {
        name: decl_group_layer.name.clone(),
        layer_type: DeclaredLayerType::Group(decl_group_layer.driven_by.clone(), option_names),
    })
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

pub fn first_pass_puppet_layer(
    _logger: &Logger,
    decl_puppet_layer: &DeclPuppetLayer,
) -> Compiled<DeclaredLayer> {
    success(DeclaredLayer {
        name: decl_puppet_layer.name.clone(),
        layer_type: DeclaredLayerType::Puppet(decl_puppet_layer.driven_by.clone()),
    })
}

pub fn first_pass_raw_layer(
    _logger: &Logger,
    decl_raw_layer: &DeclRawLayer,
) -> Compiled<DeclaredLayer> {
    // if it compiles, order will be preserved
    let state_names = decl_raw_layer
        .states
        .iter()
        .map(|s| s.name.clone())
        .collect();

    success(DeclaredLayer {
        name: decl_raw_layer.name.clone(),
        layer_type: DeclaredLayerType::Raw(state_names),
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
        let Some((Some(name), explicit_index, animation)) = compile_group_option(
            &logger,
            first_pass,
            decl_option,
            default_mesh,
            UnsetValue::Active,
        ) else {
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
                for target in targets.iter().flat_map(|t| t.clone_as_zeroed()) {
                    zeroed_option_targets.insert(target.driving_key(), target);
                }
            }

            zeroed_option_targets
                .extend(default_targets.into_iter().map(|dt| (dt.driving_key(), dt)));
            default.animation =
                LayerAnimation::Inline(zeroed_option_targets.into_values().collect());
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
            parameter: bound_parameter.name.to_string(),
            default,
            options,
        },
    })
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

    let bound_parameter = first_pass.find_parameter(
        &logger,
        &decl_switch_layer.driven_by,
        ParameterType::BOOL_TYPE,
        ParameterScope::MAYBE_INTERNAL,
    )?;
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

    success(Layer {
        name: decl_switch_layer.name,
        content: LayerContent::Switch {
            parameter: bound_parameter.name.to_string(),
            disabled,
            enabled,
        },
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

    let bound_parameter = first_pass.find_parameter(
        &logger,
        &decl_puppet_layer.driven_by,
        ParameterType::FLOAT_TYPE,
        ParameterScope::MAYBE_INTERNAL,
    )?;
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
            let Some((value, targets)) = compile_puppet_option(
                &logger,
                first_pass,
                decl_option,
                default_mesh,
                UnsetValue::Active,
            ) else {
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
            parameter: bound_parameter.name.to_string(),
            animation,
        },
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
    let layer_names = first_pass.find_raw(&logger, &decl_raw_layer.name)?;

    let mut states = vec![];
    let mut transitions = vec![];
    for (index, decl_state) in decl_raw_layer.states.into_iter().enumerate() {
        // if it compiles, order will be preserved
        let Some(animation) = compile_raw_animation_kind(&logger, first_pass, decl_state.kind)
        else {
            continue;
        };
        let state = LayerRawState {
            name: decl_state.name,
            animation,
        };
        states.push(state);

        for decl_transition in decl_state.transitions {
            let Some(transition) =
                compile_raw_transition(&logger, first_pass, decl_transition, index, layer_names)
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
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    unset_value: UnsetValue,
) -> Compiled<(Option<String>, Option<usize>, LayerAnimation)> {
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

    let (name, value) = decl_group_option
        .kind
        .as_all_selection()
        .expect("group option kind must be selection");
    let logger = logger.with_context(Context(name.clone()));

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
            let Some(targets) =
                compile_target(&logger, first_pass, default_mesh, unset_value, decl_target)
            else {
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
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    unset_value: UnsetValue,
) -> Compiled<(bool, LayerAnimation)> {
    #[derive(Debug)]
    pub struct Context(bool);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            let name = if self.0 { "enabled" } else { "disabled" };
            format!("{} option > {}", name, inner)
        }
    }

    let value = decl_group_option
        .kind
        .as_boolean()
        .expect("group option kind must be boolean");
    let logger = logger.with_context(Context(value));

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
            let Some(targets) =
                compile_target(&logger, first_pass, default_mesh, unset_value, decl_target)
            else {
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
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_group_option: DeclGroupOption,
    default_mesh: Option<&str>,
    unset_value: UnsetValue,
) -> Compiled<(f64, Vec<Target>)> {
    #[derive(Debug)]
    pub struct Context(f64);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("keyframe {} > {}", self.0, inner)
        }
    }

    let value = decl_group_option
        .kind
        .as_keyframe()
        .expect("group option kind must be keyframe");
    let logger = logger.with_context(Context(value));

    if decl_group_option.animation_asset.is_some() {
        logger.log(Log::LayerPuppetOptionMustBeInlined);
        return failure();
    }

    let mut compiled_targets = BTreeMap::new();
    for decl_target in decl_group_option.targets {
        let Some(targets) =
            compile_target(&logger, first_pass, default_mesh, unset_value, decl_target)
        else {
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
    logger: &Logger,
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
        DeclGroupOptionTarget::ParameterDrive(parameter_drive) => {
            Left(once(Target::ParameterDrive(compile_parameter_drive(
                logger,
                first_pass,
                unset_value,
                parameter_drive,
            )?)))
        }
        DeclGroupOptionTarget::TrackingControl(tracking_control) => {
            let tracking_controls: Vec<_> =
                compile_tracking_control(logger, first_pass, tracking_control)?
                    .map(Target::TrackingControl)
                    .collect();
            Right(tracking_controls.into_iter())
        }
    };
    success(target)
}

fn compile_raw_animation_kind(
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_animation_kind: DeclRawLayerAnimationKind,
) -> Compiled<LayerRawAnimationKind> {
    let animation = match decl_animation_kind {
        DeclRawLayerAnimationKind::Clip {
            animation,
            speed,
            time,
        } => {
            let animation = compile_raw_animation(logger, first_pass, animation)?;
            LayerRawAnimationKind::Clip {
                animation,
                speed: speed.0,
                speed_by: speed.1,
                time_by: time,
            }
        }
        DeclRawLayerAnimationKind::BlendTree {
            tree_type,
            fields: decl_fields,
        } => {
            let (blend_type, params) = match tree_type {
                DeclRawLayerBlendTreeType::Linear(p) => (LayerRawBlendTreeType::Linear, vec![p]),
                DeclRawLayerBlendTreeType::Simple2D(px, py) => {
                    (LayerRawBlendTreeType::Simple2D, vec![px, py])
                }
                DeclRawLayerBlendTreeType::Freeform2D(px, py) => {
                    (LayerRawBlendTreeType::Freeform2D, vec![px, py])
                }
                DeclRawLayerBlendTreeType::Cartesian2D(px, py) => {
                    (LayerRawBlendTreeType::Cartesian2D, vec![px, py])
                }
            };

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
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_animation: DeclRawLayerAnimation,
) -> Compiled<LayerAnimation> {
    let animation = match decl_animation {
        DeclRawLayerAnimation::Inline(targets) => {
            let mut compiled_targets = BTreeMap::new();
            for decl_target in targets.targets {
                let Some(targets) =
                    compile_target(logger, first_pass, None, UnsetValue::Active, decl_target)
                else {
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
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_transition: DeclRawLayerTransition,
    from_index: usize,
    layer_names: &[String],
) -> Compiled<LayerRawTransition> {
    let target_index = layer_names
        .iter()
        .position(|s| s == &decl_transition.target)?;

    let mut conditions = vec![];
    for decl_condition in decl_transition.and_terms {
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
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_condition: DeclRawLayerTransitionCondition,
) -> Compiled<LayerRawCondition> {
    let condition = match decl_condition {
        DeclRawLayerTransitionCondition::Bool(name, value) => {
            first_pass.find_parameter(
                logger,
                &name,
                ParameterType::BOOL_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            if value {
                LayerRawCondition::Be(name)
            } else {
                LayerRawCondition::Not(name)
            }
        }
        DeclRawLayerTransitionCondition::Int(name, order, value) => {
            first_pass.find_parameter(
                logger,
                &name,
                ParameterType::INT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            match order {
                DeclRawLayerTransitionOrdering::Equal => LayerRawCondition::EqInt(name, value),
                DeclRawLayerTransitionOrdering::NotEqual => LayerRawCondition::NeqInt(name, value),
                DeclRawLayerTransitionOrdering::Greater => LayerRawCondition::GtInt(name, value),
                DeclRawLayerTransitionOrdering::Lesser => LayerRawCondition::LeInt(name, value),
            }
        }
        DeclRawLayerTransitionCondition::Float(name, order, value) => {
            first_pass.find_parameter(
                logger,
                &name,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            )?;
            match order {
                DeclRawLayerTransitionOrdering::Greater => LayerRawCondition::GtFloat(name, value),
                DeclRawLayerTransitionOrdering::Lesser => LayerRawCondition::LeFloat(name, value),
                _ => {
                    logger.log(Log::LayerInvalidCondition);
                    return failure();
                }
            }
        }
    };

    success(condition)
}
