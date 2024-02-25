use crate::decl_v2::{
    data::{
        driver::{DeclParameterDrive, DeclTrackingControl},
        layer::{
            DeclControllerLayer, DeclGroupCopyMode, DeclGroupLayer,
            DeclGroupMaterialPropertyTarget, DeclGroupMaterialTarget, DeclGroupObjectTarget,
            DeclGroupOption, DeclGroupOptionKind, DeclGroupOptionTarget, DeclGroupShapeTarget,
            DeclMaterialValue, DeclPuppetLayer, DeclSwitchLayer,
        },
        StaticTypeName,
    },
    sexpr::{
        argument::SeparateArguments,
        error::{DeclSexprError, KetosResult},
        register_function, KetosValueExt,
    },
};

use ketos::{Arity, Error, ExecError, Name, NameStore, Scope, Value};

pub fn register_layer_basic_function(scope: &Scope) {
    // layer functions
    register_function(
        scope,
        "group-layer",
        declare_group_layer,
        Arity::Min(1),
        Some(&["driven-by", "default-mesh", "copy"]),
    );
    register_function(
        scope,
        "switch-layer",
        declare_switch_layer,
        Arity::Min(1),
        Some(&["driven-by", "with-gate", "default-mesh"]),
    );
    register_function(
        scope,
        "puppet-layer",
        declare_puppet_layer,
        Arity::Min(1),
        Some(&["driven-by", "default-mesh", "animation"]),
    );

    // option functions
    register_function(
        scope,
        "option",
        declare_option,
        Arity::Min(1),
        Some(&["value", "animation"]),
    );

    // set-x functions
    register_function(
        scope,
        "set-shape",
        declare_set_shape,
        Arity::Exact(1),
        Some(&["value", "mesh"]),
    );
    register_function(
        scope,
        "set-object",
        declare_set_object,
        Arity::Exact(1),
        Some(&["value"]),
    );
    register_function(
        scope,
        "set-material",
        declare_set_material,
        Arity::Exact(2),
        Some(&["mesh"]),
    );
    register_function(
        scope,
        "set-material-property",
        declare_set_material_property,
        Arity::Exact(2),
        Some(&["mesh"]),
    );

    // material value functions
    register_function(scope, "color", declare_color, Arity::Exact(4), Some(&[]));
    register_function(
        scope,
        "color-hdr",
        declare_color_hdr,
        Arity::Exact(4),
        Some(&[]),
    );
    register_function(scope, "vector", declare_vector, Arity::Exact(4), Some(&[]));
}

fn declare_group_layer(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let driven_by: &str = args.exact_kwarg_expect("driven-by")?;
    let default_mesh: Option<&str> = args.exact_kwarg("default-mesh")?;
    let copy_mode: Option<&Value> = args.exact_kwarg("copy")?;

    let mut default = None;
    let mut options = vec![];
    for option_value in args.args_after_recursive(function_name, 1)? {
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
        copy_mode: copy_mode
            .map(|v| expect_copy_mode(name_store, v))
            .transpose()?,
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
    let driven_by: Option<&str> = args.exact_kwarg("driven-by")?;
    let with_gate: Option<&str> = args.exact_kwarg("with-gate")?;
    let default_mesh: Option<&str> = args.exact_kwarg("default-mesh")?;

    let mut disabled = None;
    let mut enabled = None;
    for option_value in args.args_after_recursive(function_name, 1)? {
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
        driven_by: driven_by.map(|s| s.to_string()),
        with_gate: with_gate.map(|s| s.to_string()),
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
    let animation_asset: Option<&str> = args.exact_kwarg("animation")?;

    let mut keyframes = vec![];
    for option_value in args.args_after_recursive(function_name, 1)? {
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
        animation_asset: animation_asset.map(|a| a.to_string()),
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
    let animation_asset: Option<&str> = args.exact_kwarg("animation")?;

    let mut targets = vec![];
    for target_value in args.args_after_recursive(function_name, 1)? {
        targets.push(take_option_target(target_value)?);
    }

    Ok(DeclGroupOption {
        kind,
        animation_asset: animation_asset.map(|a| a.to_string()),
        targets,
    }
    .into())
}

fn expect_copy_mode(name_store: &NameStore, value: &Value) -> KetosResult<DeclGroupCopyMode> {
    let Value::Name(name) = value else {
        return Err(Error::Custom(DeclSexprError::MustBeScope.into()));
    };

    match name_store.get(*name) {
        "to-default-zeroed" => Ok(DeclGroupCopyMode::ToDefaultZeroed),
        "to-option" => Ok(DeclGroupCopyMode::ToOption),
        "mutual-zeroed" => Ok(DeclGroupCopyMode::MutualZeroed),
        n => Err(Error::Custom(
            DeclSexprError::InvalidCopyMode(n.to_string()).into(),
        )),
    }
}

pub fn take_option_target(target_value: &Value) -> KetosResult<DeclGroupOptionTarget> {
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
        DeclParameterDrive::TYPE_NAME => DeclGroupOptionTarget::ParameterDrive(
            target_value
                .downcast_foreign_ref::<&DeclParameterDrive>()?
                .clone(),
        ),
        DeclTrackingControl::TYPE_NAME => DeclGroupOptionTarget::TrackingControl(
            target_value
                .downcast_foreign_ref::<&DeclTrackingControl>()?
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

    Ok(target)
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

fn declare_set_material_property(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let property: &str = args.exact_arg(function_name, 0)?;
    let value: &Value = args.exact_arg(function_name, 1)?;
    let mesh: Option<&str> = args.exact_kwarg("mesh")?;

    Ok(DeclGroupMaterialPropertyTarget {
        property: property.to_string(),
        value: take_material_value(value)?,
        mesh: mesh.map(|m| m.to_string()),
    }
    .into())
}

fn take_material_value(value: &Value) -> KetosResult<DeclMaterialValue> {
    let target = match value {
        Value::Float(v) => DeclMaterialValue::Float(*v),
        _ if value.type_name() == DeclMaterialValue::TYPE_NAME => {
            value.downcast_foreign_ref::<&DeclMaterialValue>()?.clone()
        }
        _ => {
            return Err(Error::Custom(
                DeclSexprError::UnexpectedTypeValue(
                    value.type_name().to_string(),
                    "material value type".to_string(),
                )
                .into(),
            ))
        }
    };

    Ok(target)
}

fn declare_color(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let x: f64 = args.exact_arg(function_name, 0)?;
    let y: f64 = args.exact_arg(function_name, 1)?;
    let z: f64 = args.exact_arg(function_name, 2)?;
    let w: f64 = args.exact_arg(function_name, 3)?;

    Ok(DeclMaterialValue::Color([x, y, z, w]).into())
}

fn declare_color_hdr(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let x: f64 = args.exact_arg(function_name, 0)?;
    let y: f64 = args.exact_arg(function_name, 1)?;
    let z: f64 = args.exact_arg(function_name, 2)?;
    let w: f64 = args.exact_arg(function_name, 3)?;

    Ok(DeclMaterialValue::ColorHdr([x, y, z, w]).into())
}

fn declare_vector(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let x: f64 = args.exact_arg(function_name, 0)?;
    let y: f64 = args.exact_arg(function_name, 1)?;
    let z: f64 = args.exact_arg(function_name, 2)?;
    let w: f64 = args.exact_arg(function_name, 3)?;

    Ok(DeclMaterialValue::Vector([x, y, z, w]).into())
}
