use crate::decl::{
    animation::{Animation, NODE_NAME_ANIMATION},
    driver::{Driver, NODE_NAME_DRIVE},
    entry::{get_argument, split_entries},
    validate_self_node, DeclError, FromNode, FromNodeExt,
};

use kdl::KdlNode;

pub const NODE_NAME_AVATAR: &str = "avatar";

/// Avatar descriptor. It should has specific structure like below:
/// ```kdl
/// avatar "avatar-name" {
///     animation {}
///     // ...
///
///     driver {}
///     // ...
///
///     menu {}
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Avatar {
    name: String,
    animations: Vec<Animation>,
    drivers: Vec<Driver>,
}

impl FromNode for Avatar {
    type Err = DeclError;

    fn from_node(node: &KdlNode) -> Result<Self, Self::Err> {
        validate_self_node(node, NODE_NAME_AVATAR)?;

        let (args, _props) = split_entries(node.entries());
        let name = get_argument(&args, 0, "name")?;

        let mut animations = vec![];
        let mut drivers = vec![];
        let children = node
            .children()
            .ok_or(DeclError::MustHaveChildren(NODE_NAME_AVATAR.into()))?;
        for child in children.nodes() {
            let child_name = child.name().value();
            match child_name {
                NODE_NAME_ANIMATION => animations.push(child.parse()?),
                NODE_NAME_DRIVE => drivers.push(child.parse()?),
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.to_string())),
            }
        }

        Ok(Avatar {
            name,
            animations,
            drivers,
        })
    }
}
