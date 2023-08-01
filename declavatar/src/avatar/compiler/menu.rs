use crate::{
    avatar::{
        compiler::{AvatarCompiler, CompiledAnimations},
        data::{
            AnimationGroup, AnimationGroupContent, BiAxis, MenuBoolean, MenuFourAxis, MenuGroup,
            MenuItem, MenuRadial, MenuTwoAxis, Parameter, ParameterType, UniAxis,
        },
        error::Result,
    },
    compiler::{Compile, Compiler},
    decl::data::{
        BooleanControlTarget as DeclBooleanControlTarget, Menu as DeclMenu,
        MenuElement as DeclMenuElement, PuppetAxes as DeclPuppetAxes, PuppetControl as DeclPuppet,
    },
};

impl Compile<(Vec<DeclMenu>, &CompiledAnimations)> for AvatarCompiler {
    type Output = MenuGroup;

    fn compile(
        &mut self,
        (menu_blocks, compiled_anims): (Vec<DeclMenu>, &CompiledAnimations),
    ) -> Result<MenuGroup> {
        let animation_groups = &compiled_anims.animation_groups;
        let parameters = &compiled_anims.parameters;

        let mut top_items = vec![];
        let mut next_group_id = 1;

        let menu_elements = menu_blocks.into_iter().flat_map(|ab| ab.elements);
        for menu_element in menu_elements {
            let Some(menu_item) = (match menu_element {
                DeclMenuElement::SubMenu(sm) => {
                    let (submenu, next_id) = self.parse((sm.elements, next_group_id, sm.name, parameters, animation_groups))?;
                    next_group_id = next_id;
                    Some(MenuItem::SubMenu(submenu))
                }
                DeclMenuElement::Boolean(bc) => {
                    let inner = self.parse((bc.target, bc.name,parameters, animation_groups))?;
                    if bc.toggle {
                        inner.map(MenuItem::Toggle)
                    } else {
                        inner.map(MenuItem::Button)
                    }
                },
                DeclMenuElement::Puppet(p) => self.parse((p, parameters))?,
            }) else {
                continue;
            };
            top_items.push(menu_item);
        }

        if top_items.len() > 8 {
            self.warn("too many top-level menu items, first 8 item will remain".into());
            top_items.drain(8..);
        }

        Ok(MenuGroup {
            name: "".into(),
            id: 0,
            items: top_items,
        })
    }
}

impl
    Compile<(
        Vec<DeclMenuElement>,
        usize,
        String,
        &Vec<Parameter>,
        &Vec<AnimationGroup>,
    )> for AvatarCompiler
{
    type Output = (MenuGroup, usize);

    fn compile(
        &mut self,
        (menu_elements, current_group_id, name, parameters, animation_groups): (
            Vec<DeclMenuElement>,
            usize,
            String,
            &Vec<Parameter>,
            &Vec<AnimationGroup>,
        ),
    ) -> Result<(MenuGroup, usize)> {
        let mut items = vec![];
        let mut next_group_id = current_group_id + 1;

        for menu_element in menu_elements {
            let Some(menu_item) = (match menu_element {
                DeclMenuElement::SubMenu(sm) => {
                    let (submenu, next_id) = self.parse((sm.elements, next_group_id, sm.name, parameters, animation_groups))?;
                    next_group_id = next_id;
                    Some(MenuItem::SubMenu(submenu))
                }
                DeclMenuElement::Boolean(bc) => {
                    let inner = self.parse((bc.target, bc.name,parameters, animation_groups))?;
                    if bc.toggle {
                        inner.map(MenuItem::Toggle)
                    } else {
                        inner.map(MenuItem::Button)
                    }
                },
                DeclMenuElement::Puppet(p) => self.parse((p, parameters))?,
            }) else {
                continue;
            };
            items.push(menu_item);
        }

        if items.len() > 8 {
            self.warn("too many top-level menu items, first 8 item will remain".into());
            items.drain(8..);
        }

        Ok((
            MenuGroup {
                name,
                id: current_group_id,
                items,
            },
            next_group_id,
        ))
    }
}

