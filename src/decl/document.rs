use crate::decl::{
    avatar::{Avatar, NODE_NAME_AVATAR},
    entry::{get_argument, split_entries},
    DeclError, FromNodeExt,
};

use kdl::KdlDocument;

pub const NODE_NAME_VERSION: &str = "version";

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
