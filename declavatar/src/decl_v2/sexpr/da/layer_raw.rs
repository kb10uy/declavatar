use crate::decl_v2::{
    data::{
        layer::{
            DeclControllerLayer, DeclLayerInlineAnimation, DeclRawLayer, DeclRawLayerAnimation,
            DeclRawLayerAnimationKind, DeclRawLayerBlendTreeField, DeclRawLayerBlendTreeType,
            DeclRawLayerState, DeclRawLayerTransition, DeclRawLayerTransitionCondition,
            DeclRawLayerTransitionOrdering,
        },
        StaticTypeName,
    },
    sexpr::{
        argument::SeparateArguments,
        da::layer_basic::take_option_target,
        error::{DeclSexprError, KetosResult},
        register_function, KetosValueExt,
    },
};

use ketos::{Arity, Error, ExecError, Name, NameStore, Scope, Value};

pub fn register_layer_raw_function(scope: &Scope) {
    // layer functions
    register_function(
        scope,
        "raw-layer",
        declare_raw_layer,
        Arity::Min(1),
        Some(&["default"]),
    );
    register_function(scope, "state", declare_state, Arity::Min(2), Some(&[]));

    register_function(
        scope,
        "clip",
        declare_clip,
        Arity::Min(1),
        Some(&["speed", "speed-by", "time-by"]),
    );
    register_function(
        scope,
        "inline-animation",
        declare_inline_animation,
        Arity::Min(0),
        Some(&[]),
    );
    register_function(
        scope,
        "blendtree",
        declare_blendtree,
        Arity::Min(0),
        Some(&["type", "x", "y"]),
    );
    register_function(
        scope,
        "blendtree-field",
        declare_blendtree_field,
        Arity::Range(2, 3),
        Some(&[]),
    );
    register_function(
        scope,
        "transition-to",
        declare_transition_to,
        Arity::Min(1),
        Some(&["duration"]),
    );

    register_function(
        scope,
        "cond-eq",
        declare_cond_eq,
        Arity::Exact(2),
        Some(&[]),
    );
    register_function(
        scope,
        "cond-ne",
        declare_cond_ne,
        Arity::Exact(2),
        Some(&[]),
    );
    register_function(
        scope,
        "cond-gt",
        declare_cond_gt,
        Arity::Exact(2),
        Some(&[]),
    );
    register_function(
        scope,
        "cond-lt",
        declare_cond_lt,
        Arity::Exact(2),
        Some(&[]),
    );
    register_function(
        scope,
        "cond-ze",
        declare_cond_ze,
        Arity::Exact(1),
        Some(&[]),
    );
    register_function(
        scope,
        "cond-nz",
        declare_cond_nz,
        Arity::Exact(1),
        Some(&[]),
    );
}

fn declare_raw_layer(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let default: Option<&str> = args.exact_kwarg("default")?;

    let mut states = vec![];
    for state_value in args.args_after_recursive(function_name, 1)? {
        states.push(
            state_value
                .downcast_foreign_ref::<&DeclRawLayerState>()?
                .clone(),
        );
    }
    Ok(DeclControllerLayer::Raw(DeclRawLayer {
        name: name.to_string(),
        default: default.map(|d| d.to_string()),
        states,
    })
    .into())
}

fn declare_state(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let kind: &DeclRawLayerAnimationKind = args.exact_arg(function_name, 1)?;

    let mut transitions = vec![];
    for transition_value in args.args_after_recursive(function_name, 2)? {
        transitions.push(
            transition_value
                .downcast_foreign_ref::<&DeclRawLayerTransition>()?
                .clone(),
        );
    }

    Ok(DeclRawLayerState {
        name: name.to_string(),
        kind: kind.clone(),
        transitions,
    }
    .into())
}

fn declare_inline_animation(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let mut targets = vec![];
    for target_value in args.args_after_recursive(function_name, 0)? {
        targets.push(take_option_target(target_value)?);
    }

    Ok(DeclLayerInlineAnimation { targets }.into())
}

