use kdl::KdlNode;

use crate::decl::{validate_self_node, DeclError, FromNode, FromNodeExt};

pub const NODE_NAME_MENU: &str = "menu";
pub const NODE_NAME_SUBMENU: &str = "submenu";
pub const NODE_NAME_BUTTON: &str = "button";
pub const NODE_NAME_TOGGLE: &str = "toggle";
pub const NODE_NAME_RADIAL: &str = "radial";
pub const NODE_NAME_TWO_AXIS: &str = "two-axis";
pub const NODE_NAME_FOUR_AXIS: &str = "four-axis";

#[derive(Debug, Clone)]
pub struct Menu {
    elements: Vec<MenuElement>,
}

impl FromNode for Menu {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        validate_self_node(node, NODE_NAME_MENU)?;

        let mut elements = vec![];
        let children = node
            .children()
            .ok_or(DeclError::MustHaveChildren(NODE_NAME_MENU.into()))?;
        for child in children.nodes() {
            let child_name = child.name().value();
            let element = match child_name {
                NODE_NAME_SUBMENU => MenuElement::SubMenu(child.parse()?),
                NODE_NAME_BUTTON => MenuElement::Button(child.parse()?),
                NODE_NAME_TOGGLE => MenuElement::Toggle(child.parse()?),
                NODE_NAME_RADIAL => MenuElement::Radial(child.parse()?),
                NODE_NAME_TWO_AXIS => MenuElement::TwoAxis(child.parse()?),
                NODE_NAME_FOUR_AXIS => MenuElement::FourAxis(child.parse()?),
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.to_string())),
            };
            elements.push(element);
        }

        Ok(Menu { elements })
    }
}

#[derive(Debug, Clone)]
pub enum MenuElement {
    SubMenu(SubMenu),
    Button(Button),
    Toggle(Toggle),
    Radial(Radial),
    TwoAxis(TwoAxis),
    FourAxis(FourAxis),
}

impl FromNode for MenuElement {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        let name = node.name().value();
        let element = match name {
            NODE_NAME_SUBMENU => MenuElement::SubMenu(child.parse()?),
            NODE_NAME_BUTTON => MenuElement::Button(child.parse()?),
            NODE_NAME_TOGGLE => MenuElement::Toggle(child.parse()?),
            NODE_NAME_RADIAL => MenuElement::Radial(child.parse()?),
            NODE_NAME_TWO_AXIS => MenuElement::TwoAxis(child.parse()?),
            NODE_NAME_FOUR_AXIS => MenuElement::FourAxis(child.parse()?),
            otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.to_string())),
        };
        Ok(element)
    }
}

#[derive(Debug, Clone)]
pub struct SubMenu {
    name: String,
    elements: Vec<MenuElement>,
}

#[derive(Debug, Clone)]
pub struct Button {
    name: String,
    target: String,
    option: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Toggle {
    name: String,
    target: String,
    option: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Axis {
    name: String,
    target: String,
}
