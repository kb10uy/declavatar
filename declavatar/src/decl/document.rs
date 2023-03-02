use crate::decl::{
    animations::{Animations, NODE_NAME_ANIMATIONS},
    deconstruct_node,
    drivers::{Drivers, NODE_NAME_DRIVERS},
    menu::{Menu, NODE_NAME_MENU},
    parameters::{Parameters, NODE_NAME_PARAMETERS},
    DeclError, DeclErrorKind, Result,
};

use kdl::{KdlDocument, KdlNode};
use miette::SourceSpan;
use semver::Version;

pub const NODE_NAME_VERSION: &str = "version";
pub const NODE_NAME_AVATAR: &str = "avatar";

#[derive(Debug, Clone)]
pub struct Document {
    pub version: Version,
    pub avatar: Avatar,
}

impl Document {
    pub fn parse(document: &KdlDocument, nul_span: &SourceSpan) -> Result<Document> {
        let nodes = document.nodes();

        // Detect version
        let Some(version_node) = nodes.get(0) else {
            return Err(DeclError::new(&nul_span, DeclErrorKind::NodeNotFound(NODE_NAME_VERSION)));
        };
        let (_, entries, _) = deconstruct_node(version_node, Some(NODE_NAME_VERSION), Some(false))?;
        let version_text = entries.get_argument(0, "version")?;
        let version = Version::parse(version_text).map_err(|e| {
            DeclError::new(version_node.name().span(), DeclErrorKind::VersionError(e))
        })?;

        // Other nodes
        let mut avatar = None;
        for node in &nodes[1..] {
            let node_name = node.name().value();
            match node_name {
                NODE_NAME_AVATAR => match avatar {
                    None => {
                        avatar = Some(Avatar::parse(node)?);
                    }
                    _ => {
                        return Err(DeclError::new(
                            node.name().span(),
                            DeclErrorKind::DuplicateNodeFound,
                        ))
                    }
                },
                _ => {
                    return Err(DeclError::new(
                        node.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ))
                }
            }
        }

        let Some(avatar) = avatar else {
            return Err(DeclError::new(&nul_span, DeclErrorKind::NodeNotFound(NODE_NAME_VERSION)));
        };
        Ok(Document { version, avatar })
    }
}

/// Avatar descriptor. It should has specific structure like below:
#[derive(Debug, Clone)]
pub struct Avatar {
    pub name: String,
    pub animations_blocks: Vec<Animations>,
    pub drivers_blocks: Vec<Drivers>,
    pub parameters_blocks: Vec<Parameters>,
    pub menu_blocks: Vec<Menu>,
}

impl Avatar {
    pub fn parse(node: &KdlNode) -> Result<Self> {
        let (_, entries, children) = deconstruct_node(node, Some(NODE_NAME_AVATAR), Some(true))?;

        let name = entries.get_argument(0, "name")?;
        let mut animations_blocks = vec![];
        let mut drivers_blocks = vec![];
        let mut parameters_blocks = vec![];
        let mut menu_blocks = vec![];

        for child in children {
            let child_name = child.name().value();
            match child_name {
                NODE_NAME_ANIMATIONS => animations_blocks.push(Animations::parse(child)?),
                NODE_NAME_DRIVERS => drivers_blocks.push(Drivers::parse(child)?),
                NODE_NAME_PARAMETERS => parameters_blocks.push(Parameters::parse(child)?),
                NODE_NAME_MENU => menu_blocks.push(Menu::parse(child)?),
                _ => {
                    return Err(DeclError::new(
                        child.name().span(),
                        DeclErrorKind::InvalidNodeDetected,
                    ))
                }
            }
        }

        Ok(Avatar {
            name,
            animations_blocks,
            drivers_blocks,
            parameters_blocks,
            menu_blocks,
        })
    }
}