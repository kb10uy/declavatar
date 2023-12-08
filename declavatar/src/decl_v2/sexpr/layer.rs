use crate::decl_v2::{
    data::{
        layer::{
            DeclControllerLayer, DeclGroupLayer, DeclGroupMaterialTarget, DeclGroupObjectTarget,
            DeclGroupOption, DeclGroupOptionKind, DeclGroupOptionTarget, DeclGroupShapeTarget,
            DeclPuppetLayer, DeclSwitchLayer,
        },
        StaticTypeName,
    },
    error::DeclSexprError,
    sexpr::{register_function, KetosResult, KetosValueExt, SeparateArguments},
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
    register_function(scope, "option", declare_option, Arity::Min(1), &["value"]);
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
    for param_value in args.args_after(function_name, 1)? {
        let option: &DeclGroupOption = param_value.downcast_foreign_ref()?;

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
    for param_value in args.args_after(function_name, 1)? {
        let option: &DeclGroupOption = param_value.downcast_foreign_ref()?;

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
    for param_value in args.args_after(function_name, 1)? {
        let option: &DeclGroupOption = param_value.downcast_foreign_ref()?;

        match option.kind {
            DeclGroupOptionKind::Keyframe(k) => {
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
