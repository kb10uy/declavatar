use crate::decl_v2::{
    data::layer::{DeclGroupOption, DeclGroupOptionKind},
    sexpr::{
        argument::SeparateArguments, da::layer_basic::take_option_target, error::KetosResult,
        register_function, register_function_with_context,
    },
};

use ketos::{
    exec::call_function,
    rc_vec::{RcString, RcVec},
    Arity, Context, Error, ExecError, Name, NameStore, Scope, Value,
};

pub fn register_option_function(scope: &Scope) {
    register_function(
        scope,
        "option-prepend-targets",
        option_prepend_targets,
        Arity::Exact(2),
        &[],
    );
    register_function(
        scope,
        "option-extend-targets",
        option_extend_targets,
        Arity::Exact(2),
        &[],
    );
    register_function_with_context(
        scope,
        "option-replace-targets",
        option_replace_targets,
        Arity::Exact(2),
        &[],
    )
}

fn option_prepend_targets(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let option: &DeclGroupOption = args.exact_arg(function_name, 0)?;
    let target_values = {
        let targets_list: &Value = args.exact_arg(function_name, 1)?;
        let Value::List(target_values) = targets_list else {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "target list",
                found: targets_list.type_name(),
                value: None,
            }));
        };
        target_values
    };

    let mut targets = vec![];
    for target_value in target_values {
        targets.push(take_option_target(target_value)?);
    }

    let mut new_option = option.clone();
    let old_targets = new_option.targets;
    targets.extend(old_targets);
    new_option.targets = targets;

    Ok(new_option.into())
}

fn option_extend_targets(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let option: &DeclGroupOption = args.exact_arg(function_name, 0)?;
    let target_values = {
        let targets_list: &Value = args.exact_arg(function_name, 1)?;
        let Value::List(target_values) = targets_list else {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "target list",
                found: targets_list.type_name(),
                value: None,
            }));
        };
        target_values
    };

    let mut targets = vec![];
    for target_value in target_values {
        targets.push(take_option_target(target_value)?);
    }

    let mut new_option = option.clone();
    new_option.targets.extend(targets);

    Ok(new_option.into())
}

fn option_replace_targets(
    ctx: &Context,
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let map_function: &Value = args.exact_arg(function_name, 0)?;
    let original_option: &DeclGroupOption = args.exact_arg(function_name, 1)?;

    let internal_kind = match &original_option.kind {
        DeclGroupOptionKind::Boolean(v) => Value::Bool(*v),
        DeclGroupOptionKind::Selection(None, _) => Value::Unit,
        DeclGroupOptionKind::Selection(Some(v), _) => Value::String(RcString::new(v.clone())),
        DeclGroupOptionKind::Keyframe(v) => Value::Float(*v),
    };
    let target_values: Vec<Value> = original_option
        .targets
        .iter()
        .cloned()
        .map(|t| t.into())
        .collect();

    let mapped_targets = call_function(
        ctx,
        map_function.clone(),
        vec![internal_kind, Value::List(RcVec::new(target_values))],
    )?;
    let Value::List(target_values) = mapped_targets else {
        return Err(Error::ExecError(ExecError::TypeError {
            expected: "target list",
            found: mapped_targets.type_name(),
            value: None,
        }));
    };
    let mut targets = vec![];
    for target_value in target_values.iter() {
        targets.push(take_option_target(target_value)?);
    }

    Ok(DeclGroupOption {
        kind: original_option.kind.clone(),
        animation_asset: original_option.animation_asset.clone(),
        targets,
    }
    .into())
}
