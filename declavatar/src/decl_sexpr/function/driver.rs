use crate::decl_sexpr::{
    data::driver::{DeclDriveGroup, DeclDrivePuppet, DeclDriveSwitch, DeclParameterDrive},
    function::{register_function, KetosResult, SeparateArguments},
};

use ketos::{Arity, Name, NameStore, Scope, Value};

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
