use crate::decl::{
    get_argument, split_entries, DeclError, DeclNode, DeclNodeExt, VERSION_REQ_SINCE_1_0,
};

use kdl::KdlNode;

pub const NODE_NAME_DRIVERS: &str = "drivers";
pub const NODE_NAME_GROUP: &str = "group";

/*
#[derive(Debug, Clone)]
pub struct Drivers {
    groups: Vec<Driver>,
}

impl FromNode for Drivers {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        validate_self_node(node, NODE_NAME_DRIVERS)?;

        let mut groups = vec![];
        let children = node
            .children()
            .ok_or(DeclError::MustHaveChildren(NODE_NAME_DRIVERS.into()))?;
        for child in children.nodes() {
            let child_name = child.name().value();
            if child_name != NODE_NAME_GROUP {
                return Err(DeclError::InvalidNodeDetected(child_name.to_string()));
            }
            groups.push(child.parse()?);
        }

        Ok(Drivers { groups })
    }
}

#[derive(Debug, Clone)]
pub struct Driver {
    name: String,
}

impl FromNode for Driver {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        validate_self_node(node, NODE_NAME_GROUP)?;

        let (args, _props) = split_entries(node.entries());
        let name = get_argument(&args, 0, "name")?;

        Ok(Driver { name })
    }
}
*/
