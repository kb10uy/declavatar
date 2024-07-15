use crate::decl_v2::{
    data::driver::{
        DeclDriveGroup, DeclDrivePuppet, DeclDriveSwitch, DeclParameterDrive, DeclTrackingControl, DeclTrackingTarget,
    },
    sexpr::{
        argument::SeparateArguments, da::parameter::expect_parameter_reference, error::KetosResult, register_function,
    },
};

use ketos::{Arity, Error, ExecError, Name, NameStore, Scope, Value};

pub fn register_driver_function(scope: &Scope) {
    register_function(scope, "drive-group", declare_drive_group, Arity::Exact(2), Some(&[]));
    register_function(
        scope,
        "drive-switch",
        declare_drive_switch,
        Arity::Range(1, 2),
        Some(&[]),
    );
    register_function(
        scope,
        "drive-puppet",
        declare_drive_puppet,
        Arity::Range(1, 2),
        Some(&[]),
    );

    register_function(scope, "drive-int", declare_drive_int, Arity::Exact(2), Some(&[]));
    register_function(scope, "drive-bool", declare_drive_bool, Arity::Exact(1), Some(&[]));
    register_function(scope, "drive-float", declare_drive_float, Arity::Exact(1), Some(&[]));

    register_function(
        scope,
        "set-parameter",
        declare_set_parameter,
        Arity::Exact(2),
        Some(&[]),
    );
    register_function(
        scope,
        "add-parameter",
        declare_add_parameter,
        Arity::Exact(2),
        Some(&[]),
    );
    register_function(
        scope,
        "random-parameter",
        declare_random_parameter,
        Arity::Exact(2),
        Some(&[]),
    );
    register_function(
        scope,
        "copy-parameter",
        declare_copy_parameter,
        Arity::Range(2, 4),
        Some(&[]),
    );

    register_function(scope, "set-tracking", declare_set_tracking, Arity::Min(1), Some(&[]));
}

fn declare_drive_group(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let group: &str = args.exact_arg(function_name, 0)?;
    let option: &str = args.exact_arg(function_name, 1)?;

    Ok(DeclParameterDrive::Group(DeclDriveGroup {
        group: group.to_string(),
        option: option.to_string(),
    })
    .into())
}

fn declare_drive_switch(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let switch: &str = args.exact_arg(function_name, 0)?;
    let value: Option<bool> = args.try_exact_arg(1)?;

    Ok(DeclParameterDrive::Switch(DeclDriveSwitch {
        switch: switch.to_string(),
        value,
    })
    .into())
}

fn declare_drive_puppet(_name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let puppet: &str = args.exact_arg(function_name, 0)?;
    let value: Option<f64> = args.try_exact_arg(1)?;

    Ok(DeclParameterDrive::Puppet(DeclDrivePuppet {
        puppet: puppet.to_string(),
        value,
    })
    .into())
}

fn declare_drive_int(name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let parameter: &Value = args.exact_arg(function_name, 0)?;
    let value: i64 = args.exact_arg(function_name, 1)?;

    Ok(DeclParameterDrive::SetInt {
        parameter: expect_parameter_reference(name_store, parameter)?,
        value,
    }
    .into())
}

fn declare_drive_bool(name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let parameter: &Value = args.exact_arg(function_name, 0)?;

    Ok(DeclParameterDrive::SetBool {
        parameter: expect_parameter_reference(name_store, parameter)?,
        value: None,
    }
    .into())
}

fn declare_drive_float(name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let parameter: &Value = args.exact_arg(function_name, 0)?;

    Ok(DeclParameterDrive::SetFloat {
        parameter: expect_parameter_reference(name_store, parameter)?,
        value: None,
    }
    .into())
}

fn declare_set_parameter(name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let parameter: &Value = args.exact_arg(function_name, 0)?;
    let value: &Value = args.exact_arg(function_name, 1)?;

    let parameter = expect_parameter_reference(name_store, parameter)?;
    let drive = match value {
        Value::Integer(v) => DeclParameterDrive::SetInt {
            parameter,
            value: v.to_i64().expect("failed to convert"),
        },
        Value::Bool(v) => DeclParameterDrive::SetBool {
            parameter,
            value: Some(*v),
        },
        Value::Float(v) => DeclParameterDrive::SetFloat {
            parameter,
            value: Some(*v),
        },
        v => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "int, bool, or float",
                found: v.type_name(),
                value: Some(v.clone()),
            }))
        }
    };

    Ok(drive.into())
}

fn declare_add_parameter(name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let parameter: &Value = args.exact_arg(function_name, 0)?;
    let value: &Value = args.exact_arg(function_name, 1)?;

    let parameter = expect_parameter_reference(name_store, parameter)?;
    let drive = match value {
        Value::Integer(v) => DeclParameterDrive::AddInt {
            parameter,
            value: v.to_i64().expect("failed to convert"),
        },
        Value::Float(v) => DeclParameterDrive::AddFloat { parameter, value: *v },
        v => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "int or float",
                found: v.type_name(),
                value: Some(v.clone()),
            }))
        }
    };

    Ok(drive.into())
}

