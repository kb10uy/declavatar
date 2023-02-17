/*
use crate::decl::{
    animations::{Animations, NODE_NAME_ANIMATIONS},
    drivers::{Drivers, NODE_NAME_DRIVERS},
    get_argument,
    menu::{Menu, NODE_NAME_MENU},
    parameters::{Parameters, NODE_NAME_PARAMETERS},
    split_entries, DeclError, Result,
};
*/
use std::collections::HashMap;

use kdl::{KdlDocument, KdlNode, KdlValue};
use semver::Version;

pub const NODE_NAME_VERSION: &str = "version";
pub const NODE_NAME_AVATAR: &str = "avatar";
/*
#[derive(Debug, Clone)]
pub struct Document {
    version: Version,
    avatar: Avatar,
}

impl Document {
    pub fn parse(document: &KdlDocument) -> Result<Document> {
        // Detect version
        let nodes = document.nodes();
        let Some(version_node) = nodes.get(0) else {
            return Err(DeclError::NodeNotFound(NODE_NAME_VERSION, "root document"))
        };
        let (version_args, _) = split_entries(version_node.entries());
        let version_text = get_argument(&version_args, 0, "version")?;
        let version = Version::parse(version_text)?;

        // Other nodes
        let mut avatar = None;
        for node in &nodes[1..] {
            let node_name = node.name().value();
            match node_name {
                NODE_NAME_AVATAR => match avatar {
                    None => {
                        avatar = Some(node.parse(&version)?);
                    }
                    _ => return Err(DeclError::DuplicateNodeFound(NODE_NAME_VERSION)),
                },
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.to_string())),
            }
        }

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
    parameters_blocks: Vec<Parameters>,
    menu_blocks: Vec<Menu>,
}

impl DeclNode for Avatar {
    const NODE_NAME: &'static str = NODE_NAME_AVATAR;

    const CHILDREN_EXISTENCE: Option<bool> = Some(true);

    fn parse(
        version: &Version,
        _name: &str,
        args: &[&KdlValue],
        _props: &HashMap<&str, &KdlValue>,
        children: &[KdlNode],
    ) -> Result<Self> {
        let avatar_name = get_argument(&args, 0, "name")?;

        let mut animations_blocks = vec![];
        let mut drivers_blocks = vec![];
        let mut parameters_blocks = vec![];
        let mut menu_blocks = vec![];

        for child in children {
            let child_name = child.name().value();
            match child_name {
                NODE_NAME_ANIMATIONS => animations_blocks.push(child.parse(version)?),
                NODE_NAME_DRIVERS => drivers_blocks.push(child.parse(version)?),
                NODE_NAME_PARAMETERS => parameters_blocks.push(child.parse(version)?),
                NODE_NAME_MENU => menu_blocks.push(child.parse(version)?),
                otherwise => return Err(DeclError::InvalidNodeDetected(otherwise.into())),
            }
        }

        Ok(Avatar {
            name: avatar_name,
            animations_blocks,
            drivers_blocks,
            parameters_blocks,
            menu_blocks,
        })
    }
}
*/
