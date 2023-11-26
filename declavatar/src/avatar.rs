pub mod data;
pub mod error;

mod transformer;

pub use self::transformer::LogLevel;

use crate::{
    avatar::{
        data::Avatar,
        transformer::{compile_avatar, Context},
    },
    decl::data::Avatar as DeclAvatar,
};

pub struct TransformResult {
    pub avatar: Option<Avatar>,
    pub logs: Vec<(LogLevel, String)>,
}

pub fn transform_avatar(avatar: DeclAvatar) -> TransformResult {
    let mut ctx = Context::new();
    let avatar = compile_avatar(&mut ctx, avatar);
    let logs = ctx.into_logs();

    TransformResult { avatar, logs }
}
