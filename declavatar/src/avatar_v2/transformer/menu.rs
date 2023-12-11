use crate::{
    avatar_v2::{
        data::{
            menu::{
                BiAxis, MenuBoolean, MenuFourAxis, MenuGroup, MenuItem, MenuRadial, MenuTwoAxis,
                UniAxis,
            },
            parameter::{ParameterScope, ParameterType},
        },
        logging::{LogKind, LoggingContext},
        transformer::{failure, success, Compiled, CompiledAnimations},
    },
    decl_v2::data::menu::{DeclMenuElement, DeclSubMenu},
};

pub fn compile_menu(
    ctx: &mut LoggingContext,
    animations: &CompiledAnimations,
    decl_menu_blocks: Vec<DeclSubMenu>,
) -> Compiled<Vec<MenuItem>> {
    let menu_elements = decl_menu_blocks
        .into_iter()
        .flat_map(|ab| ab.elements)
        .collect();
    let menu = compile_menu_group(ctx, animations, "", menu_elements)?;
    success(menu.items)
}

fn compile_menu_group(
    ctx: &mut LoggingContext,
    animations: &CompiledAnimations,
    name: impl Into<String>,
    decl_menu_elements: Vec<DeclMenuElement>,
) -> Compiled<MenuGroup> {
    let name = name.into();
    let mut items = vec![];

    for menu_element in decl_menu_elements {
        /*
        let Some(menu_item) = (match menu_element {
            DeclMenuElement::SubMenu(sm) => {
                compile_menu_group(ctx, animations, sm.name, sm.elements).map(MenuItem::SubMenu)
            }
            DeclMenuElement::Boolean(bc) => {
                let inner = compile_boolean(ctx, animations, bc.name, bc.target);
                if bc.hold {
                    inner.map(MenuItem::Toggle)
                } else {
                    inner.map(MenuItem::Button)
                }
            }
            DeclMenuElement::Puppet(p) => compile_puppet(ctx, animations, p.name, p.puppet_type),
        }) else {
            continue;
        };
        items.push(menu_item);
        */
    }

    success(MenuGroup { name, items })
}
/*
fn compile_boolean(
    ctx: &mut LoggingContext,
    animations: &CompiledAnimations,
    name: impl Into<String>,
    target: DeclBooleanControlTarget,
) -> Compiled<MenuBoolean> {
    let sources = animations.sources();

    let (parameter, value) = match target {
        DeclBooleanControlTarget::Group {
            name: group_name,
            option,
        } => {
            let (parameter, options) = animations.find_group(ctx, &group_name)?;
            sources.find_parameter(
                ctx,
                parameter,
                ParameterType::INT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            let option_name = option.unwrap_or_else(|| group_name.clone());
            let Some(option) = options.iter().find(|o| o.name == option_name) else {
                ctx.log_error(LogKind::AnimationGroupOptionNotFound(
                    group_name.to_string(),
                    option_name,
                ));
                return failure();
            };

            (
                group_name.to_string(),
                ParameterType::Int(option.value as u8),
            )
        }
        DeclBooleanControlTarget::Switch {
            name: switch_name,
            invert,
        } => {
            let parameter = animations.find_switch(ctx, &switch_name)?;
            sources.find_parameter(
                ctx,
                parameter,
                ParameterType::BOOL_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            (
                parameter.to_string(),
                ParameterType::Bool(!invert.unwrap_or(false)),
            )
        }
        DeclBooleanControlTarget::IntParameter { name, value } => {
            sources.find_parameter(
                ctx,
                &name,
                ParameterType::INT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;
            (name, ParameterType::Int(value))
        }
        DeclBooleanControlTarget::BoolParameter { name, value } => {
            sources.find_parameter(
                ctx,
                &name,
                ParameterType::BOOL_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;
            (name, ParameterType::Bool(value))
        }
    };

    success(MenuBoolean {
        name: name.into(),
        parameter,
        value,
    })
}

fn compile_puppet(
    ctx: &mut LoggingContext,
    animations: &CompiledAnimations,
    name: impl Into<String>,
    axes: DeclPuppetAxes,
) -> Compiled<MenuItem> {
    let sources = animations.sources();

    match axes {
        DeclPuppetAxes::Radial(param) => {
            sources.find_parameter(
                ctx,
                &param,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            success(MenuItem::Radial(MenuRadial {
                name: name.into(),
                parameter: param,
            }))
        }
        DeclPuppetAxes::TwoAxis {
            horizontal,
            vertical,
        } => {
            sources.find_parameter(
                ctx,
                &horizontal.0,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;
            sources.find_parameter(
                ctx,
                &vertical.0,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            success(MenuItem::TwoAxis(MenuTwoAxis {
                name: name.into(),
                horizontal_axis: BiAxis {
                    parameter: horizontal.0,
                    label_positive: horizontal.1 .0,
                    label_negative: horizontal.1 .1,
                },
                vertical_axis: BiAxis {
                    parameter: vertical.0,
                    label_positive: vertical.1 .0,
                    label_negative: vertical.1 .1,
                },
            }))
        }
        DeclPuppetAxes::FourAxis {
            left,
            right,
            up,
            down,
        } => {
            sources.find_parameter(
                ctx,
                &left.0,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;
            sources.find_parameter(
                ctx,
                &right.0,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;
            sources.find_parameter(
                ctx,
                &up.0,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;
            sources.find_parameter(
                ctx,
                &down.0,
                ParameterType::FLOAT_TYPE,
                ParameterScope::MUST_EXPOSE,
            )?;

            success(MenuItem::FourAxis(MenuFourAxis {
                name: name.into(),
                left_axis: UniAxis {
                    parameter: left.0,
                    label: left.1,
                },
                right_axis: UniAxis {
                    parameter: right.0,
                    label: right.1,
                },
                up_axis: UniAxis {
                    parameter: up.0,
                    label: up.1,
                },
                down_axis: UniAxis {
                    parameter: down.0,
                    label: down.1,
                },
            }))
        }
    }
}
*/