fn declare_random_parameter(
    name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &Value = args.exact_arg(function_name, 0)?;
    let range: &Value = args.exact_arg(function_name, 1)?;

    let parameter = expect_parameter_reference(name_store, parameter)?;
    let drive = match range {
        Value::List(_) => {
            if let Ok(int_range) = args.exact_arg::<(u8, u8)>(function_name, 1) {
                DeclParameterDrive::RandomInt {
                    parameter,
                    range: int_range,
                }
            } else if let Ok(float_range) = args.exact_arg::<(f64, f64)>(function_name, 1) {
                DeclParameterDrive::RandomFloat {
                    parameter,
                    range: float_range,
                }
            } else {
                return Err(Error::ExecError(ExecError::TypeError {
                    expected: "float or int/float pair",
                    found: range.type_name(),
                    value: Some(range.clone()),
                }));
            }
        }
        Value::Float(v) => DeclParameterDrive::RandomBool { parameter, value: *v },
        v => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "float or int/float pair",
                found: v.type_name(),
                value: Some(v.clone()),
            }));
        }
    };

    Ok(drive.into())
}

fn declare_copy_parameter(name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let from: &Value = args.exact_arg(function_name, 0)?;
    let to: &Value = args.exact_arg(function_name, 1)?;
    let from_range: Option<(f64, f64)> = args.try_exact_arg(2)?;
    let to_range: Option<(f64, f64)> = args.try_exact_arg(3)?;

    let range = match (from_range, to_range) {
        (Some(f), Some(t)) => Some((f, t)),
        _ => None,
    };

    Ok(DeclParameterDrive::Copy {
        from: expect_parameter_reference(name_store, from)?,
        to: expect_parameter_reference(name_store, to)?,
        range,
    }
    .into())
}

fn declare_set_tracking(name_store: &NameStore, function_name: Name, args: SeparateArguments) -> KetosResult<Value> {
    let animation_desired = match args.exact_arg::<&Value>(function_name, 0)? {
        Value::Name(n) => match name_store.get(*n) {
            "animation" => true,
            "tracking" => false,
            _ => {
                return Err(Error::ExecError(ExecError::TypeError {
                    expected: "'animation or 'tracking",
                    found: "invalid name",
                    value: None,
                }));
            }
        },
        v => {
            return Err(Error::ExecError(ExecError::TypeError {
                expected: "'animation or 'tracking",
                found: v.type_name(),
                value: Some(v.clone()),
            }));
        }
    };

    let mut targets = vec![];
    for target_value in args.args_after_recursive(function_name, 1)? {
        let target_str = match target_value {
            Value::Name(n) => name_store.get(*n),
            v => {
                return Err(Error::ExecError(ExecError::TypeError {
                    expected: "target name",
                    found: v.type_name(),
                    value: None,
                }));
            }
        };

        match target_str {
            "head" => {
                targets.push(DeclTrackingTarget::Head);
            }
            "hip" => {
                targets.push(DeclTrackingTarget::Hip);
            }
            "eyes" => {
                targets.push(DeclTrackingTarget::Eyes);
            }
            "mouth" => {
                targets.push(DeclTrackingTarget::Mouth);
            }
            "hand-left" => {
                targets.push(DeclTrackingTarget::HandLeft);
            }
            "hand-right" => {
                targets.push(DeclTrackingTarget::HandRight);
            }
            "hand" => {
                targets.push(DeclTrackingTarget::HandLeft);
                targets.push(DeclTrackingTarget::HandRight);
            }
            "foot-left" => {
                targets.push(DeclTrackingTarget::FootLeft);
            }
            "foot-right" => {
                targets.push(DeclTrackingTarget::FoorRight);
            }
            "foot" => {
                targets.push(DeclTrackingTarget::FootLeft);
                targets.push(DeclTrackingTarget::FoorRight);
            }
            "fingers-left" => {
                targets.push(DeclTrackingTarget::FingersLeft);
            }
            "fingers-right" => {
                targets.push(DeclTrackingTarget::FingersRight);
            }
            "fingers" => {
                targets.push(DeclTrackingTarget::FingersLeft);
                targets.push(DeclTrackingTarget::FingersRight);
            }
            _ => {
                return Err(Error::ExecError(ExecError::TypeError {
                    expected: "target name",
                    found: "invalid name",
                    value: Some((*target_value).clone()),
                }));
            }
        }
    }

    Ok(DeclTrackingControl {
        animation_desired,
        targets,
    }
    .into())
}