fn declare_clip(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let animation: &Value = args.exact_arg(function_name, 0)?;
    let speed: Option<f64> = args.exact_kwarg("speed")?;
    let speed_by: Option<&str> = args.exact_kwarg("speed-by")?;
    let time_by: Option<&str> = args.exact_kwarg("time-by")?;

    Ok(DeclRawLayerAnimationKind::Clip {
        animation: take_animation(animation)?,
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
        "simple-2d" => {
            let x: &str = args.exact_kwarg_expect("x")?;
            let y: &str = args.exact_kwarg_expect("y")?;
            DeclRawLayerBlendTreeType::Simple2D(x.to_string(), y.to_string())
        }
        "freeform-2d" => {
            let x: &str = args.exact_kwarg_expect("x")?;
            let y: &str = args.exact_kwarg_expect("y")?;
            DeclRawLayerBlendTreeType::Freeform2D(x.to_string(), y.to_string())
        }
        "cartesian-2d" => {
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
    for field_value in args.args_after_recursive(function_name, 0)? {
        fields.push(
            field_value
                .downcast_foreign_ref::<&DeclRawLayerBlendTreeField>()?
                .clone(),
        );
    }

    Ok(DeclRawLayerAnimationKind::BlendTree { tree_type, fields }.into())
}

fn declare_blendtree_field(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let animation: &Value = args.exact_arg(function_name, 0)?;
    let x_value: Option<f64> = args.try_exact_arg(1)?;
    let y_value: Option<f64> = args.try_exact_arg(2)?;

    Ok(DeclRawLayerBlendTreeField {
        animation: take_animation(animation)?,
        values: [x_value.unwrap_or(0.0), y_value.unwrap_or(0.0)],
    }
    .into())
}

fn take_animation(animation_value: &Value) -> KetosResult<DeclRawLayerAnimation> {
    let target = match animation_value.type_name() {
        "string" => {
            let Value::String(s) = animation_value else {
                unreachable!("must be string")
            };
            DeclRawLayerAnimation::External(s.to_string())
        }
        DeclLayerInlineAnimation::TYPE_NAME => DeclRawLayerAnimation::Inline(
            animation_value
                .downcast_foreign_ref::<&DeclLayerInlineAnimation>()?
                .clone(),
        ),
        _ => {
            return Err(Error::Custom(
                DeclSexprError::UnexpectedTypeValue(
                    animation_value.type_name().to_string(),
                    "string or inline animation".to_string(),
                )
                .into(),
            ))
        }
    };

    Ok(target)
}

fn declare_transition_to(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let target: &str = args.exact_arg(function_name, 0)?;
    let duration: Option<f64> = args.exact_kwarg("duration")?;

    let mut conditions = vec![];
    for condition_value in args.args_after_recursive(function_name, 1)? {
        let condition: &DeclRawLayerTransitionCondition = condition_value.downcast_foreign_ref()?;
        conditions.push(condition.clone());
    }

    Ok(DeclRawLayerTransition {
        target: target.to_string(),
        duration,
        conditions,
    }
    .into())
}

fn declare_cond_eq(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;
    let value: &Value = args.exact_arg(function_name, 1)?;

    let condition = expect_condition(
        parameter.to_string(),
        DeclRawLayerTransitionOrdering::Equal,
        value,
    )?;
    Ok(condition.into())
}

fn declare_cond_ne(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;
    let value: &Value = args.exact_arg(function_name, 1)?;

    let condition = expect_condition(
        parameter.to_string(),
        DeclRawLayerTransitionOrdering::NotEqual,
        value,
    )?;
    Ok(condition.into())
}

fn declare_cond_gt(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;
    let value: &Value = args.exact_arg(function_name, 1)?;

    let condition = expect_condition(
        parameter.to_string(),
        DeclRawLayerTransitionOrdering::Greater,
        value,
    )?;
    Ok(condition.into())
}

fn declare_cond_lt(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;
    let value: &Value = args.exact_arg(function_name, 1)?;

    let condition = expect_condition(
        parameter.to_string(),
        DeclRawLayerTransitionOrdering::Lesser,
        value,
    )?;
    Ok(condition.into())
}

fn declare_cond_ze(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclRawLayerTransitionCondition::Zero(parameter.to_string(), false).into())
}

fn declare_cond_nz(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;
    Ok(DeclRawLayerTransitionCondition::Zero(parameter.to_string(), true).into())
}

fn expect_condition(
    parameter: String,
    ordering: DeclRawLayerTransitionOrdering,
    value: &Value,
) -> KetosResult<DeclRawLayerTransitionCondition> {
    let condition = match value {
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
            iv.to_i64()
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
