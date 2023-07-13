use crate::{
    avatar::{
        compiler::{AvatarCompiler, CompiledDependencies},
        data::{
            AnimationGroup, AnimationGroupContent, GroupOption, LayerAnimation, LayerBlendTree,
            LayerBlendTreeField, LayerBlendTreeType, LayerCondition, LayerState, LayerTransition,
            MaterialTarget, ObjectTarget, ParameterType, Preventions, PuppetKeyframe, ShapeTarget,
            Target,
        },
        error::Result,
    },
    compiler::{Compile, Compiler, Validate},
    decl::data::{
        AnimationElement as DeclAnimationElement, AnimationGroup as DeclAnimationGroup,
        AnimationSwitch as DeclAnimationSwitch, Animations as DeclAnimations,
        AssetType as DeclAssetType, DriveTarget as DeclDriveTarget, GroupBlock as DeclGroupBlock,
        Layer as DeclLayer, LayerAnimation as DeclLayerAnimation,
        LayerBlendTreeType as DeclLayerBlendTreeType, LayerCondition as DeclLayerCondition,
        LayerState as DeclLayerState, Puppet as DeclPuppet, Target as DeclTarget,
    },
};

use std::collections::HashSet;

impl Compile<(Vec<DeclAnimations>, &CompiledDependencies)> for AvatarCompiler {
    type Output = Vec<AnimationGroup>;

