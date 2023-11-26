use crate::{
    avatar::{
        data::{
            AnimationGroup, AnimationGroupContent, AssetType, GroupOption, MaterialTarget,
            ObjectTarget, ParameterScope, ParameterType, Preventions, ShapeTarget, Target,
        },
        transformer::{
            dependencies::CompiledSources, failure, success, Compiled, Context, LogKind,
        },
    },
    decl::data::{
        AnimationElement as DeclAnimationElement, AnimationGroup as DeclAnimationGroup,
        AnimationSwitch as DeclAnimationSwitch, Animations as DeclAnimations,
        DriveTarget as DeclDriveTarget, GroupBlock as DeclGroupBlock, Layer as DeclLayer,
        Puppet as DeclPuppet, Target as DeclTarget,
    },
};

use std::collections::HashSet;

pub fn compile_animations_blocks(
    ctx: &mut Context,
    sources: &CompiledSources,
    animations_blocks: Vec<DeclAnimations>,
) -> Compiled<Vec<AnimationGroup>> {
    let mut animation_groups = vec![];

    let mut used_group_names: HashSet<String> = HashSet::new();
    let decl_animations = animations_blocks.into_iter().flat_map(|ab| ab.elements);
    for decl_animation in decl_animations {
        let animation_group = match decl_animation {
            DeclAnimationElement::Group(group) => compile_group(ctx, sources, group),
            DeclAnimationElement::Switch(switch) => compile_switch(ctx, sources, switch),
            DeclAnimationElement::Puppet(puppet) => compile_puppet(ctx, sources, puppet),
            DeclAnimationElement::Layer(layer) => compile_raw_layer(ctx, sources, layer),
        };
        let Some(animation_group) = animation_group else {
            continue;
        };

        if used_group_names.contains(&animation_group.name) {
            ctx.log_warn(LogKind::DuplicateGroupName(animation_group.name.clone()));
        } else {
            used_group_names.insert(animation_group.name.clone());
        }

        animation_groups.push(animation_group);
    }

    success(animation_groups)
}

