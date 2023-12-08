pub mod data;
pub mod logging;
pub mod transformer;

use crate::{
    avatar_v2::{
        data::avatar::Avatar,
        logging::{LogLevel, LoggingContext},
        transformer::compile_avatar,
    },
    decl_v2::data::avatar::DeclAvatar,
};

pub struct TransformResult {
    pub avatar: Option<Avatar>,
    pub logs: Vec<(LogLevel, String)>,
}

pub fn transform_avatar(avatar: DeclAvatar) -> TransformResult {
    let mut ctx = LoggingContext::new();
    let avatar = compile_avatar(&mut ctx, avatar);
    let logs = ctx.into_logs();

    TransformResult { avatar, logs }
}
