use crate::{
    avatar_v2::{
        data::{
            menu::{
                BiAxis, MenuBoolean, MenuFourAxis, MenuGroup, MenuItem, MenuRadial, MenuTwoAxis,
                UniAxis,
            },
            parameter::{ParameterScope, ParameterType},
        },
        logger::{Log, Logger},
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
    ctx: &mut Logger,
    animations: &CompiledAnimations,
    decl_menu_blocks: Vec<DeclSubMenu>,
) -> Compiled<Vec<MenuItem>> {
    let elements = decl_menu_blocks
        .into_iter()
        .flat_map(|ab| ab.elements)
        .collect();
    let menu = compile_menu_group(
        ctx,
        animations,
        DeclSubMenu {
            name: "".into(),
            elements,
        },
    )?;
    success(menu.items)
}

fn compile_menu_group(
    ctx: &mut Logger,
    animations: &CompiledAnimations,
    submenu: DeclSubMenu,
) -> Compiled<MenuGroup> {
    let mut items = vec![];
    for menu_element in submenu.elements {
        let Some(menu_item) = (match menu_element {
            DeclMenuElement::SubMenu(sm) => {
                compile_menu_group(ctx, animations, sm).map(MenuItem::SubMenu)
            }
            DeclMenuElement::Boolean(bc) => compile_boolean(ctx, animations, bc),
            DeclMenuElement::Puppet(pc) => compile_puppet(ctx, animations, pc),
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
    ctx: &mut Logger,
    animations: &CompiledAnimations,
    control: DeclBooleanControl,
) -> Compiled<MenuItem> {
    let sources = animations.sources();

    let (parameter, value) = match control.parameter_drive {
        DeclParameterDrive::Group(dg) => {
            let (parameter, options) = animations.find_group(ctx, &dg.group)?;
            sources.find_parameter(
                ctx,
                parameter,
                ParameterType::INT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            let Some(option) = options.iter().find(|o| o.name == dg.option) else {
                ctx.log(Log::AnimationGroupOptionNotFound(
                    dg.group.to_string(),
                    dg.option,
                ));
                return failure();
            };

            (
                parameter.to_string(),
                ParameterType::Int(option.value as u8),
            )
        }
        DeclParameterDrive::Switch(ds) => {
            let parameter = animations.find_switch(ctx, &ds.switch)?;
            sources.find_parameter(
                ctx,
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
            let parameter = animations.find_puppet(ctx, &dp.puppet)?;
            sources.find_parameter(
                ctx,
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
    ctx: &mut Logger,
    animations: &CompiledAnimations,
    control: DeclPuppetControl,
) -> Compiled<MenuItem> {
    let puppet_type = *control.puppet_type;
    match puppet_type {
        DeclPuppetType::Radial(pt) => success(MenuItem::Radial(MenuRadial {
            name: control.name,
            parameter: take_puppet_parameter(ctx, animations, pt.target)?,
        })),
        DeclPuppetType::TwoAxis {
            horizontal,
            vertical,
        } => success(MenuItem::TwoAxis(MenuTwoAxis {
            name: control.name,
            horizontal_axis: BiAxis {
                parameter: take_puppet_parameter(ctx, animations, horizontal.target)?,
                label_positive: horizontal.label_positive.unwrap_or_default(),
                label_negative: horizontal.label_negative.unwrap_or_default(),
            },
            vertical_axis: BiAxis {
                parameter: take_puppet_parameter(ctx, animations, vertical.target)?,
                label_positive: vertical.label_positive.unwrap_or_default(),
                label_negative: vertical.label_negative.unwrap_or_default(),
            },
        })),
        DeclPuppetType::FourAxis {
            up,
            down,
            left,
            right,
        } => success(MenuItem::FourAxis(MenuFourAxis {
            name: control.name,
            left_axis: UniAxis {
                parameter: take_puppet_parameter(ctx, animations, up.target)?,
                label: up.label_positive.unwrap_or_default(),
            },
            right_axis: UniAxis {
                parameter: take_puppet_parameter(ctx, animations, down.target)?,
                label: down.label_positive.unwrap_or_default(),
            },
            up_axis: UniAxis {
                parameter: take_puppet_parameter(ctx, animations, left.target)?,
                label: left.label_positive.unwrap_or_default(),
            },
            down_axis: UniAxis {
                parameter: take_puppet_parameter(ctx, animations, right.target)?,
                label: right.label_positive.unwrap_or_default(),
            },
        })),
    }
}

fn take_puppet_parameter(
    ctx: &mut Logger,
    animations: &CompiledAnimations,
    dpt: DeclPuppetTarget,
) -> Compiled<String> {
    let parameter = match dpt {
        DeclPuppetTarget::Puppet(dp) => {
            let parameter = animations.find_puppet(ctx, &dp.puppet)?;
            parameter.to_string()
        }
    };
    animations.sources().find_parameter(
        ctx,
        &parameter,
        ParameterType::FLOAT_TYPE,
        ParameterScope::MUST_EXPOSE,
    )?;

    success(parameter)
}
