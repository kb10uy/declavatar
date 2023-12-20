use crate::decl_v2::{
    data::{
        driver::DeclParameterDrive,
        menu::{
            DeclBooleanControl, DeclMenuElement, DeclPuppetAxis, DeclPuppetControl,
            DeclPuppetTarget, DeclPuppetType, DeclSubMenu,
        },
    },
    sexpr::{
        argument::SeparateArguments,
        error::{DeclSexprError, KetosResult},
        register_function, KetosValueExt,
    },
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
    register_function(scope, "axis", declare_axis, Arity::Range(1, 3), &[]);
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
    let parameter_drive: &DeclParameterDrive = args.exact_arg(function_name, 1)?;

    Ok(DeclMenuElement::Boolean(DeclBooleanControl {
        name: name.to_string(),
        hold: false,
        parameter_drive: parameter_drive.clone(),
    })
    .into())
}

fn declare_toggle(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let parameter_drive: &DeclParameterDrive = args.exact_arg(function_name, 1)?;

    Ok(DeclMenuElement::Boolean(DeclBooleanControl {
        name: name.to_string(),
        hold: true,
        parameter_drive: parameter_drive.clone(),
    })
    .into())
}

fn declare_radial(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let target: &DeclPuppetAxis = args.exact_arg(function_name, 1)?;

    Ok(DeclMenuElement::Puppet(DeclPuppetControl {
        name: name.to_string(),
        puppet_type: Box::new(DeclPuppetType::Radial(target.clone())),
    })
    .into())
}

fn declare_two_axis(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let horizontal: &DeclPuppetAxis = args.exact_kwarg_expect("horizontal")?;
    let vertical: &DeclPuppetAxis = args.exact_kwarg_expect("vertical")?;

    Ok(DeclMenuElement::Puppet(DeclPuppetControl {
        name: name.to_string(),
        puppet_type: Box::new(DeclPuppetType::TwoAxis {
            horizontal: horizontal.clone(),
            vertical: vertical.clone(),
        }),
    })
    .into())
}

fn declare_four_axis(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let up: &DeclPuppetAxis = args.exact_kwarg_expect("up")?;
    let down: &DeclPuppetAxis = args.exact_kwarg_expect("down")?;
    let left: &DeclPuppetAxis = args.exact_kwarg_expect("left")?;
    let right: &DeclPuppetAxis = args.exact_kwarg_expect("right")?;

    Ok(DeclMenuElement::Puppet(DeclPuppetControl {
        name: name.to_string(),
        puppet_type: Box::new(DeclPuppetType::FourAxis {
            up: up.clone(),
            down: down.clone(),
            left: left.clone(),
            right: right.clone(),
        }),
    })
    .into())
}

fn declare_axis(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> KetosResult<Value> {
    let target: &DeclParameterDrive = args.exact_arg(function_name, 0)?;
    let positive: Option<&str> = args.try_exact_arg(1)?;
    let negative: Option<&str> = args.try_exact_arg(2)?;

    Ok(DeclPuppetAxis {
        target: take_puppet_target(target)?,
        label_positive: positive.map(|l| l.to_string()),
        label_negative: negative.map(|l| l.to_string()),
    }
    .into())
}

fn take_puppet_target(drive_target: &DeclParameterDrive) -> KetosResult<DeclPuppetTarget> {
    match drive_target {
        DeclParameterDrive::Puppet(puppet) => Ok(DeclPuppetTarget::Puppet(puppet.clone())),
        DeclParameterDrive::SetFloat { parameter, .. } => {
            Ok(DeclPuppetTarget::Parameter(parameter.clone()))
        }
        _ => Err(Error::Custom(
            DeclSexprError::UnexpectedTypeValue(
                "invalid drive target".to_string(),
                "puppet drive target".to_string(),
            )
            .into(),
        )),
    }
}
