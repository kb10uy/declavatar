use crate::decl_v2::{
    data::driver::{DeclDriveGroup, DeclDrivePuppet, DeclDriveSwitch, DeclParameterDrive},
    sexpr::{register_function, KetosResult, SeparateArguments},
};

use ketos::{Arity, Error, ExecError, Name, NameStore, Scope, Value};

pub fn register_driver_function(scope: &Scope) {
    register_function(
        scope,
        "drive-group",
        declare_drive_group,
        Arity::Exact(2),
        &[],
    );
    register_function(
        scope,
        "drive-switch",
        declare_drive_switch,
        Arity::Range(1, 2),
        &[],
    );
    register_function(
        scope,
        "drive-puppet",
        declare_drive_puppet,
        Arity::Range(1, 2),
        &[],
    );

    register_function(scope, "drive-int", declare_drive_int, Arity::Exact(2), &[]);
    register_function(
        scope,
        "drive-bool",
        declare_drive_bool,
        Arity::Exact(1),
        &[],
    );
    register_function(
        scope,
        "drive-float",
        declare_drive_float,
        Arity::Exact(1),
        &[],
    );

    register_function(
        scope,
        "set-parameter",
        declare_set_parameter,
        Arity::Exact(2),
        &[],
    );
    register_function(
        scope,
        "add-parameter",
        declare_add_parameter,
        Arity::Exact(2),
        &[],
    );
}

fn declare_drive_group(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let group: &str = args.exact_arg(function_name, 0)?;
    let option: &str = args.exact_arg(function_name, 1)?;

    Ok(DeclParameterDrive::Group(DeclDriveGroup {
        group: group.to_string(),
        option: option.to_string(),
    })
    .into())
}

fn declare_drive_switch(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let switch: &str = args.exact_arg(function_name, 0)?;
    let value: Option<bool> = args.try_exact_arg(1)?;

    Ok(DeclParameterDrive::Switch(DeclDriveSwitch {
        switch: switch.to_string(),
        value,
    })
    .into())
}

fn declare_drive_puppet(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let puppet: &str = args.exact_arg(function_name, 0)?;
    let value: Option<f64> = args.try_exact_arg(1)?;

    Ok(DeclParameterDrive::Puppet(DeclDrivePuppet {
        puppet: puppet.to_string(),
        value,
    })
    .into())
}

fn declare_drive_int(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;
    let value: i64 = args.exact_arg(function_name, 1)?;

    Ok(DeclParameterDrive::SetInt {
        parameter: parameter.to_string(),
        value,
    }
    .into())
}

fn declare_drive_bool(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;

    Ok(DeclParameterDrive::SetBool {
        parameter: parameter.to_string(),
        value: None,
    }
    .into())
}

fn declare_drive_float(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;

    Ok(DeclParameterDrive::SetFloat {
        parameter: parameter.to_string(),
        value: None,
    }
    .into())
}

fn declare_set_parameter(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;
    let value: &Value = args.exact_arg(function_name, 1)?;

    let parameter = parameter.to_string();
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

fn declare_add_parameter(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let parameter: &str = args.exact_arg(function_name, 0)?;
    let value: &Value = args.exact_arg(function_name, 1)?;

    let parameter = parameter.to_string();
    let drive = match value {
        Value::Integer(v) => DeclParameterDrive::AddInt {
            parameter,
            value: v.to_i64().expect("failed to convert"),
        },
        Value::Float(v) => DeclParameterDrive::AddFloat {
            parameter,
            value: *v,
        },
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
