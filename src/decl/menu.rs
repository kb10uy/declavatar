use crate::decl::{DeclError, Result};

use std::collections::HashMap;

use kdl::{KdlNode, KdlValue};
use semver::Version;

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
/*
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

impl DeclNode for Menu {
    const NODE_NAME: &'static str = NODE_NAME_MENU;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        _name: &str,
        _args: &[&KdlValue],
        _props: &HashMap<&str, &kdl::KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let mut elements = vec![];
        for child in children {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_SUBMENU => {
                    let submenu = child.parse(version)?;
                    MenuElement::SubMenu(submenu)
                }
                NODE_NAME_BUTTON | NODE_NAME_TOGGLE => {
                    let boolean = child.parse_multi(version)?;
                    MenuElement::Boolean(boolean)
                }
                NODE_NAME_RADIAL | NODE_NAME_TWO_AXIS | NODE_NAME_FOUR_AXIS => {
                    let puppet = child.parse_multi(version)?;
                    MenuElement::Puppet(puppet)
                }
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.to_string())),
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

impl DeclNode for SubMenu {
    const NODE_NAME: &'static str = NODE_NAME_SUBMENU;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        _name: &str,
        args: &[&KdlValue],
        _props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let submenu_name = get_argument(args, 0, "name")?;
        let mut elements = vec![];
        for child in children {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_SUBMENU => {
                    let submenu = child.parse(version)?;
                    MenuElement::SubMenu(submenu)
                }
                NODE_NAME_BUTTON | NODE_NAME_TOGGLE => {
                    let boolean = child.parse_multi(version)?;
                    MenuElement::Boolean(boolean)
                }
                NODE_NAME_RADIAL | NODE_NAME_TWO_AXIS | NODE_NAME_FOUR_AXIS => {
                    let puppet = child.parse_multi(version)?;
                    MenuElement::Puppet(puppet)
                }
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.to_string())),
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

impl DeclNode for Boolean {
    const NODE_NAME: &'static str = "";

    const CHILDREN_EXISTENCE: Option<bool> = Some(false);

    fn parse(
        _version: &Version,
        _name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        _children: &[KdlNode],
    ) -> Result<Self> {
        let name = get_argument(args, 0, "name")?;
        let toggle = name == NODE_NAME_TOGGLE;

        let target_group = try_get_property(props, "group")?;
        let target_parameter = try_get_property(props, "parameter")?;
        let target = match (target_group, target_parameter) {
            (Some(group), None) => {
                let option = try_get_property(props, "option")?;
                BooleanTarget::Group {
                    name: group,
                    option,
                }
            }
            (None, Some(name)) => {
                let value: &KdlValue = get_property(props, "value")?;
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
                    return Err(DeclError::IncorrectType("int or bool"));
                }
            }
            _ => {
                return Err(DeclError::InvalidNodeDetected(
                    "ambiguous menu parameter".into(),
                ));
            }
        };

        Ok(Boolean {
            name,
            toggle,
            target,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Puppet {
    name: String,
    axes: Axes,
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
    ) -> Result<Option<HashMap<&'a str, &'a KdlNode>>> {
        use std::collections::hash_map::Entry;

        let mut extracted = HashMap::new();
        for child in children {
            let child_name = child.name().value();
            let Some(expected_name) = node_names.iter().find(|&&n| n == child_name) else {
                continue;
            };

            match extracted.entry(child_name) {
                Entry::Vacant(e) => {
                    e.insert(child);
                }
                Entry::Occupied(_) => {
                    return Err(DeclError::DuplicateNodeFound(expected_name));
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
        let (args, props) = split_entries(node.entries());

        let pair = (
            get_property(&props, "parameter")?,
            (
                get_argument(&args, 0, "first_name")?,
                get_argument(&args, 1, "second_name")?,
            ),
        );
        Ok(pair)
    }

    fn make_four_axis_pair(node: &KdlNode) -> Result<(String, String)> {
        let (args, props) = split_entries(node.entries());

        let pair = (
            get_property(&props, "parameter")?,
            get_argument(&args, 0, "name")?,
        );
        Ok(pair)
    }
}

impl DeclNode for Puppet {
    const NODE_NAME: &'static str = "";

    const CHILDREN_EXISTENCE: Option<bool> = None;

    fn parse(
        _version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let puppet_name = get_argument(args, 0, "name")?;
        let axes = match name {
            NODE_NAME_RADIAL => {
                let parameter = get_property(props, "parameter")?;
                Axes::Radial(parameter)
            }
            NODE_NAME_TWO_AXIS => {
                let axes_children = Axes::extract_nodes_just(
                    children,
                    &[NODE_NAME_HORIZONTAL, NODE_NAME_VERTICAL],
                )?
                .ok_or(DeclError::MustHaveChildren(NODE_NAME_TWO_AXIS.into()))?;

                let horizontal = Axes::make_two_axis_pair(axes_children[NODE_NAME_HORIZONTAL])?;
                let vertical = Axes::make_two_axis_pair(axes_children[NODE_NAME_VERTICAL])?;

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
                )?
                .ok_or(DeclError::MustHaveChildren(NODE_NAME_FOUR_AXIS.into()))?;

                let left = Axes::make_four_axis_pair(axes_children[NODE_NAME_LEFT])?;
                let right = Axes::make_four_axis_pair(axes_children[NODE_NAME_RIGHT])?;
                let up = Axes::make_four_axis_pair(axes_children[NODE_NAME_UP])?;
                let down = Axes::make_four_axis_pair(axes_children[NODE_NAME_DOWN])?;

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
*/
