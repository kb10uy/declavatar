use crate::{
    avatar_v2::{
        data::{
            menu::{
                BiAxis, MenuBoolean, MenuFourAxis, MenuGroup, MenuItem, MenuRadial, MenuTwoAxis,
                UniAxis,
            },
            parameter::{ParameterScope, ParameterType},
        },
        logger::{Logger, Log, LoggerContext},
        transformer::{failure, success, Compiled, CompiledAnimations},
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
    animations: &CompiledAnimations,
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
        let mut menu = compile_menu_group(&logger, animations, decl_menu)?;
        elements.append(&mut menu.items);
    }

    success(elements)
}

fn compile_menu_group(
    logger: &Logger,
    animations: &CompiledAnimations,
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
                compile_menu_group(&logger, animations, sm).map(MenuItem::SubMenu)
            }
            DeclMenuElement::Boolean(bc) => compile_boolean(&logger, animations, bc),
            DeclMenuElement::Puppet(pc) => compile_puppet(&logger, animations, pc),
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
    animations: &CompiledAnimations,
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

    let sources = animations.sources();

    let logger = logger.with_context(Context(control.hold, control.name.clone()));
    let (parameter, value) = match control.parameter_drive {
        DeclParameterDrive::Group(dg) => {
            let (parameter, options) = animations.find_group(&logger, &dg.group)?;
            sources.find_parameter(
                &logger,
                parameter,
                ParameterType::INT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            let Some(option) = options.iter().find(|o| o.name == dg.option) else {
                logger.log(Log::LayerOptionNotFound(dg.option));
                return failure();
            };

            (
                parameter.to_string(),
                ParameterType::Int(option.value as u8),
            )
        }
        DeclParameterDrive::Switch(ds) => {
            let parameter = animations.find_switch(&logger, &ds.switch)?;
            sources.find_parameter(
                &logger,
                parameter,
                ParameterType::BOOL_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            (
                parameter.to_string(),
                ParameterType::Bool(ds.value.unwrap_or(false)),
            )
        }
        DeclParameterDrive::Puppet(dp) => {
            let parameter = animations.find_puppet(&logger, &dp.puppet)?;
            sources.find_parameter(
                &logger,
                parameter,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            (
                parameter.to_string(),
                ParameterType::Float(dp.value.unwrap_or(1.0)),
            )
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
    animations: &CompiledAnimations,
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
            parameter: take_puppet_parameter(&logger, animations, pt.target)?,
        }),
        DeclPuppetType::TwoAxis {
            horizontal,
            vertical,
        } => MenuItem::TwoAxis(MenuTwoAxis {
            name: control.name,
            horizontal_axis: BiAxis {
                parameter: take_puppet_parameter(&logger, animations, horizontal.target)?,
                label_positive: horizontal.label_positive.unwrap_or_default(),
                label_negative: horizontal.label_negative.unwrap_or_default(),
            },
            vertical_axis: BiAxis {
                parameter: take_puppet_parameter(&logger, animations, vertical.target)?,
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
                parameter: take_puppet_parameter(&logger, animations, up.target)?,
                label: up.label_positive.unwrap_or_default(),
            },
            right_axis: UniAxis {
                parameter: take_puppet_parameter(&logger, animations, down.target)?,
                label: down.label_positive.unwrap_or_default(),
            },
            up_axis: UniAxis {
                parameter: take_puppet_parameter(&logger, animations, left.target)?,
                label: left.label_positive.unwrap_or_default(),
            },
            down_axis: UniAxis {
                parameter: take_puppet_parameter(&logger, animations, right.target)?,
                label: right.label_positive.unwrap_or_default(),
            },
        }),
    };

    success(puppet)
}

fn take_puppet_parameter(
    logger: &Logger,
    animations: &CompiledAnimations,
    dpt: DeclPuppetTarget,
) -> Compiled<String> {
    let parameter = match dpt {
        DeclPuppetTarget::Puppet(dp) => {
            let parameter = animations.find_puppet(logger, &dp.puppet)?;
            parameter.to_string()
        }
    };
    animations.sources().find_parameter(
        logger,
        &parameter,
        ParameterType::FLOAT_TYPE,
        ParameterScope::MUST_EXPOSE,
    )?;

    success(parameter)
}
