use crate::decl_v2::{
    data::{
        layer::{
            DeclControllerLayer, DeclGroupLayer, DeclGroupMaterialTarget, DeclGroupObjectTarget,
            DeclGroupOption, DeclGroupOptionKind, DeclGroupOptionTarget, DeclGroupShapeTarget,
            DeclPuppetLayer, DeclRawLayer, DeclRawLayerAnimation, DeclRawLayerBlendTreeField,
            DeclRawLayerBlendTreeType, DeclRawLayerState, DeclRawLayerTransition,
            DeclRawLayerTransitionCondition, DeclRawLayerTransitionOrdering, DeclSwitchLayer,
        },
        StaticTypeName,
    },
    error::DeclSexprError,
    sexpr::{register_function, KetosResult, KetosValueExt, SeparateArguments},
};

use ketos::{Arity, Error, ExecError, Name, NameStore, Scope, Value};

pub fn register_layer_function(scope: &Scope) {
    // layer functions
    register_function(
        scope,
        "group-layer",
        declare_group_layer,
        Arity::Min(1),
        &["driven-by", "default-mesh"],
    );
    register_function(
        scope,
        "switch-layer",
        declare_switch_layer,
        Arity::Min(1),
        &["driven-by", "default-mesh"],
    );
    register_function(
        scope,
        "puppet-layer",
        declare_puppet_layer,
        Arity::Min(1),
        &["driven-by", "default-mesh"],
    );
    register_function(
        scope,
        "raw-layer",
        declare_raw_layer,
        Arity::Min(1),
        &["default"],
    );

    // option functions
    register_function(scope, "option", declare_option, Arity::Min(1), &["value"]);
    register_function(scope, "state", declare_state, Arity::Min(2), &[]);

    // set-x functions
    register_function(
        scope,
        "set-shape",
        declare_set_shape,
        Arity::Exact(1),
        &["value", "mesh"],
    );
    register_function(
        scope,
        "set-object",
        declare_set_object,
        Arity::Exact(1),
        &["value"],
    );
    register_function(
        scope,
        "set-material",
        declare_set_material,
        Arity::Exact(2),
        &["mesh"],
    );

    // raw layer functions
    register_function(
        scope,
        "clip",
        declare_clip,
        Arity::Min(1),
        &["speed", "speed-by", "time-by"],
    );
    register_function(
        scope,
        "blendtree",
        declare_blendtree,
        Arity::Min(0),
        &["type", "x", "y"],
    );
    register_function(
        scope,
        "blendtree-field",
        declare_blendtree_field,
        Arity::Range(2, 3),
        &[],
    );
    register_function(
        scope,
        "transition-to",
        declare_transition_to,
        Arity::Min(1),
        &["duration"],
    );
}

fn declare_group_layer(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let driven_by: &str = args.exact_kwarg_expect("driven-by")?;
    let default_mesh: Option<&str> = args.exact_kwarg("default-mesh")?;

    let mut default = None;
    let mut options = vec![];
    for option_value in args.args_after(function_name, 1)? {
        let option: &DeclGroupOption = option_value.downcast_foreign_ref()?;

        match option.kind {
            DeclGroupOptionKind::Selection(None, None) => {
                if default.is_some() {
                    return Err(Error::Custom(DeclSexprError::DuplicateNamedOption.into()));
                }
                default = Some(option.clone());
            }
            DeclGroupOptionKind::Selection(_, _) => {
                options.push(option.clone());
            }
            _ => {
                return Err(Error::Custom(
                    DeclSexprError::InvalidGroupOption(option.kind.clone()).into(),
                ));
            }
        }
    }

    Ok(DeclControllerLayer::Group(DeclGroupLayer {
        name: name.to_string(),
        driven_by: driven_by.to_string(),
        default_mesh: default_mesh.map(|dm| dm.to_string()),
        default,
        options,
    })
    .into())
}

