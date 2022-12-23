use crate::decl::{
    entry::{get_argument, split_entries},
    validate_self_node, DeclError, FromNode, FromNodeExt,
};

use kdl::KdlNode;

pub const NODE_NAME_DRIVE: &str = "driver";
pub const NODE_NAME_GROUP: &str = "group";

#[derive(Debug, Clone)]
pub struct Driver {
    groups: Vec<Group>,
}

impl FromNode for Driver {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        validate_self_node(node, NODE_NAME_DRIVE)?;

        let mut groups = vec![];
        let children = node
            .children()
            .ok_or(DeclError::MustHaveChildren(NODE_NAME_DRIVE.into()))?;
        for child in children.nodes() {
            let child_name = child.name().value();
            if child_name != NODE_NAME_GROUP {
                return Err(DeclError::InvalidNodeDetected(child_name.to_string()));
            }
            groups.push(child.parse()?);
        }

        Ok(Driver { groups })
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    name: String,
}

impl FromNode for Group {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        validate_self_node(node, NODE_NAME_GROUP)?;

        let (args, _props) = split_entries(node.entries());
        let name = get_argument(&args, 0, "name")?;

        Ok(Group { name })
    }
}
