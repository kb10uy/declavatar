pub mod data;
pub mod logger;
pub mod transformer;

use crate::{
    avatar_v2::{data::avatar::Avatar, transformer::compile_avatar},
    decl_v2::data::avatar::DeclAvatar,
    log::{Logger, SerializedLog},
};

pub struct TransformResult {
    pub avatar: Option<Avatar>,
    pub logs: Vec<SerializedLog>,
}

pub fn transform_avatar(avatar: DeclAvatar) -> TransformResult {
    let logger = Logger::new();
    let avatar = compile_avatar(&logger, avatar);
    let logs = logger.serialize_logs();

    TransformResult { avatar, logs }
}
