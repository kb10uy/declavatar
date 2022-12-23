use crate::decl::{validate_self_node, DeclError, FromNode, FromNodeExt};

pub const NODE_NAME_MENU: &str = "menu";
pub const NODE_NAME_: &str = "menu";
pub const NODE_NAME_MENU: &str = "menu";
pub const NODE_NAME_MENU: &str = "menu";
pub const NODE_NAME_MENU: &str = "menu";
pub const NODE_NAME_MENU: &str = "menu";

#[derive(Debug, Clone)]
pub struct Menu {
    elements: Vec<Group>,
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
            if child_name != NODE_NAME_GROUP {
                return Err(DeclError::InvalidNodeDetected(child_name.to_string()));
            }
            elements.push(child.parse()?);
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