fn declare_switch_layer(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let driven_by: &str = args.exact_kwarg_expect("driven-by")?;
    let default_mesh: Option<&str> = args.exact_kwarg("default-mesh")?;

    let mut disabled = None;
    let mut enabled = None;
    for option_value in args.args_after(function_name, 1)? {
        let option: &DeclGroupOption = option_value.downcast_foreign_ref()?;

        match option.kind {
            DeclGroupOptionKind::Boolean(false) => {
                if disabled.is_some() {
                    return Err(Error::Custom(DeclSexprError::DuplicateNamedOption.into()));
                }
                disabled = Some(option.clone());
            }
            DeclGroupOptionKind::Boolean(true) => {
                if enabled.is_some() {
                    return Err(Error::Custom(DeclSexprError::DuplicateNamedOption.into()));
                }
                enabled = Some(option.clone());
            }
            _ => {
                return Err(Error::Custom(
                    DeclSexprError::InvalidGroupOption(option.kind.clone()).into(),
                ));
            }
        }
    }

    let (Some(disabled), Some(enabled)) = (disabled, enabled) else {
        // TODO fill exact error
        return Err(Error::Custom(
            DeclSexprError::InvalidGroupOption(DeclGroupOptionKind::Boolean(false)).into(),
        ));
    };

    Ok(DeclControllerLayer::Switch(DeclSwitchLayer {
        name: name.to_string(),
        driven_by: driven_by.to_string(),
        default_mesh: default_mesh.map(|dm| dm.to_string()),
        disabled,
        enabled,
    })
    .into())
}

fn declare_puppet_layer(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let driven_by: &str = args.exact_kwarg_expect("driven-by")?;
    let default_mesh: Option<&str> = args.exact_kwarg("default-mesh")?;

    let mut keyframes = vec![];
    for option_value in args.args_after(function_name, 1)? {
        let option: &DeclGroupOption = option_value.downcast_foreign_ref()?;

        match option.kind {
            DeclGroupOptionKind::Keyframe(_) => {
                keyframes.push(option.clone());
            }
            DeclGroupOptionKind::Selection(_, _) => {}
            _ => {
                return Err(Error::Custom(
                    DeclSexprError::InvalidGroupOption(option.kind.clone()).into(),
                ));
            }
        }
    }

    Ok(DeclControllerLayer::Puppet(DeclPuppetLayer {
        name: name.to_string(),
        driven_by: driven_by.to_string(),
        default_mesh: default_mesh.map(|dm| dm.to_string()),
        keyframes,
    })
    .into())
}

fn declare_raw_layer(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let default: Option<&str> = args.exact_kwarg("default")?;

    let mut states = vec![];
    for state_value in args.args_after(function_name, 1)? {
        let state: &DeclRawLayerState = state_value.downcast_foreign_ref()?;
        states.push(state.clone());
    }
    Ok(DeclControllerLayer::Raw(DeclRawLayer {
        name: name.to_string(),
        default: default.map(|d| d.to_string()),
        states,
    })
    .into())
}

