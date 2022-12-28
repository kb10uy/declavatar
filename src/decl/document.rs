use crate::decl::{
    animations::Animations, drivers::Drivers, get_argument, split_entries,
    DeclError, FromNode, FromNodeExt,
};

use kdl::{KdlDocument, KdlNode};

pub const NODE_NAME_VERSION: &str = "version";
pub const NODE_NAME_AVATAR: &str = "avatar";

#[derive(Debug, Clone)]
pub struct Document {
    version: String,
    avatar: Avatar,
}

impl Document {
    pub fn parse(document: &KdlDocument) -> Result<Document, DeclError> {
        let mut version = None;
        let mut avatar = None;
        for node in document.nodes() {
            let node_name = node.name().value();
            match node_name {
                NODE_NAME_VERSION => match version {
                    None => {
                        let (args, _) = split_entries(node.entries());
                        version = Some(get_argument(&args, 0, "version")?);
                    }
                    _ => return Err(DeclError::DuplicateNodeFound(NODE_NAME_VERSION)),
                },
                NODE_NAME_AVATAR => match avatar {
                    None => {
                        avatar = Some(node.parse()?);
                    }
                    _ => return Err(DeclError::DuplicateNodeFound(NODE_NAME_VERSION)),
                },
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.to_string())),
            }
        }

        let Some(version) = version else {
            return Err(DeclError::NodeNotFound(NODE_NAME_VERSION, "root document"))
        };
        let Some(avatar) = avatar else {
            return Err(DeclError::NodeNotFound(NODE_NAME_AVATAR, "root document"))
        };
        Ok(Document { version, avatar })
    }
}

/// Avatar descriptor. It should has specific structure like below:
#[derive(Debug, Clone)]
pub struct Avatar {
    name: String,
    animations_blocks: Vec<Animations>,
    drivers_blocks: Vec<Drivers>,
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
                NODE_NAME_ANIMATIONS => animations.push(child.parse()?),
                NODE_NAME_DRIVERS => drivers.push(child.parse()?),
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.to_string())),
            }
        }

        Ok(Avatar {
            name,
            animations_blocks: animations,
            drivers_blocks: drivers,
        })
    }
}
