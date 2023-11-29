use crate::decl_sexpr::{
    data::menu::{
        DeclBooleanControl, DeclMenuElement, DeclPuppetControl, DeclPuppetType, DeclSubMenu,
    },
    function::{register_function, SeparateArguments},
};

use ketos::{Arity, Error, Name, NameStore, Scope, Value};

use super::KetosValueExt;

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
        Arity::Exact(0),
        &["horizontal", "vertical"],
    );
    register_function(
        scope,
        "four-axis",
        declare_four_axis,
        Arity::Exact(0),
        &["up", "down", "left", "right"],
    );
}

fn declare_menu(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
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
) -> Result<Value, Error> {
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
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let drive_target: &Value = args.exact_arg(function_name, 1)?;

    Ok(DeclMenuElement::Boolean(DeclBooleanControl {
        name: name.to_string(),
        hold: false,
    })
    .into())
}

fn declare_toggle(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let drive_target: &Value = args.exact_arg(function_name, 1)?;

    Ok(DeclMenuElement::Boolean(DeclBooleanControl {
        name: name.to_string(),
        hold: true,
    })
    .into())
}

fn declare_radial(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let drive_target: &Value = args.exact_arg(function_name, 1)?;

    Ok(DeclMenuElement::Puppet(DeclPuppetControl {
        name: name.to_string(),
        puppet_type: DeclPuppetType::Radial(),
    })
    .into())
}

fn declare_two_axis(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let horizontal: &Value = args.exact_kwarg_expect("horizontal")?;
    let vertical: &Value = args.exact_kwarg_expect("vertical")?;

    Ok(DeclMenuElement::Puppet(DeclPuppetControl {
        name: name.to_string(),
        puppet_type: DeclPuppetType::TwoAxis(),
    })
    .into())
}

fn declare_four_axis(
    _name_store: &NameStore,
    function_name: Name,
    args: SeparateArguments,
) -> Result<Value, Error> {
    let name: &str = args.exact_arg(function_name, 0)?;
    let up: &Value = args.exact_kwarg_expect("up")?;
    let down: &Value = args.exact_kwarg_expect("down")?;
    let left: &Value = args.exact_kwarg_expect("left")?;
    let right: &Value = args.exact_kwarg_expect("right")?;

    Ok(DeclMenuElement::Puppet(DeclPuppetControl {
        name: name.to_string(),
        puppet_type: DeclPuppetType::FourAxis(),
    })
    .into())
}