fn compile_group(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_group: DeclAnimationGroup,
) -> Compiled<AnimationGroup> {
    sources.find_parameter(
        ctx,
        &decl_group.parameter,
        ParameterType::INT_TYPE,
        ParameterScope::MAYBE_INTERNAL,
    )?;

    let default_mesh = decl_group.default_mesh.as_deref();
    let mut options = vec![];
    let mut default_targets: Vec<_> = match decl_group.default_block {
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
}

fn compile_group_option(
    ctx: &mut Context,
    sources: &CompiledSources,
    default_mesh: Option<&str>,
    default_to_one: bool,
    decl_group_block: DeclGroupBlock,
) -> Compiled<GroupOption> {
    let name = decl_group_block.name.unwrap_or_default();

    let targets = if decl_group_block.indeterminate {
        let block_targets = decl_group_block.targets;
        let target = block_targets.into_iter().next();
        let Some(DeclTarget::Indeterminate {
            label,
            object,
            mesh,
            shape,
            value,
        }) = target
        else {
            unreachable!("must be indeterminate");
        };

        if let Some(single_target) = compile_indeterminate_target(
            ctx,
            sources,
            &name,
            default_mesh,
            default_to_one,
            (label, mesh, shape, object, value),
        ) {
            vec![single_target]
        } else {
            vec![]
        }
    } else {
        let mut targets = vec![];
        for decl_target in decl_group_block.targets {
            let Some(target) = compile_animation_target(
                ctx,
                sources,
                &name,
                default_mesh,
                default_to_one,
                decl_target,
            ) else {
                continue;
            };
            targets.push(target);
        }
        targets
    };

    success(GroupOption {
        name,
        order: decl_group_block.declared_order,
        targets,
    })
}

fn compile_animation_target(
    ctx: &mut Context,
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

fn compile_indeterminate_target(
    ctx: &mut Context,
    sources: &CompiledSources,
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

fn compile_switch(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_switch: DeclAnimationSwitch,
) -> Compiled<AnimationGroup> {
    failure()
}

fn compile_puppet(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_puppet: DeclPuppet,
) -> Compiled<AnimationGroup> {
    failure()
}

fn compile_raw_layer(
    ctx: &mut Context,
    sources: &CompiledSources,
    decl_layer: DeclLayer,
) -> Compiled<AnimationGroup> {
    failure()
}

/*

impl Compile<(DeclAnimationSwitch, &CompiledDependencies)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (switch, compiled_deps): (DeclAnimationSwitch, &CompiledDependencies),
    ) -> Result<Option<AnimationGroup>> {
        let parameters = &compiled_deps.parameters;
        let assets = &compiled_deps.assets;

        if !self.ensure((
            parameters,
            switch.parameter.as_str(),
            ParameterType::BOOL_TYPE,
            ParameterScope::MAYBE_INTERNAL,
        ))? {
            return Ok(None);
        };

        let mut disabled = vec![];
        let mut enabled = vec![];
        let default_mesh = switch.default_mesh.as_deref();
        for target in switch.enabled {
            match target {
                DeclTarget::Shape {
                    shape, mesh, value, ..
                } => {
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
                        cancel_to: None,
                    }));
                }
                DeclTarget::Object { object, value, .. } => {
                    enabled.push(Target::Object(ObjectTarget {
                        name: object,
                        enabled: value.unwrap_or(true),
                        cancel_to: None,
                    }));
                }
                DeclTarget::Material {
                    slot, value, mesh, ..
                } => {
                    let Some(asset_key) = value else {
                        self.error(format!("material change for '{slot}' must have material"));
                        continue;
                    };
                    if !self.ensure((assets, &asset_key, DeclAssetType::Material))? {
                        continue;
                    }

                    let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                        self.error(format!(
                            "material change slot '{slot}' in switch '{}' has no specified mesh",
                            switch.name
                        ));
                        continue;
                    };

                    enabled.push(Target::Material(MaterialTarget {
                        mesh: mesh_name.to_string(),
                        slot,
                        asset_key: asset_key.key,
                        cancel_to: None,
                    }));
                }
                DeclTarget::Indeterminate { .. } => unreachable!("must be determinate"),
            }
        }
        for target in switch.disabled {
            match target {
                DeclTarget::Shape {
                    shape, mesh, value, ..
                } => {
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
                        cancel_to: None,
                    }));
                }
                DeclTarget::Object { object, value, .. } => {
                    disabled.push(Target::Object(ObjectTarget {
                        name: object,
                        enabled: value.unwrap_or(false),
                        cancel_to: None,
                    }));
                }
                DeclTarget::Material {
                    slot, value, mesh, ..
                } => {
                    let Some(asset_key) = value else {
                        self.error(format!("material change for '{slot}' must have material"));
                        continue;
                    };
                    if !self.ensure((assets, &asset_key, DeclAssetType::Material))? {
                        continue;
                    }

                    let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                        self.error(format!(
                            "material change slot '{slot}' in switch '{}' has no specified mesh",
                            switch.name
                        ));
                        continue;
                    };

                    disabled.push(Target::Material(MaterialTarget {
                        mesh: mesh_name.to_string(),
                        slot,
                        asset_key: asset_key.key,
                        cancel_to: None,
                    }));
                }
                DeclTarget::Indeterminate { .. } => unreachable!("must be determinate"),
            }
        }

        Ok(Some(AnimationGroup {
            name: switch.name,
            content: AnimationGroupContent::Switch {
                parameter: switch.parameter,
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

impl Compile<(DeclPuppet, &CompiledDependencies)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (puppet, compiled_deps): (DeclPuppet, &CompiledDependencies),
    ) -> Result<Option<AnimationGroup>> {
        let parameters = &compiled_deps.parameters;
        let assets = &compiled_deps.assets;

        if !self.ensure((
            parameters,
            puppet.parameter.as_str(),
            ParameterType::FLOAT_TYPE,
            ParameterScope::MAYBE_INTERNAL,
        ))? {
            return Ok(None);
        };

        let default_mesh = puppet.mesh.as_deref();

        let mut keyframes = vec![];
        for decl_keyframe in puppet.keyframes {
            let mut targets = vec![];
            for decl_target in decl_keyframe.targets {
                let target = match decl_target {
                    DeclTarget::Shape {
                        shape, mesh, value, ..
                    } => {
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
                            cancel_to: None,
                        })
                    }
                    DeclTarget::Object { object, value, .. } => Target::Object(ObjectTarget {
                        name: object,
                        enabled: value.unwrap_or(true),
                        cancel_to: None,
                    }),
                    DeclTarget::Material {
                        slot, value, mesh, ..
                    } => {
                        let Some(asset_key) = value else {
                            self.error(format!("material change for '{slot}' must have material"));
                            continue;
                        };
                        if !self.ensure((assets, &asset_key, DeclAssetType::Material))? {
                            continue;
                        }

                        let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                            self.error(format!("material change slot '{slot}' in puppet '{}' has no specified mesh", puppet.name));
                            continue;
                        };

                        Target::Material(MaterialTarget {
                            mesh: mesh_name.to_string(),
                            slot,
                            asset_key: asset_key.key,
                            cancel_to: None,
                        })
                    }
                    DeclTarget::Indeterminate { .. } => unreachable!("must be determinate"),
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
            content: AnimationGroupContent::Puppet {
                parameter: puppet.parameter,
                keyframes,
            },
        }))
    }
}

impl Compile<(DeclLayer, &CompiledDependencies)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (layer, compiled_deps): (DeclLayer, &CompiledDependencies),
    ) -> Result<Option<AnimationGroup>> {
        let mut compiled_states = vec![];
        // if it compiles, states will be registered same order
        let state_names: Vec<_> = layer.states.iter().map(|s| s.name.clone()).collect();
        for state in layer.states {
            let Some(state) = self.parse((state, &state_names, compiled_deps))? else {
                continue;
            };
            compiled_states.push(state);
        }
        let default_index = match layer.default_state {
            Some(ds) => {
                let Some(i) = compiled_states.iter().position(|s| s.name == ds) else {
                    self.error(format!("state {ds} not found in layer {}", layer.name));
                    return Ok(None);
                };
                i
            }
            None => 0,
        };

        Ok(Some(AnimationGroup {
            name: layer.name,
            content: AnimationGroupContent::Layer {
                default_index,
                states: compiled_states,
            },
        }))
    }
}

impl Compile<(DeclLayerState, &Vec<String>, &CompiledDependencies)> for AvatarCompiler {
    type Output = Option<LayerState>;

    fn compile(
        &mut self,
        (state, state_names, compiled_deps): (DeclLayerState, &Vec<String>, &CompiledDependencies),
    ) -> Result<Option<LayerState>> {
        let assets = &compiled_deps.assets;
        let parameters = &compiled_deps.parameters;

        let animation = match state.animation {
            DeclLayerAnimation::Clip(anim) => {
                if !self.ensure((assets, &anim, DeclAssetType::Animation))? {
                    return Ok(None);
                }
                LayerAnimation::Clip(anim.key)
            }
            DeclLayerAnimation::BlendTree(bt) => {
                let blend_type = match bt.ty {
                    Some(DeclLayerBlendTreeType::Linear) => LayerBlendTreeType::Linear,
                    Some(DeclLayerBlendTreeType::Simple2D) => LayerBlendTreeType::Simple2D,
                    Some(DeclLayerBlendTreeType::Freeform2D) => LayerBlendTreeType::Freeform2D,
                    Some(DeclLayerBlendTreeType::Cartesian2D) => LayerBlendTreeType::Cartesian2D,
                    None => {
                        self.error(format!(
                            "BlendTree type must be specified for State {}",
                            state.name
                        ));
                        return Ok(None);
                    }
                };
                let params = match blend_type {
                    LayerBlendTreeType::Linear => {
                        let Some(x) = bt.x else {
                            self.error(format!(
                                "BlendTree parameter must be specified for {}",
                                state.name
                            ));
                            return Ok(None);
                        };
                        vec![x]
                    }
                    LayerBlendTreeType::Simple2D
                    | LayerBlendTreeType::Freeform2D
                    | LayerBlendTreeType::Cartesian2D => {
                        let (Some(x), Some(y)) = (bt.x, bt.y) else {
                            self.error(format!(
                                "BlendTree parameter must be specified for {}",
                                state.name
                            ));
                            return Ok(None);
                        };
                        vec![x, y]
                    }
                };
                for param_name in &params {
                    if !self.ensure((
                        &compiled_deps.parameters,
                        param_name.as_str(),
                        ParameterType::FLOAT_TYPE,
                        ParameterScope::MAYBE_INTERNAL,
                    ))? {
                        return Ok(None);
                    };
                }

                let mut fields = vec![];
                for decl_field in bt.fields {
                    if !self.ensure((assets, &decl_field.clip, DeclAssetType::Animation))? {
                        return Ok(None);
                    }
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
        for decl_transition in state.transitions {
            let Some(target) = state_names
                .iter()
                .position(|n| &decl_transition.target == n)
            else {
                self.error(format!("state {} not found", decl_transition.target));
                continue;
            };
            let duration = decl_transition.duration.unwrap_or(0.0);

            let mut conditions = vec![];
            for decl_condition in decl_transition.conditions {
                let condition = match decl_condition {
                    DeclLayerCondition::Be(param) => {
                        if self.ensure((
                            parameters,
                            param.as_str(),
                            ParameterType::BOOL_TYPE,
                            ParameterScope::MAYBE_INTERNAL,
                        ))? {
                            LayerCondition::Be(param)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::Not(param) => {
                        if self.ensure((
                            parameters,
                            param.as_str(),
                            ParameterType::BOOL_TYPE,
                            ParameterScope::MAYBE_INTERNAL,
                        ))? {
                            LayerCondition::Not(param)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::EqInt(param, value) => {
                        if self.ensure((
                            parameters,
                            param.as_str(),
                            ParameterType::INT_TYPE,
                            ParameterScope::MAYBE_INTERNAL,
                        ))? {
                            LayerCondition::EqInt(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::NeqInt(param, value) => {
                        if self.ensure((
                            parameters,
                            param.as_str(),
                            ParameterType::INT_TYPE,
                            ParameterScope::MAYBE_INTERNAL,
                        ))? {
                            LayerCondition::NeqInt(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::GtInt(param, value) => {
                        if self.ensure((
                            parameters,
                            param.as_str(),
                            ParameterType::INT_TYPE,
                            ParameterScope::MAYBE_INTERNAL,
                        ))? {
                            LayerCondition::GtInt(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::LeInt(param, value) => {
                        if self.ensure((
                            parameters,
                            param.as_str(),
                            ParameterType::INT_TYPE,
                            ParameterScope::MAYBE_INTERNAL,
                        ))? {
                            LayerCondition::LeInt(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::GtFloat(param, value) => {
                        if self.ensure((
                            parameters,
                            param.as_str(),
                            ParameterType::FLOAT_TYPE,
                            ParameterScope::MAYBE_INTERNAL,
                        ))? {
                            LayerCondition::GtFloat(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::LeFloat(param, value) => {
                        if self.ensure((
                            parameters,
                            param.as_str(),
                            ParameterType::FLOAT_TYPE,
                            ParameterScope::MAYBE_INTERNAL,
                        ))? {
                            LayerCondition::LeFloat(param, value)
                        } else {
                            return Ok(None);
                        }
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

        if let Some(speed_param) = state.speed.1.as_deref() {
            if !self.ensure((
                &compiled_deps.parameters,
                speed_param,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            ))? {
                return Ok(None);
            };
        }
        if let Some(time_param) = state.time.as_deref() {
            if !self.ensure((
                &compiled_deps.parameters,
                time_param,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MAYBE_INTERNAL,
            ))? {
                return Ok(None);
            };
        }

        Ok(Some(LayerState {
            name: state.name,
            animation,
            speed: state.speed,
            time: state.time,
            transitions,
        }))
    }
}

*/
