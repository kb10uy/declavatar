use crate::decl::{DeclError, DeclNode, DeclNodeExt, VERSION_REQ_SINCE_1_0};

use std::collections::HashMap;

use kdl::{KdlNode, KdlValue};
use semver::{Version, VersionReq};

use super::get_argument;

pub const NODE_NAME_MENU: &str = "menu";
pub const NODE_NAME_SUBMENU: &str = "submenu";
pub const NODE_NAME_BUTTON: &str = "button";
pub const NODE_NAME_TOGGLE: &str = "toggle";
pub const NODE_NAME_RADIAL: &str = "radial";
pub const NODE_NAME_TWO_AXIS: &str = "two-axis";
pub const NODE_NAME_FOUR_AXIS: &str = "four-axis";
pub const NODE_NAME_HORIZONTAL: &str = "four-axis";
pub const NODE_NAME_VERTICAL: &str = "four-axis";
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

impl DeclNode for Menu {
    const NODE_NAME: &'static str = NODE_NAME_MENU;

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &kdl::KdlValue>,
        children: &[KdlNode],
    ) -> super::Result<Self> {
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

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> super::Result<Self> {
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
    Group { group: String, option: String },
    IntParameter { name: String, value: u8 },
    BoolParameter { name: String, value: bool },
}

impl DeclNode for Boolean {
    const NODE_NAME: &'static str = "";

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = Some(false);

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> super::Result<Self> {
        todo!()
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
        horizontal: (String, String, String),
        vertical: (String, String, String),
    },
    FourAxis {
        left: (String, String),
        right: (String, String),
        up: (String, String),
        down: (String, String),
    },
}

impl DeclNode for Puppet {
    const NODE_NAME: &'static str = "";

    const REQUIRED_VERSION: VersionReq = VERSION_REQ_SINCE_1_0;

    const CHILDREN_EXISTENCE: Option<bool> = None;

    fn parse(
        version: &Version,
        name: &str,
        args: &[&KdlValue],
        props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> super::Result<Self> {
        todo!()
    }
}
