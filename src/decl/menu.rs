use crate::decl::{deconstruct_node, DeclError, DeclErrorKind, Result};

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

#[derive(Debug, Clone)]
pub struct Menu {
    elements: Vec<MenuElement>,
}

#[derive(Debug, Clone)]
pub enum MenuElement {
    SubMenu(SubMenu),
    Boolean(Boolean),
    Puppet(Puppet),
}

impl Menu {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (_, _, children) = deconstruct_node(source, node, Some(NODE_NAME_MENU), Some(true))?;

        let mut elements = vec![];
        for child in children {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_SUBMENU => {
                    let submenu = SubMenu::parse(child, source)?;
                    MenuElement::SubMenu(submenu)
                }
                NODE_NAME_BUTTON | NODE_NAME_TOGGLE => {
                    let boolean = Boolean::parse(child, source)?;
                    MenuElement::Boolean(boolean)
                }
                NODE_NAME_RADIAL | NODE_NAME_TWO_AXIS | NODE_NAME_FOUR_AXIS => {
                    let puppet = Puppet::parse(child, source)?;
                    MenuElement::Puppet(puppet)
                }
                otherwise => {
                    return Err(DeclError::new(
                        source,
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ));
                }
            };
            elements.push(element);
        }

        Ok(Menu { elements })
    }
}

#[derive(Debug, Clone)]
pub struct SubMenu {
    name: String,
    elements: Vec<MenuElement>,
}

impl SubMenu {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (_, entries, children) =
            deconstruct_node(source, node, Some(NODE_NAME_SUBMENU), Some(true))?;

        let submenu_name = entries.get_argument(0, "name")?;
        let mut elements = vec![];
        for child in children {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_SUBMENU => {
                    let submenu = SubMenu::parse(child, source)?;
                    MenuElement::SubMenu(submenu)
                }
                NODE_NAME_BUTTON | NODE_NAME_TOGGLE => {
                    let boolean = Boolean::parse(child, source)?;
                    MenuElement::Boolean(boolean)
                }
                NODE_NAME_RADIAL | NODE_NAME_TWO_AXIS | NODE_NAME_FOUR_AXIS => {
                    let puppet = Puppet::parse(child, source)?;
                    MenuElement::Puppet(puppet)
                }
                otherwise => {
                    return Err(DeclError::new(
                        source,
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
}

#[derive(Debug, Clone)]
pub struct Boolean {
    name: String,
    toggle: bool,
    target: BooleanTarget,
}

impl Boolean {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (name, entries, _) = deconstruct_node(source, node, None, Some(true))?;
        let toggle = name == NODE_NAME_TOGGLE;

        let item_name = entries.get_argument(0, "name")?;
        let target_group = entries.try_get_property("group")?;
        let target_parameter = entries.try_get_property("parameter")?;
        let target = match (target_group, target_parameter) {
            (Some(group), None) => {
                let option = entries.try_get_property("option")?;
                BooleanTarget::Group {
                    name: group,
                    option,
                }
            }
            (None, Some(name)) => {
                let value: &KdlValue = entries.get_property("value")?;
                let int_value = value.as_i64();
                let bool_value = value.as_bool();
                if let Some(value) = int_value {
                    BooleanTarget::IntParameter {
                        name,
                        value: value as u8,
                    }
                } else if let Some(value) = bool_value {
                    BooleanTarget::BoolParameter { name, value }
                } else {
                    let entry_span = node.get("value").expect("must have entry").span();
                    return Err(DeclError::new(
                        source,
                        entry_span,
                        DeclErrorKind::IncorrectType("int or bool"),
                    ));
                }
            }
            _ => {
                return Err(DeclError::new(
                    source,
                    node.name().span(),
                    DeclErrorKind::InvalidNodeDetected,
                ));
            }
        };

        Ok(Boolean {
            name: item_name,
            toggle,
            target,
        })
    }
}

#[derive(Debug, Clone)]
pub enum BooleanTarget {
    Group {
        name: String,
        option: Option<String>,
    },
    IntParameter {
        name: String,
        value: u8,
    },
    BoolParameter {
        name: String,
        value: bool,
    },
}

#[derive(Debug, Clone)]
pub struct Puppet {
    name: String,
    axes: Axes,
}

impl Puppet {
    pub fn parse(node: &KdlNode, source: &str) -> Result<Self> {
        let (name, entries, children) = deconstruct_node(source, node, None, None)?;

        let puppet_name = entries.get_argument(0, "name")?;
        let axes = match name {
            NODE_NAME_RADIAL => {
                let parameter = entries.get_property("parameter")?;
                Axes::Radial(parameter)
            }
            NODE_NAME_TWO_AXIS => {
                let axes_children = Axes::extract_nodes_just(
                    children,
                    &[NODE_NAME_HORIZONTAL, NODE_NAME_VERTICAL],
                    source,
                )?
                .ok_or_else(|| {
                    DeclError::new(source, node.name().span(), DeclErrorKind::MustHaveChildren)
                })?;

                let horizontal =
                    Axes::make_two_axis_pair(axes_children[NODE_NAME_HORIZONTAL], source)?;
                let vertical = Axes::make_two_axis_pair(axes_children[NODE_NAME_VERTICAL], source)?;

                Axes::TwoAxis {
                    horizontal,
                    vertical,
                }
            }
            NODE_NAME_FOUR_AXIS => {
                let axes_children = Axes::extract_nodes_just(
                    children,
                    &[
                        NODE_NAME_LEFT,
                        NODE_NAME_RIGHT,
                        NODE_NAME_UP,
                        NODE_NAME_DOWN,
                    ],
                    source,
                )?
                .ok_or_else(|| {
                    DeclError::new(source, node.name().span(), DeclErrorKind::MustHaveChildren)
                })?;

                let left = Axes::make_four_axis_pair(axes_children[NODE_NAME_LEFT], source)?;
                let right = Axes::make_four_axis_pair(axes_children[NODE_NAME_RIGHT], source)?;
                let up = Axes::make_four_axis_pair(axes_children[NODE_NAME_UP], source)?;
                let down = Axes::make_four_axis_pair(axes_children[NODE_NAME_DOWN], source)?;

                Axes::FourAxis {
                    left,
                    right,
                    up,
                    down,
                }
            }
            _ => unreachable!("axis type already refined"),
        };

        Ok(Puppet {
            name: puppet_name,
            axes,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Axes {
    Radial(String),
    TwoAxis {
        horizontal: (String, (String, String)),
        vertical: (String, (String, String)),
    },
    FourAxis {
        left: (String, String),
        right: (String, String),
        up: (String, String),
        down: (String, String),
    },
}

impl Axes {
    fn extract_nodes_just<'a>(
        children: &'a [KdlNode],
        node_names: &'a [&'static str],
        source: &'a str,
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
                        source,
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

    fn make_two_axis_pair(node: &KdlNode, source: &str) -> Result<(String, (String, String))> {
        let (_, entries, _) = deconstruct_node(source, node, None, Some(false))?;

        let pair = (
            entries.get_property("parameter")?,
            (
                entries.get_argument(0, "first_name")?,
                entries.get_argument(1, "second_name")?,
            ),
        );
        Ok(pair)
    }

    fn make_four_axis_pair(node: &KdlNode, source: &str) -> Result<(String, String)> {
        let (_, entries, _) = deconstruct_node(source, node, None, Some(false))?;

        let pair = (
            entries.get_property("parameter")?,
            entries.get_argument(0, "name")?,
        );
        Ok(pair)
    }
}
