use crate::{
    avatar_v2::{
        data::{
            menu::{
                BiAxis, MenuBoolean, MenuFourAxis, MenuGroup, MenuItem, MenuRadial, MenuTwoAxis,
                UniAxis,
            },
            parameter::{ParameterScope, ParameterType},
        },
        logger::{Log, Logger, LoggerContext},
        transformer::{failure, success, Compiled, FirstPassData},
    },
    decl_v2::data::{
        driver::DeclParameterDrive,
        menu::{
            DeclBooleanControl, DeclMenuElement, DeclPuppetControl, DeclPuppetTarget,
            DeclPuppetType, DeclSubMenu,
        },
    },
};

pub fn compile_menu(
    logger: &Logger,
    first_pass: &FirstPassData,
    decl_menu_blocks: Vec<DeclSubMenu>,
) -> Compiled<Vec<MenuItem>> {
    #[derive(Debug)]
    pub struct Context(usize);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("menu block {} > {}", self.0, inner)
        }
    }

    let mut elements = vec![];
    for (index, decl_menu) in decl_menu_blocks.into_iter().enumerate() {
        let logger = logger.with_context(Context(index));
        let mut menu = compile_menu_group(&logger, first_pass, decl_menu)?;
        elements.append(&mut menu.items);
    }

    success(elements)
}

fn compile_menu_group(
    logger: &Logger,
    first_pass: &FirstPassData,
    submenu: DeclSubMenu,
) -> Compiled<MenuGroup> {
    #[derive(Debug)]
    pub struct Context(String);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            if self.0.is_empty() {
                inner
            } else {
                format!("menu group '{}' > {}", self.0, inner)
            }
        }
    }

    let logger = logger.with_context(Context(submenu.name.clone()));
    let mut items = vec![];
    for menu_element in submenu.elements {
        let Some(menu_item) = (match menu_element {
            DeclMenuElement::SubMenu(sm) => {
                compile_menu_group(&logger, first_pass, sm).map(MenuItem::SubMenu)
            }
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
    logger: &Logger,
    first_pass: &FirstPassData,
    control: DeclBooleanControl,
) -> Compiled<MenuItem> {
    #[derive(Debug)]
    pub struct Context(bool, String);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            if self.0 {
                format!("toggle '{}' > {}", self.0, inner)
            } else {
                format!("button '{}' > {}", self.0, inner)
            }
        }
    }

    let logger = logger.with_context(Context(control.hold, control.name.clone()));
    let (parameter, value) = match control.parameter_drive {
        DeclParameterDrive::Group(dg) => {
            let (parameter, options) =
                first_pass.find_group(&logger, &dg.group, ParameterScope::MUST_EXPOSE)?;
            let Some((_, value)) = options.iter().find(|(name, _)| name == &dg.option) else {
                logger.log(Log::LayerOptionNotFound(dg.option));
                return failure();
            };

            (parameter.to_string(), ParameterType::Int(*value as u8))
        }
        DeclParameterDrive::Switch(ds) => {
            let parameter =
                first_pass.find_switch(&logger, &ds.switch, ParameterScope::MUST_EXPOSE)?;

            (
                parameter.to_string(),
                ParameterType::Bool(ds.value.unwrap_or(false)),
            )
        }
        DeclParameterDrive::Puppet(dp) => {
            let parameter =
                first_pass.find_puppet(&logger, &dp.puppet, ParameterScope::MUST_EXPOSE)?;

            (
                parameter.to_string(),
                ParameterType::Float(dp.value.unwrap_or(1.0)),
            )
        }
        DeclParameterDrive::SetInt { parameter, value } => {
            first_pass.find_parameter(
                &logger,
                &parameter,
                ParameterType::INT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            (parameter, ParameterType::Int(value as u8))
        }
        DeclParameterDrive::SetBool { parameter, value } => {
            first_pass.find_parameter(
                &logger,
                &parameter,
                ParameterType::BOOL_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            (parameter, ParameterType::Bool(value.unwrap_or(true)))
        }
        DeclParameterDrive::SetFloat { parameter, value } => {
            first_pass.find_parameter(
                &logger,
                &parameter,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            (parameter, ParameterType::Float(value.unwrap_or(1.0)))
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

fn compile_puppet(
    logger: &Logger,
    first_pass: &FirstPassData,
    control: DeclPuppetControl,
) -> Compiled<MenuItem> {
    #[derive(Debug)]
    pub struct Context(String);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("puppet '{}' > {}", self.0, inner)
        }
    }

    let logger = logger.with_context(Context(control.name.clone()));
    let puppet_type = *control.puppet_type;
    let puppet = match puppet_type {
        DeclPuppetType::Radial(pt) => MenuItem::Radial(MenuRadial {
            name: control.name,
            parameter: take_puppet_parameter(&logger, first_pass, pt.target)?,
        }),
        DeclPuppetType::TwoAxis {
            horizontal,
            vertical,
        } => MenuItem::TwoAxis(MenuTwoAxis {
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
        DeclPuppetType::FourAxis {
            up,
            down,
            left,
            right,
        } => MenuItem::FourAxis(MenuFourAxis {
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

fn take_puppet_parameter(
    logger: &Logger,
    first_pass: &FirstPassData,
    dpt: DeclPuppetTarget,
) -> Compiled<String> {
    let parameter = match dpt {
        DeclPuppetTarget::Puppet(dp) => {
            let parameter =
                first_pass.find_puppet(logger, &dp.puppet, ParameterScope::MUST_EXPOSE)?;
            parameter.to_string()
        }
        DeclPuppetTarget::Parameter(parameter) => {
            first_pass.find_parameter(
                logger,
                &parameter,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;
            parameter
        }
    };

    success(parameter)
}
