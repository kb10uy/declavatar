use crate::decl::{
    compiler::deconstruct_node,
    data::{
        BooleanControl, BooleanControlTarget, Menu, MenuElement, PuppetAxes, PuppetControl, SubMenu,
    },
    error::{DeclError, DeclErrorKind, Result},
};

use std::collections::HashMap;

use kdl::{KdlNode, KdlValue};

pub const NODE_NAME_MENU: &str = "menu";
pub const NODE_NAME_SUBMENU: &str = "submenu";
pub const NODE_NAME_BUTTON: &str = "button";
pub const NODE_NAME_TOGGLE: &str = "toggle";
pub const NODE_NAME_RADIAL: &str = "radial";
pub const NODE_NAME_TWO_AXIS: &str = "two-axis";
pub const NODE_NAME_FOUR_AXIS: &str = "four-axis";
pub const NODE_NAME_HORIZONTAL: &str = "horizontal";
pub const NODE_NAME_VERTICAL: &str = "vertical";
pub const NODE_NAME_UP: &str = "up";
pub const NODE_NAME_DOWN: &str = "down";
pub const NODE_NAME_LEFT: &str = "left";
pub const NODE_NAME_RIGHT: &str = "right";

pub fn compile_menu(node: &KdlNode) -> Result<Menu> {
    let (_, _, children) = deconstruct_node(node, Some(NODE_NAME_MENU), Some(true))?;

    let mut elements = vec![];
    for child in children {
        let child_name = child.name().value();
        let element = match child_name {
            NODE_NAME_SUBMENU => {
                let submenu = compile_submenu(child)?;
                MenuElement::SubMenu(submenu)
            }
            NODE_NAME_BUTTON | NODE_NAME_TOGGLE => {
                let boolean = compile_boolean_control(child)?;
                MenuElement::Boolean(boolean)
            }
            NODE_NAME_RADIAL | NODE_NAME_TWO_AXIS | NODE_NAME_FOUR_AXIS => {
                let puppet = compile_puppet(child)?;
                MenuElement::Puppet(puppet)
            }
            _ => {
                return Err(DeclError::new(
                    child.name().span(),
                    DeclErrorKind::InvalidNodeDetected,
                ));
            }
        };
        elements.push(element);
    }

    Ok(Menu { elements })
}

fn compile_submenu(node: &KdlNode) -> Result<SubMenu> {
    let (_, entries, children) = deconstruct_node(node, Some(NODE_NAME_SUBMENU), Some(true))?;

    let submenu_name = entries.get_argument(0, "name")?;
    let mut elements = vec![];
    for child in children {
        let child_name = child.name().value();
        let element = match child_name {
            NODE_NAME_SUBMENU => {
                let submenu = compile_submenu(child)?;
                MenuElement::SubMenu(submenu)
            }
            NODE_NAME_BUTTON | NODE_NAME_TOGGLE => {
                let boolean = compile_boolean_control(child)?;
                MenuElement::Boolean(boolean)
            }
            NODE_NAME_RADIAL | NODE_NAME_TWO_AXIS | NODE_NAME_FOUR_AXIS => {
                let puppet = compile_puppet(child)?;
                MenuElement::Puppet(puppet)
            }
            _ => {
                return Err(DeclError::new(
                    child.name().span(),
                    DeclErrorKind::InvalidNodeDetected,
                ));
            }
        };
        elements.push(element);
    }

    Ok(SubMenu {
        name: submenu_name,
        elements,
    })
}

