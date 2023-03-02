use crate::{
    compiler::Compile,
    decl::{
        compiler::{
            animations::ForAnimations, deconstruct_node, drivers::ForDrivers, menu::ForMenu,
            parameters::ForParameters, DeclCompiler,
        },
        data::{Avatar, Document},
        DeclError, DeclErrorKind, Result,
    },
};

use kdl::{KdlDocument, KdlNode};
use miette::{SourceOffset, SourceSpan};
use semver::Version;

pub const NODE_NAME_VERSION: &str = "version";
pub const NODE_NAME_AVATAR: &str = "avatar";

struct ForShapeGroup;
impl Compile<KdlDocument> for DeclCompiler {
    type Output = Document;

    fn compile(&mut self, document: KdlDocument) -> Result<Document> {
        let nul_span = SourceSpan::new(
            SourceOffset::from_location("", 1, 1),
            SourceOffset::from_location("", 1, 1),
        );
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
                        avatar = Some(self.compile((ForAvatar, node))?);
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

struct ForAvatar;
impl Compile<(ForAvatar, &KdlNode)> for DeclCompiler {
    type Output = Avatar;

    fn compile(&mut self, (_, node): (ForAvatar, &KdlNode)) -> Result<Avatar> {
        let (_, entries, children) = deconstruct_node(node, Some(NODE_NAME_AVATAR), Some(true))?;

        let name = entries.get_argument(0, "name")?;
        let mut animations_blocks = vec![];
        let mut drivers_blocks = vec![];
        let mut parameters_blocks = vec![];
        let mut menu_blocks = vec![];

        for child in children {
            let child_name = child.name().value();
            match child_name {
                NODE_NAME_ANIMATIONS => {
                    animations_blocks.push(self.compile((ForAnimations, child))?)
                }
                NODE_NAME_DRIVERS => drivers_blocks.push(self.compile((ForDrivers, child))?),
                NODE_NAME_PARAMETERS => {
                    parameters_blocks.push(self.compile((ForParameters, child))?)
                }
                NODE_NAME_MENU => menu_blocks.push(self.compile((ForMenu, child))?),
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