    fn compile(
        &mut self,
        (animations_blocks, compiled_deps): (Vec<DeclAnimations>, &CompiledDependencies),
    ) -> Result<Vec<AnimationGroup>> {
        let mut animation_groups = vec![];

        let mut used_group_names: HashSet<String> = HashSet::new();
        let decl_animations = animations_blocks.into_iter().flat_map(|ab| ab.elements);
        for decl_animation in decl_animations {
            let Some(animation_group) = (match decl_animation {
                DeclAnimationElement::Group(group) => {
                    self.compile((group, compiled_deps))?
                }
                DeclAnimationElement::Switch(switch) => {
                    self.compile((switch, compiled_deps))?
                }
                DeclAnimationElement::Puppet(puppet) => {
                    self.compile((puppet, compiled_deps))?
                }
                DeclAnimationElement::Layer(layer) => {
                    self.compile((layer, compiled_deps))?
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

            animation_groups.push(animation_group);
        }

        Ok(animation_groups)
    }
}

impl Compile<(DeclAnimationGroup, &CompiledDependencies)> for AvatarCompiler {
    type Output = Option<AnimationGroup>;

    fn compile(
        &mut self,
        (group, compiled_deps): (DeclAnimationGroup, &CompiledDependencies),
    ) -> Result<Option<AnimationGroup>> {
        if !self.ensure((
            &compiled_deps.parameters,
            group.parameter.as_str(),
            ParameterType::INT_TYPE,
            true,
        ))? {
            return Ok(None);
        };

        let default_mesh = group.default_mesh.as_deref();
        let mut options = vec![];
        let mut default_targets: Vec<_> = match group.default_block {
            Some(db) => self
                .compile((db, compiled_deps, default_mesh, false))?
                .map(|b| b.targets)
                .unwrap_or_default(),
            None => vec![],
        };
        let mut default_shape_indices: HashSet<_> =
            default_targets.iter().map(|t| t.index()).collect();

        let canceled_defaults: Vec<_> = default_targets
            .iter()
            .flat_map(|dt| dt.clone_as_canceled())
            .collect();

        for decl_option in group.options {
            let cancel_default = decl_option.cancel_default.unwrap_or(false);
            let Some(mut option) = self.compile((decl_option, compiled_deps, default_mesh, true))? else {
                continue;
            };

            for target in &option.targets {
                let shape_index = target.index();
                if default_shape_indices.contains(&shape_index) {
                    continue;
                }
                let Some(disabled_target) = target.clone_as_disabled() else {
                    self.error(format!("disabled target cannot be generated automatically; {target:?}"));
                    return Ok(None);
                };
                default_targets.push(disabled_target);
                default_shape_indices.insert(shape_index);
            }

            if cancel_default {
                option.targets.extend_from_slice(&canceled_defaults);
            }

            options.push(option);
        }

        Ok(Some(AnimationGroup {
            name: group.name,
            content: AnimationGroupContent::Group {
                parameter: group.parameter,
                preventions: Preventions {
                    mouth: group.preventions.mouth.unwrap_or(false),
                    eyelids: group.preventions.eyelids.unwrap_or(false),
                },
                default_targets,
                options,
            },
        }))
    }
}

impl Compile<(DeclGroupBlock, &CompiledDependencies, Option<&str>, bool)> for AvatarCompiler {
    type Output = Option<GroupOption>;

    fn compile(
        &mut self,
        (group_block, compiled_deps, default_mesh, is_selection_item): (
            DeclGroupBlock,
            &CompiledDependencies,
            Option<&str>,
            bool,
        ),
    ) -> Result<Option<GroupOption>> {
        let assets = &compiled_deps.assets;

        let name = group_block.name.unwrap_or_default();
        let default_shape_value = if is_selection_item { 1.0 } else { 0.0 };

        let targets = if group_block.indeterminate {
            let block_targets = group_block.targets;
            let target = block_targets.into_iter().next();
            let Some(DeclTarget::Indeterminate {
                label,
                object,
                mesh,
                shape,
                value,
            }) = target else {
                unreachable!("must be indeterminate");
            };

            let single_target = match (mesh, shape, object, value) {
                // shape 1
                (
                    Some(mesh),
                    Some(shape),
                    None,
                    Some(DeclDriveTarget::FloatParameter { value, .. }),
                ) => Target::Shape(ShapeTarget {
                    mesh,
                    name: shape,
                    value,
                    cancel_to: None,
                }),
                (Some(mesh), Some(shape), None, None) => Target::Shape(ShapeTarget {
                    mesh,
                    name: shape,
                    value: default_shape_value,
                    cancel_to: None,
                }),
                // shape 2
                (Some(mesh), None, None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
                    Target::Shape(ShapeTarget {
                        mesh,
                        name: label,
                        value,
                        cancel_to: None,
                    })
                }
                (Some(mesh), None, None, None) => Target::Shape(ShapeTarget {
                    mesh,
                    name: label,
                    value: default_shape_value,
                    cancel_to: None,
                }),
                // shape 3
                (None, Some(shape), None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
                    let Some(mesh_name) = default_mesh else {
                        self.error(format!(
                            "shape '{}' in group '{}' has no specified mesh",
                            shape, name,
                        ));
                        return Ok(None);
                    };
                    Target::Shape(ShapeTarget {
                        mesh: mesh_name.to_string(),
                        name: shape,
                        value,
                        cancel_to: None,
                    })
                }
                (None, Some(shape), None, None) => {
                    let Some(mesh_name) = default_mesh else {
                        self.error(format!(
                            "shape '{}' in group '{}' has no specified mesh",
                            shape, name,
                        ));
                        return Ok(None);
                    };
                    Target::Shape(ShapeTarget {
                        mesh: mesh_name.to_string(),
                        name: shape,
                        value: default_shape_value,
                        cancel_to: None,
                    })
                }
                // object
                (None, None, Some(object), Some(DeclDriveTarget::BoolParameter { value, .. })) => {
                    Target::Object(ObjectTarget {
                        name: object,
                        enabled: value,
                        cancel_to: None,
                    })
                }
                (None, None, Some(object), None) => Target::Object(ObjectTarget {
                    name: object,
                    enabled: is_selection_item,
                    cancel_to: None,
                }),
                // dependent
                (None, None, None, Some(DeclDriveTarget::FloatParameter { value, .. })) => {
                    let Some(mesh_name) = default_mesh else {
                        self.error(format!(
                            "shape '{}' in group '{}' has no specified mesh",
                            label, name,
                        ));
                        return Ok(None);
                    };
                    Target::Shape(ShapeTarget {
                        mesh: mesh_name.to_string(),
                        name: label,
                        value,
                        cancel_to: None,
                    })
                }
                (None, None, None, Some(DeclDriveTarget::BoolParameter { value, .. })) => {
                    Target::Object(ObjectTarget {
                        name: label,
                        enabled: value,
                        cancel_to: None,
                    })
                }
                (None, None, None, None) => {
                    if let Some(mesh_name) = default_mesh {
                        Target::Shape(ShapeTarget {
                            mesh: mesh_name.to_string(),
                            name: label,
                            value: default_shape_value,
                            cancel_to: None,
                        })
                    } else {
                        Target::Object(ObjectTarget {
                            name: label,
                            enabled: is_selection_item,
                            cancel_to: None,
                        })
                    }
                }
                // indeterminate
                _ => {
                    self.error(format!(
                        "indeterminate option definition in {}: {}",
                        name, label,
                    ));
                    return Ok(None);
                }
            };
            vec![single_target]
        } else {
            let mut targets = vec![];
            for decl_target in group_block.targets {
                let target = match decl_target {
                    DeclTarget::Shape {
                        shape,
                        mesh,
                        value,
                        cancel_to,
                    } => {
                        let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                            self.error(format!("shape '{shape}' in group '{name}' has no specified mesh"));
                            continue;
                        };
                        Target::Shape(ShapeTarget {
                            mesh: mesh_name.to_string(),
                            name: shape,
                            value: value.unwrap_or(default_shape_value),
                            cancel_to,
                        })
                    }
                    DeclTarget::Object {
                        object,
                        value,
                        cancel_to,
                    } => Target::Object(ObjectTarget {
                        name: object,
                        enabled: value.unwrap_or(is_selection_item),
                        cancel_to,
                    }),
                    DeclTarget::Material {
                        slot,
                        value,
                        mesh,
                        cancel_to,
                    } => {
                        let Some(asset_key) = value else {
                            self.error(format!("material change for '{slot}' must have material"));
                            continue;
                        };
                        if !self.validate((assets, &asset_key, DeclAssetType::Material))? {
                            continue;
                        }

                        if let Some(cancel_asset_key) = &cancel_to {
                            if !self.validate((
                                assets,
                                cancel_asset_key,
                                DeclAssetType::Material,
                            ))? {
                                continue;
                            }
                        }

                        let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                            self.error(format!("material change slot '{slot}' in group '{name}' has no specified mesh"));
                            continue;
                        };

                        Target::Material(MaterialTarget {
                            mesh: mesh_name.to_string(),
                            slot,
                            asset_key: asset_key.key,
                            cancel_to: cancel_to.map(|c| c.key),
                        })
                    }
                    DeclTarget::Indeterminate { .. } => unreachable!("must be determinate"),
                };
                targets.push(target);
            }
            targets
        };

        Ok(Some(GroupOption {
            name,
            order: group_block.declared_order,
            targets,
        }))
    }
}

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
            true,
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
                    if !self.validate((assets, &asset_key, DeclAssetType::Material))? {
                        continue;
                    }

