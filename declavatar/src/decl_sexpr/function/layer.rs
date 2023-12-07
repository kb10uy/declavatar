use crate::decl_sexpr::{
    data::layer::{DeclGroupLayer, DeclGroupOption, DeclGroupOptionKind, DeclGroupOptionTarget},
    error::DeclError,
    function::{register_function, KetosResult, KetosValueExt, SeparateArguments},
};

use ketos::{Arity, Error, ExecError, Name, NameStore, Scope, Value};

pub fn register_layer_function(scope: &Scope) {
    register_function(
        scope,
        "group-layer",
        declare_group_layer,
        Arity::Min(1),
        &["driven-by", "default-mesh"],
    );
    register_function(scope, "option", declare_option, Arity::Min(1), &["value"]);
}

fn declare_group_layer(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let driven_by: &str = args.exact_kwarg_expect("driven-by")?;
    let default_mesh: Option<&str> = args.exact_kwarg("default-mesh")?;

    let mut options = vec![];
    for param_value in args.args_after(function_name, 0)? {
        let option: &DeclGroupOption = param_value.downcast_foreign_ref()?;
        options.push(option.clone());
    }

    Ok(DeclGroupLayer {
        name: name.to_string(),
        driven_by: driven_by.to_string(),
        default_mesh: default_mesh.map(|dm| dm.to_string()),
        options,
    }
    .into())
}

fn declare_option(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let kind = match args.exact_arg::<&Value>(function_name, 0)? {
        Value::Float(keyframe) => DeclGroupOptionKind::Keyframe(*keyframe),
        Value::Keyword(name) => match name_store.get(*name) {
            "default" => DeclGroupOptionKind::Selection(None, None),
            "disabled" => DeclGroupOptionKind::Boolean(false),
            "enabled" => DeclGroupOptionKind::Boolean(true),
            _ => {
                return Err(Error::Custom(
                    DeclError::KeywordExpected("default, disabled, enabled".into()).into(),
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
    for param_value in args.args_after(function_name, 0)? {
        let target: &DeclGroupOptionTarget = param_value.downcast_foreign_ref()?;
        targets.push(target.clone());
    }

    Ok(DeclGroupOption { kind, targets }.into())
}
