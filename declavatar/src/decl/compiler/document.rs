use crate::decl::{
    compiler::{
        animations::{compile_animations, NODE_NAME_ANIMATIONS},
        assets::{compile_assets, NODE_NAME_ASSETS},
        deconstruct_node,
        drivers::{compile_drivers, NODE_NAME_DRIVERS},
        menu::{compile_menu, NODE_NAME_MENU},
        parameters::{compile_parameters, NODE_NAME_PARAMETERS},
    },
    data::{Avatar, Document},
    DeclError, DeclErrorKind, Result,
};

use kdl::{KdlDocument, KdlNode};
use miette::{SourceOffset, SourceSpan};
use semver::Version;

pub const NODE_NAME_VERSION: &str = "version";
pub const NODE_NAME_AVATAR: &str = "avatar";

pub fn compile_document(document: KdlDocument) -> Result<Document> {
    let nul_span = SourceSpan::new(
        SourceOffset::from_location("", 1, 1),
        SourceOffset::from_location("", 1, 1),
    );
    let nodes = document.nodes();

    // Detect version
    let Some(version_node) = nodes.get(0) else {
        return Err(DeclError::new(
            &nul_span,
            DeclErrorKind::NodeNotFound(NODE_NAME_VERSION),
        ));
    };
    let (_, entries, _) = deconstruct_node(version_node, Some(NODE_NAME_VERSION), Some(false))?;
    let version_text = entries.get_argument(0, "version")?;
    let version = Version::parse(version_text)
        .map_err(|e| DeclError::new(version_node.name().span(), DeclErrorKind::VersionError(e)))?;

    // Other nodes
    let mut avatar = None;
    for node in &nodes[1..] {
        let node_name = node.name().value();
        match node_name {
            NODE_NAME_AVATAR => match avatar {
                None => {
                    avatar = Some(compile_avatar(node)?);
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
        return Err(DeclError::new(
            &nul_span,
            DeclErrorKind::NodeNotFound(NODE_NAME_VERSION),
        ));
    };
    Ok(Document { version, avatar })
}

fn compile_avatar(node: &KdlNode) -> Result<Avatar> {
    let (_, entries, children) = deconstruct_node(node, Some(NODE_NAME_AVATAR), Some(true))?;

    let name = entries.get_argument(0, "name")?;
    let mut animations_blocks = vec![];
    let mut drivers_blocks = vec![];
    let mut parameters_blocks = vec![];
    let mut assets_blocks = vec![];
    let mut menu_blocks = vec![];

    for child in children {
        let child_name = child.name().value();
        match child_name {
            NODE_NAME_PARAMETERS => parameters_blocks.push(compile_parameters(child)?),
            NODE_NAME_ASSETS => assets_blocks.push(compile_assets(child)?),
            NODE_NAME_ANIMATIONS => animations_blocks.push(compile_animations(child)?),
            NODE_NAME_DRIVERS => drivers_blocks.push(compile_drivers(child)?),
            NODE_NAME_MENU => menu_blocks.push(compile_menu(child)?),
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
        parameters_blocks,
        assets_blocks,
        animations_blocks,
        drivers_blocks,
        menu_blocks,
    })
}