                    let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                        self.error(format!("material change slot '{slot}' in switch '{}' has no specified mesh", switch.name));
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
                    if !self.validate((assets, &asset_key, DeclAssetType::Material))? {
                        continue;
                    }

                    let Some(mesh_name) = mesh.as_deref().or(default_mesh) else {
                        self.error(format!("material change slot '{slot}' in switch '{}' has no specified mesh", switch.name));
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
            true,
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
                        if !self.validate((assets, &asset_key, DeclAssetType::Material))? {
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
            let Some(state) = self.compile((state, &state_names, compiled_deps))? else {
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
                if !self.validate((assets, &anim, DeclAssetType::Animation))? {
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

                let mut fields = vec![];
                for decl_field in bt.fields {
                    if !self.validate((assets, &decl_field.clip, DeclAssetType::Animation))? {
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
            let Some(target) = state_names.iter().position(|n| &decl_transition.target == n) else {
                self.error(format!("state {} not found", decl_transition.target));
                continue;
            };
            let duration = decl_transition.duration.unwrap_or(0.0);

            let mut conditions = vec![];
            for decl_condition in decl_transition.conditions {
                let condition = match decl_condition {
                    DeclLayerCondition::Be(param) => {
                        if self.validate((
                            parameters,
                            param.as_str(),
                            ParameterType::BOOL_TYPE,
                            false,
                        ))? {
                            LayerCondition::Be(param)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::Not(param) => {
                        if self.validate((
                            parameters,
                            param.as_str(),
                            ParameterType::BOOL_TYPE,
                            false,
                        ))? {
                            LayerCondition::Not(param)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::EqInt(param, value) => {
                        if self.validate((
                            parameters,
                            param.as_str(),
                            ParameterType::INT_TYPE,
                            false,
                        ))? {
                            LayerCondition::EqInt(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::NeqInt(param, value) => {
                        if self.validate((
                            parameters,
                            param.as_str(),
                            ParameterType::INT_TYPE,
                            false,
                        ))? {
                            LayerCondition::NeqInt(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::GtInt(param, value) => {
                        if self.validate((
                            parameters,
                            param.as_str(),
                            ParameterType::INT_TYPE,
                            false,
                        ))? {
                            LayerCondition::GtInt(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::LeInt(param, value) => {
                        if self.validate((
                            parameters,
                            param.as_str(),
                            ParameterType::INT_TYPE,
                            false,
                        ))? {
                            LayerCondition::LeInt(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::GtFloat(param, value) => {
                        if self.validate((
                            parameters,
                            param.as_str(),
                            ParameterType::FLOAT_TYPE,
                            false,
                        ))? {
                            LayerCondition::GtFloat(param, value)
                        } else {
                            return Ok(None);
                        }
                    }
                    DeclLayerCondition::LeFloat(param, value) => {
                        if self.validate((
                            parameters,
                            param.as_str(),
                            ParameterType::FLOAT_TYPE,
                            false,
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

        Ok(Some(LayerState {
            name: state.name,
            animation,
            speed: state.speed,
            time: state.time,
            transitions,
        }))
    }
}