impl
    Compile<(
        DeclBooleanControlTarget,
        String,
        &Vec<Parameter>,
        &Vec<AnimationGroup>,
    )> for AvatarCompiler
{
    type Output = Option<MenuBoolean>;

    fn compile(
        &mut self,
        (decl_boolean, name, parameters, animation_groups): (
            DeclBooleanControlTarget,
            String,
            &Vec<Parameter>,
            &Vec<AnimationGroup>,
        ),
    ) -> Result<Option<MenuBoolean>> {
        let (parameter, value) = match decl_boolean {
            DeclBooleanControlTarget::Group {
                name: group_name,
                option,
            } => {
                let Some(group) = animation_groups.iter().find(|ag| ag.name == group_name) else {
                    self.error(format!("animation group '{group_name}' not found"));
                    return Ok(None);
                };
                let AnimationGroup { content: AnimationGroupContent::Group { parameter, options, .. }, .. } = group else {
                    self.error(format!("animation group '{group_name}' is not selection group"));
                    return Ok(None);
                };
                if !self.ensure((
                    parameters,
                    parameter.as_str(),
                    ParameterType::INT_TYPE,
                    true,
                ))? {
                    self.error(format!(
                        "animation group '{group_name}' should refer int parameter"
                    ));
                    return Ok(None);
                };

                let option_name = option.unwrap_or_else(|| group_name.clone());
                let Some(option) = options.iter().find(|o| o.name == option_name) else {
                    self.error(format!("option '{option_name}' not found in '{group_name}'"));
                    return Ok(None);
                };
                (group_name, ParameterType::Int(option.order as u8))
            }
            DeclBooleanControlTarget::Switch {
                name: switch_name,
                invert,
            } => {
                let Some(switch) = animation_groups.iter().find(|ag| ag.name == switch_name) else {
                    self.error(format!("animation switch '{switch_name}' not found"));
                    return Ok(None);
                };
                let AnimationGroup { content: AnimationGroupContent::Switch { parameter, .. }, .. } = switch else {
                    self.error(format!("animation group '{switch_name}' is not switch group"));
                    return Ok(None);
                };
                if !self.ensure((
                    parameters,
                    parameter.as_str(),
                    ParameterType::BOOL_TYPE,
                    true,
                ))? {
                    self.error(format!(
                        "animation group '{switch_name}' should refer bool parameter"
                    ));
                    return Ok(None);
                };

                (
                    parameter.clone(),
                    ParameterType::Bool(!invert.unwrap_or(false)),
                )
            }
            DeclBooleanControlTarget::IntParameter { name, value } => {
                if !self.ensure((parameters, name.as_str(), ParameterType::INT_TYPE, true))? {
                    return Ok(None);
                };
                (name, ParameterType::Int(value))
            }
            DeclBooleanControlTarget::BoolParameter { name, value } => {
                if !self.ensure((parameters, name.as_str(), ParameterType::BOOL_TYPE, true))? {
                    return Ok(None);
                };
                (name, ParameterType::Bool(value))
            }
        };

        Ok(Some(MenuBoolean {
            name,
            parameter,
            value,
        }))
    }
}

impl Compile<(DeclPuppet, &Vec<Parameter>)> for AvatarCompiler {
    type Output = Option<MenuItem>;

    fn compile(
        &mut self,
        (decl_puppet, parameters): (DeclPuppet, &Vec<Parameter>),
    ) -> Result<Option<MenuItem>> {
        match decl_puppet.axes {
            DeclPuppetAxes::Radial(param) => {
                if !self.ensure((parameters, param.as_str(), ParameterType::FLOAT_TYPE, true))? {
                    return Ok(None);
                };

                Ok(Some(MenuItem::Radial(MenuRadial {
                    name: decl_puppet.name,
                    parameter: param,
                })))
            }
            DeclPuppetAxes::TwoAxis {
                horizontal,
                vertical,
            } => {
                if !self.ensure((
                    parameters,
                    horizontal.0.as_str(),
                    ParameterType::FLOAT_TYPE,
                    true,
                ))? {
                    return Ok(None);
                };
                if !self.ensure((
                    parameters,
                    vertical.0.as_str(),
                    ParameterType::FLOAT_TYPE,
                    true,
                ))? {
                    return Ok(None);
                };

                Ok(Some(MenuItem::TwoAxis(MenuTwoAxis {
                    name: decl_puppet.name,
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
                })))
            }
            DeclPuppetAxes::FourAxis {
                left,
                right,
                up,
                down,
            } => {
                if !self.ensure((parameters, left.0.as_str(), ParameterType::FLOAT_TYPE, true))? {
                    return Ok(None);
                };
                if !self.ensure((
                    parameters,
                    right.0.as_str(),
                    ParameterType::FLOAT_TYPE,
                    true,
                ))? {
                    return Ok(None);
                };
                if !self.ensure((parameters, up.0.as_str(), ParameterType::FLOAT_TYPE, true))? {
                    return Ok(None);
                };
                if !self.ensure((parameters, down.0.as_str(), ParameterType::FLOAT_TYPE, true))? {
                    return Ok(None);
                };

                Ok(Some(MenuItem::FourAxis(MenuFourAxis {
                    name: decl_puppet.name,
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
                })))
            }
        }
    }
}