fn declare_option(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let kind = match args.exact_arg::<&Value>(function_name, 0)? {
        Value::Float(keyframe) => DeclGroupOptionKind::Keyframe(*keyframe),
        Value::Name(name) => match name_store.get(*name) {
            "default" => DeclGroupOptionKind::Selection(None, None),
            "disabled" => DeclGroupOptionKind::Boolean(false),
            "enabled" => DeclGroupOptionKind::Boolean(true),
            _ => {
                return Err(Error::Custom(
                    DeclSexprError::KeywordExpected("default, disabled, enabled".into()).into(),
                ));
            }
        },
        Value::String(option) => {
            let value: Option<usize> = args.exact_kwarg("value")?;
            DeclGroupOptionKind::Selection(Some(option.to_string()), value)
        }
        kind => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "'default, 'disabled, 'enabled, string, or float",
                found: kind.type_name(),
                value: Some(kind.clone()),
            }));
        }
    };

    let mut targets = vec![];
    for target_value in args.args_after(function_name, 1)? {
        let target = match target_value.type_name() {
            DeclGroupShapeTarget::TYPE_NAME => DeclGroupOptionTarget::Shape(
                target_value
                    .downcast_foreign_ref::<&DeclGroupShapeTarget>()?
                    .clone(),
            ),
            DeclGroupObjectTarget::TYPE_NAME => DeclGroupOptionTarget::Object(
                target_value
                    .downcast_foreign_ref::<&DeclGroupObjectTarget>()?
                    .clone(),
            ),
            DeclGroupMaterialTarget::TYPE_NAME => DeclGroupOptionTarget::Material(
                target_value
                    .downcast_foreign_ref::<&DeclGroupMaterialTarget>()?
                    .clone(),
            ),
            _ => {
                return Err(Error::Custom(
                    DeclSexprError::UnexpectedTypeValue(
                        target_value.type_name().to_string(),
                        "target".to_string(),
                    )
                    .into(),
                ))
            }
        };
        targets.push(target.clone());
    }

    Ok(DeclGroupOption { kind, targets }.into())
}

fn declare_set_shape(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let shape: &str = args.exact_arg(function_name, 0)?;
    let value: Option<f64> = args.exact_kwarg("value")?;
    let mesh: Option<&str> = args.exact_kwarg("mesh")?;

    Ok(DeclGroupShapeTarget {
        shape: shape.to_string(),
        value,
        mesh: mesh.map(|m| m.to_string()),
    }
    .into())
}

fn declare_set_object(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let object: &str = args.exact_arg(function_name, 0)?;
    let value: Option<bool> = args.exact_kwarg("value")?;

    Ok(DeclGroupObjectTarget {
        object: object.to_string(),
        value,
    }
    .into())
}

fn declare_set_material(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let index: usize = args.exact_arg(function_name, 0)?;
    let value: &str = args.exact_arg(function_name, 1)?;
    let mesh: Option<&str> = args.exact_kwarg("mesh")?;

    Ok(DeclGroupMaterialTarget {
        index,
        value: value.to_string(),
        mesh: mesh.map(|m| m.to_string()),
    }
    .into())
}

fn declare_state(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let animation: &DeclRawLayerAnimation = args.exact_arg(function_name, 1)?;

    let mut transitions = vec![];
    for transition_value in args.args_after(function_name, 1)? {
        let transition: &DeclRawLayerTransition = transition_value.downcast_foreign_ref()?;
        transitions.push(transition.clone());
    }

    Ok(DeclRawLayerState {
        name: name.to_string(),
        animation: animation.clone(),
        transitions,
    }
    .into())
}

fn declare_clip(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let speed: Option<f64> = args.exact_kwarg("speed")?;
    let speed_by: Option<&str> = args.exact_kwarg("speed-by")?;
    let time_by: Option<&str> = args.exact_kwarg("time-by")?;

    Ok(DeclRawLayerAnimation::Clip {
        name: name.to_string(),
        speed: (speed, speed_by.map(|s| s.to_string())),
        time: time_by.map(|t| t.to_string()),
    }
    .into())
}

