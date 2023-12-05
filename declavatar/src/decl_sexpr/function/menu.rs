use crate::decl_sexpr::{
    data::{
        driver::{DeclDriveGroup, DeclDrivePuppet, DeclDriveSwitch},
        menu::{
            DeclBooleanControl, DeclBooleanTarget, DeclMenuElement, DeclPuppetControl,
            DeclPuppetTarget, DeclPuppetType, DeclSubMenu,
        },
        StaticTypeName,
    },
    error::DeclError,
    function::{register_function, KetosResult, KetosValueExt, SeparateArguments},
};

use ketos::{Arity, Error, Name, NameStore, Scope, Value};

pub fn register_menu_function(scope: &Scope) {
    register_function(scope, "menu", declare_menu, Arity::Min(0), &[]);
    register_function(scope, "submenu", declare_submenu, Arity::Min(1), &[]);
    register_function(scope, "button", declare_button, Arity::Exact(2), &[]);
    register_function(scope, "toggle", declare_toggle, Arity::Exact(2), &[]);
    register_function(scope, "radial", declare_radial, Arity::Exact(2), &[]);
    register_function(
        scope,
        "two-axis",
        declare_two_axis,
        Arity::Exact(1),
        &["horizontal", "vertical"],
    );
    register_function(
        scope,
        "four-axis",
        declare_four_axis,
        Arity::Exact(1),
        &["up", "down", "left", "right"],
    );
}

fn declare_menu(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let mut elements = vec![];
    for element_value in args.args_after(function_name, 0)? {
        let element: &DeclMenuElement = element_value.downcast_foreign_ref()?;
        elements.push(element.clone());
    }

    Ok(DeclSubMenu {
        name: "".into(),
        elements,
    }
    .into())
}

fn declare_submenu(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;

    let mut elements = vec![];
    for element_value in args.args_after(function_name, 1)? {
        let element: &DeclMenuElement = element_value.downcast_foreign_ref()?;
        elements.push(element.clone());
    }

    Ok(DeclMenuElement::SubMenu(DeclSubMenu {
        name: name.to_string(),
        elements,
    })
    .into())
}

fn declare_button(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let drive_target: &Value = args.exact_arg(function_name, 1)?;

    Ok(DeclMenuElement::Boolean(DeclBooleanControl {
        name: name.to_string(),
        hold: false,
        boolean_type: take_boolean_target(drive_target)?,
    })
    .into())
}

fn declare_toggle(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let drive_target: &Value = args.exact_arg(function_name, 1)?;

    Ok(DeclMenuElement::Boolean(DeclBooleanControl {
        name: name.to_string(),
        hold: true,
        boolean_type: take_boolean_target(drive_target)?,
    })
    .into())
}

fn declare_radial(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let target: &Value = args.exact_arg(function_name, 1)?;

    Ok(DeclMenuElement::Puppet(DeclPuppetControl {
        name: name.to_string(),
        puppet_type: DeclPuppetType::Radial(take_puppet_target(target)?),
    })
    .into())
}

fn declare_two_axis(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let horizontal: &Value = args.exact_kwarg_expect("horizontal")?;
    let vertical: &Value = args.exact_kwarg_expect("vertical")?;

    Ok(DeclMenuElement::Puppet(DeclPuppetControl {
        name: name.to_string(),
        puppet_type: DeclPuppetType::TwoAxis {
            horizontal: take_puppet_target(horizontal)?,
            vertical: take_puppet_target(vertical)?,
        },
    })
    .into())
}

fn declare_four_axis(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let up: &Value = args.exact_kwarg_expect("up")?;
    let down: &Value = args.exact_kwarg_expect("down")?;
    let left: &Value = args.exact_kwarg_expect("left")?;
    let right: &Value = args.exact_kwarg_expect("right")?;

    Ok(DeclMenuElement::Puppet(DeclPuppetControl {
        name: name.to_string(),
        puppet_type: DeclPuppetType::FourAxis {
            up: take_puppet_target(up)?,
            down: take_puppet_target(down)?,
            left: take_puppet_target(left)?,
            right: take_puppet_target(right)?,
        },
    })
    .into())
}

fn take_boolean_target(drive_target: &Value) -> KetosResult<DeclBooleanTarget> {
    match drive_target.type_name() {
        DeclDriveGroup::TYPE_NAME => {
            let value_ref: &DeclDriveGroup = drive_target.downcast_foreign_ref()?;
            Ok(DeclBooleanTarget::Group(value_ref.clone()))
        }
        DeclDriveSwitch::TYPE_NAME => {
            let value_ref: &DeclDriveSwitch = drive_target.downcast_foreign_ref()?;
            Ok(DeclBooleanTarget::Switch(value_ref.clone()))
        }
        DeclDrivePuppet::TYPE_NAME => {
            let value_ref: &DeclDrivePuppet = drive_target.downcast_foreign_ref()?;
            Ok(DeclBooleanTarget::Puppet(value_ref.clone()))
        }
        _ => Err(Error::Custom(
            DeclError::UnexpectedTypeValue(
                drive_target.type_name().to_string(),
                "drive target".to_string(),
            )
            .into(),
        )),
    }
}

fn take_puppet_target(drive_target: &Value) -> KetosResult<DeclPuppetTarget> {
    match drive_target.type_name() {
        DeclDrivePuppet::TYPE_NAME => {
            let value_ref: &DeclDrivePuppet = drive_target.downcast_foreign_ref()?;
            Ok(DeclPuppetTarget::Puppet(value_ref.clone()))
        }
        _ => Err(Error::Custom(
            DeclError::UnexpectedTypeValue(
                drive_target.type_name().to_string(),
                "drive target".to_string(),
            )
            .into(),
        )),
    }
}
