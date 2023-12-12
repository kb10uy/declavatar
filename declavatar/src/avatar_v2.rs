pub mod data;
pub mod logger;
pub mod transformer;

use crate::{
    avatar_v2::{
        data::avatar::Avatar,
        logger::{Logger, Severity},
        transformer::compile_avatar,
    },
    decl_v2::data::avatar::DeclAvatar,
};

pub struct TransformResult {
    pub avatar: Option<Avatar>,
    pub logs: Vec<(Severity, String)>,
}

pub fn transform_avatar(avatar: DeclAvatar) -> TransformResult {
    let mut logger = Logger::new();
    let avatar = compile_avatar(&mut logger, avatar);
    let logs = logger.logs();

    TransformResult { avatar, logs }
}