fn declare_blendtree(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let blend_type: &str = match args.exact_kwarg_expect::<&Value>("type")? {
        Value::Name(s) => name_store.get(*s),
        v => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "blendtree type name",
                found: v.type_name(),
                value: Some(v.clone()),
            }))
        }
    };
    let tree_type = match blend_type {
        "linear" => {
            let x: &str = args.exact_kwarg_expect("x")?;
            DeclRawLayerBlendTreeType::Linear(x.to_string())
        }
        "2d-simple" => {
            let x: &str = args.exact_kwarg_expect("x")?;
            let y: &str = args.exact_kwarg_expect("y")?;
            DeclRawLayerBlendTreeType::Simple2D(x.to_string(), y.to_string())
        }
        "2d-freeform" => {
            let x: &str = args.exact_kwarg_expect("x")?;
            let y: &str = args.exact_kwarg_expect("y")?;
            DeclRawLayerBlendTreeType::Freeform2D(x.to_string(), y.to_string())
        }
        "2d-cartesian" => {
            let x: &str = args.exact_kwarg_expect("x")?;
            let y: &str = args.exact_kwarg_expect("y")?;
            DeclRawLayerBlendTreeType::Cartesian2D(x.to_string(), y.to_string())
        }
        _ => {
            return Err(Error::Custom(
                DeclSexprError::KeywordExpected("blendtree type name".to_string()).into(),
            ))
        }
    };

    let mut fields = vec![];
    for field_value in args.args_after(function_name, 0)? {
        let field: &DeclRawLayerBlendTreeField = field_value.downcast_foreign_ref()?;
        fields.push(field.clone());
    }

    Ok(DeclRawLayerAnimation::BlendTree { tree_type, fields }.into())
}

fn declare_blendtree_field(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let x_value: Option<f64> = args.try_exact_arg(1)?;
    let y_value: Option<f64> = args.try_exact_arg(2)?;

    Ok(DeclRawLayerBlendTreeField {
        name: name.to_string(),
        values: [x_value.unwrap_or(0.0), y_value.unwrap_or(0.0)],
    }
    .into())
}

fn declare_transition_to(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let target: &str = args.exact_arg(function_name, 0)?;
    let duration: Option<f64> = args.exact_kwarg("duration")?;

    let mut and_terms = vec![];
    for condition_value in args.args_after(function_name, 1)? {
        let Value::List(condition_list) = condition_value else {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "list that expresses condition",
                found: condition_value.type_name(),
                value: Some((*condition_value).clone()),
            }));
        };
        let condition = parse_condition(name_store, condition_list)?;
        and_terms.push(condition);
    }

    Ok(DeclRawLayerTransition {
        target: target.to_string(),
        duration,
        and_terms,
    }
    .into())
}

fn parse_condition(
    name_store: &NameStore,
    condition_list: &[Value],
) -> KetosResult<DeclRawLayerTransitionCondition> {
    if condition_list.len() != 3 {
        return Err(Error::Custom(DeclSexprError::InvalidCondition.into()));
    }

    let parameter = match &condition_list[1] {
        Value::String(p) => p.to_string(),
        other => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "string",
                found: other.type_name(),
                value: Some(other.clone()),
            }))
        }
    };
    let ordering = match &condition_list[0] {
        Value::Name(n) => match name_store.get(*n) {
            "=" => DeclRawLayerTransitionOrdering::Equal,
            "/=" => DeclRawLayerTransitionOrdering::NotEqual,
            ">" => DeclRawLayerTransitionOrdering::Greater,
            "<" => DeclRawLayerTransitionOrdering::Lesser,
            _ => return Err(Error::Custom(DeclSexprError::InvalidCondition.into())),
        },
        other => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "operator",
                found: other.type_name(),
                value: Some(other.clone()),
            }))
        }
    };
    let condition = match &condition_list[2] {
        Value::Bool(bv) => match ordering {
            DeclRawLayerTransitionOrdering::Equal => {
                DeclRawLayerTransitionCondition::Bool(parameter, *bv)
            }
            DeclRawLayerTransitionOrdering::NotEqual => {
                DeclRawLayerTransitionCondition::Bool(parameter, !*bv)
            }
            _ => return Err(Error::Custom(DeclSexprError::InvalidCondition.into())),
        },
        Value::Integer(iv) => DeclRawLayerTransitionCondition::Int(
            parameter,
            ordering,
            iv.to_u8()
                .ok_or_else(|| Error::Custom(DeclSexprError::InvalidCondition.into()))?,
        ),
        Value::Float(fv) => DeclRawLayerTransitionCondition::Float(parameter, ordering, *fv),
        other => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "int, bool, or float",
                found: other.type_name(),
                value: Some(other.clone()),
            }))
        }
    };
    Ok(condition)
}
