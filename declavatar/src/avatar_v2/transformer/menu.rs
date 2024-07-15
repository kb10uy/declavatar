use crate::{
    avatar_v2::{
        data::{
            menu::{BiAxis, MenuBoolean, MenuFourAxis, MenuGroup, MenuItem, MenuRadial, MenuTwoAxis, UniAxis},
            parameter::ParameterType,
        },
        log::Log,
        transformer::{failure, success, Compiled, FirstPassData},
    },
    decl_v2::data::{
        driver::DeclParameterDrive,
        menu::{DeclBooleanControl, DeclMenuElement, DeclPuppetControl, DeclPuppetTarget, DeclPuppetType, DeclSubMenu},
    },
    log::Logger,
};

pub fn compile_menu(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    decl_menu_blocks: Vec<DeclSubMenu>,
) -> Compiled<Vec<MenuItem>> {
    let mut elements = vec![];
    for (index, decl_menu) in decl_menu_blocks.into_iter().enumerate() {
        let logger = logger.with_context(format!("menu block {index}"));
        let menu = compile_menu_group(&logger, first_pass, decl_menu)?;
        elements.extend(menu.items);
    }

    success(elements)
}

fn compile_menu_group(logger: &Logger<Log>, first_pass: &FirstPassData, submenu: DeclSubMenu) -> Compiled<MenuGroup> {
    let logger = if submenu.name.is_empty() {
        logger.clone()
    } else {
        logger.with_context(format!("submenu {}", submenu.name))
    };
    let mut items = vec![];
    for menu_element in submenu.elements {
        let Some(menu_item) = (match menu_element {
            DeclMenuElement::SubMenu(sm) => compile_menu_group(&logger, first_pass, sm).map(MenuItem::SubMenu),
            DeclMenuElement::Boolean(bc) => compile_boolean(&logger, first_pass, bc),
            DeclMenuElement::Puppet(pc) => compile_puppet(&logger, first_pass, pc),
        }) else {
            continue;
        };
        items.push(menu_item);
    }

    success(MenuGroup {
        name: submenu.name,
        items,
    })
}

fn compile_boolean(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    control: DeclBooleanControl,
) -> Compiled<MenuItem> {
    let logger = logger.with_context(if control.hold {
        format!("toggle '{}'", control.name)
    } else {
        format!("button '{}'", control.name)
    });
    let (parameter, value) = match control.parameter_drive {
        DeclParameterDrive::Group(dg) => {
            let (query, options) = first_pass.find_group(&logger, &dg.group)?;
            let qualified = first_pass.find_writable_parameter(&logger, query, ParameterType::INT_TYPE)?;
            let Some((_, value)) = options.iter().find(|(name, _)| name == &dg.option) else {
                logger.log(Log::LayerOptionNotFound(dg.option));
                return failure();
            };
            (qualified.name, ParameterType::Int(*value as u8))
        }
        DeclParameterDrive::Switch(ds) => {
            let query = first_pass.find_switch(&logger, &ds.switch)?;
            let qualified = first_pass.find_writable_parameter(&logger, query, ParameterType::BOOL_TYPE)?;
            (qualified.name, ParameterType::Bool(ds.value.unwrap_or(true)))
        }
        DeclParameterDrive::Puppet(dp) => {
            let query = first_pass.find_puppet(&logger, &dp.puppet)?;
            let qualified = first_pass.find_writable_parameter(&logger, query, ParameterType::FLOAT_TYPE)?;
            (qualified.name, ParameterType::Float(dp.value.unwrap_or(1.0)))
        }
        DeclParameterDrive::SetInt { parameter, value } => {
            let qualified = first_pass.find_writable_parameter(&logger, &parameter.into(), ParameterType::INT_TYPE)?;
            (qualified.name, ParameterType::Int(value as u8))
        }
        DeclParameterDrive::SetBool { parameter, value } => {
            let qualified = first_pass.find_writable_parameter(&logger, &parameter.into(), ParameterType::BOOL_TYPE)?;
            (qualified.name, ParameterType::Bool(value.unwrap_or(true)))
        }
        DeclParameterDrive::SetFloat { parameter, value } => {
            let qualified =
                first_pass.find_writable_parameter(&logger, &parameter.into(), ParameterType::FLOAT_TYPE)?;
            (qualified.name, ParameterType::Float(value.unwrap_or(1.0)))
        }
        _ => {
            logger.log(Log::MenuInvalidDrive);
            return failure();
        }
    };

    let menu_boolean = MenuBoolean {
        name: control.name,
        parameter,
        value,
    };
    if control.hold {
        success(MenuItem::Toggle(menu_boolean))
    } else {
        success(MenuItem::Button(menu_boolean))
    }
}

fn compile_puppet(logger: &Logger<Log>, first_pass: &FirstPassData, control: DeclPuppetControl) -> Compiled<MenuItem> {
    let logger = logger.with_context(format!("puppet '{}'", control.name));
    let puppet_type = *control.puppet_type;
    let puppet = match puppet_type {
        DeclPuppetType::Radial(pt) => MenuItem::Radial(MenuRadial {
            name: control.name,
            parameter: take_puppet_parameter(&logger, first_pass, pt.target)?,
        }),
        DeclPuppetType::TwoAxis { horizontal, vertical } => MenuItem::TwoAxis(MenuTwoAxis {
            name: control.name,
            horizontal_axis: BiAxis {
                parameter: take_puppet_parameter(&logger, first_pass, horizontal.target)?,
                label_positive: horizontal.label_positive.unwrap_or_default(),
                label_negative: horizontal.label_negative.unwrap_or_default(),
            },
            vertical_axis: BiAxis {
                parameter: take_puppet_parameter(&logger, first_pass, vertical.target)?,
                label_positive: vertical.label_positive.unwrap_or_default(),
                label_negative: vertical.label_negative.unwrap_or_default(),
            },
        }),
        DeclPuppetType::FourAxis { up, down, left, right } => MenuItem::FourAxis(MenuFourAxis {
            name: control.name,
            left_axis: UniAxis {
                parameter: take_puppet_parameter(&logger, first_pass, up.target)?,
                label: up.label_positive.unwrap_or_default(),
            },
            right_axis: UniAxis {
                parameter: take_puppet_parameter(&logger, first_pass, down.target)?,
                label: down.label_positive.unwrap_or_default(),
            },
            up_axis: UniAxis {
                parameter: take_puppet_parameter(&logger, first_pass, left.target)?,
                label: left.label_positive.unwrap_or_default(),
            },
            down_axis: UniAxis {
                parameter: take_puppet_parameter(&logger, first_pass, right.target)?,
                label: right.label_positive.unwrap_or_default(),
            },
        }),
    };

    success(puppet)
}

fn take_puppet_parameter(logger: &Logger<Log>, first_pass: &FirstPassData, dpt: DeclPuppetTarget) -> Compiled<String> {
    let parameter = match dpt {
        DeclPuppetTarget::Puppet(dp) => {
            let parameter = first_pass.find_puppet(logger, &dp.puppet)?;
            let qualified = first_pass.find_writable_parameter(logger, parameter, ParameterType::FLOAT_TYPE)?;
            qualified.name
        }
        DeclPuppetTarget::Parameter(parameter) => {
            let qualified = first_pass.find_writable_parameter(logger, &parameter.into(), ParameterType::FLOAT_TYPE)?;
            qualified.name
        }
    };

    success(parameter)
}