fn compile_boolean_control(node: &KdlNode) -> Result<BooleanControl> {
    let (name, entries, _) = deconstruct_node(node, None, Some(false))?;
    let toggle = name == NODE_NAME_TOGGLE;

    let item_name = entries.get_argument(0, "name")?;
    let target_group = entries.try_get_property("group")?;
    let target_switch = entries.try_get_property("switch")?;
    let target_parameter = entries.try_get_property("parameter")?;

    let target = match (target_group, target_switch, target_parameter) {
        (Some(group_name), None, None) => {
            let option = entries.try_get_property("option")?;
            BooleanControlTarget::Group {
                name: group_name,
                option,
            }
        }
        (None, Some(switch_name), None) => {
            let invert = entries.try_get_property("invert")?;
            BooleanControlTarget::Switch {
                name: switch_name,
                invert,
            }
        }
        (None, None, Some(parameter_name)) => {
            let value: &KdlValue = entries.get_property("value")?;
            let int_value = value.as_i64();
            let bool_value = value.as_bool();
            if let Some(value) = int_value {
                BooleanControlTarget::IntParameter {
                    name: parameter_name,
                    value: value as u8,
                }
            } else if let Some(value) = bool_value {
                BooleanControlTarget::BoolParameter {
                    name: parameter_name,
                    value,
                }
            } else {
                let entry_span = node.get("value").expect("must have entry").span();
                return Err(DeclError::new(
                    entry_span,
                    DeclErrorKind::IncorrectType("int or bool"),
                ));
            }
        }

        _ => {
            return Err(DeclError::new(
                node.name().span(),
                DeclErrorKind::InvalidNodeDetected,
            ));
        }
    };

    Ok(BooleanControl {
        name: item_name,
        toggle,
        target,
    })
}

fn compile_puppet(node: &KdlNode) -> Result<PuppetControl> {
    let (name, entries, children) = deconstruct_node(node, None, None)?;

    let puppet_name = entries.get_argument(0, "name")?;
    let axes = match name {
        NODE_NAME_RADIAL => {
            let parameter = entries.get_property("parameter")?;
            PuppetAxes::Radial(parameter)
        }
        NODE_NAME_TWO_AXIS => {
            let axes_children =
                extract_nodes_just(children, &[NODE_NAME_HORIZONTAL, NODE_NAME_VERTICAL])?
                    .ok_or_else(|| {
                        DeclError::new(node.name().span(), DeclErrorKind::MustHaveChildren)
                    })?;

            let horizontal = make_two_axis_pair(axes_children[NODE_NAME_HORIZONTAL])?;
            let vertical = make_two_axis_pair(axes_children[NODE_NAME_VERTICAL])?;

            PuppetAxes::TwoAxis {
                horizontal,
                vertical,
            }
        }
        NODE_NAME_FOUR_AXIS => {
            let axes_children = extract_nodes_just(
                children,
                &[
                    NODE_NAME_LEFT,
                    NODE_NAME_RIGHT,
                    NODE_NAME_UP,
                    NODE_NAME_DOWN,
                ],
            )?
            .ok_or_else(|| DeclError::new(node.name().span(), DeclErrorKind::MustHaveChildren))?;

            let left = make_four_axis_pair(axes_children[NODE_NAME_LEFT])?;
            let right = make_four_axis_pair(axes_children[NODE_NAME_RIGHT])?;
            let up = make_four_axis_pair(axes_children[NODE_NAME_UP])?;
            let down = make_four_axis_pair(axes_children[NODE_NAME_DOWN])?;

            PuppetAxes::FourAxis {
                left,
                right,
                up,
                down,
            }
        }
        _ => unreachable!("axis type already refined"),
    };

    Ok(PuppetControl {
        name: puppet_name,
        axes,
    })
}

fn extract_nodes_just<'a>(
    children: &'a [KdlNode],
    node_names: &'a [&'static str],
) -> Result<Option<HashMap<&'a str, &'a KdlNode>>> {
    use std::collections::hash_map::Entry;

    let mut extracted = HashMap::new();
    for child in children {
        let child_name = child.name().value();

        match extracted.entry(child_name) {
            Entry::Vacant(e) => {
                e.insert(child);
            }
            Entry::Occupied(_) => {
                return Err(DeclError::new(
                    child.name().span(),
                    DeclErrorKind::DuplicateNodeFound,
                ));
            }
        }
    }

    if extracted.len() == node_names.len() {
        Ok(Some(extracted))
    } else {
        Ok(None)
    }
}

fn make_two_axis_pair(node: &KdlNode) -> Result<(String, (String, String))> {
    let (_, entries, _) = deconstruct_node(node, None, Some(false))?;

    let pair = (
        entries.get_property("parameter")?,
        (
            entries.get_argument(0, "first_name")?,
            entries.get_argument(1, "second_name")?,
        ),
    );
    Ok(pair)
}

fn make_four_axis_pair(node: &KdlNode) -> Result<(String, String)> {
    let (_, entries, _) = deconstruct_node(node, None, Some(false))?;

    let pair = (
        entries.get_property("parameter")?,
        entries.get_argument(0, "name")?,
    );
    Ok(pair)
}
