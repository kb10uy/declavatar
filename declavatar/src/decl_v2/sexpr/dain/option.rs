use crate::decl_v2::{
    data::layer::DeclGroupOption,
    sexpr::{
        argument::SeparateArguments, da::layer::take_option_target, error::KetosResult,
        register_function,
    },
};

use ketos::{Arity, Error, ExecError, Name, NameStore, Scope, Value};

pub fn register_internal_function(scope: &Scope) {
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
    register_function(
        scope,
        "option-replace-targets",
        option_replace_targets,
        Arity::Exact(2),
        &[],
    )
}

pub fn option_prepend_targets(
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

pub fn option_extend_targets(
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

pub fn option_replace_targets(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let map_function: &Value = args.exact_arg(function_name, 0)?;
    let original_option: &DeclGroupOption = args.exact_arg(function_name, 1)?;

    match map_function {
        Value::Function(f) => todo!(),
        _ => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "map function",
                found: map_function.type_name(),
                value: None,
            }))
        }
    }
}
